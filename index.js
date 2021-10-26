const { loadBinding } = require('@node-rs/helper')

const {
  isValid: _isValid,
  validate: _validate,
  isValidSync: _isValidSync,
  validateSync: _validateSync,
} = loadBinding(__dirname, 'jsonschema', '@node-rs/jsonschema')

module.exports.isValid = function isValid(input, schema) {
  return _isValid(Buffer.from(input), Buffer.from(schema))
}

module.exports.isValidSync = function isValidSync(input, schema) {
  return _isValidSync(input, schema)
}

module.exports.validate = function validate(input, schema) {
  return _validate(Buffer.from(input), Buffer.from(schema))
}

module.exports.validateSync = function uncompress(input, schema) {
  return _validateSync(input, schema)
}
