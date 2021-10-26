#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use jsonschema::JSONSchema;
use serde_json::from_str;
use std::str::from_utf8;

use napi::{
  CallContext, Env, Error, JsBoolean, JsBuffer, JsBufferValue, JsNull, JsObject, JsString,
  JsUndefined, Ref, Result, Status, Task,
};

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct AsyncIsValidTask {
  input: Ref<JsBufferValue>,
  schema: Ref<JsBufferValue>,
}

impl Task for AsyncIsValidTask {
  type Output = bool;
  type JsValue = JsBoolean;

  fn compute(&mut self) -> Result<Self::Output> {
    let input = from_str(
      from_utf8(&self.input).map_err(|e| Error::new(Status::StringExpected, format!("{}", e)))?,
    )?;
    let schema = from_str(
      from_utf8(&self.schema).map_err(|e| Error::new(Status::StringExpected, format!("{}", e)))?,
    )?;
    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    Ok(compiled.is_valid(&input))
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    self.input.unref(env)?;
    self.schema.unref(env)?;
    env.get_boolean(output)
  }

  fn reject(self, env: Env, err: Error) -> Result<Self::JsValue> {
    self.input.unref(env)?;
    self.schema.unref(env)?;
    Err(err)
  }
}

struct AsyncValidateTask {
  input: Ref<JsBufferValue>,
  schema: Ref<JsBufferValue>,
}

impl Task for AsyncValidateTask {
  type Output = ();
  type JsValue = JsNull;

  fn compute(&mut self) -> Result<Self::Output> {
    let input = from_str(
      from_utf8(&self.input).map_err(|e| Error::new(Status::StringExpected, format!("{}", e)))?,
    )?;
    let schema = from_str(
      from_utf8(&self.schema).map_err(|e| Error::new(Status::StringExpected, format!("{}", e)))?,
    )?;

    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    let result = compiled.validate(&input);
    if let Err(errors) = result {
      let mut error_message = String::from("");
      for error in errors {
        error_message += &format!(
          "Validation error: {}; Instance path: {}; \n",
          error, error.instance_path
        );
      }
      return Err(Error::new(Status::GenericFailure, error_message));
    }
    Ok(())
  }

  fn resolve(self, env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    self.input.unref(env)?;
    self.schema.unref(env)?;
    env.get_null()
  }

  fn reject(self, env: Env, err: Error) -> Result<Self::JsValue> {
    self.input.unref(env)?;
    self.schema.unref(env)?;
    Err(err)
  }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("isValidSync", is_valid_sync)?;
  exports.create_named_method("isValid", is_valid)?;
  exports.create_named_method("validateSync", validate_sync)?;
  exports.create_named_method("validate", validate)?;
  Ok(())
}

#[js_function(2)]
fn is_valid_sync(ctx: CallContext) -> Result<JsBoolean> {
  let input = ctx.get::<JsString>(0)?.into_utf8()?;
  let schema = ctx.get::<JsString>(1)?.into_utf8()?;
  let input_json = from_str(input.as_str()?)?;
  let schema_json = from_str(schema.as_str()?)?;
  let compiled = JSONSchema::compile(&schema_json).expect("A valid schema");
  ctx.env.get_boolean(compiled.is_valid(&input_json))
}

#[js_function(2)]
fn validate_sync(ctx: CallContext) -> Result<JsUndefined> {
  let input = ctx.get::<JsString>(0)?.into_utf8()?;
  let schema = ctx.get::<JsString>(1)?.into_utf8()?;
  let input_json = from_str(input.as_str()?)?;
  let schema_json = from_str(schema.as_str()?)?;
  let compiled = JSONSchema::compile(&schema_json).expect("A valid schema");
  let result = compiled.validate(&input_json);
  if let Err(errors) = result {
    let mut error_message = String::from("");
    for error in errors {
      error_message += &format!(
        "Validation error: {}; Instance path: {}; \n",
        error, error.instance_path
      );
    }
    return Err(Error::new(Status::GenericFailure, error_message));
  }
  ctx.env.get_undefined()
}

#[js_function(2)]
fn is_valid(ctx: CallContext) -> Result<JsObject> {
  let input = ctx.get::<JsBuffer>(0)?;
  let schema = ctx.get::<JsBuffer>(1)?;
  let task = AsyncIsValidTask {
    input: input.into_ref()?,
    schema: schema.into_ref()?,
  };
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}

#[js_function(2)]
fn validate(ctx: CallContext) -> Result<JsObject> {
  let input = ctx.get::<JsBuffer>(0)?;
  let schema = ctx.get::<JsBuffer>(1)?;
  let task = AsyncValidateTask {
    input: input.into_ref()?,
    schema: schema.into_ref()?,
  };
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}
