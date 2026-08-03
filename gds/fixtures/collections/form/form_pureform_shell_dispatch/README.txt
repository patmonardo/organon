PureForm Shell Dispatch Fixture

Namespace: form::shell_dispatch

Architectural Reading
Shell is the job-dispatch mediation surface for Form-directed execution.
Form is the applications manager that selects capability-bearing forms and
deposits feature truth into fixture artifacts.

00 Graph Put Response
artifact: fixtures/collections/form/form_pureform_shell_dispatch/00-graph-put-response.json
meaning: adapter-level graph materialization through graph_store.put.

01 Form Request
artifact: fixtures/collections/form/form_pureform_shell_dispatch/01-form-request.json
meaning: PureForm Program declaring an application-managed feature request through form_eval.

02 Form Response
artifact: fixtures/collections/form/form_pureform_shell_dispatch/02-form-response.json
meaning: proof payload including programForm, formCapabilities, and serviceManifest activation.

03 Dispatch Summary
artifact: fixtures/collections/form/form_pureform_shell_dispatch/03-dispatch-summary.txt
meaning: concise witness that Form-managed features are dispatched by Shell and retained as fixture evidence.
