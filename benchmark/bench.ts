import Ajv from 'ajv'
import b from 'benny'

import { compile } from '../index'

const input = {
  foo: 1,
  bar: 'abc',
}

const schema = {
  type: 'object',
  properties: {
    foo: { type: 'integer' },
    bar: { type: 'string' },
  },
  required: ['foo'],
  additionalProperties: false,
}

const ajv = new Ajv()
const ajvValidator = ajv.compile(schema)
const nativeValidator = compile(schema)
const inputStr = JSON.stringify(input)

async function run() {
  await b.suite(
    'Validate Sync',

    b.add('@node-rs/jsonschema::validate', () => {
      nativeValidator(inputStr)
    }),

    b.add('ajv::validate', () => {
      ajvValidator(input)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
