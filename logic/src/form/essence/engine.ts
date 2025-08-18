// EssenceEngine — thin alias over ShapeEngine (ADR-0002 Essence -> Shape)
// This reduces naming confusion by exposing the ShapeEngine under the
// canonical "essence" namespace for container engines.

import { ShapeEngine } from '../shape/engine';

export class EssenceEngine extends ShapeEngine {}

export default EssenceEngine;
