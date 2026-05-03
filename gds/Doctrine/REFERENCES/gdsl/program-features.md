# GDSL Program Features

Program Features are the atomic commitments of a GDSL program. They are not runtime instructions. They are typed semantic units that compile into Dataset artifacts, GDS kernel work, and serviceable ontology images.

---

## Core Shape

A Program Feature has three fields:

```rust
pub struct ProgramFeature {
    pub kind: ProgramFeatureKind,
    pub value: String,
    pub source: String,
}
```

- `kind`: the epistemic role
- `value`: the local feature name or label
- `source`: the canonical GDSL phrase preserved for provenance

A set of features becomes `ProgramFeatures`, the compiler contract between GDSL and Dataset.

In the Shell, `ProgramFeatures` are also the mediated account of the Dataset middle.
The DataFrame body supplies the immediate register; the Dataset envelope supplies the
Model side; `ProgramFeatures` supply the Feature declarations and the Plan of return.
`GdsShell::model_feature_plan_knowledge()` reads these together as
`ShellModelFeaturePlanKnowledge`.

---

## Canonical Taxonomy

The full Doctrine taxonomy is:

| Kind | Keyword | Role |
|------|---------|------|
| `Import` | `use` | Declares a module or standard library dependency |
| `Source` | `source` | Names a concrete data source |
| `Observation` | `appearance` | Declares empirical appearance and retained/derived fields |
| `Reflection` | `reflection` | Stages appearance through seven moments |
| `Logogenesis` | `logogenesis` | Names self-unfolding into kind, law, or relation |
| `Principle` | `principle` | Nomological gate before Concept |
| `Condition` | `require`, contextual condition | A single admissibility condition |
| `Mark` | `mark` | Named quality, measure, ground, or determinacy |
| `Concept` | `concept` | Scientific object emerging after Principle |
| `Judgment` | `judgment` | Determination over a Concept |
| `Syllogism` | `syllogism` | Inference by middle term |
| `Inference` | `infer`, `conclude` | Atomic conditional inference rule |
| `Query` | `query` | Named result projection |
| `Procedure` | `procedure` | Entry into Objectivity; emits process artifacts |
| `SpecificationBinding` | `module` | Program/specification identity |

The Rust enum now carries this Doctrine taxonomy directly. Two native PureForm bridge
variants also remain for legacy `ProgramSpec` execution plans:

| Kind | Role |
|------|------|
| `ApplicationForm` | Native selected form bridge for `ProgramSpec` |
| `OperatorPattern` | Native morph/opcode bridge for `ProgramSpec` |

GDSL lowering should prefer the doctrinal variants above. `ApplicationForm` and
`OperatorPattern` are kernel bridge facets, not the normal GDSL authoring surface.

---

## Ordering Rule

Program Features obey the arc:

```text
Source -> Observation -> Reflection -> Principle -> Concept -> Judgment -> Syllogism -> Procedure
```

Do not place Principle after Concept in new Doctrine work. Principle is the threshold that permits Concept to emerge.

## Shell Middle Rule

The Shell integrates `Model:Feature::Plan` without turning it into a separate runtime.
It reads the middle this way:

| Middle moment | Shell source | Meaning |
|---|---|---|
| `Model` | Dataset name, artifact kind, and semantic envelope | The mediated body as a framework object |
| `Feature` | `ProgramFeatures.features` | Typed commitments: source, reflection, principle, concept, procedure |
| `Plan` | Shell concept-return order | The staged return through `dataframe.seed`, `dataset.model`, `dataset.feature`, `dataset.plan`, `shell.program`, `shell.concept-return` |

This is why the Shell comes into its own at `Model:Feature::Plan`: it can see the
DataFrame body, the Dataset mediation, and the Program declaration at once. From there
it returns to its own Concept through `ShellPipelineDescriptor::to_pure_form_principle()`.

## Shell DataPipeline Return Rule

The current Shell is not a multipipeline system. It is one pipeline type:

```text
ShellPipelineKind::DataPipeline
```

The DataPipeline gathers:

```text
DataFrame body -> Dataset middle -> Program declaration -> Shell descriptor
```

and returns:

```text
ShellPureFormReturn -> PureFormPrinciple(Shape:Context::Morph)
```

`ShellPipelineDescriptor::to_pure_form_return()` names this passage. The older
`to_pure_form_principle()` remains as the direct extraction of the principle, but the
return object is the doctrinally clearer form: it says which kind of pipeline returned
and at which Shell address.

---

## Compilation Rule

Every feature must be able to answer:

1. What artifact does this emit?
2. What previous features does it depend on?
3. What provenance does it carry?
4. What stage of the arc does it serve?

If a feature cannot answer these, it is not yet a stable Program Feature.

---

## Service Rule

Applications and examples must call procedures or services, not internal algorithm modules. Program Features define semantic intent. Procedures and services provide the controlled execution surface.
