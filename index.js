const { loadBinding } = require('@node-rs/helper')

const { compile: _compile } = loadBinding(__dirname, 'jsonschema', '@node-rs/jsonschema')

module.exports.compile = function (schema) {
  return _compile(JSON.stringify(schema))
}
