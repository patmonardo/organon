import { z } from 'zod'

export const NumSchema = z.object({ kind: z.literal('num'), value: z.number() })
export const StrSchema = z.object({ kind: z.literal('str'), value: z.string() })
export const SymSchema = z.object({ kind: z.literal('sym'), name: z.string() })
export const VarSchema = z.object({ kind: z.literal('var'), name: z.string() })

export const TermSchema = z.union([NumSchema, StrSchema, SymSchema, VarSchema])

export type Num = z.infer<typeof NumSchema>
export type Str = z.infer<typeof StrSchema>
export type Sym = z.infer<typeof SymSchema>
export type Var = z.infer<typeof VarSchema>
export type Term = z.infer<typeof TermSchema>
