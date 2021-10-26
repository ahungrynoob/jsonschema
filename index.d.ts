export const isValidSync: (
  input: Buffer | string | ArrayBuffer | Uint8Array,
  schema: Buffer | string | ArrayBuffer | Uint8Array,
) => boolean

export const validateSync: (
  input: Buffer | string | ArrayBuffer | Uint8Array,
  schema: Buffer | string | ArrayBuffer | Uint8Array,
) => void

export const isValid: (
  input: Buffer | string | ArrayBuffer | Uint8Array,
  schema: Buffer | string | ArrayBuffer | Uint8Array,
) => Promise<boolean>

export const validate: (
  input: Buffer | string | ArrayBuffer | Uint8Array,
  schema: Buffer | string | ArrayBuffer | Uint8Array,
) => Promise<null>
