Pipelines Node Classification Setup Fixture

Namespace: procedures::pipelines::node_classification

00 Facade
artifact: fixtures/procedures/033-proc-pipelines-nc/00-facade.txt
meaning: the top-level facade boundary that exposes node classification setup.

01 Create Pipeline
artifact: fixtures/procedures/033-proc-pipelines-nc/01-create-pipeline.json
meaning: the named pipeline created in the facade catalog.

02 Configure Split
artifact: fixtures/procedures/033-proc-pipelines-nc/02-configure-split.json
meaning: the split configuration applied to the pipeline.

03 Auto Tuning
artifact: fixtures/procedures/033-proc-pipelines-nc/03-configure-auto-tuning.json
meaning: the auto-tuning configuration applied to the pipeline.

04 Node Property Step
artifact: fixtures/procedures/033-proc-pipelines-nc/04-add-node-property.json
meaning: a node-property step added before training.

05 Select Features
artifact: fixtures/procedures/033-proc-pipelines-nc/05-select-features.json
meaning: the feature selection list used for node classification.

06 Training Methods
artifact: fixtures/procedures/033-proc-pipelines-nc/06-training-methods.json
meaning: the parameter-space setup for training methods.

07 Train Request
artifact: fixtures/procedures/033-proc-pipelines-nc/07-train-request.json
meaning: the raw request map shape that would be passed to nodeClassification.train.

08 Stream Request
artifact: fixtures/procedures/033-proc-pipelines-nc/08-stream-request.json
meaning: the raw request map shape for nodeClassification.stream.

09 Mutate Request
artifact: fixtures/procedures/033-proc-pipelines-nc/09-mutate-request.json
meaning: the raw request map shape for nodeClassification.mutate.

10 Write Request
artifact: fixtures/procedures/033-proc-pipelines-nc/10-write-request.json
meaning: the raw request map shape for nodeClassification.write.
