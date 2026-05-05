# Shell External Program Grammar

The external `.gdsl` syntax is a Shell program artifact. It names sources,
observations, reflection paths, principles, concepts, judgments, syllogisms,
queries, and procedures, then lowers into the Shell-readable feature envelope.

---

## Canonical Form

```gdsl
module <qualified.name>;

use <module.path>;
source <kind> <name> : <format>(<path>);

appearance <name> from <source> {
  key <field>;
  retain <fields>;
  derive <mark> = <expr>;
}

reflection <name> for <appearance> {
  preserve being;
  stage identity;
  stage opposition;
  stage contradiction;
  stage ground;
  stage condition;
  culminate mark;
}

logogenesis <name> for <appearance> {
  from <reflection>;
  unfold <kind-or-law>;
}

mark <name> on <appearance> := <expr>;

principle <name> for <candidate> {
  require <condition>;
  unify concept, judgment, syllogism;
  infer <label> when <condition>;
}

concept <Name> from <appearance> {
  identity <field>;
  mark <mark-name>;
}

judgment <name> for <Concept> {
  infer <label> when <condition>;
}

syllogism <name> for <Concept> {
  middle <term>;
  conclude <label> when <condition>;
}

query <name> :=
  select <fields>
  from <Concept>
  where <condition>;

procedure <name> {
  emit <stage> dataset;
}
```

---

## Reading Rule

Read an external Shell program artifact as a scientific knowledge arc, not as a
data pipeline script.

- `source` asks: what is given?
- `appearance` asks: how does it appear?
- `reflection` asks: how does essence articulate itself?
- `principle` asks: can this become scientific object?
- `concept` asks: what is now named?
- `judgment` asks: how is it determined?
- `syllogism` asks: what follows through the middle?
- `procedure` asks: what Process is emitted?

---

## Current Parser Note

The current lightweight parser recognizes these surface keywords and lowers them into the doctrinal `ProgramFeatureKind` taxonomy. It is still intentionally lightweight: it records the epistemic role of each feature, while deeper validation and full grammar parsing remain future compiler work.
