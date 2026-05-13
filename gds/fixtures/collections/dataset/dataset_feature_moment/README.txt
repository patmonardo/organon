Dataset Feature Moment Fixture

Namespace: dataset::feature (R5)

Files:
00-frame.csv                  — immediate DataFrame body
01-features.txt               — Feature construction (Projection role)
02-feature-space.txt          — FeatureSpace: named feature registry
03-chain.txt                  — Feature chaining (plan composition)
04-attention-reports.txt      — attention_report for token/role/chain
05-eval-token.csv             — token feature evaluated to Dataset
05-eval-role.csv              — role feature evaluated to Dataset
06-feature-views.txt          — FeatureView: named schema selectors
README.txt                    — this manifest

Feature roles demonstrated:
Projection — Plan-backed address; .new(plan), .named(), .chain(), .eval_dataset()
(Binder, Reentrancy, Annotation are doctrine commitments, not yet typed)
