# Exemplar 031 - Shell Model First

**Source file**: `gds/examples/shell_model_first.rs`
**Fixture root**: `gds/fixtures/collections/shell/shell_model_first/`
**Arc position**: Shell-first entry into the Model moment of `Model:Feature::Plan`
**Descends into**: [027 - Model:Feature::Plan as Essence Middle](../dataset/027-model-feature-plan-middle.md)

---

## Principle

The Shell thread should begin simply. For now, Shell means the internal Rust DSL as it becomes useful at the Dataset middle. The first Shell-first question is not the whole platform. It is only this: how does Shell name a framed body as Model?

---

## What This Example Does

It stages the first moment only:

1. Creates a small framed body
2. Enters Shell through `DatasetDataFrameNameSpace::into_shell_with_program_features(...)`
3. Reads `GdsShell::model_moment_knowledge()`
4. Persists a Shell Model report
5. Points the reader down into `dataset_model_feature_plan.rs`, where Feature and Plan are expanded

This is deliberately narrower than `shell_compute_protocol.rs`. It does not try to show Corpus, SemDataset, PureForm return, or the whole help catalog.

---

## Doctrinal Method

The order is Shell first, Dataset second:

```text
Shell Model moment
  -> Dataset Model:Feature::Plan
  -> dataset_model_feature_plan.rs
```

The Shell knows the Model moment as a reflective address. Dataset then descends into the actual middle, where Model, Feature, and Plan are no longer merely named but materially expanded.

---

## Student Notes

- Stay with Model first. Do not rush into Feature and Plan until the Shell-side Model report is stable.
- AI assistance is presupposed only as a reader of the Shell Model report. It is not yet an autonomous platform controller.
- The help resource remains `ShellHelp`, but this exemplar filters it to the Model moment.
