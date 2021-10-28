const { loadBinding } = require('@node-rs/helper')

const {
  compile: _compile,
  isValid: _isValid,
  validate: _validate,
} = loadBinding(__dirname, 'jsonschema', '@node-rs/jsonschema')

module.exports.compile = function (schema) {
  // const obj = {
  //   _compile,
  //   isValid: (input) => _isValid.call(obj, JSON.stringify(input)),
  //   validate: (input) => _validate.call(obj, JSON.stringify(input)),
  // }
  // obj._compile(JSON.stringify(schema))
  // return obj
  return _compile(JSON.stringify(schema))
}
