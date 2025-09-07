//@/src/values/primitive/ShortLongArrayImpl.ts
import { ValueType } from "@/api/ValueType";
import { LongArray } from "../abstract/LongArray";
import { ArrayEquals } from "../ArrayEquals";

/**
 * Implementation of LongArray that stores values as shorts.
 */
export class DefShortLongArray implements LongArray {
  private readonly value: Int16Array;

  /**
   * Creates a new short-backed long array.
   *
   * @param value Array of short values
   */
  constructor(value: Int16Array) {
    this.value = value;
  }

  /**
   * Returns a copy of this array as a number array.
   *
   * @returns Array of number values
   */
  public longArrayValue(): number[] {
    const copy = new Array<number>(this.value.length);
    for (let i = 0; i < this.value.length; i++) {
      copy[i] = this.value[i];
    }
    return copy;
  }

  /**
   * Returns the long value at the specified index.
   *
   * @param idx Index to access
   * @returns Long value at that index
   */
  public longValue(idx: number): number {
    return this.value[idx];
  }

  /**
   * Returns the length of this array.
   *
   * @returns Array length
   */
  public length(): number {
    return this.value.length;
  }

  /**
   * Returns the value type of this array.
   *
   * @returns Value type (LONG_ARRAY)
   */
  public type(): ValueType {
    return ValueType.LONG_ARRAY;
  }

  /**
   * Returns the underlying short array.
   *
   * @returns Short array
   */
  public asObject(): Int16Array {
    return this.value;
  }

  /**
   * Compares this array with another object for equality.
   *
   * @param o Object to compare with
   * @returns true if objects are equal
   */
  public equals(o: unknown): boolean {
    return (
      o != null && ArrayEquals.shortAndObject(this.value, (o as any).value)
    );
  }

  /**
   * Compares this array with a byte array for equality.
   *
   * @param o Byte array to compare with
   * @returns true if arrays are equal
   */
  public equalsBytes(o: Uint8Array): boolean {
    return ArrayEquals.byteAndShort(o, this.value);
  }

  /**
   * Compares this array with a short array for equality.
   *
   * @param o Short array to compare with
   * @returns true if arrays are equal
   */
  public equalsShorts(o: Int16Array): boolean {
    if (this.value.length !== o.length) return false;

    for (let i = 0; i < this.value.length; i++) {
      if (this.value[i] !== o[i]) return false;
    }

    return true;
  }

  /**
   * Compares this array with an int array for equality.
   *
   * @param o Int array to compare with
   * @returns true if arrays are equal
   */
  public equalsInts(o: Int32Array): boolean {
    return ArrayEquals.shortAndInt(this.value, o);
  }

  /**
   * Compares this array with a long array for equality.
   *
   * @param other Long array to compare with
   * @returns true if arrays are equal
   */
  public equalsLongs(other: number[]): boolean {
    return ArrayEquals.shortAndLong(this.value, other);
  }

  /**
   * Compares this array with a float array for equality.
   *
   * @param o Float array to compare with
   * @returns true if arrays are equal
   */
  public equalsFloats(o: Float32Array): boolean {
    return ArrayEquals.shortAndFloat(this.value, o);
  }

  /**
   * Compares this array with a double array for equality.
   *
   * @param o Double array to compare with
   * @returns true if arrays are equal
   */
  public equalsDoubles(o: Float64Array): boolean {
    return ArrayEquals.shortAndDouble(this.value, o);
  }

  /**
   * Returns a hash code for this array.
   *
   * @returns Hash code
   */
  public hashCode(): number {
    let result = 1;
    for (let i = 0; i < this.value.length; i++) {
      result = 31 * result + this.value[i];
    }
    return result;
  }

  /**
   * Returns a string representation of this array.
   *
   * @returns String representation
   */
  public toString(): string {
    return `LongArray[${Array.from(this.value).join(", ")}]`;
  }
}
