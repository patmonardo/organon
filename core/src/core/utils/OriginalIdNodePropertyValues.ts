import { IdMap } from "@/api";
import { LongNodePropertyValues } from "@/api/properties";

/**
 * A node property values implementation where the property value
 * is the original ID of the node.
 */
export class OriginalIdNodePropertyValues implements LongNodePropertyValues {
  private readonly toOriginalNodeId: (nodeId: number) => number;
  private readonly _nodeCount: number;

  /**
   * Creates a new original ID node property values.
   *
   * @param idMap ID map used to map node IDs to their original IDs
   */
  constructor(idMap: IdMap);

  /**
   * Creates a new original ID node property values.
   *
   * @param toOriginalNodeId Function that maps node IDs to their original IDs
   * @param nodeCount The number of nodes
   */
  constructor(toOriginalNodeId: (nodeId: number) => number, nodeCount: number);

  /**
   * Constructor implementation.
   *
   * @param idMapOrToOriginalNodeId ID map or mapping function
   * @param nodeCount Optional node count (required if first param is a function)
   */
  constructor(
    idMapOrToOriginalNodeId: IdMap | ((nodeId: number) => number),
    nodeCount?: number
  ) {
    if (typeof idMapOrToOriginalNodeId === "function") {
      this.toOriginalNodeId = idMapOrToOriginalNodeId;
      this._nodeCount = nodeCount!;
    } else {
      this.toOriginalNodeId = (nodeId) =>
        idMapOrToOriginalNodeId.toOriginalNodeId(nodeId);
      this._nodeCount = idMapOrToOriginalNodeId.nodeCount();
    }
  }

  /**
   * Returns the original ID for the given node ID.
   *
   * @param nodeId The node ID
   * @returns The original ID
   */
  public longValue(nodeId: number): number {
    return this.toOriginalNodeId(nodeId);
  }

  /**
   * Returns an empty optional since there's no defined maximum value.
   *
   * @returns An empty optional
   */
  public getMaxLongPropertyValue(): number {
    return Number.MAX_SAFE_INTEGER;
  }

  /**
   * Returns the number of nodes.
   *
   * @returns Node count
   */
  public nodeCount(): number {
    return this._nodeCount;
  }
}
