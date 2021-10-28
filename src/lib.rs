#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use jsonschema::JSONSchema;
use serde_json::from_str;

use napi::{
  CallContext, Error, JsBoolean, JsFunction, JsObject, JsString, JsUndefined, Result, Status,
};

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("compile", compile)?;
  exports.create_named_method("isValid", is_valid)?;
  exports.create_named_method("validate", validate)?;
  Ok(())
}

#[js_function(1)]
fn compile(ctx: CallContext) -> Result<JsFunction> {
  let arg0 = ctx.get::<JsString>(0)?.into_utf8()?;
  let schema = from_str(arg0.as_str()?)?;
  let compiled =
    JSONSchema::compile(&schema).map_err(|e| Error::new(Status::InvalidArg, format!("{}", e)))?;
  // let mut this: JsObject = ctx.this_unchecked();
  // ctx.env.wrap(&mut this, compiled)?;
  // ctx.env.get_undefined()
  ctx.env.create_function_from_closure("isValid", move |ctx| {
    let arg0 = ctx.get::<JsString>(0)?.into_utf8()?;
    let input = from_str(arg0.as_str()?)?;
    ctx.env.get_boolean(compiled.is_valid(&input))
  })
}

#[js_function(1)]
fn is_valid(ctx: CallContext) -> Result<JsBoolean> {
  let arg0 = ctx.get::<JsString>(0)?.into_utf8()?;
  let input = from_str(arg0.as_str()?)?;
  let mut this: JsObject = ctx.this()?;
  let compiled: &mut JSONSchema = ctx.env.unwrap(&mut this)?;
  ctx.env.get_boolean(compiled.is_valid(&input))
}

#[js_function(1)]
fn validate(ctx: CallContext) -> Result<JsUndefined> {
  let arg0 = ctx.get::<JsString>(0)?.into_utf8()?;
  let input = from_str(arg0.as_str()?)?;
  let mut this: JsObject = ctx.this()?;
  let compiled: &mut JSONSchema = ctx.env.unwrap(&mut this)?;
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
  ctx.env.get_undefined()
}
