MERGE (s:SourceText {id: 'source-condition'})
SET s.title = 'Condition'
SET s.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET s.totalLines = 528;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-condition'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:con-1'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 6
SET segment.lineEnd = 14
SET segment.text = '1. Ground is the immediate,\nand the grounded the mediated.\nBut ground is positing reflection;\nas such, it makes itself into positedness\nand is presupposing reflection;\nas such it refers itself to itself\nas to something sublated,\nto an immediate through which\nit is itself mediated.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-1'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-1'
SET topic.title = 'a. The relatively unconditioned — condition as immediate'
SET topic.description = 'Ground is immediate, grounded is mediated. But ground is positing reflection, makes itself into positedness. Is presupposing reflection, refers itself to itself as to something sublated, to immediate through which it is itself mediated. Mediation, as advance from immediate to ground, is not external reflection but ground\'s own doing. Ground-connection as reflection into self-identity is just as essentially self-externalizing reflection. Immediate to which ground refers as to its essential presupposition is condition. Real ground is essentially conditioned.'
SET topic.keyPoints = ['Ground is positing reflection, makes itself into positedness', 'Ground is presupposing reflection, refers to immediate through which mediated', 'Mediation is ground\'s own doing, not external reflection', 'Immediate to which ground refers is condition', 'Real ground is essentially conditioned'];
MATCH (segment:ChunkSegment {id: 'chunk:con-1'})
MATCH (topic:Topic {id: 'topic:con-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-1'})
SET c.title = 'a. The relatively unconditioned — condition as immediate'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 6
SET c.lineEnd = 14
SET c.description = 'Ground is immediate, grounded is mediated. But ground is positing reflection, makes itself into positedness. Is presupposing reflection, refers itself to itself as to something sublated, to immediate through which it is itself mediated. Mediation, as advance from immediate to ground, is not external reflection but ground\'s own doing. Ground-connection as reflection into self-identity is just as essentially self-externalizing reflection. Immediate to which ground refers as to its essential presupposition is condition. Real ground is essentially conditioned.'
SET c.keyPoints = ['Ground is positing reflection, makes itself into positedness', 'Ground is presupposing reflection, refers to immediate through which mediated', 'Mediation is ground\'s own doing, not external reflection', 'Immediate to which ground refers is condition', 'Real ground is essentially conditioned']
SET c.tags = ['sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 103
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '1. Ground is the immediate,\nand the grounded the mediated.\nBut ground is positing reflection;\nas such, it makes itself into positedness\nand is presupposing reflection;\nas such it refers itself to itself\nas to something sublated,\nto an immediate through which\nit is itself mediated.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-1'})
MATCH (c:IntegratedChunk {id: 'con-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-1:kp:1'})
SET kp.chunkId = 'con-1'
SET kp.ordinal = 1
SET kp.text = 'Ground is positing reflection, makes itself into positedness';
MATCH (c:IntegratedChunk {id: 'con-1'})
MATCH (kp:KeyPoint {id: 'con-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-1:kp:2'})
SET kp.chunkId = 'con-1'
SET kp.ordinal = 2
SET kp.text = 'Ground is presupposing reflection, refers to immediate through which mediated';
MATCH (c:IntegratedChunk {id: 'con-1'})
MATCH (kp:KeyPoint {id: 'con-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-1:kp:3'})
SET kp.chunkId = 'con-1'
SET kp.ordinal = 3
SET kp.text = 'Mediation is ground\'s own doing, not external reflection';
MATCH (c:IntegratedChunk {id: 'con-1'})
MATCH (kp:KeyPoint {id: 'con-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-1:kp:4'})
SET kp.chunkId = 'con-1'
SET kp.ordinal = 4
SET kp.text = 'Immediate to which ground refers is condition';
MATCH (c:IntegratedChunk {id: 'con-1'})
MATCH (kp:KeyPoint {id: 'con-1:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-1:kp:5'})
SET kp.chunkId = 'con-1'
SET kp.ordinal = 5
SET kp.text = 'Real ground is essentially conditioned';
MATCH (c:IntegratedChunk {id: 'con-1'})
MATCH (kp:KeyPoint {id: 'con-1:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-2'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 22
SET segment.lineEnd = 71
SET segment.text = 'The immediate to which ground refers as\nto its essential presupposition is condition;\nreal ground is accordingly essentially conditioned.\nThe determinateness that it contains is\nthe otherness of itself.\nCondition is therefore,\nfirst, an immediate, manifold existence.\nSecond, it is this existence referred to an other,\nto something which is ground,\nnot of this existence but in some other respect,\nfor existence itself is immediate and without ground.\nAccording to this reference, it is something posited;\nas condition, the immediate existence is supposed to be\nnot for itself but for another.\nBut this, that it thus is for another, is at the same time\nitself only a positedness;\nthat it is posited is sublated in its immediacy:\nan existence is indifferent to being a condition.\nThird, condition is something immediate in the sense\nthat it constitutes the presupposition of ground.\nIn this determination, it is the form-connection of ground\nwithdrawn into self-identity, hence the content of ground.\nBut content is as such only the indifferent unity of ground,\nas in the form: without form, no content.\nIt nevertheless frees itself\nfrom this indifferent unity\nin that the ground-connection,\nin the complete ground,\nbecomes a connection external to its identity,\nwhereby content acquires immediacy.\nIn so far, therefore, as condition is\nthat in which the ground-connection has\nits identity with itself,\nit constitutes the content of ground;\nbut since this content is indifferent to form,\nit is only implicitly the content of form,\nis something which has yet to become content\nand hence constitutes the material for the ground.\nPosited as condition,\nand in accordance with the second moment,\nexistence is determined to lose its indifferent immediacy\nand to become the moment of another.\nBy virtue of its immediacy, it is indifferent to this connection;\ninasmuch as it enters into it, however,\nit constitutes the in-itself of the ground\nand is for it the unconditioned.\nIn order to be condition,\nit has its presupposition in the ground\nand is itself conditioned;\nbut this condition is external to it.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-2'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-2'
SET topic.title = 'Condition — three moments'
SET topic.description = 'Condition is, first, immediate, manifold existence. Second, existence referred to other, to something which is ground, not of this existence but in some other respect. According to reference, something posited. As condition, immediate existence supposed to be not for itself but for another. That it is for another is itself only positedness. Existence indifferent to being condition. Third, condition is immediate in sense that constitutes presupposition of ground. Form-connection of ground withdrawn into self-identity, hence content of ground. Condition is that in which ground-connection has its identity with itself, constitutes content of ground. But since content indifferent to form, only implicitly content of form, constitutes material for ground. Constitutes in-itself of ground, is for it unconditioned. Has presupposition in ground and is itself conditioned, but condition external to it.'
SET topic.keyPoints = ['Condition is immediate, manifold existence', 'Existence referred to other, to ground, something posited', 'Condition constitutes presupposition of ground', 'Constitutes content of ground, material for ground', 'Constitutes in-itself of ground, is unconditioned'];
MATCH (segment:ChunkSegment {id: 'chunk:con-2'})
MATCH (topic:Topic {id: 'topic:con-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-2'})
SET c.title = 'Condition — three moments'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 22
SET c.lineEnd = 71
SET c.description = 'Condition is, first, immediate, manifold existence. Second, existence referred to other, to something which is ground, not of this existence but in some other respect. According to reference, something posited. As condition, immediate existence supposed to be not for itself but for another. That it is for another is itself only positedness. Existence indifferent to being condition. Third, condition is immediate in sense that constitutes presupposition of ground. Form-connection of ground withdrawn into self-identity, hence content of ground. Condition is that in which ground-connection has its identity with itself, constitutes content of ground. But since content indifferent to form, only implicitly content of form, constitutes material for ground. Constitutes in-itself of ground, is for it unconditioned. Has presupposition in ground and is itself conditioned, but condition external to it.'
SET c.keyPoints = ['Condition is immediate, manifold existence', 'Existence referred to other, to ground, something posited', 'Condition constitutes presupposition of ground', 'Constitutes content of ground, material for ground', 'Constitutes in-itself of ground, is unconditioned']
SET c.tags = ['mediation', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 104
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The immediate to which ground refers as\nto its essential presupposition is condition;\nreal ground is accordingly essentially conditioned.\nThe determinateness that it contains is\nthe otherness of itself.\nCondition is therefore,\nfirst, an immediate, manifold existence.\nSecond, it is this existence referred to an other,\nto something which is ground,\nnot of this existence but in some other respect,\nfor existence itself is immediate and without ground.\nAccording to this reference, it is something posited;\nas condition, the immediate existence is supposed to be\nnot for itself but for another.\nBut this, that it thus is for another, is at the same time\nitself only a positedness;\nthat it is posited is sublated in its immediacy:\nan existence is indifferent to being a condition.\nThird, condition is something immediate in the sense\nthat it constitutes the presupposition of ground.\nIn this determination, it is the form-connection of ground\nwithdrawn into self-identity, hence the content of ground.\nBut content is as such only the indifferent unity of ground,\nas in the form: without form, no content.\nIt nevertheless frees itself\nfrom this indifferent unity\nin that the ground-connection,\nin the complete ground,\nbecomes a connection external to its identity,\nwhereby content acquires immediacy.\nIn so far, therefore, as condition is\nthat in which the ground-connection has\nits identity with itself,\nit constitutes the content of ground;\nbut since this content is indifferent to form,\nit is only implicitly the content of form,\nis something which has yet to become content\nand hence constitutes the material for the ground.\nPosited as condition,\nand in accordance with the second moment,\nexistence is determined to lose its indifferent immediacy\nand to become the moment of another.\nBy virtue of its immediacy, it is indifferent to this connection;\ninasmuch as it enters into it, however,\nit constitutes the in-itself of the ground\nand is for it the unconditioned.\nIn order to be condition,\nit has its presupposition in the ground\nand is itself conditioned;\nbut this condition is external to it.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-2'})
MATCH (c:IntegratedChunk {id: 'con-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-2:kp:1'})
SET kp.chunkId = 'con-2'
SET kp.ordinal = 1
SET kp.text = 'Condition is immediate, manifold existence';
MATCH (c:IntegratedChunk {id: 'con-2'})
MATCH (kp:KeyPoint {id: 'con-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-2:kp:2'})
SET kp.chunkId = 'con-2'
SET kp.ordinal = 2
SET kp.text = 'Existence referred to other, to ground, something posited';
MATCH (c:IntegratedChunk {id: 'con-2'})
MATCH (kp:KeyPoint {id: 'con-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-2:kp:3'})
SET kp.chunkId = 'con-2'
SET kp.ordinal = 3
SET kp.text = 'Condition constitutes presupposition of ground';
MATCH (c:IntegratedChunk {id: 'con-2'})
MATCH (kp:KeyPoint {id: 'con-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-2:kp:4'})
SET kp.chunkId = 'con-2'
SET kp.ordinal = 4
SET kp.text = 'Constitutes content of ground, material for ground';
MATCH (c:IntegratedChunk {id: 'con-2'})
MATCH (kp:KeyPoint {id: 'con-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-2:kp:5'})
SET kp.chunkId = 'con-2'
SET kp.ordinal = 5
SET kp.text = 'Constitutes in-itself of ground, is unconditioned';
MATCH (c:IntegratedChunk {id: 'con-2'})
MATCH (kp:KeyPoint {id: 'con-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-3'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 73
SET segment.lineEnd = 113
SET segment.text = '2. Something is not through its condition;\nits condition is not its ground.\nCondition is for the ground\nthe moment of unconditioned immediacy,\nbut is not itself the movement and the positing\nthat refers itself to itself negatively\nand that makes itself into a positedness.\nOver against condition there stands,\ntherefore, the ground-connection.\nSomething has, besides its condition, also a ground.\nThis ground is the empty movement of reflection,\nfor the latter has the immediacy\nwhich is its presupposition outside it.\nBut it is the whole form\nand the self-subsistent process of mediation,\nfor the condition is not its ground.\nSince this mediating refers itself to itself as positing,\nit equally is according to this side\nsomething immediate and unconditioned;\nit does indeed presuppose itself,\nbut as an externalized or sublated positing;\nwhatever it is in accordance with its determination,\nthat it is, on the contrary, in and for itself.\nInasmuch as the ground-connection is\nthus a self-subsisting self-reference\nand has within it the identity of reflection,\nit has a content which is peculiarly its own\nas against the content of the condition.\nThe one content is that of the ground\nand is therefore essentially informed;\nthe other content, that of the condition,\nis on the contrary only an immediate material\nwhose connecting reference to the ground,\nwhile at the same time constituting\nthe in-itself of the latter,\nis also equally external to it;\nit is thus a mingling of a self-subsisting content\nthat has no reference to the content of the ground determination\nand of the content that enters into the latter\nand, as its material,\nshould become a moment of it.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-3'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-3'
SET topic.title = 'Something has condition and ground'
SET topic.description = 'Something is not through its condition, condition is not its ground. Condition is for ground moment of unconditioned immediacy. Over against condition stands ground-connection. Something has, besides condition, also ground. Ground is empty movement of reflection, whole form and self-subsistent process of mediation. Since mediating refers itself to itself as positing, equally is something immediate and unconditioned. Ground-connection is self-subsisting self-reference, has content peculiarly its own. One content is that of ground, essentially informed. Other content, that of condition, only immediate material. Connecting reference to ground, while constituting in-itself, also external to it.'
SET topic.keyPoints = ['Something has condition and ground', 'Ground is empty movement of reflection, whole form', 'Ground-connection is self-subsisting self-reference', 'Ground content essentially informed, condition content immediate material', 'Condition\'s reference to ground external to it'];
MATCH (segment:ChunkSegment {id: 'chunk:con-3'})
MATCH (topic:Topic {id: 'topic:con-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-3'})
SET c.title = 'Something has condition and ground'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 73
SET c.lineEnd = 113
SET c.description = 'Something is not through its condition, condition is not its ground. Condition is for ground moment of unconditioned immediacy. Over against condition stands ground-connection. Something has, besides condition, also ground. Ground is empty movement of reflection, whole form and self-subsistent process of mediation. Since mediating refers itself to itself as positing, equally is something immediate and unconditioned. Ground-connection is self-subsisting self-reference, has content peculiarly its own. One content is that of ground, essentially informed. Other content, that of condition, only immediate material. Connecting reference to ground, while constituting in-itself, also external to it.'
SET c.keyPoints = ['Something has condition and ground', 'Ground is empty movement of reflection, whole form', 'Ground-connection is self-subsisting self-reference', 'Ground content essentially informed, condition content immediate material', 'Condition\'s reference to ground external to it']
SET c.tags = ['reflection', 'mediation', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 105
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '2. Something is not through its condition;\nits condition is not its ground.\nCondition is for the ground\nthe moment of unconditioned immediacy,\nbut is not itself the movement and the positing\nthat refers itself to itself negatively\nand that makes itself into a positedness.\nOver against condition there stands,\ntherefore, the ground-connection.\nSomething has, besides its condition, also a ground.\nThis ground is the empty movement of reflection,\nfor the latter has the immediacy\nwhich is its presupposition outside it.\nBut it is the whole form\nand the self-subsistent process of mediation,\nfor the condition is not its ground.\nSince this mediating refers itself to itself as positing,\nit equally is according to this side\nsomething immediate and unconditioned;\nit does indeed presuppose itself,\nbut as an externalized or sublated positing;\nwhatever it is in accordance with its determination,\nthat it is, on the contrary, in and for itself.\nInasmuch as the ground-connection is\nthus a self-subsisting self-reference\nand has within it the identity of reflection,\nit has a content which is peculiarly its own\nas against the content of the condition.\nThe one content is that of the ground\nand is therefore essentially informed;\nthe other content, that of the condition,\nis on the contrary only an immediate material\nwhose connecting reference to the ground,\nwhile at the same time constituting\nthe in-itself of the latter,\nis also equally external to it;\nit is thus a mingling of a self-subsisting content\nthat has no reference to the content of the ground determination\nand of the content that enters into the latter\nand, as its material,\nshould become a moment of it.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-3'})
MATCH (c:IntegratedChunk {id: 'con-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-3:kp:1'})
SET kp.chunkId = 'con-3'
SET kp.ordinal = 1
SET kp.text = 'Something has condition and ground';
MATCH (c:IntegratedChunk {id: 'con-3'})
MATCH (kp:KeyPoint {id: 'con-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-3:kp:2'})
SET kp.chunkId = 'con-3'
SET kp.ordinal = 2
SET kp.text = 'Ground is empty movement of reflection, whole form';
MATCH (c:IntegratedChunk {id: 'con-3'})
MATCH (kp:KeyPoint {id: 'con-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-3:kp:3'})
SET kp.chunkId = 'con-3'
SET kp.ordinal = 3
SET kp.text = 'Ground-connection is self-subsisting self-reference';
MATCH (c:IntegratedChunk {id: 'con-3'})
MATCH (kp:KeyPoint {id: 'con-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-3:kp:4'})
SET kp.chunkId = 'con-3'
SET kp.ordinal = 4
SET kp.text = 'Ground content essentially informed, condition content immediate material';
MATCH (c:IntegratedChunk {id: 'con-3'})
MATCH (kp:KeyPoint {id: 'con-3:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-3:kp:5'})
SET kp.chunkId = 'con-3'
SET kp.ordinal = 5
SET kp.text = 'Condition\'s reference to ground external to it';
MATCH (c:IntegratedChunk {id: 'con-3'})
MATCH (kp:KeyPoint {id: 'con-3:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-4'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 115
SET segment.lineEnd = 146
SET segment.text = '3. The two sides of the whole,\ncondition and ground,\nare thus, on the one hand,\nindifferent and unconditioned\nwith respect to each other:\nthe one as the non-referred-to side,\nto which the connecting reference\nin which it is the condition is external;\nthe other as the connecting reference, or form,\nfor which the determinate existence of\nthe condition is only a material,\nsomething passive whose form,\nsuch as it possesses on its own account,\nis unessential.\nOn the other hand, the two sides are also mediated.\nCondition is the in-itself of the ground;\nso much is it the essential moment of the ground-connection,\nthat it is the simple self-identity of the ground.\nBut this also is sublated;\nthis in-itself is only something posited;\nimmediate existence is indifferent to being a condition.\nThe fact, therefore, that condition is the in-itself\nof the ground constitutes the side of it\nby which it is a mediated condition.\nLikewise, the ground-connection has\nin its self-subsistence also a presupposition;\nit has its in-itself outside itself.\nConsequently, each of the two sides is this contradiction,\nthat they are indifferent immediacy and essential mediation,\nboth in one reference\nor the contradiction of independent subsistence\nand of being determined as only moments.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-4'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-4'
SET topic.title = 'Two sides — indifferent and mediated'
SET topic.description = 'Two sides of whole, condition and ground. On one hand, indifferent and unconditioned with respect to each other. One as non-referred-to side, other as connecting reference or form. On other hand, two sides also mediated. Condition is in-itself of ground, essential moment of ground-connection, simple self-identity of ground. But this also sublated, in-itself only something posited. Ground-connection has in self-subsistence also presupposition, has its in-itself outside itself. Each of two sides is contradiction: indifferent immediacy and essential mediation, both in one reference. Contradiction of independent subsistence and being determined as only moments.'
SET topic.keyPoints = ['Two sides indifferent and unconditioned, also mediated', 'Condition is in-itself of ground, simple self-identity of ground', 'Ground-connection has its in-itself outside itself', 'Each side is contradiction: indifferent immediacy and essential mediation', 'Contradiction of independent subsistence and being determined as moments'];
MATCH (segment:ChunkSegment {id: 'chunk:con-4'})
MATCH (topic:Topic {id: 'topic:con-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-4'})
SET c.title = 'Two sides — indifferent and mediated'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 115
SET c.lineEnd = 146
SET c.description = 'Two sides of whole, condition and ground. On one hand, indifferent and unconditioned with respect to each other. One as non-referred-to side, other as connecting reference or form. On other hand, two sides also mediated. Condition is in-itself of ground, essential moment of ground-connection, simple self-identity of ground. But this also sublated, in-itself only something posited. Ground-connection has in self-subsistence also presupposition, has its in-itself outside itself. Each of two sides is contradiction: indifferent immediacy and essential mediation, both in one reference. Contradiction of independent subsistence and being determined as only moments.'
SET c.keyPoints = ['Two sides indifferent and unconditioned, also mediated', 'Condition is in-itself of ground, simple self-identity of ground', 'Ground-connection has its in-itself outside itself', 'Each side is contradiction: indifferent immediacy and essential mediation', 'Contradiction of independent subsistence and being determined as moments']
SET c.tags = ['sublation', 'mediation', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 106
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '3. The two sides of the whole,\ncondition and ground,\nare thus, on the one hand,\nindifferent and unconditioned\nwith respect to each other:\nthe one as the non-referred-to side,\nto which the connecting reference\nin which it is the condition is external;\nthe other as the connecting reference, or form,\nfor which the determinate existence of\nthe condition is only a material,\nsomething passive whose form,\nsuch as it possesses on its own account,\nis unessential.\nOn the other hand, the two sides are also mediated.\nCondition is the in-itself of the ground;\nso much is it the essential moment of the ground-connection,\nthat it is the simple self-identity of the ground.\nBut this also is sublated;\nthis in-itself is only something posited;\nimmediate existence is indifferent to being a condition.\nThe fact, therefore, that condition is the in-itself\nof the ground constitutes the side of it\nby which it is a mediated condition.\nLikewise, the ground-connection has\nin its self-subsistence also a presupposition;\nit has its in-itself outside itself.\nConsequently, each of the two sides is this contradiction,\nthat they are indifferent immediacy and essential mediation,\nboth in one reference\nor the contradiction of independent subsistence\nand of being determined as only moments.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-4'})
MATCH (c:IntegratedChunk {id: 'con-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-4:kp:1'})
SET kp.chunkId = 'con-4'
SET kp.ordinal = 1
SET kp.text = 'Two sides indifferent and unconditioned, also mediated';
MATCH (c:IntegratedChunk {id: 'con-4'})
MATCH (kp:KeyPoint {id: 'con-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-4:kp:2'})
SET kp.chunkId = 'con-4'
SET kp.ordinal = 2
SET kp.text = 'Condition is in-itself of ground, simple self-identity of ground';
MATCH (c:IntegratedChunk {id: 'con-4'})
MATCH (kp:KeyPoint {id: 'con-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-4:kp:3'})
SET kp.chunkId = 'con-4'
SET kp.ordinal = 3
SET kp.text = 'Ground-connection has its in-itself outside itself';
MATCH (c:IntegratedChunk {id: 'con-4'})
MATCH (kp:KeyPoint {id: 'con-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-4:kp:4'})
SET kp.chunkId = 'con-4'
SET kp.ordinal = 4
SET kp.text = 'Each side is contradiction: indifferent immediacy and essential mediation';
MATCH (c:IntegratedChunk {id: 'con-4'})
MATCH (kp:KeyPoint {id: 'con-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-4:kp:5'})
SET kp.chunkId = 'con-4'
SET kp.ordinal = 5
SET kp.text = 'Contradiction of independent subsistence and being determined as moments';
MATCH (c:IntegratedChunk {id: 'con-4'})
MATCH (kp:KeyPoint {id: 'con-4:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-5'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 150
SET segment.lineEnd = 156
SET segment.text = 'At first, each of the two relatively unconditioned sides\nreflectively shines in the other;\ncondition, as an immediate, is reflected\nin the form connection of the ground,\nand this form in the immediate existence as its positedness;\nbut each, apart from this reflective shine of its other in it,\nstands out on its own and has a content of its own.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-5'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-5'
SET topic.title = 'b. The absolutely unconditioned — reflective shine'
SET topic.description = 'At first, each of two relatively unconditioned sides reflectively shines in other. Condition, as immediate, reflected in form connection of ground. Form in immediate existence as its positedness. But each, apart from reflective shine of other in it, stands out on own, has content of own.'
SET topic.keyPoints = ['Two sides reflectively shine in other', 'Each stands out on own, has content of own'];
MATCH (segment:ChunkSegment {id: 'chunk:con-5'})
MATCH (topic:Topic {id: 'topic:con-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-5'})
SET c.title = 'b. The absolutely unconditioned — reflective shine'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 150
SET c.lineEnd = 156
SET c.description = 'At first, each of two relatively unconditioned sides reflectively shines in other. Condition, as immediate, reflected in form connection of ground. Form in immediate existence as its positedness. But each, apart from reflective shine of other in it, stands out on own, has content of own.'
SET c.keyPoints = ['Two sides reflectively shine in other', 'Each stands out on own, has content of own']
SET c.tags = ['reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 5
SET c.globalOrder = 107
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'At first, each of the two relatively unconditioned sides\nreflectively shines in the other;\ncondition, as an immediate, is reflected\nin the form connection of the ground,\nand this form in the immediate existence as its positedness;\nbut each, apart from this reflective shine of its other in it,\nstands out on its own and has a content of its own.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-5'})
MATCH (c:IntegratedChunk {id: 'con-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-5:kp:1'})
SET kp.chunkId = 'con-5'
SET kp.ordinal = 1
SET kp.text = 'Two sides reflectively shine in other';
MATCH (c:IntegratedChunk {id: 'con-5'})
MATCH (kp:KeyPoint {id: 'con-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-5:kp:2'})
SET kp.chunkId = 'con-5'
SET kp.ordinal = 2
SET kp.text = 'Each stands out on own, has content of own';
MATCH (c:IntegratedChunk {id: 'con-5'})
MATCH (kp:KeyPoint {id: 'con-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-6'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 158
SET segment.lineEnd = 180
SET segment.text = 'Condition is at first immediate existence;\nits form has these two moments:\nthat of positedness, according to which it is, as condition,\nmaterial and moment of the ground;\nand that of the in-itself, according to which\nit constitutes the essentiality of ground\nor its simple reflection into itself.\nBoth sides of the form are external to immediate existence,\nfor the latter is the sublated ground-connection.\n\nBut, first, existence is in it only this:\nto sublate itself in its immediacy\nand to founder, going to the ground.\nBeing is as such only the becoming of essence;\nit is its essential nature to\nmake itself into a positedness\nand into an identity which is\nan immediacy through the negation of itself.\nThe form determinations of positedness\nand of self-identical in-itself,\nthe form through which immediate existence is condition,\nare not, therefore, external to that existence;\nthe latter is, rather, this very reflection.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-6'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-6'
SET topic.title = 'Condition\'s form — two moments'
SET topic.description = 'Condition is at first immediate existence. Form has two moments: positedness, according to which it is material and moment of ground; and in-itself, according to which constitutes essentiality of ground or simple reflection into itself. Both sides of form external to immediate existence. But existence is in it only this: sublate itself in immediacy and founder, going to ground. Being is as such only becoming of essence. Essential nature to make itself into positedness and identity which is immediacy through negation of itself. Form determinations are not external to existence, latter is this very reflection.'
SET topic.keyPoints = ['Form has two moments: positedness and in-itself', 'Existence sublates itself in immediacy and founders', 'Being is becoming of essence', 'Form determinations are not external to existence', 'Existence is this very reflection'];
MATCH (segment:ChunkSegment {id: 'chunk:con-6'})
MATCH (topic:Topic {id: 'topic:con-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-6'})
SET c.title = 'Condition\'s form — two moments'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 158
SET c.lineEnd = 180
SET c.description = 'Condition is at first immediate existence. Form has two moments: positedness, according to which it is material and moment of ground; and in-itself, according to which constitutes essentiality of ground or simple reflection into itself. Both sides of form external to immediate existence. But existence is in it only this: sublate itself in immediacy and founder, going to ground. Being is as such only becoming of essence. Essential nature to make itself into positedness and identity which is immediacy through negation of itself. Form determinations are not external to existence, latter is this very reflection.'
SET c.keyPoints = ['Form has two moments: positedness and in-itself', 'Existence sublates itself in immediacy and founders', 'Being is becoming of essence', 'Form determinations are not external to existence', 'Existence is this very reflection']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 6
SET c.globalOrder = 108
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Condition is at first immediate existence;\nits form has these two moments:\nthat of positedness, according to which it is, as condition,\nmaterial and moment of the ground;\nand that of the in-itself, according to which\nit constitutes the essentiality of ground\nor its simple reflection into itself.\nBoth sides of the form are external to immediate existence,\nfor the latter is the sublated ground-connection.\n\nBut, first, existence is in it only this:\nto sublate itself in its immediacy\nand to founder, going to the ground.\nBeing is as such only the becoming of essence;\nit is its essential nature to\nmake itself into a positedness\nand into an identity which is\nan immediacy through the negation of itself.\nThe form determinations of positedness\nand of self-identical in-itself,\nthe form through which immediate existence is condition,\nare not, therefore, external to that existence;\nthe latter is, rather, this very reflection.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-6'})
MATCH (c:IntegratedChunk {id: 'con-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-6:kp:1'})
SET kp.chunkId = 'con-6'
SET kp.ordinal = 1
SET kp.text = 'Form has two moments: positedness and in-itself';
MATCH (c:IntegratedChunk {id: 'con-6'})
MATCH (kp:KeyPoint {id: 'con-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-6:kp:2'})
SET kp.chunkId = 'con-6'
SET kp.ordinal = 2
SET kp.text = 'Existence sublates itself in immediacy and founders';
MATCH (c:IntegratedChunk {id: 'con-6'})
MATCH (kp:KeyPoint {id: 'con-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-6:kp:3'})
SET kp.chunkId = 'con-6'
SET kp.ordinal = 3
SET kp.text = 'Being is becoming of essence';
MATCH (c:IntegratedChunk {id: 'con-6'})
MATCH (kp:KeyPoint {id: 'con-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-6:kp:4'})
SET kp.chunkId = 'con-6'
SET kp.ordinal = 4
SET kp.text = 'Form determinations are not external to existence';
MATCH (c:IntegratedChunk {id: 'con-6'})
MATCH (kp:KeyPoint {id: 'con-6:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-6:kp:5'})
SET kp.chunkId = 'con-6'
SET kp.ordinal = 5
SET kp.text = 'Existence is this very reflection';
MATCH (c:IntegratedChunk {id: 'con-6'})
MATCH (kp:KeyPoint {id: 'con-6:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-7'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 182
SET segment.lineEnd = 208
SET segment.text = 'Second, as condition, being is now posited as\nthat which it essentially is,\nnamely as a moment and consequently as the being of an other,\nand at the same time as the in-itself of an other;\nit is in itself but only through the negation of itself,\nnamely through the ground and through its self-sublating\nand consequent presupposing reflection;\nthe in-itself of being is thus only something posited.\nThis in-itself of the condition has two sides:\none side is its essentiality as essentiality of the ground,\nwhile the other is the immediacy of its existence.\nOr rather, both sides are the same thing.\nExistence is an immediate, but immediacy\nis essentially something mediated,\nnamely through the self-sublating ground.\nExistence, as this immediacy mediated by a self-sublating mediating,\nis at the same time the in-itself of the ground and its unconditioned side;\nbut again, this in-itself is at the same time itself\nequally only moment or positedness, since it is mediated.\nCondition is, therefore, the whole form of the ground-connection;\nit is the presupposed in-itself of the latter,\nbut, consequently, is itself a positedness\nand its immediacy is this, to make itself into a positedness\nand thereby to repel itself from itself,\nin such as way that it both founders to the ground and is ground,\nthe ground that makes itself into a positedness\nand thereby into a grounded, and both are one and the same.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-7'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-7'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-7'
SET topic.title = 'Condition as whole form — ground'
SET topic.description = 'As condition, being posited as that which essentially is: moment and being of other, and in-itself of other. In itself but only through negation of itself, through ground and self-sublating presupposing reflection. In-itself of being only something posited. In-itself of condition has two sides: essentiality as essentiality of ground, immediacy of existence. Both sides same thing. Existence is immediate, but immediacy essentially mediated through self-sublating ground. Existence is at same time in-itself of ground and unconditioned side, but equally only moment or positedness. Condition is whole form of ground-connection. Presupposed in-itself, but consequently itself positedness. Both founders to ground and is ground. Ground that makes itself into positedness and thereby into grounded, both one and same.'
SET topic.keyPoints = ['Condition is moment and in-itself of other', 'In-itself only through negation of itself', 'Existence is immediate but essentially mediated', 'Condition is whole form of ground-connection', 'Both founders to ground and is ground, both one and same'];
MATCH (segment:ChunkSegment {id: 'chunk:con-7'})
MATCH (topic:Topic {id: 'topic:con-7'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-7'})
SET c.title = 'Condition as whole form — ground'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 182
SET c.lineEnd = 208
SET c.description = 'As condition, being posited as that which essentially is: moment and being of other, and in-itself of other. In itself but only through negation of itself, through ground and self-sublating presupposing reflection. In-itself of being only something posited. In-itself of condition has two sides: essentiality as essentiality of ground, immediacy of existence. Both sides same thing. Existence is immediate, but immediacy essentially mediated through self-sublating ground. Existence is at same time in-itself of ground and unconditioned side, but equally only moment or positedness. Condition is whole form of ground-connection. Presupposed in-itself, but consequently itself positedness. Both founders to ground and is ground. Ground that makes itself into positedness and thereby into grounded, both one and same.'
SET c.keyPoints = ['Condition is moment and in-itself of other', 'In-itself only through negation of itself', 'Existence is immediate but essentially mediated', 'Condition is whole form of ground-connection', 'Both founders to ground and is ground, both one and same']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 7
SET c.globalOrder = 109
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Second, as condition, being is now posited as\nthat which it essentially is,\nnamely as a moment and consequently as the being of an other,\nand at the same time as the in-itself of an other;\nit is in itself but only through the negation of itself,\nnamely through the ground and through its self-sublating\nand consequent presupposing reflection;\nthe in-itself of being is thus only something posited.\nThis in-itself of the condition has two sides:\none side is its essentiality as essentiality of the ground,\nwhile the other is the immediacy of its existence.\nOr rather, both sides are the same thing.\nExistence is an immediate, but immediacy\nis essentially something mediated,\nnamely through the self-sublating ground.\nExistence, as this immediacy mediated by a self-sublating mediating,\nis at the same time the in-itself of the ground and its unconditioned side;\nbut again, this in-itself is at the same time itself\nequally only moment or positedness, since it is mediated.\nCondition is, therefore, the whole form of the ground-connection;\nit is the presupposed in-itself of the latter,\nbut, consequently, is itself a positedness\nand its immediacy is this, to make itself into a positedness\nand thereby to repel itself from itself,\nin such as way that it both founders to the ground and is ground,\nthe ground that makes itself into a positedness\nand thereby into a grounded, and both are one and the same.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-7'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-7'})
MATCH (c:IntegratedChunk {id: 'con-7'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-7'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-7'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-7:kp:1'})
SET kp.chunkId = 'con-7'
SET kp.ordinal = 1
SET kp.text = 'Condition is moment and in-itself of other';
MATCH (c:IntegratedChunk {id: 'con-7'})
MATCH (kp:KeyPoint {id: 'con-7:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-7:kp:2'})
SET kp.chunkId = 'con-7'
SET kp.ordinal = 2
SET kp.text = 'In-itself only through negation of itself';
MATCH (c:IntegratedChunk {id: 'con-7'})
MATCH (kp:KeyPoint {id: 'con-7:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-7:kp:3'})
SET kp.chunkId = 'con-7'
SET kp.ordinal = 3
SET kp.text = 'Existence is immediate but essentially mediated';
MATCH (c:IntegratedChunk {id: 'con-7'})
MATCH (kp:KeyPoint {id: 'con-7:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-7:kp:4'})
SET kp.chunkId = 'con-7'
SET kp.ordinal = 4
SET kp.text = 'Condition is whole form of ground-connection';
MATCH (c:IntegratedChunk {id: 'con-7'})
MATCH (kp:KeyPoint {id: 'con-7:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-7:kp:5'})
SET kp.chunkId = 'con-7'
SET kp.ordinal = 5
SET kp.text = 'Both founders to ground and is ground, both one and same';
MATCH (c:IntegratedChunk {id: 'con-7'})
MATCH (kp:KeyPoint {id: 'con-7:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-8'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 210
SET segment.lineEnd = 224
SET segment.text = 'Likewise in the conditioned ground, the in-itself is not\njust as the reflective shining of an other in it.\nThis ground is the self-subsistent,\nthat is, self-referring reflection of the positing,\nand consequently the self-identical;\nor it is in it its in-itself and its content.\nBut it is at the same time presupposing reflection;\nit negatively refers to itself\nand posits its in-itself as an other opposite to it,\nand condition, according to both its moment of in-itself\nand of immediate existence, is\nthe ground-connection\'s own moment;\nthe immediate existence essentially is only through its ground\nand is a moment of itself as a presupposing.\nThis ground, therefore, is equally the whole itself.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-8'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-8'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-8'
SET topic.title = 'Conditioned ground — whole itself'
SET topic.description = 'In conditioned ground, in-itself not just reflective shining of other in it. Ground is self-subsistent, self-referring reflection of positing, self-identical. In it its in-itself and content. But at same time presupposing reflection. Negatively refers to itself, posits in-itself as other opposite to it. Condition, according to both moment of in-itself and immediate existence, is ground-connection\'s own moment. Immediate existence essentially only through ground, moment of itself as presupposing. Ground is equally whole itself.'
SET topic.keyPoints = ['Ground is self-subsistent, self-referring reflection', 'Ground posits in-itself as other opposite to it', 'Condition is ground-connection\'s own moment', 'Immediate existence essentially only through ground', 'Ground is equally whole itself'];
MATCH (segment:ChunkSegment {id: 'chunk:con-8'})
MATCH (topic:Topic {id: 'topic:con-8'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-8'})
SET c.title = 'Conditioned ground — whole itself'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 210
SET c.lineEnd = 224
SET c.description = 'In conditioned ground, in-itself not just reflective shining of other in it. Ground is self-subsistent, self-referring reflection of positing, self-identical. In it its in-itself and content. But at same time presupposing reflection. Negatively refers to itself, posits in-itself as other opposite to it. Condition, according to both moment of in-itself and immediate existence, is ground-connection\'s own moment. Immediate existence essentially only through ground, moment of itself as presupposing. Ground is equally whole itself.'
SET c.keyPoints = ['Ground is self-subsistent, self-referring reflection', 'Ground posits in-itself as other opposite to it', 'Condition is ground-connection\'s own moment', 'Immediate existence essentially only through ground', 'Ground is equally whole itself']
SET c.tags = ['negation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 8
SET c.globalOrder = 110
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Likewise in the conditioned ground, the in-itself is not\njust as the reflective shining of an other in it.\nThis ground is the self-subsistent,\nthat is, self-referring reflection of the positing,\nand consequently the self-identical;\nor it is in it its in-itself and its content.\nBut it is at the same time presupposing reflection;\nit negatively refers to itself\nand posits its in-itself as an other opposite to it,\nand condition, according to both its moment of in-itself\nand of immediate existence, is\nthe ground-connection\'s own moment;\nthe immediate existence essentially is only through its ground\nand is a moment of itself as a presupposing.\nThis ground, therefore, is equally the whole itself.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-8'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-8'})
MATCH (c:IntegratedChunk {id: 'con-8'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-8'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-8'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-8:kp:1'})
SET kp.chunkId = 'con-8'
SET kp.ordinal = 1
SET kp.text = 'Ground is self-subsistent, self-referring reflection';
MATCH (c:IntegratedChunk {id: 'con-8'})
MATCH (kp:KeyPoint {id: 'con-8:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-8:kp:2'})
SET kp.chunkId = 'con-8'
SET kp.ordinal = 2
SET kp.text = 'Ground posits in-itself as other opposite to it';
MATCH (c:IntegratedChunk {id: 'con-8'})
MATCH (kp:KeyPoint {id: 'con-8:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-8:kp:3'})
SET kp.chunkId = 'con-8'
SET kp.ordinal = 3
SET kp.text = 'Condition is ground-connection\'s own moment';
MATCH (c:IntegratedChunk {id: 'con-8'})
MATCH (kp:KeyPoint {id: 'con-8:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-8:kp:4'})
SET kp.chunkId = 'con-8'
SET kp.ordinal = 4
SET kp.text = 'Immediate existence essentially only through ground';
MATCH (c:IntegratedChunk {id: 'con-8'})
MATCH (kp:KeyPoint {id: 'con-8:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-8:kp:5'})
SET kp.chunkId = 'con-8'
SET kp.ordinal = 5
SET kp.text = 'Ground is equally whole itself';
MATCH (c:IntegratedChunk {id: 'con-8'})
MATCH (kp:KeyPoint {id: 'con-8:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-9'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 226
SET segment.lineEnd = 242
SET segment.text = 'What we have here, therefore, is only one whole of form,\nbut equally so only one whole of content.\nFor the proper content of condition is essential content\nonly in so far as it is the self-identity of reflection in the form,\nor the ground-connection is in it this immediate existence.\nFurther, this existence is condition only through\nthe presupposing reflection of the ground;\nit is the ground\'s self-identity, or its content,\nto which the ground posits itself as opposite.\nTherefore, the existence is not a merely\nformless material for the ground-connection;\non the contrary, because it has this form in it, it is informed matter,\nand because in its identity with it it is at the same time\nindifferent to it, it is content.\nFinally, it is the same content as that possessed by the ground,\nfor it is precisely content as that which is self-identical\nin the form connection.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-9'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-9'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-9'
SET topic.title = 'One whole of form and content'
SET topic.description = 'Only one whole of form, equally only one whole of content. Proper content of condition is essential content only in so far as self-identity of reflection in form. Or ground-connection is in it immediate existence. Existence is condition only through presupposing reflection of ground. Ground\'s self-identity, or content, to which ground posits itself as opposite. Existence not formless material, because has form in it, informed matter. In identity with it at same time indifferent to it, is content. Same content as possessed by ground, content as self-identical in form connection.'
SET topic.keyPoints = ['Only one whole of form and content', 'Proper content of condition is self-identity of reflection in form', 'Existence is informed matter, is content', 'Same content as possessed by ground', 'Content as self-identical in form connection'];
MATCH (segment:ChunkSegment {id: 'chunk:con-9'})
MATCH (topic:Topic {id: 'topic:con-9'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-9'})
SET c.title = 'One whole of form and content'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 226
SET c.lineEnd = 242
SET c.description = 'Only one whole of form, equally only one whole of content. Proper content of condition is essential content only in so far as self-identity of reflection in form. Or ground-connection is in it immediate existence. Existence is condition only through presupposing reflection of ground. Ground\'s self-identity, or content, to which ground posits itself as opposite. Existence not formless material, because has form in it, informed matter. In identity with it at same time indifferent to it, is content. Same content as possessed by ground, content as self-identical in form connection.'
SET c.keyPoints = ['Only one whole of form and content', 'Proper content of condition is self-identity of reflection in form', 'Existence is informed matter, is content', 'Same content as possessed by ground', 'Content as self-identical in form connection']
SET c.tags = ['reflection', 'mediation', 'citta']
SET c.orderInSource = 9
SET c.globalOrder = 111
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'What we have here, therefore, is only one whole of form,\nbut equally so only one whole of content.\nFor the proper content of condition is essential content\nonly in so far as it is the self-identity of reflection in the form,\nor the ground-connection is in it this immediate existence.\nFurther, this existence is condition only through\nthe presupposing reflection of the ground;\nit is the ground\'s self-identity, or its content,\nto which the ground posits itself as opposite.\nTherefore, the existence is not a merely\nformless material for the ground-connection;\non the contrary, because it has this form in it, it is informed matter,\nand because in its identity with it it is at the same time\nindifferent to it, it is content.\nFinally, it is the same content as that possessed by the ground,\nfor it is precisely content as that which is self-identical\nin the form connection.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-9'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-9'})
MATCH (c:IntegratedChunk {id: 'con-9'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-9'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-9'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-9:kp:1'})
SET kp.chunkId = 'con-9'
SET kp.ordinal = 1
SET kp.text = 'Only one whole of form and content';
MATCH (c:IntegratedChunk {id: 'con-9'})
MATCH (kp:KeyPoint {id: 'con-9:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-9:kp:2'})
SET kp.chunkId = 'con-9'
SET kp.ordinal = 2
SET kp.text = 'Proper content of condition is self-identity of reflection in form';
MATCH (c:IntegratedChunk {id: 'con-9'})
MATCH (kp:KeyPoint {id: 'con-9:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-9:kp:3'})
SET kp.chunkId = 'con-9'
SET kp.ordinal = 3
SET kp.text = 'Existence is informed matter, is content';
MATCH (c:IntegratedChunk {id: 'con-9'})
MATCH (kp:KeyPoint {id: 'con-9:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-9:kp:4'})
SET kp.chunkId = 'con-9'
SET kp.ordinal = 4
SET kp.text = 'Same content as possessed by ground';
MATCH (c:IntegratedChunk {id: 'con-9'})
MATCH (kp:KeyPoint {id: 'con-9:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-9:kp:5'})
SET kp.chunkId = 'con-9'
SET kp.ordinal = 5
SET kp.text = 'Content as self-identical in form connection';
MATCH (c:IntegratedChunk {id: 'con-9'})
MATCH (kp:KeyPoint {id: 'con-9:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-10'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 244
SET segment.lineEnd = 273
SET segment.text = 'The two sides of the whole,\ncondition and ground,\nare therefore one essential unity,\nas content as well as form.\nThey pass into one another,\nor, since they are reflections,\nthey posit themselves as sublated,\nrefer themselves to this their negation,\nand reciprocally presuppose each other.\nBut this is at the same time only one reflection of the two,\nand their presupposing is, therefore, one presupposing only;\nthe reciprocity of this presupposing ultimately amounts to this,\nthat they both presuppose one identity\nfor their subsistence and their substrate.\nThis substrate, the one content and unity of form of both,\nis the truly unconditioned; the fact in itself.\nCondition is, as it was shown above, only the relatively unconditioned.\nIt is usual, therefore, to consider it as itself something conditioned\nand to ask for a new condition,\nwhereby the customary progression ad infinitum\nfrom condition to condition is set in motion.\nBut now, why is it that at one condition\na new condition is asked for, that is,\nwhy is that condition assumed to be something conditioned?\nBecause it is some finite determinate existence or other.\nBut this is a further determination of condition\nthat does not enter into its concept.\nCondition is as such conditioned solely because\nit is the posited in-itselfness;\nit is, therefore, sublated in the absolutely unconditioned.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-10'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-10'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-10'
SET topic.title = 'One essential unity — absolutely unconditioned'
SET topic.description = 'Two sides of whole, condition and ground, one essential unity, as content as well as form. Pass into one another, posit themselves as sublated, reciprocally presuppose each other. But only one reflection of two, presupposing one presupposing only. Reciprocity amounts to both presuppose one identity for subsistence and substrate. Substrate, one content and unity of form of both, is truly unconditioned, fact in itself. Condition only relatively unconditioned. Usual to consider as itself conditioned, ask for new condition, progression ad infinitum. But condition as such conditioned solely because posited in-itselfness. Sublated in absolutely unconditioned.'
SET topic.keyPoints = ['Two sides one essential unity, as content and form', 'Both presuppose one identity for subsistence and substrate', 'Substrate is truly unconditioned, fact in itself', 'Condition only relatively unconditioned', 'Sublated in absolutely unconditioned'];
MATCH (segment:ChunkSegment {id: 'chunk:con-10'})
MATCH (topic:Topic {id: 'topic:con-10'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-10'})
SET c.title = 'One essential unity — absolutely unconditioned'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 244
SET c.lineEnd = 273
SET c.description = 'Two sides of whole, condition and ground, one essential unity, as content as well as form. Pass into one another, posit themselves as sublated, reciprocally presuppose each other. But only one reflection of two, presupposing one presupposing only. Reciprocity amounts to both presuppose one identity for subsistence and substrate. Substrate, one content and unity of form of both, is truly unconditioned, fact in itself. Condition only relatively unconditioned. Usual to consider as itself conditioned, ask for new condition, progression ad infinitum. But condition as such conditioned solely because posited in-itselfness. Sublated in absolutely unconditioned.'
SET c.keyPoints = ['Two sides one essential unity, as content and form', 'Both presuppose one identity for subsistence and substrate', 'Substrate is truly unconditioned, fact in itself', 'Condition only relatively unconditioned', 'Sublated in absolutely unconditioned']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 10
SET c.globalOrder = 112
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The two sides of the whole,\ncondition and ground,\nare therefore one essential unity,\nas content as well as form.\nThey pass into one another,\nor, since they are reflections,\nthey posit themselves as sublated,\nrefer themselves to this their negation,\nand reciprocally presuppose each other.\nBut this is at the same time only one reflection of the two,\nand their presupposing is, therefore, one presupposing only;\nthe reciprocity of this presupposing ultimately amounts to this,\nthat they both presuppose one identity\nfor their subsistence and their substrate.\nThis substrate, the one content and unity of form of both,\nis the truly unconditioned; the fact in itself.\nCondition is, as it was shown above, only the relatively unconditioned.\nIt is usual, therefore, to consider it as itself something conditioned\nand to ask for a new condition,\nwhereby the customary progression ad infinitum\nfrom condition to condition is set in motion.\nBut now, why is it that at one condition\na new condition is asked for, that is,\nwhy is that condition assumed to be something conditioned?\nBecause it is some finite determinate existence or other.\nBut this is a further determination of condition\nthat does not enter into its concept.\nCondition is as such conditioned solely because\nit is the posited in-itselfness;\nit is, therefore, sublated in the absolutely unconditioned.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-10'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-10'})
MATCH (c:IntegratedChunk {id: 'con-10'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-10'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-10'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-10:kp:1'})
SET kp.chunkId = 'con-10'
SET kp.ordinal = 1
SET kp.text = 'Two sides one essential unity, as content and form';
MATCH (c:IntegratedChunk {id: 'con-10'})
MATCH (kp:KeyPoint {id: 'con-10:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-10:kp:2'})
SET kp.chunkId = 'con-10'
SET kp.ordinal = 2
SET kp.text = 'Both presuppose one identity for subsistence and substrate';
MATCH (c:IntegratedChunk {id: 'con-10'})
MATCH (kp:KeyPoint {id: 'con-10:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-10:kp:3'})
SET kp.chunkId = 'con-10'
SET kp.ordinal = 3
SET kp.text = 'Substrate is truly unconditioned, fact in itself';
MATCH (c:IntegratedChunk {id: 'con-10'})
MATCH (kp:KeyPoint {id: 'con-10:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-10:kp:4'})
SET kp.chunkId = 'con-10'
SET kp.ordinal = 4
SET kp.text = 'Condition only relatively unconditioned';
MATCH (c:IntegratedChunk {id: 'con-10'})
MATCH (kp:KeyPoint {id: 'con-10:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-10:kp:5'})
SET kp.chunkId = 'con-10'
SET kp.ordinal = 5
SET kp.text = 'Sublated in absolutely unconditioned';
MATCH (c:IntegratedChunk {id: 'con-10'})
MATCH (kp:KeyPoint {id: 'con-10:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-11'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 275
SET segment.lineEnd = 307
SET segment.text = 'Now this contains within itself the two sides,\ncondition and ground, as its moments;\nit is the unity to which they have returned.\nTogether, the two constitute its form or its positedness.\nThe unconditioned fact is the condition of both,\nbut the condition which is absolute, that is to say,\none which is itself ground.\nAs ground, the fact is now the negative identity\nthat has repelled itself into those two moments:\nfirst, in the shape of the sublated ground-connection,\nthe shape of an immediate manifold void of\nunity and external to itself,\none that refers to the ground as an other to it\nand at the same time constitutes its in-itself;\nsecond, in the shape of an inner, simple form which is ground,\nbut which refers to the self-identical\nimmediate as to an other, determining it as condition,\nthat is, determining the in-itself of it as its own moment.\nThese two sides presuppose the totality,\npresuppose that it is that which posits them.\nContrariwise, because they presuppose the totality,\nthe latter seems to be in turn also conditioned by them,\nand the fact to spring forth from its condition and its ground.\nBut since these two sides have shown themselves to be an identity,\nthe relation of condition and ground has disappeared;\nthe two are reduced to a mere reflective shine;\nthe absolutely unconditioned is in its movement of positing\nand presupposing only the movement in which this shine sublates itself.\nIt is the fact\'s own doing that it conditions itself\nand places itself as ground over against its conditions;\nbut in connecting conditions and ground,\nthe fact is a reflection shining in itself;\nits relation to them is a rejoining itself.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-11'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-11'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-11'
SET topic.title = 'Absolutely unconditioned — fact in itself'
SET topic.description = 'Absolutely unconditioned contains two sides, condition and ground, as moments. Unity to which they returned. Together constitute its form or positedness. Unconditioned fact is condition of both, but condition which is absolute, one which is itself ground. As ground, fact is negative identity repelled into two moments: sublated ground-connection, immediate manifold void of unity; and inner simple form which is ground. Two sides presuppose totality, presuppose it posits them. But since two sides identity, relation of condition and ground disappeared. Two reduced to mere reflective shine. Absolutely unconditioned in movement of positing and presupposing only movement in which shine sublates itself. Fact\'s own doing that conditions itself and places itself as ground. In connecting conditions and ground, fact reflection shining in itself, relation to them rejoining itself.'
SET topic.keyPoints = ['Absolutely unconditioned contains condition and ground as moments', 'Unconditioned fact is condition of both, itself ground', 'Two sides presuppose totality', 'Relation of condition and ground disappeared, reduced to reflective shine', 'Fact reflection shining in itself, rejoining itself'];
MATCH (segment:ChunkSegment {id: 'chunk:con-11'})
MATCH (topic:Topic {id: 'topic:con-11'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-11'})
SET c.title = 'Absolutely unconditioned — fact in itself'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 275
SET c.lineEnd = 307
SET c.description = 'Absolutely unconditioned contains two sides, condition and ground, as moments. Unity to which they returned. Together constitute its form or positedness. Unconditioned fact is condition of both, but condition which is absolute, one which is itself ground. As ground, fact is negative identity repelled into two moments: sublated ground-connection, immediate manifold void of unity; and inner simple form which is ground. Two sides presuppose totality, presuppose it posits them. But since two sides identity, relation of condition and ground disappeared. Two reduced to mere reflective shine. Absolutely unconditioned in movement of positing and presupposing only movement in which shine sublates itself. Fact\'s own doing that conditions itself and places itself as ground. In connecting conditions and ground, fact reflection shining in itself, relation to them rejoining itself.'
SET c.keyPoints = ['Absolutely unconditioned contains condition and ground as moments', 'Unconditioned fact is condition of both, itself ground', 'Two sides presuppose totality', 'Relation of condition and ground disappeared, reduced to reflective shine', 'Fact reflection shining in itself, rejoining itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 11
SET c.globalOrder = 113
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Now this contains within itself the two sides,\ncondition and ground, as its moments;\nit is the unity to which they have returned.\nTogether, the two constitute its form or its positedness.\nThe unconditioned fact is the condition of both,\nbut the condition which is absolute, that is to say,\none which is itself ground.\nAs ground, the fact is now the negative identity\nthat has repelled itself into those two moments:\nfirst, in the shape of the sublated ground-connection,\nthe shape of an immediate manifold void of\nunity and external to itself,\none that refers to the ground as an other to it\nand at the same time constitutes its in-itself;\nsecond, in the shape of an inner, simple form which is ground,\nbut which refers to the self-identical\nimmediate as to an other, determining it as condition,\nthat is, determining the in-itself of it as its own moment.\nThese two sides presuppose the totality,\npresuppose that it is that which posits them.\nContrariwise, because they presuppose the totality,\nthe latter seems to be in turn also conditioned by them,\nand the fact to spring forth from its condition and its ground.\nBut since these two sides have shown themselves to be an identity,\nthe relation of condition and ground has disappeared;\nthe two are reduced to a mere reflective shine;\nthe absolutely unconditioned is in its movement of positing\nand presupposing only the movement in which this shine sublates itself.\nIt is the fact\'s own doing that it conditions itself\nand places itself as ground over against its conditions;\nbut in connecting conditions and ground,\nthe fact is a reflection shining in itself;\nits relation to them is a rejoining itself.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-11'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-11'})
MATCH (c:IntegratedChunk {id: 'con-11'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-11'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-11'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-11:kp:1'})
SET kp.chunkId = 'con-11'
SET kp.ordinal = 1
SET kp.text = 'Absolutely unconditioned contains condition and ground as moments';
MATCH (c:IntegratedChunk {id: 'con-11'})
MATCH (kp:KeyPoint {id: 'con-11:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-11:kp:2'})
SET kp.chunkId = 'con-11'
SET kp.ordinal = 2
SET kp.text = 'Unconditioned fact is condition of both, itself ground';
MATCH (c:IntegratedChunk {id: 'con-11'})
MATCH (kp:KeyPoint {id: 'con-11:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-11:kp:3'})
SET kp.chunkId = 'con-11'
SET kp.ordinal = 3
SET kp.text = 'Two sides presuppose totality';
MATCH (c:IntegratedChunk {id: 'con-11'})
MATCH (kp:KeyPoint {id: 'con-11:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-11:kp:4'})
SET kp.chunkId = 'con-11'
SET kp.ordinal = 4
SET kp.text = 'Relation of condition and ground disappeared, reduced to reflective shine';
MATCH (c:IntegratedChunk {id: 'con-11'})
MATCH (kp:KeyPoint {id: 'con-11:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-11:kp:5'})
SET kp.chunkId = 'con-11'
SET kp.ordinal = 5
SET kp.text = 'Fact reflection shining in itself, rejoining itself';
MATCH (c:IntegratedChunk {id: 'con-11'})
MATCH (kp:KeyPoint {id: 'con-11:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-12'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 311
SET segment.lineEnd = 319
SET segment.text = 'The absolutely unconditioned is the absolute ground\nthat is identical with its condition,\nthe immediate fact as the truly essential.\nAs ground, it refers negatively to itself\nand makes itself into a positedness;\nbut this positedness is a reflection\nthat is complete in both its sides\nand is in them the self-identical form of connection,\nas has transpired from its concept.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-12'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-12'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-12'
SET topic.title = 'c. Procession — absolutely unconditioned as absolute ground'
SET topic.description = 'Absolutely unconditioned is absolute ground identical with condition, immediate fact as truly essential. As ground, refers negatively to itself, makes itself into positedness. Positedness is reflection complete in both sides, self-identical form of connection. Positedness is first sublated ground, fact as immediacy void of reflection, side of conditions.'
SET topic.keyPoints = ['Absolutely unconditioned is absolute ground identical with condition', 'Makes itself into positedness', 'Positedness is reflection complete in both sides', 'Positedness is sublated ground, side of conditions'];
MATCH (segment:ChunkSegment {id: 'chunk:con-12'})
MATCH (topic:Topic {id: 'topic:con-12'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-12'})
SET c.title = 'c. Procession — absolutely unconditioned as absolute ground'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 311
SET c.lineEnd = 319
SET c.description = 'Absolutely unconditioned is absolute ground identical with condition, immediate fact as truly essential. As ground, refers negatively to itself, makes itself into positedness. Positedness is reflection complete in both sides, self-identical form of connection. Positedness is first sublated ground, fact as immediacy void of reflection, side of conditions.'
SET c.keyPoints = ['Absolutely unconditioned is absolute ground identical with condition', 'Makes itself into positedness', 'Positedness is reflection complete in both sides', 'Positedness is sublated ground, side of conditions']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 12
SET c.globalOrder = 114
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The absolutely unconditioned is the absolute ground\nthat is identical with its condition,\nthe immediate fact as the truly essential.\nAs ground, it refers negatively to itself\nand makes itself into a positedness;\nbut this positedness is a reflection\nthat is complete in both its sides\nand is in them the self-identical form of connection,\nas has transpired from its concept.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-12'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-12'})
MATCH (c:IntegratedChunk {id: 'con-12'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-12'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-12'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-12:kp:1'})
SET kp.chunkId = 'con-12'
SET kp.ordinal = 1
SET kp.text = 'Absolutely unconditioned is absolute ground identical with condition';
MATCH (c:IntegratedChunk {id: 'con-12'})
MATCH (kp:KeyPoint {id: 'con-12:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-12:kp:2'})
SET kp.chunkId = 'con-12'
SET kp.ordinal = 2
SET kp.text = 'Makes itself into positedness';
MATCH (c:IntegratedChunk {id: 'con-12'})
MATCH (kp:KeyPoint {id: 'con-12:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-12:kp:3'})
SET kp.chunkId = 'con-12'
SET kp.ordinal = 3
SET kp.text = 'Positedness is reflection complete in both sides';
MATCH (c:IntegratedChunk {id: 'con-12'})
MATCH (kp:KeyPoint {id: 'con-12:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-12:kp:4'})
SET kp.chunkId = 'con-12'
SET kp.ordinal = 4
SET kp.text = 'Positedness is sublated ground, side of conditions';
MATCH (c:IntegratedChunk {id: 'con-12'})
MATCH (kp:KeyPoint {id: 'con-12:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-13'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 320
SET segment.lineEnd = 380
SET segment.text = 'This positedness is therefore first the sublated ground,\nthe fact as an immediacy void of reflection,\nthe side of the conditions.\nThis is the totality of the determinations of the fact,\nthe fact itself, but the fact as thrown into\nthe externality of being, the restored circle of being.\nIn condition, essence lets go of the unity of its immanent reflection;\nbut it lets it go as an immediacy that now carries\nthe character of being a conditioning presupposition\nand of essentially constituting only one of its sides.\nFor this reason the conditions are the whole content of the fact,\nbecause they are the unconditioned in the form of formless being.\nBut because of this form, they also have yet another shape besides\nthe conditions of the content as this is in the fact as such.\nThey appear as a manifold without unity,\nmingled with extra-essential elements\nand other circumstances that do not belong\nto the circle of existence as constituting\nthe conditions of this determinate fact.\nFor the absolute, unrestricted fact,\nthe sphere of being itself is the condition.\nThe ground, returning into itself, posits\nthat sphere as the first immediacy\nto which it refers as to its unconditioned.\nThis immediacy, as sublated reflection,\nis reflection in the element of being,\nwhich thus forms itself as such into a whole;\nform proliferates as determinateness of being\nand thus appears as a manifold distinct from\nthe determination of reflection\nand as a content indifferent to it.\nThe unessential, which is in the sphere of being\nbut which the latter sheds in so far as it is condition,\nis the determinateness of the immediacy into which\nthe unity of form has sunk.\nThis unity of form, as the connection of being,\nis in the latter at first as becoming the passing over\nof a determinateness of being into another.\nBut the becoming of being is also the coming to be\nof essence and a return to the ground.\nThe existence that constitutes the conditions, therefore,\nis in truth not determined as condition by an other\nand is not used by it as material;\non the contrary, it itself makes itself, through itself,\ninto the moment of an other.\nFurther, the becoming of this existence\ndoes not start off from itself\nas if it were truly the first and immediate;\non the contrary, its immediacy is\nsomething only presupposed, and the movement of\nits becoming is the doing of reflection itself.\nThe truth of existence is thus that it is condition;\nits immediacy is solely by virtue of\nthe reflection of the ground-connection\nthat posits itself as sublated.\nConsequently, like immediacy, becoming is only\nthe reflective shine of the unconditioned\ninasmuch as this presupposes itself\nand has its form in this presupposing,\nand hence the immediacy of being is essentially\nonly a moment of the form.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-13'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-13'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-13'
SET topic.title = 'Conditions — totality of determinations'
SET topic.description = 'Totality of determinations of fact, fact as thrown into externality of being, restored circle of being. In condition, essence lets go unity of immanent reflection. Conditions are whole content of fact, unconditioned in form of formless being. Appear as manifold without unity, mingled with extra-essential elements. For absolute fact, sphere of being itself is condition. Ground posits sphere as first immediacy. Immediacy, as sublated reflection, is reflection in element of being. Form proliferates as determinateness of being. Existence that constitutes conditions not determined as condition by other, itself makes itself into moment of other. Becoming does not start from itself, immediacy only presupposed, movement doing of reflection itself. Truth of existence is that it is condition. Immediacy essentially only moment of form.'
SET topic.keyPoints = ['Conditions are totality of determinations, whole content of fact', 'For absolute fact, sphere of being itself is condition', 'Existence itself makes itself into moment of other', 'Truth of existence is that it is condition', 'Immediacy essentially only moment of form'];
MATCH (segment:ChunkSegment {id: 'chunk:con-13'})
MATCH (topic:Topic {id: 'topic:con-13'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-13'})
SET c.title = 'Conditions — totality of determinations'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 320
SET c.lineEnd = 380
SET c.description = 'Totality of determinations of fact, fact as thrown into externality of being, restored circle of being. In condition, essence lets go unity of immanent reflection. Conditions are whole content of fact, unconditioned in form of formless being. Appear as manifold without unity, mingled with extra-essential elements. For absolute fact, sphere of being itself is condition. Ground posits sphere as first immediacy. Immediacy, as sublated reflection, is reflection in element of being. Form proliferates as determinateness of being. Existence that constitutes conditions not determined as condition by other, itself makes itself into moment of other. Becoming does not start from itself, immediacy only presupposed, movement doing of reflection itself. Truth of existence is that it is condition. Immediacy essentially only moment of form.'
SET c.keyPoints = ['Conditions are totality of determinations, whole content of fact', 'For absolute fact, sphere of being itself is condition', 'Existence itself makes itself into moment of other', 'Truth of existence is that it is condition', 'Immediacy essentially only moment of form']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 13
SET c.globalOrder = 115
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'This positedness is therefore first the sublated ground,\nthe fact as an immediacy void of reflection,\nthe side of the conditions.\nThis is the totality of the determinations of the fact,\nthe fact itself, but the fact as thrown into\nthe externality of being, the restored circle of being.\nIn condition, essence lets go of the unity of its immanent reflection;\nbut it lets it go as an immediacy that now carries\nthe character of being a conditioning presupposition\nand of essentially constituting only one of its sides.\nFor this reason the conditions are the whole content of the fact,\nbecause they are the unconditioned in the form of formless being.\nBut because of this form, they also have yet another shape besides\nthe conditions of the content as this is in the fact as such.\nThey appear as a manifold without unity,\nmingled with extra-essential elements\nand other circumstances that do not belong\nto the circle of existence as constituting\nthe conditions of this determinate fact.\nFor the absolute, unrestricted fact,\nthe sphere of being itself is the condition.\nThe ground, returning into itself, posits\nthat sphere as the first immediacy\nto which it refers as to its unconditioned.\nThis immediacy, as sublated reflection,\nis reflection in the element of being,\nwhich thus forms itself as such into a whole;\nform proliferates as determinateness of being\nand thus appears as a manifold distinct from\nthe determination of reflection\nand as a content indifferent to it.\nThe unessential, which is in the sphere of being\nbut which the latter sheds in so far as it is condition,\nis the determinateness of the immediacy into which\nthe unity of form has sunk.\nThis unity of form, as the connection of being,\nis in the latter at first as becoming the passing over\nof a determinateness of being into another.\nBut the becoming of being is also the coming to be\nof essence and a return to the ground.\nThe existence that constitutes the conditions, therefore,\nis in truth not determined as condition by an other\nand is not used by it as material;\non the contrary, it itself makes itself, through itself,\ninto the moment of an other.\nFurther, the becoming of this existence\ndoes not start off from itself\nas if it were truly the first and immediate;\non the contrary, its immediacy is\nsomething only presupposed, and the movement of\nits becoming is the doing of reflection itself.\nThe truth of existence is thus that it is condition;\nits immediacy is solely by virtue of\nthe reflection of the ground-connection\nthat posits itself as sublated.\nConsequently, like immediacy, becoming is only\nthe reflective shine of the unconditioned\ninasmuch as this presupposes itself\nand has its form in this presupposing,\nand hence the immediacy of being is essentially\nonly a moment of the form.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-13'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-13'})
MATCH (c:IntegratedChunk {id: 'con-13'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-13'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-13'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-13:kp:1'})
SET kp.chunkId = 'con-13'
SET kp.ordinal = 1
SET kp.text = 'Conditions are totality of determinations, whole content of fact';
MATCH (c:IntegratedChunk {id: 'con-13'})
MATCH (kp:KeyPoint {id: 'con-13:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-13:kp:2'})
SET kp.chunkId = 'con-13'
SET kp.ordinal = 2
SET kp.text = 'For absolute fact, sphere of being itself is condition';
MATCH (c:IntegratedChunk {id: 'con-13'})
MATCH (kp:KeyPoint {id: 'con-13:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-13:kp:3'})
SET kp.chunkId = 'con-13'
SET kp.ordinal = 3
SET kp.text = 'Existence itself makes itself into moment of other';
MATCH (c:IntegratedChunk {id: 'con-13'})
MATCH (kp:KeyPoint {id: 'con-13:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-13:kp:4'})
SET kp.chunkId = 'con-13'
SET kp.ordinal = 4
SET kp.text = 'Truth of existence is that it is condition';
MATCH (c:IntegratedChunk {id: 'con-13'})
MATCH (kp:KeyPoint {id: 'con-13:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-13:kp:5'})
SET kp.chunkId = 'con-13'
SET kp.ordinal = 5
SET kp.text = 'Immediacy essentially only moment of form';
MATCH (c:IntegratedChunk {id: 'con-13'})
MATCH (kp:KeyPoint {id: 'con-13:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-14'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 382
SET segment.lineEnd = 438
SET segment.text = 'The other side of this reflective shining of\nthe unconditioned is the ground-connection as such,\ndetermined as form as against the immediacy\nof the conditions and the content.\nBut this side is the form of the absolute fact\nthat possesses the unity of its form with itself\nor its content within it,\nand, in determining this content as condition,\nin this very positing sublates the diversity of the content\nand reduces it to a moment;\njust as, contrariwise, as a form void of essence,\nin this self-identity it gives itself\nthe immediacy of subsistence.\nThe reflection of the ground\nsublates the immediacy of the conditions,\nconnecting them and making them\nmoments within the unity of the fact;\nbut the conditions are that which\nthe unconditioned fact itself presupposes\nand the latter, therefore, sublates its own positing;\nconsequently, its positing converts itself\njust as immediately into a becoming.\nThe two, therefore, are one unity;\nthe internal movement of the conditions is a becoming,\nthe return into the ground and the positing of the ground;\nbut the ground as posited, and this means as sublated, is the immediate.\nThe ground refers negatively to itself,\nmakes itself into a positedness and grounds the conditions;\nin this, however, in that the immediate existence is\nthus determined as a positedness,\nthe ground sublates it and only then makes itself into a ground.\nThis reflection is therefore the self-mediation of\nthe unconditioned fact through its negation.\nOr rather, the reflection of the unconditioned is\nat first a presupposing,\nbut this sublating of itself is\nimmediately a positing which determines;\nsecondly, in this positing the reflection is\nimmediately the sublating of the presupposed\nand a determining from within itself;\nthis determining is thus in turn the sublating of the positing:\nit is a becoming within itself.\nIn this, the mediation as a turning back\nto itself through negation has disappeared;\nmediation is simple reflection\nreflectively shining within itself\nand groundless, absolute becoming.\nThe fact\'s movement of being posited,\non the one hand through its conditions,\nand on the other hand through its ground,\nnow is the disappearing of\nthe reflective shine of mediation.\nThe process by which the fact is posited\nis accordingly a coming forth,\nthe simple self-staging of\nthe fact in concrete existence,\nthe pure movement of the fact to itself.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-14'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-14'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-14'
SET topic.title = 'Ground-connection — form'
SET topic.description = 'Other side is ground-connection as such, determined as form. Form of absolute fact that possesses unity of form with itself or content within it. In determining content as condition, sublates diversity of content, reduces to moment. Reflection of ground sublates immediacy of conditions, connecting them, making moments within unity of fact. But conditions are that which unconditioned fact presupposes. Latter sublates own positing, positing converts immediately into becoming. Two one unity: internal movement is becoming, return into ground and positing of ground. Ground refers negatively to itself, makes itself into positedness, grounds conditions. Reflection is self-mediation of unconditioned fact through its negation. Reflection is presupposing, sublating of itself immediately positing. In positing, immediately sublating of presupposed. Mediation as turning back through negation has disappeared. Mediation is simple reflection reflectively shining within itself, groundless, absolute becoming. Process is coming forth, simple self-staging of fact in concrete existence, pure movement of fact to itself.'
SET topic.keyPoints = ['Ground-connection determined as form', 'Sublates immediacy of conditions, making moments within unity', 'Positing converts immediately into becoming', 'Self-mediation through negation', 'Mediation is groundless, absolute becoming, pure movement to itself'];
MATCH (segment:ChunkSegment {id: 'chunk:con-14'})
MATCH (topic:Topic {id: 'topic:con-14'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-14'})
SET c.title = 'Ground-connection — form'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 382
SET c.lineEnd = 438
SET c.description = 'Other side is ground-connection as such, determined as form. Form of absolute fact that possesses unity of form with itself or content within it. In determining content as condition, sublates diversity of content, reduces to moment. Reflection of ground sublates immediacy of conditions, connecting them, making moments within unity of fact. But conditions are that which unconditioned fact presupposes. Latter sublates own positing, positing converts immediately into becoming. Two one unity: internal movement is becoming, return into ground and positing of ground. Ground refers negatively to itself, makes itself into positedness, grounds conditions. Reflection is self-mediation of unconditioned fact through its negation. Reflection is presupposing, sublating of itself immediately positing. In positing, immediately sublating of presupposed. Mediation as turning back through negation has disappeared. Mediation is simple reflection reflectively shining within itself, groundless, absolute becoming. Process is coming forth, simple self-staging of fact in concrete existence, pure movement of fact to itself.'
SET c.keyPoints = ['Ground-connection determined as form', 'Sublates immediacy of conditions, making moments within unity', 'Positing converts immediately into becoming', 'Self-mediation through negation', 'Mediation is groundless, absolute becoming, pure movement to itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 14
SET c.globalOrder = 116
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The other side of this reflective shining of\nthe unconditioned is the ground-connection as such,\ndetermined as form as against the immediacy\nof the conditions and the content.\nBut this side is the form of the absolute fact\nthat possesses the unity of its form with itself\nor its content within it,\nand, in determining this content as condition,\nin this very positing sublates the diversity of the content\nand reduces it to a moment;\njust as, contrariwise, as a form void of essence,\nin this self-identity it gives itself\nthe immediacy of subsistence.\nThe reflection of the ground\nsublates the immediacy of the conditions,\nconnecting them and making them\nmoments within the unity of the fact;\nbut the conditions are that which\nthe unconditioned fact itself presupposes\nand the latter, therefore, sublates its own positing;\nconsequently, its positing converts itself\njust as immediately into a becoming.\nThe two, therefore, are one unity;\nthe internal movement of the conditions is a becoming,\nthe return into the ground and the positing of the ground;\nbut the ground as posited, and this means as sublated, is the immediate.\nThe ground refers negatively to itself,\nmakes itself into a positedness and grounds the conditions;\nin this, however, in that the immediate existence is\nthus determined as a positedness,\nthe ground sublates it and only then makes itself into a ground.\nThis reflection is therefore the self-mediation of\nthe unconditioned fact through its negation.\nOr rather, the reflection of the unconditioned is\nat first a presupposing,\nbut this sublating of itself is\nimmediately a positing which determines;\nsecondly, in this positing the reflection is\nimmediately the sublating of the presupposed\nand a determining from within itself;\nthis determining is thus in turn the sublating of the positing:\nit is a becoming within itself.\nIn this, the mediation as a turning back\nto itself through negation has disappeared;\nmediation is simple reflection\nreflectively shining within itself\nand groundless, absolute becoming.\nThe fact\'s movement of being posited,\non the one hand through its conditions,\nand on the other hand through its ground,\nnow is the disappearing of\nthe reflective shine of mediation.\nThe process by which the fact is posited\nis accordingly a coming forth,\nthe simple self-staging of\nthe fact in concrete existence,\nthe pure movement of the fact to itself.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-14'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-14'})
MATCH (c:IntegratedChunk {id: 'con-14'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-14'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-14'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-14:kp:1'})
SET kp.chunkId = 'con-14'
SET kp.ordinal = 1
SET kp.text = 'Ground-connection determined as form';
MATCH (c:IntegratedChunk {id: 'con-14'})
MATCH (kp:KeyPoint {id: 'con-14:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-14:kp:2'})
SET kp.chunkId = 'con-14'
SET kp.ordinal = 2
SET kp.text = 'Sublates immediacy of conditions, making moments within unity';
MATCH (c:IntegratedChunk {id: 'con-14'})
MATCH (kp:KeyPoint {id: 'con-14:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-14:kp:3'})
SET kp.chunkId = 'con-14'
SET kp.ordinal = 3
SET kp.text = 'Positing converts immediately into becoming';
MATCH (c:IntegratedChunk {id: 'con-14'})
MATCH (kp:KeyPoint {id: 'con-14:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-14:kp:4'})
SET kp.chunkId = 'con-14'
SET kp.ordinal = 4
SET kp.text = 'Self-mediation through negation';
MATCH (c:IntegratedChunk {id: 'con-14'})
MATCH (kp:KeyPoint {id: 'con-14:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-14:kp:5'})
SET kp.chunkId = 'con-14'
SET kp.ordinal = 5
SET kp.text = 'Mediation is groundless, absolute becoming, pure movement to itself';
MATCH (c:IntegratedChunk {id: 'con-14'})
MATCH (kp:KeyPoint {id: 'con-14:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-15'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 440
SET segment.lineEnd = 489
SET segment.text = 'When all the conditions of a fact are at hand,\nthe fact steps into concrete existence.\nThe fact is, before it exists concretely;\nit is, first, as essence or as unconditioned;\nsecond, it has immediate existence or is determined,\nand this in the twofold manner just considered,\non the one hand in its conditions\nand on the other in its ground.\nIn the former case, it has given itself the form\nof the external, groundless being,\nfor as absolute reflection the fact is\nnegative self-reference\nand makes itself into its presupposition.\nThis presupposed unconditioned is,\ntherefore, the groundless immediate\nwhose being is just to be there, without grounds.\nIf, therefore, all the conditions of the fact are at hand,\nthat is, if the totality of the fact is\nposited as a groundless immediate,\nthen this scattered manifold\ninternally recollects itself.\nThe whole fact must be there,\nwithin its conditions,\nor all the conditions belong\nto its concrete existence;\nfor the all of them constitutes\nthe reflection of the fact.\nOr again, immediate existence,\nsince it is condition,\nis determined by form;\nits determinations are therefore determinations of reflection\nand with the positing of one the rest also are essentially posited.\nThe recollecting of the conditions is at first\nthe foundering to the ground of immediate existence\nand the coming to be of the ground.\nBut the ground is thereby a posited ground, that is,\nto the extent that it is ground,\nto that extent it is sublated as ground\nand is immediate being.\nIf, therefore, all the conditions of the fact are at hand,\nthey sublate themselves as immediate existence and as presupposition,\nand the ground is equally sublated.\nThe latter proves to be only a reflective shine\nthat immediately disappears;\nthis coming forth is thus the tautological movement\nof the fact to itself:\nits mediation through the conditions and through the ground\nis the disappearing of both of these.\nThe coming forth into concrete existence is therefore so immediate,\nthat it is mediated only by the disappearing of the mediation.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-15'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-15'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-15'
SET topic.title = 'All conditions at hand — concrete existence'
SET topic.description = 'When all conditions of fact at hand, fact steps into concrete existence. Fact is, before exists concretely: as essence or unconditioned, and immediate existence or determined. In former case, given itself form of external, groundless being. As absolute reflection, makes itself into presupposition. Presupposed unconditioned is groundless immediate whose being is to be there, without grounds. If all conditions at hand, scattered manifold internally recollects itself. Whole fact must be there within conditions. All conditions belong to concrete existence. Recollecting is foundering to ground and coming to be of ground. Ground is posited ground, to extent ground, to that extent sublated as ground, is immediate being. If all conditions at hand, sublate themselves, ground equally sublated. Latter proves only reflective shine that immediately disappears. Coming forth is tautological movement of fact to itself. Mediation through conditions and ground is disappearing of both. Coming forth so immediate, mediated only by disappearing of mediation.'
SET topic.keyPoints = ['When all conditions at hand, fact steps into concrete existence', 'Presupposed unconditioned is groundless immediate', 'Scattered manifold internally recollects itself', 'Ground sublated, proves only reflective shine', 'Coming forth mediated only by disappearing of mediation'];
MATCH (segment:ChunkSegment {id: 'chunk:con-15'})
MATCH (topic:Topic {id: 'topic:con-15'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-15'})
SET c.title = 'All conditions at hand — concrete existence'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 440
SET c.lineEnd = 489
SET c.description = 'When all conditions of fact at hand, fact steps into concrete existence. Fact is, before exists concretely: as essence or unconditioned, and immediate existence or determined. In former case, given itself form of external, groundless being. As absolute reflection, makes itself into presupposition. Presupposed unconditioned is groundless immediate whose being is to be there, without grounds. If all conditions at hand, scattered manifold internally recollects itself. Whole fact must be there within conditions. All conditions belong to concrete existence. Recollecting is foundering to ground and coming to be of ground. Ground is posited ground, to extent ground, to that extent sublated as ground, is immediate being. If all conditions at hand, sublate themselves, ground equally sublated. Latter proves only reflective shine that immediately disappears. Coming forth is tautological movement of fact to itself. Mediation through conditions and ground is disappearing of both. Coming forth so immediate, mediated only by disappearing of mediation.'
SET c.keyPoints = ['When all conditions at hand, fact steps into concrete existence', 'Presupposed unconditioned is groundless immediate', 'Scattered manifold internally recollects itself', 'Ground sublated, proves only reflective shine', 'Coming forth mediated only by disappearing of mediation']
SET c.tags = ['sublation', 'reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 15
SET c.globalOrder = 117
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'When all the conditions of a fact are at hand,\nthe fact steps into concrete existence.\nThe fact is, before it exists concretely;\nit is, first, as essence or as unconditioned;\nsecond, it has immediate existence or is determined,\nand this in the twofold manner just considered,\non the one hand in its conditions\nand on the other in its ground.\nIn the former case, it has given itself the form\nof the external, groundless being,\nfor as absolute reflection the fact is\nnegative self-reference\nand makes itself into its presupposition.\nThis presupposed unconditioned is,\ntherefore, the groundless immediate\nwhose being is just to be there, without grounds.\nIf, therefore, all the conditions of the fact are at hand,\nthat is, if the totality of the fact is\nposited as a groundless immediate,\nthen this scattered manifold\ninternally recollects itself.\nThe whole fact must be there,\nwithin its conditions,\nor all the conditions belong\nto its concrete existence;\nfor the all of them constitutes\nthe reflection of the fact.\nOr again, immediate existence,\nsince it is condition,\nis determined by form;\nits determinations are therefore determinations of reflection\nand with the positing of one the rest also are essentially posited.\nThe recollecting of the conditions is at first\nthe foundering to the ground of immediate existence\nand the coming to be of the ground.\nBut the ground is thereby a posited ground, that is,\nto the extent that it is ground,\nto that extent it is sublated as ground\nand is immediate being.\nIf, therefore, all the conditions of the fact are at hand,\nthey sublate themselves as immediate existence and as presupposition,\nand the ground is equally sublated.\nThe latter proves to be only a reflective shine\nthat immediately disappears;\nthis coming forth is thus the tautological movement\nof the fact to itself:\nits mediation through the conditions and through the ground\nis the disappearing of both of these.\nThe coming forth into concrete existence is therefore so immediate,\nthat it is mediated only by the disappearing of the mediation.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-15'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-15'})
MATCH (c:IntegratedChunk {id: 'con-15'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-15'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-15'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-15:kp:1'})
SET kp.chunkId = 'con-15'
SET kp.ordinal = 1
SET kp.text = 'When all conditions at hand, fact steps into concrete existence';
MATCH (c:IntegratedChunk {id: 'con-15'})
MATCH (kp:KeyPoint {id: 'con-15:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-15:kp:2'})
SET kp.chunkId = 'con-15'
SET kp.ordinal = 2
SET kp.text = 'Presupposed unconditioned is groundless immediate';
MATCH (c:IntegratedChunk {id: 'con-15'})
MATCH (kp:KeyPoint {id: 'con-15:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-15:kp:3'})
SET kp.chunkId = 'con-15'
SET kp.ordinal = 3
SET kp.text = 'Scattered manifold internally recollects itself';
MATCH (c:IntegratedChunk {id: 'con-15'})
MATCH (kp:KeyPoint {id: 'con-15:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-15:kp:4'})
SET kp.chunkId = 'con-15'
SET kp.ordinal = 4
SET kp.text = 'Ground sublated, proves only reflective shine';
MATCH (c:IntegratedChunk {id: 'con-15'})
MATCH (kp:KeyPoint {id: 'con-15:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-15:kp:5'})
SET kp.chunkId = 'con-15'
SET kp.ordinal = 5
SET kp.text = 'Coming forth mediated only by disappearing of mediation';
MATCH (c:IntegratedChunk {id: 'con-15'})
MATCH (kp:KeyPoint {id: 'con-15:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:con-16'})
SET segment.sourceId = 'source-condition'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET segment.lineStart = 491
SET segment.lineEnd = 527
SET segment.text = 'The fact proceeds from the ground.\nIt is not grounded or posited by it\nin such a manner that the ground\nwould still stay underneath, as a substrate;\non the contrary, the positing is\nthe outward movement of ground to itself\nand the simple disappearing of it.\nThrough its union with the conditions,\nit obtains the external immediacy\nand the moment of being.\nBut it does not obtain them\nas a something external,\nnor by referring to them externally;\nrather, as ground it makes\nitself into a positedness;\nits simple essentiality rejoins\nitself in the positedness\nand, in this sublating of itself,\nit is the disappearing of\nits difference from its positedness,\nand is thus simple essential immediacy.\nIt does not, therefore, linger on\nas something distinct from the grounded;\non the contrary, the truth of the grounding is\nthat in grounding the ground unites with itself,\nand its reflection into another is\nconsequently its reflection into itself.\nThe fact is thus the unconditioned\nand, as such, equally so the groundless;\nit arises from the ground only in so far as\nthe latter has foundered and is no longer ground:\nit rises up from the groundless, that is,\nfrom its own essential negativity or pure form.\n\nThis immediacy, mediated by ground and condition\nand self-identical through the sublating of mediation,\nis concrete existence.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (segment:ChunkSegment {id: 'chunk:con-16'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:con-16'})
SET topic.sourceId = 'source-condition'
SET topic.topicRef = 'con-16'
SET topic.title = 'Fact proceeds from ground — concrete existence'
SET topic.description = 'Fact proceeds from ground. Not grounded in manner ground stays underneath as substrate. Positing is outward movement of ground to itself and simple disappearing of it. Through union with conditions, obtains external immediacy and moment of being. As ground makes itself into positedness. Simple essentiality rejoins itself in positedness. In sublating itself, disappearing of difference from positedness, simple essential immediacy. Truth of grounding is in grounding ground unites with itself. Reflection into another is reflection into itself. Fact is unconditioned and, as such, groundless. Arises from ground only in so far as latter foundered and no longer ground. Rises up from groundless, from own essential negativity or pure form. Immediacy, mediated by ground and condition and self-identical through sublating of mediation, is concrete existence.'
SET topic.keyPoints = ['Fact proceeds from ground, positing is disappearing of ground', 'Ground makes itself into positedness, rejoins itself', 'Truth of grounding is ground unites with itself', 'Fact is unconditioned and groundless', 'Immediacy mediated by ground and condition, self-identical through sublating of mediation, is concrete existence'];
MATCH (segment:ChunkSegment {id: 'chunk:con-16'})
MATCH (topic:Topic {id: 'topic:con-16'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'con-16'})
SET c.title = 'Fact proceeds from ground — concrete existence'
SET c.sourceId = 'source-condition'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/condition.txt'
SET c.lineStart = 491
SET c.lineEnd = 527
SET c.description = 'Fact proceeds from ground. Not grounded in manner ground stays underneath as substrate. Positing is outward movement of ground to itself and simple disappearing of it. Through union with conditions, obtains external immediacy and moment of being. As ground makes itself into positedness. Simple essentiality rejoins itself in positedness. In sublating itself, disappearing of difference from positedness, simple essential immediacy. Truth of grounding is in grounding ground unites with itself. Reflection into another is reflection into itself. Fact is unconditioned and, as such, groundless. Arises from ground only in so far as latter foundered and no longer ground. Rises up from groundless, from own essential negativity or pure form. Immediacy, mediated by ground and condition and self-identical through sublating of mediation, is concrete existence.'
SET c.keyPoints = ['Fact proceeds from ground, positing is disappearing of ground', 'Ground makes itself into positedness, rejoins itself', 'Truth of grounding is ground unites with itself', 'Fact is unconditioned and groundless', 'Immediacy mediated by ground and condition, self-identical through sublating of mediation, is concrete existence']
SET c.tags = ['sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 16
SET c.globalOrder = 118
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The fact proceeds from the ground.\nIt is not grounded or posited by it\nin such a manner that the ground\nwould still stay underneath, as a substrate;\non the contrary, the positing is\nthe outward movement of ground to itself\nand the simple disappearing of it.\nThrough its union with the conditions,\nit obtains the external immediacy\nand the moment of being.\nBut it does not obtain them\nas a something external,\nnor by referring to them externally;\nrather, as ground it makes\nitself into a positedness;\nits simple essentiality rejoins\nitself in the positedness\nand, in this sublating of itself,\nit is the disappearing of\nits difference from its positedness,\nand is thus simple essential immediacy.\nIt does not, therefore, linger on\nas something distinct from the grounded;\non the contrary, the truth of the grounding is\nthat in grounding the ground unites with itself,\nand its reflection into another is\nconsequently its reflection into itself.\nThe fact is thus the unconditioned\nand, as such, equally so the groundless;\nit arises from the ground only in so far as\nthe latter has foundered and is no longer ground:\nit rises up from the groundless, that is,\nfrom its own essential negativity or pure form.\n\nThis immediacy, mediated by ground and condition\nand self-identical through the sublating of mediation,\nis concrete existence.';
MATCH (s:SourceText {id: 'source-condition'})
MATCH (c:IntegratedChunk {id: 'con-16'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:con-16'})
MATCH (c:IntegratedChunk {id: 'con-16'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'con-16'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'con-16'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'con-16:kp:1'})
SET kp.chunkId = 'con-16'
SET kp.ordinal = 1
SET kp.text = 'Fact proceeds from ground, positing is disappearing of ground';
MATCH (c:IntegratedChunk {id: 'con-16'})
MATCH (kp:KeyPoint {id: 'con-16:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-16:kp:2'})
SET kp.chunkId = 'con-16'
SET kp.ordinal = 2
SET kp.text = 'Ground makes itself into positedness, rejoins itself';
MATCH (c:IntegratedChunk {id: 'con-16'})
MATCH (kp:KeyPoint {id: 'con-16:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-16:kp:3'})
SET kp.chunkId = 'con-16'
SET kp.ordinal = 3
SET kp.text = 'Truth of grounding is ground unites with itself';
MATCH (c:IntegratedChunk {id: 'con-16'})
MATCH (kp:KeyPoint {id: 'con-16:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-16:kp:4'})
SET kp.chunkId = 'con-16'
SET kp.ordinal = 4
SET kp.text = 'Fact is unconditioned and groundless';
MATCH (c:IntegratedChunk {id: 'con-16'})
MATCH (kp:KeyPoint {id: 'con-16:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'con-16:kp:5'})
SET kp.chunkId = 'con-16'
SET kp.ordinal = 5
SET kp.text = 'Immediacy mediated by ground and condition, self-identical through sublating of mediation, is concrete existence';
MATCH (c:IntegratedChunk {id: 'con-16'})
MATCH (kp:KeyPoint {id: 'con-16:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
