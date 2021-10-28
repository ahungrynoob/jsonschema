#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use jsonschema::{CompilationOptions, Draft, JSONSchema};
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
  let compiled = JSONSchema::options()
    .with_draft(Draft::Draft7)
    .compile(&schema)
    .map_err(|e| Error::new(Status::InvalidArg, format!("{}", e)))?;
  ctx.env.create_function_from_closure("isValid", move |ctx| {
    let arg0 = ctx.get::<JsString>(0)?.into_utf8()?;
    let input = from_str(arg0.as_str()?)?;
    ctx.env.get_boolean(compiled.is_valid(&input))
  })
}
