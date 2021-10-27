# `@node-rs/jsonschema`

![https://github.com/ahungrynoob/jsonschema/actions](https://github.com/ahungrynoob/jsonschema/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/jsonschema.svg?sanitize=true)

> A node package based on jsonschema-rs for performing JSON schema validation.

## Install

```
yarn add @node-rs/jsonschema
```

## Support matrix

| Operating Systems| node12 | node14 | node16 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## Usage
```javascript
const { isValidSync, validateSync, isValid, validate } = require("@node-rs/jsonschema");

const schema = JSON.stringify({
  type: 'object',
  properties: {
    foo: { type: 'integer' },
    bar: { type: 'string' },
  },
  required: ['foo'],
  additionalProperties: false,
})

const input = JSON.stringify({
  foo: 1,
  bar: 'abc',
})

const exceptionInput = JSON.stringify({
  foo: 'abc',
  bar: 1,
})

// check whether the input meet schema
const result = isValidSync(input, schema);
console.log(result); // true

try {
  validateSync(exceptionInput, schema);
}catch(e){
  // it will throw error if input doesn't meet schema
  console.log(e.message); // Validation error: 1 is not of type "string"; Instance path: /bar; \nValidation error: "abc" is not of type "integer"; Instance path: /foo; \n
}

// promise version of isValidSync
isValid(input, schema).then((result) => {
  console.log(result); // true
})

// promise version of validateSync
validate(input, schema).then(() => {
  console.log("feel good and input meet schema");
}).catch((e) => {
  // it will reject if input doesn't meet schema
  console.log(e.message); // Validation error: 1 is not of type "string"; Instance path: /bar; \nValidation error: "abc" is not of type "integer"; Instance path: /foo; \n
})
```

## API
```typescript
export const isValidSync: (input: string, schema: string) => boolean

export const validateSync: (input: string, schema: string) => void

export const isValid: (
  input: Buffer | string | ArrayBuffer | Uint8Array,
  schema: Buffer | string | ArrayBuffer | Uint8Array,
) => Promise<boolean>

export const validate: (
  input: Buffer | string | ArrayBuffer | Uint8Array,
  schema: Buffer | string | ArrayBuffer | Uint8Array,
) => Promise<null>
```

## Bench

### Hardware
```
Model Name:	MacBook Pro
Model Identifier:	MacBookPro16,1
Processor Name:	6-Core Intel Core i7
Processor Speed:	2.6 GHz
Number of Processors:	1
Total Number of Cores:	6
L2 Cache (per Core):	256 KB
L3 Cache:	12 MB
Hyper-Threading Technology:	Enabled
Memory:	32 GB
```
### Result
```
Running "Validate Sync" suite...
Progress: 100%

  @node-rs/jsonschema::validateSync:
    105 138 ops/s, ±2.04%   | fastest

  ajv::validateSync:
    178 ops/s, ±2.46%       | slowest, 99.83% slower

Finished 2 cases!
  Fastest: @node-rs/jsonschema::validateSync
  Slowest: ajv::validateSync
```