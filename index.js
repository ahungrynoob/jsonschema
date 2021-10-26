const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'jsonschema' means native addon name is `jsonschema`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `jsonschema.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@node-rs/jsonschema-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'jsonschema', '@node-rs/jsonschema')
