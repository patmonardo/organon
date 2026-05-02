# Spans

A span is a stable range into a source document.

---

## Doctrine

Spans are how marks remain accountable to source material. A mark without a span or source reference is hard to audit.

---

## Shape

```text
source_id | document_id | start | end | label | provenance
```

---

## Review Rule

If an annotation points into text, it should carry a span. If it cannot, document why.
