import type { DialecticIR } from '@schema/dialectic';
import { qualityIR } from './quality-ir';

// Compatibility export: use `qualityIR` (Dialectical States format).
export const integratedTopicMapIR: DialecticIR = qualityIR;
