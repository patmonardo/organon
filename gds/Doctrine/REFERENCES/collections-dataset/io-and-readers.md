# IO and Readers

Readers bring external material into the Dataset world.

---

## Doctrine

IO is Stage 1 support. A reader fetches or loads a source. Observation begins only after the material is parsed, retained, derived, or marked.

---

## Reader Duties

1. Load deterministically
2. Preserve source identity
3. Report errors clearly
4. Avoid hidden semantic interpretation

---

## Review Rule

A reader should say what it loaded and where it came from. It should not silently decide what the material means.
