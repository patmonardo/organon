MERGE (s:SourceText {id: 'source-essence'})
SET s.title = 'Essence'
SET s.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET s.totalLines = 303;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-essence'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:ess-1'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 7
SET segment.lineEnd = 43
SET segment.text = 'The truth of being is essence.\n\nBeing is the immediate.\nSince the goal of knowledge is the truth,\nwhat being is in and for itself,\nknowledge does not stop at\nthe immediate and its determinations,\nbut penetrates beyond it\non the presupposition that\nbehind this being there still is\nsomething other than being itself,\nand that this background\nconstitutes the truth of being.\nThis cognition is a mediated knowledge,\nfor it is not to be found\nwith and in essence immediately,\nbut starts off from an other, from being,\nand has a prior way to make,\nthe way that leads over and beyond being\nor that rather penetrates into it.\nOnly inasmuch as knowledge recollects\nitself into itself out of immediate being,\ndoes it find essence through this mediation.\nThe German language has kept “essence” (Wesen)\nin the past participle (gewesen) of the verb “to be” (sein),\nfor essence is past [but timelessly past] being.\n\nWhen this movement is represented as a pathway of knowledge,\nthis beginning with being and the subsequent advance\nwhich sublates being and arrives at essence as a mediated term\nappears to be an activity of cognition external to being\nand indifferent to its nature.\n\nBut this course is the movement of being itself.\nThat it is being\'s nature to recollect itself,\nand that it becomes essence by virtue of this interiorizing,\nthis has been displayed in being itself.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-1'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-1'
SET topic.title = 'Truth of being is essence — mediated knowledge'
SET topic.description = 'Being is immediate; knowledge seeks truth beyond immediacy. Essence is the truth of being, found through mediation. Knowledge recollects itself from immediate being to find essence. German etymology: Wesen (essence) from gewesen (past being). This movement is being\'s own nature, not external cognition.'
SET topic.keyPoints = ['Being is the immediate', 'Knowledge penetrates beyond immediate being to find essence', 'Essence found through mediation, recollection into itself', 'German: Wesen (essence) from gewesen (past being)', 'This course is the movement of being itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-1'})
MATCH (topic:Topic {id: 'topic:ess-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-1'})
SET c.title = 'Truth of being is essence — mediated knowledge'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 7
SET c.lineEnd = 43
SET c.description = 'Being is immediate; knowledge seeks truth beyond immediacy. Essence is the truth of being, found through mediation. Knowledge recollects itself from immediate being to find essence. German etymology: Wesen (essence) from gewesen (past being). This movement is being\'s own nature, not external cognition.'
SET c.keyPoints = ['Being is the immediate', 'Knowledge penetrates beyond immediate being to find essence', 'Essence found through mediation, recollection into itself', 'German: Wesen (essence) from gewesen (past being)', 'This course is the movement of being itself']
SET c.tags = ['mediation', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 1
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The truth of being is essence.\n\nBeing is the immediate.\nSince the goal of knowledge is the truth,\nwhat being is in and for itself,\nknowledge does not stop at\nthe immediate and its determinations,\nbut penetrates beyond it\non the presupposition that\nbehind this being there still is\nsomething other than being itself,\nand that this background\nconstitutes the truth of being.\nThis cognition is a mediated knowledge,\nfor it is not to be found\nwith and in essence immediately,\nbut starts off from an other, from being,\nand has a prior way to make,\nthe way that leads over and beyond being\nor that rather penetrates into it.\nOnly inasmuch as knowledge recollects\nitself into itself out of immediate being,\ndoes it find essence through this mediation.\nThe German language has kept “essence” (Wesen)\nin the past participle (gewesen) of the verb “to be” (sein),\nfor essence is past [but timelessly past] being.\n\nWhen this movement is represented as a pathway of knowledge,\nthis beginning with being and the subsequent advance\nwhich sublates being and arrives at essence as a mediated term\nappears to be an activity of cognition external to being\nand indifferent to its nature.\n\nBut this course is the movement of being itself.\nThat it is being\'s nature to recollect itself,\nand that it becomes essence by virtue of this interiorizing,\nthis has been displayed in being itself.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-1'})
MATCH (c:IntegratedChunk {id: 'ess-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-1:kp:1'})
SET kp.chunkId = 'ess-1'
SET kp.ordinal = 1
SET kp.text = 'Being is the immediate';
MATCH (c:IntegratedChunk {id: 'ess-1'})
MATCH (kp:KeyPoint {id: 'ess-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-1:kp:2'})
SET kp.chunkId = 'ess-1'
SET kp.ordinal = 2
SET kp.text = 'Knowledge penetrates beyond immediate being to find essence';
MATCH (c:IntegratedChunk {id: 'ess-1'})
MATCH (kp:KeyPoint {id: 'ess-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-1:kp:3'})
SET kp.chunkId = 'ess-1'
SET kp.ordinal = 3
SET kp.text = 'Essence found through mediation, recollection into itself';
MATCH (c:IntegratedChunk {id: 'ess-1'})
MATCH (kp:KeyPoint {id: 'ess-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-1:kp:4'})
SET kp.chunkId = 'ess-1'
SET kp.ordinal = 4
SET kp.text = 'German: Wesen (essence) from gewesen (past being)';
MATCH (c:IntegratedChunk {id: 'ess-1'})
MATCH (kp:KeyPoint {id: 'ess-1:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-1:kp:5'})
SET kp.chunkId = 'ess-1'
SET kp.ordinal = 5
SET kp.text = 'This course is the movement of being itself';
MATCH (c:IntegratedChunk {id: 'ess-1'})
MATCH (kp:KeyPoint {id: 'ess-1:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-2'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 45
SET segment.lineEnd = 86
SET segment.text = 'If, therefore, the absolute was\nat first determined as being,\nnow it is determined as essence.\nCognition cannot in general stop\nat the manifold of existence;\nbut neither can it stop at being, pure being;\nimmediately one is forced to the reflection that\nthis pure being, this negation of everything finite,\npresupposes a recollection and a movement\nwhich has distilled immediate existence into pure being.\nBeing thus comes to be determined as essence,\nas a being in which everything determined and finite is negated.\nSo it is simple unity, void of determination,\nfrom which the determinate has been\nremoved in an external manner;\nto this unity the determinate was\nitself something external\nand, after this removal,\nit still remains opposite to it;\nfor it has not been sublated in itself but relatively,\nonly with reference to this unity.\nWe already noted above that if pure essence is defined\nas the sum total of all realities,\nthese realities are equally subject to\nthe nature of determinateness and abstractive reflection\nand their sum total is reduced to empty simplicity.\nThus defined, essence is only a product, an artifact.\nExternal reflection, which is abstraction,\nonly lifts the determinacies of being\nout of what is left over as essence\nand only deposits them, as it were,\nsomewhere else, letting them exist as before.\nIn this way, however, essence is\nneither in itself nor for itself;\nit is by virtue of another,\nthrough external abstractive reflection;\nand it is for another, namely for abstraction\nand in general for the existent\nwhich still remains opposite to it.\nIn its determination, therefore,\nit is a dead and empty absence of determinateness.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-2'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-2'
SET topic.title = 'Essence as sublated being — external abstraction critique'
SET topic.description = 'Absolute determined first as being, now as essence. Pure being presupposes recollection and movement. External abstraction produces only empty artifact. Essence through external reflection is neither in-itself nor for-itself. Dead and empty absence of determinateness.'
SET topic.keyPoints = ['Absolute first determined as being, now as essence', 'Pure being presupposes recollection and movement', 'External abstraction produces empty artifact', 'Essence through external reflection is neither in-itself nor for-itself', 'Dead and empty absence of determinateness'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-2'})
MATCH (topic:Topic {id: 'topic:ess-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-2'})
SET c.title = 'Essence as sublated being — external abstraction critique'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 45
SET c.lineEnd = 86
SET c.description = 'Absolute determined first as being, now as essence. Pure being presupposes recollection and movement. External abstraction produces only empty artifact. Essence through external reflection is neither in-itself nor for-itself. Dead and empty absence of determinateness.'
SET c.keyPoints = ['Absolute first determined as being, now as essence', 'Pure being presupposes recollection and movement', 'External abstraction produces empty artifact', 'Essence through external reflection is neither in-itself nor for-itself', 'Dead and empty absence of determinateness']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 2
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'If, therefore, the absolute was\nat first determined as being,\nnow it is determined as essence.\nCognition cannot in general stop\nat the manifold of existence;\nbut neither can it stop at being, pure being;\nimmediately one is forced to the reflection that\nthis pure being, this negation of everything finite,\npresupposes a recollection and a movement\nwhich has distilled immediate existence into pure being.\nBeing thus comes to be determined as essence,\nas a being in which everything determined and finite is negated.\nSo it is simple unity, void of determination,\nfrom which the determinate has been\nremoved in an external manner;\nto this unity the determinate was\nitself something external\nand, after this removal,\nit still remains opposite to it;\nfor it has not been sublated in itself but relatively,\nonly with reference to this unity.\nWe already noted above that if pure essence is defined\nas the sum total of all realities,\nthese realities are equally subject to\nthe nature of determinateness and abstractive reflection\nand their sum total is reduced to empty simplicity.\nThus defined, essence is only a product, an artifact.\nExternal reflection, which is abstraction,\nonly lifts the determinacies of being\nout of what is left over as essence\nand only deposits them, as it were,\nsomewhere else, letting them exist as before.\nIn this way, however, essence is\nneither in itself nor for itself;\nit is by virtue of another,\nthrough external abstractive reflection;\nand it is for another, namely for abstraction\nand in general for the existent\nwhich still remains opposite to it.\nIn its determination, therefore,\nit is a dead and empty absence of determinateness.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-2'})
MATCH (c:IntegratedChunk {id: 'ess-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-2:kp:1'})
SET kp.chunkId = 'ess-2'
SET kp.ordinal = 1
SET kp.text = 'Absolute first determined as being, now as essence';
MATCH (c:IntegratedChunk {id: 'ess-2'})
MATCH (kp:KeyPoint {id: 'ess-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-2:kp:2'})
SET kp.chunkId = 'ess-2'
SET kp.ordinal = 2
SET kp.text = 'Pure being presupposes recollection and movement';
MATCH (c:IntegratedChunk {id: 'ess-2'})
MATCH (kp:KeyPoint {id: 'ess-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-2:kp:3'})
SET kp.chunkId = 'ess-2'
SET kp.ordinal = 3
SET kp.text = 'External abstraction produces empty artifact';
MATCH (c:IntegratedChunk {id: 'ess-2'})
MATCH (kp:KeyPoint {id: 'ess-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-2:kp:4'})
SET kp.chunkId = 'ess-2'
SET kp.ordinal = 4
SET kp.text = 'Essence through external reflection is neither in-itself nor for-itself';
MATCH (c:IntegratedChunk {id: 'ess-2'})
MATCH (kp:KeyPoint {id: 'ess-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-2:kp:5'})
SET kp.chunkId = 'ess-2'
SET kp.ordinal = 5
SET kp.text = 'Dead and empty absence of determinateness';
MATCH (c:IntegratedChunk {id: 'ess-2'})
MATCH (kp:KeyPoint {id: 'ess-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-3'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 87
SET segment.lineEnd = 103
SET segment.text = 'As it has come to be here, however,\nessence is what it is,\nnot through a negativity foreign to it,\nbut through one which is its own:\nthe infinite movement of being.\nIt is being-in-and-for-itself,\nabsolute in-itselfness;\nsince it is indifferent to\nevery determinateness of being,\notherness and reference to other have been sublated.\nBut neither is it only this in-itselfness;\nas merely being-in-itself,\nit would be only the abstraction of pure essence;\nbut it is being-for-itself just as essentially;\nit is itself this negativity,\nthe self-sublation of otherness\nand of determinateness.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-3'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-3'
SET topic.title = 'Essence as absolute negativity — being-in-and-for-itself'
SET topic.description = 'Essence through its own negativity, infinite movement of being. Being-in-and-for-itself, absolute in-itselfness. Indifferent to every determinateness of being. Not only in-itself but also being-for-itself. Self-sublation of otherness and determinateness.'
SET topic.keyPoints = ['Essence through its own negativity, infinite movement of being', 'Being-in-and-for-itself, absolute in-itselfness', 'Indifferent to every determinateness of being', 'Not only in-itself but also being-for-itself', 'Self-sublation of otherness and determinateness'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-3'})
MATCH (topic:Topic {id: 'topic:ess-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-3'})
SET c.title = 'Essence as absolute negativity — being-in-and-for-itself'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 87
SET c.lineEnd = 103
SET c.description = 'Essence through its own negativity, infinite movement of being. Being-in-and-for-itself, absolute in-itselfness. Indifferent to every determinateness of being. Not only in-itself but also being-for-itself. Self-sublation of otherness and determinateness.'
SET c.keyPoints = ['Essence through its own negativity, infinite movement of being', 'Being-in-and-for-itself, absolute in-itselfness', 'Indifferent to every determinateness of being', 'Not only in-itself but also being-for-itself', 'Self-sublation of otherness and determinateness']
SET c.tags = ['sublation', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 3
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'As it has come to be here, however,\nessence is what it is,\nnot through a negativity foreign to it,\nbut through one which is its own:\nthe infinite movement of being.\nIt is being-in-and-for-itself,\nabsolute in-itselfness;\nsince it is indifferent to\nevery determinateness of being,\notherness and reference to other have been sublated.\nBut neither is it only this in-itselfness;\nas merely being-in-itself,\nit would be only the abstraction of pure essence;\nbut it is being-for-itself just as essentially;\nit is itself this negativity,\nthe self-sublation of otherness\nand of determinateness.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-3'})
MATCH (c:IntegratedChunk {id: 'ess-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-3:kp:1'})
SET kp.chunkId = 'ess-3'
SET kp.ordinal = 1
SET kp.text = 'Essence through its own negativity, infinite movement of being';
MATCH (c:IntegratedChunk {id: 'ess-3'})
MATCH (kp:KeyPoint {id: 'ess-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-3:kp:2'})
SET kp.chunkId = 'ess-3'
SET kp.ordinal = 2
SET kp.text = 'Being-in-and-for-itself, absolute in-itselfness';
MATCH (c:IntegratedChunk {id: 'ess-3'})
MATCH (kp:KeyPoint {id: 'ess-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-3:kp:3'})
SET kp.chunkId = 'ess-3'
SET kp.ordinal = 3
SET kp.text = 'Indifferent to every determinateness of being';
MATCH (c:IntegratedChunk {id: 'ess-3'})
MATCH (kp:KeyPoint {id: 'ess-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-3:kp:4'})
SET kp.chunkId = 'ess-3'
SET kp.ordinal = 4
SET kp.text = 'Not only in-itself but also being-for-itself';
MATCH (c:IntegratedChunk {id: 'ess-3'})
MATCH (kp:KeyPoint {id: 'ess-3:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-3:kp:5'})
SET kp.chunkId = 'ess-3'
SET kp.ordinal = 5
SET kp.text = 'Self-sublation of otherness and determinateness';
MATCH (c:IntegratedChunk {id: 'ess-3'})
MATCH (kp:KeyPoint {id: 'ess-3:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-4'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 105
SET segment.lineEnd = 135
SET segment.text = 'Essence, as the complete turning back of being into itself,\nis thus at first the indeterminate essence;\nthe determinacies of being are sublated in it;\nit holds them in itself but without their being posited in it.\nAbsolute essence in this simple unity with itself has no existence.\nBut it must pass over into existence,\nfor it is being-in-and-for-itself;\nthat is to say, it differentiates\nthe determinations which it holds in itself,\nand, since it is the repelling of itself from itself\nor indifference towards itself, negative self-reference,\nit thereby posits itself over against itself\nand is infinite being-for-itself\nonly in so far as in thus\ndifferentiating itself from itself\nit is in unity with itself.\nThis determining is thus of another nature than\nthe determining in the sphere of being,\nand the determinations of essence have\nanother character than the determinations of being.\nEssence is absolute unity of being-in-itself and being-for-itself;\nconsequently, its determining remains inside this unity;\nit is neither a becoming nor a passing over,\njust as the determinations themselves are\nneither an other as other nor references to some other;\nthey are self-subsisting but, as such,\nat the same time conjoined in the unity of essence.\nSince essence is at first simple negativity,\nin order to give itself existence and then being-for-itself,\nit must now posit in its sphere the determinateness\nwhich it contains in principle only in itself.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-4'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-4'
SET topic.title = 'Indeterminate essence — must differentiate into existence'
SET topic.description = 'Essence as complete turning back of being into itself. At first indeterminate essence, holds determinations without positing. Must pass over into existence, differentiate itself. Repelling of itself from itself, negative self-reference. Determinations of essence different from determinations of being.'
SET topic.keyPoints = ['Essence as complete turning back of being into itself', 'At first indeterminate essence, holds determinations without positing', 'Must pass over into existence, differentiate itself', 'Repelling of itself from itself, negative self-reference', 'Determinations of essence different from determinations of being'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-4'})
MATCH (topic:Topic {id: 'topic:ess-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-4'})
SET c.title = 'Indeterminate essence — must differentiate into existence'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 105
SET c.lineEnd = 135
SET c.description = 'Essence as complete turning back of being into itself. At first indeterminate essence, holds determinations without positing. Must pass over into existence, differentiate itself. Repelling of itself from itself, negative self-reference. Determinations of essence different from determinations of being.'
SET c.keyPoints = ['Essence as complete turning back of being into itself', 'At first indeterminate essence, holds determinations without positing', 'Must pass over into existence, differentiate itself', 'Repelling of itself from itself, negative self-reference', 'Determinations of essence different from determinations of being']
SET c.tags = ['negation', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 4
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Essence, as the complete turning back of being into itself,\nis thus at first the indeterminate essence;\nthe determinacies of being are sublated in it;\nit holds them in itself but without their being posited in it.\nAbsolute essence in this simple unity with itself has no existence.\nBut it must pass over into existence,\nfor it is being-in-and-for-itself;\nthat is to say, it differentiates\nthe determinations which it holds in itself,\nand, since it is the repelling of itself from itself\nor indifference towards itself, negative self-reference,\nit thereby posits itself over against itself\nand is infinite being-for-itself\nonly in so far as in thus\ndifferentiating itself from itself\nit is in unity with itself.\nThis determining is thus of another nature than\nthe determining in the sphere of being,\nand the determinations of essence have\nanother character than the determinations of being.\nEssence is absolute unity of being-in-itself and being-for-itself;\nconsequently, its determining remains inside this unity;\nit is neither a becoming nor a passing over,\njust as the determinations themselves are\nneither an other as other nor references to some other;\nthey are self-subsisting but, as such,\nat the same time conjoined in the unity of essence.\nSince essence is at first simple negativity,\nin order to give itself existence and then being-for-itself,\nit must now posit in its sphere the determinateness\nwhich it contains in principle only in itself.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-4'})
MATCH (c:IntegratedChunk {id: 'ess-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-4:kp:1'})
SET kp.chunkId = 'ess-4'
SET kp.ordinal = 1
SET kp.text = 'Essence as complete turning back of being into itself';
MATCH (c:IntegratedChunk {id: 'ess-4'})
MATCH (kp:KeyPoint {id: 'ess-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-4:kp:2'})
SET kp.chunkId = 'ess-4'
SET kp.ordinal = 2
SET kp.text = 'At first indeterminate essence, holds determinations without positing';
MATCH (c:IntegratedChunk {id: 'ess-4'})
MATCH (kp:KeyPoint {id: 'ess-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-4:kp:3'})
SET kp.chunkId = 'ess-4'
SET kp.ordinal = 3
SET kp.text = 'Must pass over into existence, differentiate itself';
MATCH (c:IntegratedChunk {id: 'ess-4'})
MATCH (kp:KeyPoint {id: 'ess-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-4:kp:4'})
SET kp.chunkId = 'ess-4'
SET kp.ordinal = 4
SET kp.text = 'Repelling of itself from itself, negative self-reference';
MATCH (c:IntegratedChunk {id: 'ess-4'})
MATCH (kp:KeyPoint {id: 'ess-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-4:kp:5'})
SET kp.chunkId = 'ess-4'
SET kp.ordinal = 5
SET kp.text = 'Determinations of essence different from determinations of being';
MATCH (c:IntegratedChunk {id: 'ess-4'})
MATCH (kp:KeyPoint {id: 'ess-4:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-5'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 137
SET segment.lineEnd = 150
SET segment.text = 'Essence is in the whole what quality was in the sphere of being:\nabsolute indifference with respect to limit.\nQuantity is instead this indifference in immediate determination,\nlimit being in it an immediate external determinateness;\nquantity passes over into quantum;\nthe external limit is necessary to it and exists in it.\nIn essence, by contrast, the determinateness does not exist;\nit is posited only by the essence itself,\nnot free but only with reference to\nthe unity of the essence.\nThe negativity of essence is reflection,\nand the determinations are reflected,\nposited by the essence itself\nin which they remain as sublated.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-5'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-5'
SET topic.title = 'Essence compared to quality and quantity — reflection'
SET topic.description = 'Essence is to whole what quality was to being: absolute indifference to limit. Quantity is indifference in immediate determination. In essence, determinateness does not exist but is posited by essence. Negativity of essence is reflection. Determinations are reflected, posited by essence, remain as sublated.'
SET topic.keyPoints = ['Essence is to whole what quality was to being: absolute indifference to limit', 'Quantity is indifference in immediate determination', 'In essence, determinateness does not exist but is posited by essence', 'Negativity of essence is reflection', 'Determinations are reflected, posited by essence, remain as sublated'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-5'})
MATCH (topic:Topic {id: 'topic:ess-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-5'})
SET c.title = 'Essence compared to quality and quantity — reflection'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 137
SET c.lineEnd = 150
SET c.description = 'Essence is to whole what quality was to being: absolute indifference to limit. Quantity is indifference in immediate determination. In essence, determinateness does not exist but is posited by essence. Negativity of essence is reflection. Determinations are reflected, posited by essence, remain as sublated.'
SET c.keyPoints = ['Essence is to whole what quality was to being: absolute indifference to limit', 'Quantity is indifference in immediate determination', 'In essence, determinateness does not exist but is posited by essence', 'Negativity of essence is reflection', 'Determinations are reflected, posited by essence, remain as sublated']
SET c.tags = ['sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 5
SET c.globalOrder = 5
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Essence is in the whole what quality was in the sphere of being:\nabsolute indifference with respect to limit.\nQuantity is instead this indifference in immediate determination,\nlimit being in it an immediate external determinateness;\nquantity passes over into quantum;\nthe external limit is necessary to it and exists in it.\nIn essence, by contrast, the determinateness does not exist;\nit is posited only by the essence itself,\nnot free but only with reference to\nthe unity of the essence.\nThe negativity of essence is reflection,\nand the determinations are reflected,\nposited by the essence itself\nin which they remain as sublated.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-5'})
MATCH (c:IntegratedChunk {id: 'ess-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-5:kp:1'})
SET kp.chunkId = 'ess-5'
SET kp.ordinal = 1
SET kp.text = 'Essence is to whole what quality was to being: absolute indifference to limit';
MATCH (c:IntegratedChunk {id: 'ess-5'})
MATCH (kp:KeyPoint {id: 'ess-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-5:kp:2'})
SET kp.chunkId = 'ess-5'
SET kp.ordinal = 2
SET kp.text = 'Quantity is indifference in immediate determination';
MATCH (c:IntegratedChunk {id: 'ess-5'})
MATCH (kp:KeyPoint {id: 'ess-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-5:kp:3'})
SET kp.chunkId = 'ess-5'
SET kp.ordinal = 3
SET kp.text = 'In essence, determinateness does not exist but is posited by essence';
MATCH (c:IntegratedChunk {id: 'ess-5'})
MATCH (kp:KeyPoint {id: 'ess-5:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-5:kp:4'})
SET kp.chunkId = 'ess-5'
SET kp.ordinal = 4
SET kp.text = 'Negativity of essence is reflection';
MATCH (c:IntegratedChunk {id: 'ess-5'})
MATCH (kp:KeyPoint {id: 'ess-5:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-5:kp:5'})
SET kp.chunkId = 'ess-5'
SET kp.ordinal = 5
SET kp.text = 'Determinations are reflected, posited by essence, remain as sublated';
MATCH (c:IntegratedChunk {id: 'ess-5'})
MATCH (kp:KeyPoint {id: 'ess-5:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-6'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 152
SET segment.lineEnd = 175
SET segment.text = 'Essence stands between being and concept;\nit makes up their middle,\nits movement constituting the transition\nof being into the concept.\nEssence is being-in-and-for-itself,\nbut it is this in the determination of being-in-itself;\nfor its general determination is that it emerges from being\nor that it is the first negation of being.\nIts movement consists in positing negation\nor determination in being,\nthereby giving itself existence\nand becoming as infinite being-for-itself\nwhat it is in itself.\nIt thus gives itself its existence\nwhich is equal to its being-in-itself\nand becomes concept.\nFor the concept is the absolute\nas it is absolutely,\nor in and for itself,\nin its existence.\nBut the existence which essence gives to itself is\nnot yet existence as it is in and for itself\nbut as essence gives it to itself or as posited,\nand hence still distinct from the existence of the concept.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-6'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-6'
SET topic.title = 'Essence as middle term — transition to concept'
SET topic.description = 'Essence stands between being and concept, makes up their middle. Essence is being-in-and-for-itself in determination of being-in-itself. First negation of being. Movement: positing negation/determination, giving itself existence. Becomes concept when existence equals being-in-itself. But essence\'s existence still distinct from concept\'s existence.'
SET topic.keyPoints = ['Essence stands between being and concept, makes up their middle', 'Essence is being-in-and-for-itself in determination of being-in-itself', 'First negation of being', 'Movement: positing negation/determination, giving itself existence', 'Becomes concept when existence equals being-in-itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-6'})
MATCH (topic:Topic {id: 'topic:ess-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-6'})
SET c.title = 'Essence as middle term — transition to concept'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 152
SET c.lineEnd = 175
SET c.description = 'Essence stands between being and concept, makes up their middle. Essence is being-in-and-for-itself in determination of being-in-itself. First negation of being. Movement: positing negation/determination, giving itself existence. Becomes concept when existence equals being-in-itself. But essence\'s existence still distinct from concept\'s existence.'
SET c.keyPoints = ['Essence stands between being and concept, makes up their middle', 'Essence is being-in-and-for-itself in determination of being-in-itself', 'First negation of being', 'Movement: positing negation/determination, giving itself existence', 'Becomes concept when existence equals being-in-itself']
SET c.tags = ['negation', 'citta']
SET c.orderInSource = 6
SET c.globalOrder = 6
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Essence stands between being and concept;\nit makes up their middle,\nits movement constituting the transition\nof being into the concept.\nEssence is being-in-and-for-itself,\nbut it is this in the determination of being-in-itself;\nfor its general determination is that it emerges from being\nor that it is the first negation of being.\nIts movement consists in positing negation\nor determination in being,\nthereby giving itself existence\nand becoming as infinite being-for-itself\nwhat it is in itself.\nIt thus gives itself its existence\nwhich is equal to its being-in-itself\nand becomes concept.\nFor the concept is the absolute\nas it is absolutely,\nor in and for itself,\nin its existence.\nBut the existence which essence gives to itself is\nnot yet existence as it is in and for itself\nbut as essence gives it to itself or as posited,\nand hence still distinct from the existence of the concept.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-6'})
MATCH (c:IntegratedChunk {id: 'ess-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-6:kp:1'})
SET kp.chunkId = 'ess-6'
SET kp.ordinal = 1
SET kp.text = 'Essence stands between being and concept, makes up their middle';
MATCH (c:IntegratedChunk {id: 'ess-6'})
MATCH (kp:KeyPoint {id: 'ess-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-6:kp:2'})
SET kp.chunkId = 'ess-6'
SET kp.ordinal = 2
SET kp.text = 'Essence is being-in-and-for-itself in determination of being-in-itself';
MATCH (c:IntegratedChunk {id: 'ess-6'})
MATCH (kp:KeyPoint {id: 'ess-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-6:kp:3'})
SET kp.chunkId = 'ess-6'
SET kp.ordinal = 3
SET kp.text = 'First negation of being';
MATCH (c:IntegratedChunk {id: 'ess-6'})
MATCH (kp:KeyPoint {id: 'ess-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-6:kp:4'})
SET kp.chunkId = 'ess-6'
SET kp.ordinal = 4
SET kp.text = 'Movement: positing negation/determination, giving itself existence';
MATCH (c:IntegratedChunk {id: 'ess-6'})
MATCH (kp:KeyPoint {id: 'ess-6:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-6:kp:5'})
SET kp.chunkId = 'ess-6'
SET kp.ordinal = 5
SET kp.text = 'Becomes concept when existence equals being-in-itself';
MATCH (c:IntegratedChunk {id: 'ess-6'})
MATCH (kp:KeyPoint {id: 'ess-6:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-7'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 177
SET segment.lineEnd = 192
SET segment.text = 'First, essence shines within itself\nor is reflection;\nsecond, it appears;\nthird, it reveals itself.\n\nIn the course of its movement,\nit posits itself in the following determinations:\n\nI. As simple essence existing in itself,\nremaining in itself in its determinations;\n\nII. As emerging into existence,\nor according to its concrete existence and appearance;\n\nIII. As essence which is one with its appearance,\nas actuality.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-7'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-7'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-7'
SET topic.title = 'Threefold structure — shine, appearance, actuality'
SET topic.description = 'First: essence shines within itself (reflection). Second: it appears. Third: it reveals itself. Three determinations: simple essence in itself; emerging into existence; essence one with appearance (actuality).'
SET topic.keyPoints = ['First: essence shines within itself (reflection)', 'Second: it appears', 'Third: it reveals itself', 'Three determinations: simple essence in itself; emerging into existence; essence one with appearance (actuality)'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-7'})
MATCH (topic:Topic {id: 'topic:ess-7'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-7'})
SET c.title = 'Threefold structure — shine, appearance, actuality'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 177
SET c.lineEnd = 192
SET c.description = 'First: essence shines within itself (reflection). Second: it appears. Third: it reveals itself. Three determinations: simple essence in itself; emerging into existence; essence one with appearance (actuality).'
SET c.keyPoints = ['First: essence shines within itself (reflection)', 'Second: it appears', 'Third: it reveals itself', 'Three determinations: simple essence in itself; emerging into existence; essence one with appearance (actuality)']
SET c.tags = ['reflection', 'shine', 'appearance', 'citta']
SET c.orderInSource = 7
SET c.globalOrder = 7
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'First, essence shines within itself\nor is reflection;\nsecond, it appears;\nthird, it reveals itself.\n\nIn the course of its movement,\nit posits itself in the following determinations:\n\nI. As simple essence existing in itself,\nremaining in itself in its determinations;\n\nII. As emerging into existence,\nor according to its concrete existence and appearance;\n\nIII. As essence which is one with its appearance,\nas actuality.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-7'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-7'})
MATCH (c:IntegratedChunk {id: 'ess-7'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-7'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-7'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-7:kp:1'})
SET kp.chunkId = 'ess-7'
SET kp.ordinal = 1
SET kp.text = 'First: essence shines within itself (reflection)';
MATCH (c:IntegratedChunk {id: 'ess-7'})
MATCH (kp:KeyPoint {id: 'ess-7:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-7:kp:2'})
SET kp.chunkId = 'ess-7'
SET kp.ordinal = 2
SET kp.text = 'Second: it appears';
MATCH (c:IntegratedChunk {id: 'ess-7'})
MATCH (kp:KeyPoint {id: 'ess-7:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-7:kp:3'})
SET kp.chunkId = 'ess-7'
SET kp.ordinal = 3
SET kp.text = 'Third: it reveals itself';
MATCH (c:IntegratedChunk {id: 'ess-7'})
MATCH (kp:KeyPoint {id: 'ess-7:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-7:kp:4'})
SET kp.chunkId = 'ess-7'
SET kp.ordinal = 4
SET kp.text = 'Three determinations: simple essence in itself; emerging into existence; essence one with appearance (actuality)';
MATCH (c:IntegratedChunk {id: 'ess-7'})
MATCH (kp:KeyPoint {id: 'ess-7:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-8'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 198
SET segment.lineEnd = 216
SET segment.text = 'Essence issues from being;\nhence it is not immediately in and for itself\nbut is a result of that movement.\nOr, since essence is taken at first as something immediate,\nit is a determinate existence to which another stands opposed;\nit is only essential existence, as against the unessential.\nBut essence is being which has been sublated in and for itself;\nwhat stands over against it is only shine.\nThe shine, however, is essence\'s own positing.\n\nFirst, essence is reflection.\nReflection determines itself;\nits determinations are a positedness\nwhich is immanent reflection at the same time.\nSecond, these reflective determinations\nor essentialities are to be considered.\nThird, as the reflection of its immanent determining,\nessence turns into foundation and passes over\ninto concrete existence and appearance.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-8'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-8'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-8'
SET topic.title = 'Section I introduction — essence as reflection within'
SET topic.description = 'Essence issues from being, result of movement. At first immediate, essential existence vs. unessential. Essence is sublated being; what stands over against it is shine. Shine is essence\'s own positing. Three moments: reflection; reflective determinations; foundation.'
SET topic.keyPoints = ['Essence issues from being, result of movement', 'At first immediate, essential existence vs. unessential', 'Essence is sublated being; what stands over against it is shine', 'Shine is essence\'s own positing', 'Three moments: reflection; reflective determinations; foundation'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-8'})
MATCH (topic:Topic {id: 'topic:ess-8'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-8'})
SET c.title = 'Section I introduction — essence as reflection within'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 198
SET c.lineEnd = 216
SET c.description = 'Essence issues from being, result of movement. At first immediate, essential existence vs. unessential. Essence is sublated being; what stands over against it is shine. Shine is essence\'s own positing. Three moments: reflection; reflective determinations; foundation.'
SET c.keyPoints = ['Essence issues from being, result of movement', 'At first immediate, essential existence vs. unessential', 'Essence is sublated being; what stands over against it is shine', 'Shine is essence\'s own positing', 'Three moments: reflection; reflective determinations; foundation']
SET c.tags = ['sublation', 'reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 8
SET c.globalOrder = 8
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Essence issues from being;\nhence it is not immediately in and for itself\nbut is a result of that movement.\nOr, since essence is taken at first as something immediate,\nit is a determinate existence to which another stands opposed;\nit is only essential existence, as against the unessential.\nBut essence is being which has been sublated in and for itself;\nwhat stands over against it is only shine.\nThe shine, however, is essence\'s own positing.\n\nFirst, essence is reflection.\nReflection determines itself;\nits determinations are a positedness\nwhich is immanent reflection at the same time.\nSecond, these reflective determinations\nor essentialities are to be considered.\nThird, as the reflection of its immanent determining,\nessence turns into foundation and passes over\ninto concrete existence and appearance.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-8'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-8'})
MATCH (c:IntegratedChunk {id: 'ess-8'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-8'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-8'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-8:kp:1'})
SET kp.chunkId = 'ess-8'
SET kp.ordinal = 1
SET kp.text = 'Essence issues from being, result of movement';
MATCH (c:IntegratedChunk {id: 'ess-8'})
MATCH (kp:KeyPoint {id: 'ess-8:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-8:kp:2'})
SET kp.chunkId = 'ess-8'
SET kp.ordinal = 2
SET kp.text = 'At first immediate, essential existence vs. unessential';
MATCH (c:IntegratedChunk {id: 'ess-8'})
MATCH (kp:KeyPoint {id: 'ess-8:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-8:kp:3'})
SET kp.chunkId = 'ess-8'
SET kp.ordinal = 3
SET kp.text = 'Essence is sublated being; what stands over against it is shine';
MATCH (c:IntegratedChunk {id: 'ess-8'})
MATCH (kp:KeyPoint {id: 'ess-8:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-8:kp:4'})
SET kp.chunkId = 'ess-8'
SET kp.ordinal = 4
SET kp.text = 'Shine is essence\'s own positing';
MATCH (c:IntegratedChunk {id: 'ess-8'})
MATCH (kp:KeyPoint {id: 'ess-8:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-8:kp:5'})
SET kp.chunkId = 'ess-8'
SET kp.ordinal = 5
SET kp.text = 'Three moments: reflection; reflective determinations; foundation';
MATCH (c:IntegratedChunk {id: 'ess-8'})
MATCH (kp:KeyPoint {id: 'ess-8:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-9'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 222
SET segment.lineEnd = 230
SET segment.text = 'As it issues from being, essence seems to stand over against it;\nthis immediate being is, first, the unessential.\n\nBut, second, it is more than just the unessential;\nit is being void of essence; it is shine.\n\nThird, this shine is not something external,\nsomething other than essence, but is essence\'s own shining.\nThis shining of essence within it is reflection.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-9'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-9'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-9'
SET topic.title = 'Chapter 1 introduction — shine'
SET topic.description = 'Essence seems to stand over against being. Immediate being is unessential. More than unessential: being void of essence, shine. Shine is essence\'s own shining, reflection.'
SET topic.keyPoints = ['Essence seems to stand over against being', 'Immediate being is unessential', 'More than unessential: being void of essence, shine', 'Shine is essence\'s own shining, reflection'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-9'})
MATCH (topic:Topic {id: 'topic:ess-9'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-9'})
SET c.title = 'Chapter 1 introduction — shine'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 222
SET c.lineEnd = 230
SET c.description = 'Essence seems to stand over against being. Immediate being is unessential. More than unessential: being void of essence, shine. Shine is essence\'s own shining, reflection.'
SET c.keyPoints = ['Essence seems to stand over against being', 'Immediate being is unessential', 'More than unessential: being void of essence, shine', 'Shine is essence\'s own shining, reflection']
SET c.tags = ['reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 9
SET c.globalOrder = 9
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'As it issues from being, essence seems to stand over against it;\nthis immediate being is, first, the unessential.\n\nBut, second, it is more than just the unessential;\nit is being void of essence; it is shine.\n\nThird, this shine is not something external,\nsomething other than essence, but is essence\'s own shining.\nThis shining of essence within it is reflection.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-9'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-9'})
MATCH (c:IntegratedChunk {id: 'ess-9'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-9'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-9'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-9:kp:1'})
SET kp.chunkId = 'ess-9'
SET kp.ordinal = 1
SET kp.text = 'Essence seems to stand over against being';
MATCH (c:IntegratedChunk {id: 'ess-9'})
MATCH (kp:KeyPoint {id: 'ess-9:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-9:kp:2'})
SET kp.chunkId = 'ess-9'
SET kp.ordinal = 2
SET kp.text = 'Immediate being is unessential';
MATCH (c:IntegratedChunk {id: 'ess-9'})
MATCH (kp:KeyPoint {id: 'ess-9:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-9:kp:3'})
SET kp.chunkId = 'ess-9'
SET kp.ordinal = 3
SET kp.text = 'More than unessential: being void of essence, shine';
MATCH (c:IntegratedChunk {id: 'ess-9'})
MATCH (kp:KeyPoint {id: 'ess-9:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-9:kp:4'})
SET kp.chunkId = 'ess-9'
SET kp.ordinal = 4
SET kp.text = 'Shine is essence\'s own shining, reflection';
MATCH (c:IntegratedChunk {id: 'ess-9'})
MATCH (kp:KeyPoint {id: 'ess-9:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-10'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 234
SET segment.lineEnd = 260
SET segment.text = 'Essence is sublated being.\n\nIt is simple equality with itself\nbut is such as the negation of\nthe sphere of being in general.\nAnd so it has immediacy over against it,\nas something from which it has come to be\nbut which has preserved and maintained itself in this sublating.\nEssence itself is in this determination\nan existent immediate essence,\nand with reference to it\nbeing is only something negative,\nnothing in and for itself:\nessence, therefore, is a determined negation.\nBeing and essence relate to each other in this fashion\nas against others in general which are mutually indifferent,\nfor each has a being, an immediacy,\nand according to this being they stand in equal value.\n\nBut as contrasted with essence,\nbeing is at the same time the unessential;\nas against essence, it has the determination of something sublated.\nAnd in so far as it thus relates to essence\nas an other only in general,\nessence itself is not essence proper\nbut is just another existence, the essential.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-10'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-10'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-10'
SET topic.title = 'Essential and unessential — determined negation'
SET topic.description = 'Essence is sublated being, simple equality with itself. Negation of sphere of being in general. Has immediacy over against it, preserved in sublating. Essence is determined negation. Being and essence relate as mutually indifferent others. Being is unessential, essence is essential existence.'
SET topic.keyPoints = ['Essence is sublated being, simple equality with itself', 'Negation of sphere of being in general', 'Has immediacy over against it, preserved in sublating', 'Essence is determined negation', 'Being and essence relate as mutually indifferent others'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-10'})
MATCH (topic:Topic {id: 'topic:ess-10'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-10'})
SET c.title = 'Essential and unessential — determined negation'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 234
SET c.lineEnd = 260
SET c.description = 'Essence is sublated being, simple equality with itself. Negation of sphere of being in general. Has immediacy over against it, preserved in sublating. Essence is determined negation. Being and essence relate as mutually indifferent others. Being is unessential, essence is essential existence.'
SET c.keyPoints = ['Essence is sublated being, simple equality with itself', 'Negation of sphere of being in general', 'Has immediacy over against it, preserved in sublating', 'Essence is determined negation', 'Being and essence relate as mutually indifferent others']
SET c.tags = ['negation', 'sublation', 'citta']
SET c.orderInSource = 10
SET c.globalOrder = 10
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Essence is sublated being.\n\nIt is simple equality with itself\nbut is such as the negation of\nthe sphere of being in general.\nAnd so it has immediacy over against it,\nas something from which it has come to be\nbut which has preserved and maintained itself in this sublating.\nEssence itself is in this determination\nan existent immediate essence,\nand with reference to it\nbeing is only something negative,\nnothing in and for itself:\nessence, therefore, is a determined negation.\nBeing and essence relate to each other in this fashion\nas against others in general which are mutually indifferent,\nfor each has a being, an immediacy,\nand according to this being they stand in equal value.\n\nBut as contrasted with essence,\nbeing is at the same time the unessential;\nas against essence, it has the determination of something sublated.\nAnd in so far as it thus relates to essence\nas an other only in general,\nessence itself is not essence proper\nbut is just another existence, the essential.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-10'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-10'})
MATCH (c:IntegratedChunk {id: 'ess-10'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-10'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-10'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-10:kp:1'})
SET kp.chunkId = 'ess-10'
SET kp.ordinal = 1
SET kp.text = 'Essence is sublated being, simple equality with itself';
MATCH (c:IntegratedChunk {id: 'ess-10'})
MATCH (kp:KeyPoint {id: 'ess-10:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-10:kp:2'})
SET kp.chunkId = 'ess-10'
SET kp.ordinal = 2
SET kp.text = 'Negation of sphere of being in general';
MATCH (c:IntegratedChunk {id: 'ess-10'})
MATCH (kp:KeyPoint {id: 'ess-10:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-10:kp:3'})
SET kp.chunkId = 'ess-10'
SET kp.ordinal = 3
SET kp.text = 'Has immediacy over against it, preserved in sublating';
MATCH (c:IntegratedChunk {id: 'ess-10'})
MATCH (kp:KeyPoint {id: 'ess-10:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-10:kp:4'})
SET kp.chunkId = 'ess-10'
SET kp.ordinal = 4
SET kp.text = 'Essence is determined negation';
MATCH (c:IntegratedChunk {id: 'ess-10'})
MATCH (kp:KeyPoint {id: 'ess-10:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-10:kp:5'})
SET kp.chunkId = 'ess-10'
SET kp.ordinal = 5
SET kp.text = 'Being and essence relate as mutually indifferent others';
MATCH (c:IntegratedChunk {id: 'ess-10'})
MATCH (kp:KeyPoint {id: 'ess-10:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-11'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 261
SET segment.lineEnd = 281
SET segment.text = 'The distinction of essential and unessential has\nmade essence relapse into the sphere of existence,\nfor as essence is at first,\nit is determined with respect to being\nas an existent and therefore as an other.\nThe sphere of existence is thus laid out as foundation,\nand that in this sphere being is being-in-and-for-itself,\nis a further determination external to existence,\njust as, contrariwise, essence is indeed being-in-and-for-itself,\nbut only over against an other, in a determinate respect.\nConsequently, inasmuch as essential and unessential aspects are\ndistinguished in an existence from each other,\nthis distinguishing is an external positing,\na taking apart that leaves the existence itself untouched;\nit is a separation which falls on the side of\na third and leaves undetermined\nwhat belongs to the essential\nand what belongs to the unessential.\nIt is dependent on some external standpoint or consideration\nand the same content can therefore sometimes be considered\nas essential, sometimes as unessential.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-11'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-11'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-11'
SET topic.title = 'Distinction relapses into existence — external positing'
SET topic.description = 'Distinction of essential/unessential makes essence relapse into existence. Essence determined as existent, therefore as other. Distinguishing is external positing, separation falling on third. Dependent on external standpoint. Same content can be essential or unessential.'
SET topic.keyPoints = ['Distinction of essential/unessential makes essence relapse into existence', 'Essence determined as existent, therefore as other', 'Distinguishing is external positing, separation falling on third', 'Dependent on external standpoint', 'Same content can be essential or unessential'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-11'})
MATCH (topic:Topic {id: 'topic:ess-11'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-11'})
SET c.title = 'Distinction relapses into existence — external positing'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 261
SET c.lineEnd = 281
SET c.description = 'Distinction of essential/unessential makes essence relapse into existence. Essence determined as existent, therefore as other. Distinguishing is external positing, separation falling on third. Dependent on external standpoint. Same content can be essential or unessential.'
SET c.keyPoints = ['Distinction of essential/unessential makes essence relapse into existence', 'Essence determined as existent, therefore as other', 'Distinguishing is external positing, separation falling on third', 'Dependent on external standpoint', 'Same content can be essential or unessential']
SET c.tags = ['citta']
SET c.orderInSource = 11
SET c.globalOrder = 11
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The distinction of essential and unessential has\nmade essence relapse into the sphere of existence,\nfor as essence is at first,\nit is determined with respect to being\nas an existent and therefore as an other.\nThe sphere of existence is thus laid out as foundation,\nand that in this sphere being is being-in-and-for-itself,\nis a further determination external to existence,\njust as, contrariwise, essence is indeed being-in-and-for-itself,\nbut only over against an other, in a determinate respect.\nConsequently, inasmuch as essential and unessential aspects are\ndistinguished in an existence from each other,\nthis distinguishing is an external positing,\na taking apart that leaves the existence itself untouched;\nit is a separation which falls on the side of\na third and leaves undetermined\nwhat belongs to the essential\nand what belongs to the unessential.\nIt is dependent on some external standpoint or consideration\nand the same content can therefore sometimes be considered\nas essential, sometimes as unessential.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-11'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-11'})
MATCH (c:IntegratedChunk {id: 'ess-11'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-11'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-11'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-11:kp:1'})
SET kp.chunkId = 'ess-11'
SET kp.ordinal = 1
SET kp.text = 'Distinction of essential/unessential makes essence relapse into existence';
MATCH (c:IntegratedChunk {id: 'ess-11'})
MATCH (kp:KeyPoint {id: 'ess-11:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-11:kp:2'})
SET kp.chunkId = 'ess-11'
SET kp.ordinal = 2
SET kp.text = 'Essence determined as existent, therefore as other';
MATCH (c:IntegratedChunk {id: 'ess-11'})
MATCH (kp:KeyPoint {id: 'ess-11:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-11:kp:3'})
SET kp.chunkId = 'ess-11'
SET kp.ordinal = 3
SET kp.text = 'Distinguishing is external positing, separation falling on third';
MATCH (c:IntegratedChunk {id: 'ess-11'})
MATCH (kp:KeyPoint {id: 'ess-11:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-11:kp:4'})
SET kp.chunkId = 'ess-11'
SET kp.ordinal = 4
SET kp.text = 'Dependent on external standpoint';
MATCH (c:IntegratedChunk {id: 'ess-11'})
MATCH (kp:KeyPoint {id: 'ess-11:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-11:kp:5'})
SET kp.chunkId = 'ess-11'
SET kp.ordinal = 5
SET kp.text = 'Same content can be essential or unessential';
MATCH (c:IntegratedChunk {id: 'ess-11'})
MATCH (kp:KeyPoint {id: 'ess-11:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ess-12'})
SET segment.sourceId = 'source-essence'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET segment.lineStart = 283
SET segment.lineEnd = 302
SET segment.text = 'On closer consideration, essence becomes something\nonly essential as contrasted with an unessential\nbecause essence is only taken,\nis as sublated being or existence.\nIn this fashion, essence is only the first negation,\nor the negation, which is determinateness,\nthrough which being becomes only existence,\nor existence only an other.\nBut essence is the absolute negativity of being;\nit is being itself, but not being determined only as an other:\nit is being rather that has sublated itself\nboth as immediate being\nand as immediate negation,\nas the negation which is affected by an otherness.\nBeing or existence, therefore, does not persist\nexcept as what essence is,\nand the immediate which still differs from essence is not just\nan unessential existence but an immediate\nwhich is null in and for itself;\nit only is a non-essence, shine.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (segment:ChunkSegment {id: 'chunk:ess-12'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ess-12'})
SET topic.sourceId = 'source-essence'
SET topic.topicRef = 'ess-12'
SET topic.title = 'Essence as absolute negativity — shine'
SET topic.description = 'Essence only essential as contrasted with unessential because taken as sublated being. First negation: determinateness through which being becomes existence. But essence is absolute negativity of being. Has sublated itself as immediate being and immediate negation. Being does not persist except as what essence is. Immediate differing from essence is null in and for itself: non-essence, shine.'
SET topic.keyPoints = ['Essence only essential as contrasted with unessential because taken as sublated being', 'First negation: determinateness through which being becomes existence', 'But essence is absolute negativity of being', 'Has sublated itself as immediate being and immediate negation', 'Being does not persist except as what essence is', 'Immediate differing from essence is null in and for itself: non-essence, shine'];
MATCH (segment:ChunkSegment {id: 'chunk:ess-12'})
MATCH (topic:Topic {id: 'topic:ess-12'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ess-12'})
SET c.title = 'Essence as absolute negativity — shine'
SET c.sourceId = 'source-essence'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/essence.txt'
SET c.lineStart = 283
SET c.lineEnd = 302
SET c.description = 'Essence only essential as contrasted with unessential because taken as sublated being. First negation: determinateness through which being becomes existence. But essence is absolute negativity of being. Has sublated itself as immediate being and immediate negation. Being does not persist except as what essence is. Immediate differing from essence is null in and for itself: non-essence, shine.'
SET c.keyPoints = ['Essence only essential as contrasted with unessential because taken as sublated being', 'First negation: determinateness through which being becomes existence', 'But essence is absolute negativity of being', 'Has sublated itself as immediate being and immediate negation', 'Being does not persist except as what essence is', 'Immediate differing from essence is null in and for itself: non-essence, shine']
SET c.tags = ['negation', 'sublation', 'shine', 'mediation', 'citta']
SET c.orderInSource = 12
SET c.globalOrder = 12
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'On closer consideration, essence becomes something\nonly essential as contrasted with an unessential\nbecause essence is only taken,\nis as sublated being or existence.\nIn this fashion, essence is only the first negation,\nor the negation, which is determinateness,\nthrough which being becomes only existence,\nor existence only an other.\nBut essence is the absolute negativity of being;\nit is being itself, but not being determined only as an other:\nit is being rather that has sublated itself\nboth as immediate being\nand as immediate negation,\nas the negation which is affected by an otherness.\nBeing or existence, therefore, does not persist\nexcept as what essence is,\nand the immediate which still differs from essence is not just\nan unessential existence but an immediate\nwhich is null in and for itself;\nit only is a non-essence, shine.';
MATCH (s:SourceText {id: 'source-essence'})
MATCH (c:IntegratedChunk {id: 'ess-12'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ess-12'})
MATCH (c:IntegratedChunk {id: 'ess-12'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ess-12'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ess-12'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ess-12:kp:1'})
SET kp.chunkId = 'ess-12'
SET kp.ordinal = 1
SET kp.text = 'Essence only essential as contrasted with unessential because taken as sublated being';
MATCH (c:IntegratedChunk {id: 'ess-12'})
MATCH (kp:KeyPoint {id: 'ess-12:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-12:kp:2'})
SET kp.chunkId = 'ess-12'
SET kp.ordinal = 2
SET kp.text = 'First negation: determinateness through which being becomes existence';
MATCH (c:IntegratedChunk {id: 'ess-12'})
MATCH (kp:KeyPoint {id: 'ess-12:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-12:kp:3'})
SET kp.chunkId = 'ess-12'
SET kp.ordinal = 3
SET kp.text = 'But essence is absolute negativity of being';
MATCH (c:IntegratedChunk {id: 'ess-12'})
MATCH (kp:KeyPoint {id: 'ess-12:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-12:kp:4'})
SET kp.chunkId = 'ess-12'
SET kp.ordinal = 4
SET kp.text = 'Has sublated itself as immediate being and immediate negation';
MATCH (c:IntegratedChunk {id: 'ess-12'})
MATCH (kp:KeyPoint {id: 'ess-12:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-12:kp:5'})
SET kp.chunkId = 'ess-12'
SET kp.ordinal = 5
SET kp.text = 'Being does not persist except as what essence is';
MATCH (c:IntegratedChunk {id: 'ess-12'})
MATCH (kp:KeyPoint {id: 'ess-12:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ess-12:kp:6'})
SET kp.chunkId = 'ess-12'
SET kp.ordinal = 6
SET kp.text = 'Immediate differing from essence is null in and for itself: non-essence, shine';
MATCH (c:IntegratedChunk {id: 'ess-12'})
MATCH (kp:KeyPoint {id: 'ess-12:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
