#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use jsonschema::{Draft, JSONSchema};
use serde_json::from_str;
use std::convert::TryInto;

use napi::{CallContext, Env, JsBoolean, JsNumber, JsObject, JsString, Result, Task};

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct AsyncTask(u32);

impl Task for AsyncTask {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_millis(self.0 as u64));
    Ok(self.0 * 2)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output)
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
fn validate_sync(ctx: CallContext) -> Result<JsNumber> {
  let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;

  ctx.env.create_uint32(argument + 100)
}

#[js_function(2)]
fn is_valid(ctx: CallContext) -> Result<JsObject> {
  let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let task = AsyncTask(argument);
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}

#[js_function(2)]
fn validate(ctx: CallContext) -> Result<JsObject> {
  let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let task = AsyncTask(argument);
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}
