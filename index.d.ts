export declare class JSONSchema {
  isValid(input: any): boolean
  validate(input: any): void
}

export const compile: (schema: any) => (input: any) => boolean
