export declare class JSONSchema {
  isValid(input: any): boolean
}

export const compile: (schema: any) => (input: string) => boolean
