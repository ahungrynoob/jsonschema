# `@node-rs/jsonschema`

![https://github.com/ahungrynoob/jsonschema/actions](https://github.com/ahungrynoob/jsonschema/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/jsonschema.svg?sanitize=true)

> A node package based on jsonschema-rs for performing JSON schema validation.

## Bench
**[ajv](https://github.com/ajv-validator/ajv) is much faster than this lib.**
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

  @node-rs/jsonschema::validate:
    2 642 863 ops/s, ±1.42%    | slowest, 92.86% slower

  ajv::validate:
    36 997 776 ops/s, ±0.46%   | fastest

Finished 2 cases!
  Fastest: ajv::validate
  Slowest: @node-rs/jsonschema::validate
```

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
const { compile } = require("@node-rs/jsonschema");

const schema = {
  type: 'object',
  properties: {
    foo: { type: 'integer' },
    bar: { type: 'string' },
  },
  required: ['foo'],
  additionalProperties: false,
};

const input = JSON.stringify({
  foo: 1,
  bar: 'abc',
})

const exceptionInput = JSON.stringify({
  foo: 'abc',
  bar: 1,
})

const validator = compile(schema);

// check whether the input meet schema
const result = validator(input);
console.log(result); // true

const result = validator(exceptionInput);
console.log(result); // false
```

## API
```typescript
export declare class JSONSchema {
  isValid(input: any): boolean
}

export const compile: (schema: any) => (input: string) => boolean
```