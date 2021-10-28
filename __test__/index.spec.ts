import test from 'ava'

import { compile } from '../index'

const schema = {
  type: 'object',
  properties: {
    foo: { type: 'integer' },
    bar: { type: 'string' },
  },
  required: ['foo'],
  additionalProperties: false,
}

const correctData = JSON.stringify({
  foo: 1,
  bar: 'abc',
})

const exceptionData = JSON.stringify({
  foo: 'abc',
  bar: 1,
})

test('isValid function from native code', (t) => {
  const compiled = compile(schema)
  t.assert(compiled(correctData))
  t.assert(!compiled(exceptionData))
})
