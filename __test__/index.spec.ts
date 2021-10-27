import test from 'ava'

import { isValidSync, validateSync, isValid, validate } from '../index'

const schema = JSON.stringify({
  type: 'object',
  properties: {
    foo: { type: 'integer' },
    bar: { type: 'string' },
  },
  required: ['foo'],
  additionalProperties: false,
})

const correctData = JSON.stringify({
  foo: 1,
  bar: 'abc',
})

const exceptionData = JSON.stringify({
  foo: 'abc',
  bar: 1,
})

test('isValidSync function from native code', (t) => {
  t.assert(isValidSync(correctData, schema))
  t.assert(!isValidSync(exceptionData, schema))
})

test('validateSync function from native code', (t) => {
  t.notThrows(() => validateSync(correctData, schema))
  t.throws(() => validateSync(exceptionData, schema))
})

test('isValid async function from native code', async (t) => {
  const trueValue = await isValid(correctData, schema)
  t.assert(trueValue)
  const falseValue = await isValid(exceptionData, schema)
  t.assert(!falseValue)
})

test('validate async function from native code', async (t) => {
  await t.notThrowsAsync(validate(correctData, schema))
  const error = await t.throwsAsync(validate(exceptionData, schema))
  t.is(
    error.message,
    `Validation error: 1 is not of type "string"; Instance path: /bar; \nValidation error: "abc" is not of type "integer"; Instance path: /foo; \n`,
  )
})
