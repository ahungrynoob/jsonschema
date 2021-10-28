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

const correctData = {
  foo: 1,
  bar: 'abc',
}

const exceptionData = {
  foo: 'abc',
  bar: 1,
}

test('isValid function from native code', (t) => {
  const compiled = compile(schema)
  t.assert(compiled(correctData))
  t.assert(!compiled(exceptionData))
})

// test('validate function from native code', (t) => {
//   const compiled = compile(schema)
//   t.notThrows(() => compiled.validate(correctData))
//   t.throws(() => compiled.validate(exceptionData), {
//     message: `Validation error: 1 is not of type "string"; Instance path: /bar; \nValidation error: "abc" is not of type "integer"; Instance path: /foo; \n`,
//   })
// })
