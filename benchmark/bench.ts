import Ajv from 'ajv'
import b from 'benny'

import { validateSync } from '../index'

const fooObject = {
  foo: 1,
  bar: 'abc',
}
const foo = JSON.stringify(fooObject)

const fooSchemaObject = {
  type: 'object',
  properties: {
    foo: { type: 'integer' },
    bar: { type: 'string' },
  },
  required: ['foo'],
  additionalProperties: false,
}
const fooSchema = JSON.stringify(fooSchemaObject)

async function run() {
  await b.suite(
    'Validate Sync',

    b.add('Native validateSync', () => {
      validateSync(foo, fooSchema)
    }),

    b.add('JavaScript validateSync', () => {
      const ajv = new Ajv()
      const validate = ajv.compile(fooSchemaObject)
      validate(fooObject)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
