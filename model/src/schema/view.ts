/**
 * View Schema: Representation–Perspective (Active Essence)
 * -------------------------------------------------------
 * In the ORGANON dialectic, View is the advancement of Essence into activity:
 *   - Representation: The concrete rendering or display of a Model's state (Pratyaya)
 *   - Perspective: The vantage point or filter through which the Model is viewed (Drishti)
 * This schema encodes View as the synthesis of Representation and Perspective—
 * an "Active Essence" that mediates between Model and Controller.
 *
 * Philosophical Note:
 * This structure resonates with key concepts in Yoga and Abhidharma:
 *   - Pratyaya (Representation): The content or appearance presented to awareness
 *   - Drishti (Perspective): The standpoint, lens, or mode of seeing
 * View thus becomes the locus of mediation, context, and meaning.
 */

import { z } from 'zod';

export const RepresentationSchema = z.object({
  type: z.string(), // e.g., 'table', 'graph', 'text', etc.
  data: z.any(),    // the rendered data or content
  // Add more fields as needed for specific representations
});

export const PerspectiveSchema = z.object({
  viewpoint: z.string(), // e.g., 'user', 'system', 'admin', etc.
  filters: z.record(z.string(), z.any()).optional(), // e.g., { status: 'active' }
  // Add more fields as needed for specific perspectives
});

export const ViewSchema = z.object({
  representation: RepresentationSchema,
  perspective: PerspectiveSchema,
  // Optionally, add metadata or context fields
}).catchall(z.any());
