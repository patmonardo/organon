# ZeroCopy: The Kernel-Agent Boundary Contract

This reference explains the ZeroCopy architectural requirement at the Absolute Reflection
moment and what it means for system design.

---

## The Problem

At Absolute Reflection (Moment 7), the agent must evaluate whether Principle conditions hold.
This requires inspecting the entity's shape data that the kernel has prepared.

Naive approach: serialize the shape to JSON, send it to the agent, the agent deserializes,
evaluates, sends back a result. This is expensive and unnecessary.

**ZeroCopy approach**: Both the kernel and the agent read the same memory representation of
the shape. No serialization. No copy. Just shared access.

---

## What ZeroCopy Means

ZeroCopy means:

1. **Single representation**: Form/Context/Morph shapes exist in a single serialization format
   that both Rust (kernel) and JavaScript (agent) can read directly.

2. **No marshaling at the boundary**: The kernel does not convert to JSON for the agent.
   The agent does not request data through a REST API. Both read the same bytes.

3. **Deterministic access patterns**: Both sides know the exact byte layout of the shape.
   They can compute offsets and read values directly.

4. **In-memory or shared storage**: The shape lives in memory that both processes can access
   (e.g., shared memory, memory-mapped file, or a common buffer).

---

## Why ZeroCopy at Absolute Reflection

Absolute Reflection is the critical moment because:

1. **It is the decision point**: Principle evaluation happens here. The decision cannot be
   stale or approximate. It must see the current, exact shape state.

2. **It is high-frequency**: Many entities pass through Absolute Reflection. Serialization
   overhead at this point becomes system-wide latency.

3. **It is the agent's only read**: This is where the agent touches the kernel's world.
   The access pattern must be direct and predictable.

4. **It enables correctness proofs**: If both sides read the same bytes in the same order,
   the evaluation is reproducible and auditable.

---

## Implementation Strategy

ZeroCopy requires careful design of the shape data structures. The key is that both Rust
and JavaScript must be able to read the same bytes.

### Option 1: Arrow Format (Recommended)

Apache Arrow provides a canonical columnar format readable by both Rust and JavaScript/WASM.

**Advantages**:
- Standard format, well-documented
- Polars (Rust) and nodejs-polars (JS) both support Arrow natively
- No custom serialization code
- Already aligned with the DataFrame backend

**Implementation**:
```rust
// Kernel side (Rust)
let form_shape: ArrowRecordBatch = /* encode Form/Context/Morph */;
let buffer = form_shape.to_bytes();

// Agent side (JS)
const formShape = RecordBatch.from(buffer);
const principleHolds = evaluatePrinciple(formShape);
```

### Option 2: Flatbuffers

Google Flatbuffers provides zero-copy serialization readable by C++, Java, JavaScript, and Rust.

**Advantages**:
- Designed for zero-copy access
- Both languages have first-class support
- Excellent for nested structures

### Option 3: Native Bindings (NAPI)

Direct Rust-to-JavaScript bindings via Node.js NAPI.

**Advantages**:
- Fastest possible access
- Direct pointer sharing

**Disadvantages**:
- Tight coupling between Rust and JS implementations
- Harder to test independently

---

## The Contract

Both the kernel and agent must agree on:

1. **Byte order** (little-endian or big-endian)
2. **Field layout** (which fields come first, what offsets)
3. **Type sizes** (how many bytes for an int, a float, a string)
4. **Version ID** (so both sides know they're reading the same format version)

This contract is enforced at compile time for the Rust kernel and at runtime for the
JavaScript agent (with type guards).

---

## What Gets Copied

Even with ZeroCopy, some data movement is necessary:

1. **Principle decision result**: The boolean result of Principle evaluation must be communicated
   back to the kernel. This is a single bit; acceptable.

2. **Concept attributes** (on success): Once Concept emerges, attributes are copied into the
   concept table. This is post-Principle, so the high-performance boundary is past.

3. **Judgment evaluations**: Later in the arc, Judgment and Syllogism work with concept tables
   that can be marshaled normally.

The ZeroCopy requirement is **local to the Absolute Reflection moment**, not system-wide.

---

## Verification

To verify ZeroCopy correctness:

1. **Byte-identical reads**: Run the same shape through the kernel's deserializer and the agent's
   deserializer. Verify they produce identical results.

2. **Reproducible evaluation**: Run Principle evaluation multiple times. The result must be
   deterministic.

3. **Audit trails**: Log the byte offset and value accessed at each Principle condition check.
   The log should match between runs.

---

## Why This Matters

ZeroCopy is not a performance optimization. It is an **architectural requirement** because:

1. It makes the kernel-agent boundary explicit and auditable
2. It forces both sides to speak in the same shape language
3. It prevents accidental format drift (if the bytes don't match, nothing works)
4. It enables the agent to evaluate conditions with direct access to the ground truth

A system that marshals everything to JSON can hide inconsistencies. A ZeroCopy system cannot.
The bytes must match or the whole thing fails. This is why ZeroCopy supports absolute correctness.

---

## Current State and Next Steps

**Current**: The GDS kernel and TS engines speak through FormDB shapes in form/program.rs.
The boundary is not yet ZeroCopy; it uses JSON marshaling.

**Next**: Redesign the shape serialization to use Arrow format or Flatbuffers. Make the
contract explicit. Verify that both sides read the same bytes.

**Payoff**: Once ZeroCopy is in place, the Absolute Reflection boundary becomes the
strongest part of the system. Both sides can trust the data absolutely.
