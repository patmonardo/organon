import { z } from 'zod';

/**
 * FormDB persistence-layer props stored on Neo4j records.
 *
 * These are engine-level props (transport/persistence metadata),
 * not KnowledgeGraph semantic root-shape fields.
 *
 * Values are intentionally JSON strings to mirror current persistence behavior.
 */
export const FormDbPropsSchema = z
  .object({
    signatureJson: z.string().optional(),
    facetJson: z.string().optional(),
    facetsJson: z.string().optional(),
  })
  .catchall(z.string());

export type FormDbProps = z.infer<typeof FormDbPropsSchema>;
