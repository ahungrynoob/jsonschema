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
