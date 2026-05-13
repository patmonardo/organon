Dataset Plan Moment Fixture

Namespace: dataset::plan (R6)

Files:
00-frame.csv                  — immediate DataFrame body
02-describe-steps.txt         — Plan.describe_steps() trace
03-attention-report.txt       — PlanAttentionReport (steps + planned cols)
04-eval-preview.csv           — eval_dataset(Preview): filtered + enriched rows
05-eval-fit.csv               — eval_dataset(Fit): full dataset
06-var-plan.txt               — Source::Var + PlanEnv binding
README.txt                    — this manifest

Steps demonstrated:
Filter        — keep rows where score >= 20
Select        — choose columns
WithColumns   — add label column
Batch         — hint for streaming evaluation
Item          — canonical item projection (Struct)
