MERGE (s:SourceText {id: 'source-reflection'})
SET s.title = 'Reflection'
SET s.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET s.totalLines = 482;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-reflection'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:ref-1'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 4
SET segment.lineEnd = 8
SET segment.text = 'Shine is the same as what reflection is;\nbut it is reflection as immediate.\nFor this shine which is internalized\nand therefore alienated from its immediacy,\nthe German has a word from an alien language, “Reflexion.”';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-1'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-1'
SET topic.title = 'Introduction — shine and reflection'
SET topic.description = 'Shine is the same as reflection, but reflection as immediate. Internalized shine alienated from immediacy. German: \'Reflexion\' from alien language.'
SET topic.keyPoints = ['Shine is the same as reflection, but reflection as immediate', 'Internalized shine alienated from immediacy', 'German: \'Reflexion\' from alien language'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-1'})
MATCH (topic:Topic {id: 'topic:ref-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-1'})
SET c.title = 'Introduction — shine and reflection'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 4
SET c.lineEnd = 8
SET c.description = 'Shine is the same as reflection, but reflection as immediate. Internalized shine alienated from immediacy. German: \'Reflexion\' from alien language.'
SET c.keyPoints = ['Shine is the same as reflection, but reflection as immediate', 'Internalized shine alienated from immediacy', 'German: \'Reflexion\' from alien language']
SET c.tags = ['reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 13
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Shine is the same as what reflection is;\nbut it is reflection as immediate.\nFor this shine which is internalized\nand therefore alienated from its immediacy,\nthe German has a word from an alien language, “Reflexion.”';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-1'})
MATCH (c:IntegratedChunk {id: 'ref-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-1:kp:1'})
SET kp.chunkId = 'ref-1'
SET kp.ordinal = 1
SET kp.text = 'Shine is the same as reflection, but reflection as immediate';
MATCH (c:IntegratedChunk {id: 'ref-1'})
MATCH (kp:KeyPoint {id: 'ref-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-1:kp:2'})
SET kp.chunkId = 'ref-1'
SET kp.ordinal = 2
SET kp.text = 'Internalized shine alienated from immediacy';
MATCH (c:IntegratedChunk {id: 'ref-1'})
MATCH (kp:KeyPoint {id: 'ref-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-1:kp:3'})
SET kp.chunkId = 'ref-1'
SET kp.ordinal = 3
SET kp.text = 'German: \'Reflexion\' from alien language';
MATCH (c:IntegratedChunk {id: 'ref-1'})
MATCH (kp:KeyPoint {id: 'ref-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-2'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 10
SET segment.lineEnd = 54
SET segment.text = 'Essence is reflection, the movement of\nbecoming and transition that remains within itself,\nwherein that which is distinguished is determined\nsimply and solely as the negative in itself, as shine.\nIn the becoming of being, it is being which lies\nat the foundation of determinateness,\nand determinateness is reference to an other.\nReflective movement is by contrast\nthe other as negation in itself,\na negation which has being only as self-referring.\nOr, since this self-referring is\nprecisely this negating of negation,\nwhat we have is negation as negation,\nnegation that has its being in its being-negated, as shine.\nHere, therefore, the other is not\nbeing with negation or limit,\nbut negation with negation.\nBut the first over against this other,\nthe immediate or being,\nis only this self-equality itself of negation,\nthe negated negation, the absolute negativity.\nThis self-equality or immediacy, therefore, is\nnot a first from which the beginning is made\nand which would pass over into its negation;\nnor is there an existent substrate which would\ngo through the moves of reflection;\nimmediacy is rather just this movement itself.\n\nIn essence, therefore, the becoming,\nthe reflective movement of essence,\nis the movement from nothing to nothing\nand thereby back to itself.\nTransition or becoming sublates itself in its transition;\nthe other which comes to be in this transition is\nnot the non-being of a being but the nothingness of a nothingness,\nand this, to be the negation of a nothingness, constitutes being.\nBeing is only as the movement of nothingness to nothingness,\nand so it is essence;\nand this essence does not have this movement in itself,\nbut the movement is rather the absolute shine itself,\nthe pure negativity which has nothing outside it\nwhich it would negate but which rather negates only its negative,\nthe negative which is only in this negating.\nThis pure absolute reflection, which is the movement\nfrom nothing to nothing, further determines itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-2'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-2'
SET topic.title = 'Essence as reflection — movement from nothing to nothing'
SET topic.description = 'Essence is reflection, movement of becoming that remains within itself. Distinguished determined as negative in itself, as shine. Reflective movement: other as negation in itself, self-referring. Negation as negation, negation with negation. Immediacy is the movement itself, not substrate. Movement from nothing to nothing, back to itself. Being only as movement of nothingness to nothingness. Pure absolute reflection further determines itself.'
SET topic.keyPoints = ['Essence is reflection, movement of becoming that remains within itself', 'Distinguished determined as negative in itself, as shine', 'Reflective movement: other as negation in itself, self-referring', 'Movement from nothing to nothing, back to itself', 'Being only as movement of nothingness to nothingness'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-2'})
MATCH (topic:Topic {id: 'topic:ref-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-2'})
SET c.title = 'Essence as reflection — movement from nothing to nothing'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 10
SET c.lineEnd = 54
SET c.description = 'Essence is reflection, movement of becoming that remains within itself. Distinguished determined as negative in itself, as shine. Reflective movement: other as negation in itself, self-referring. Negation as negation, negation with negation. Immediacy is the movement itself, not substrate. Movement from nothing to nothing, back to itself. Being only as movement of nothingness to nothingness. Pure absolute reflection further determines itself.'
SET c.keyPoints = ['Essence is reflection, movement of becoming that remains within itself', 'Distinguished determined as negative in itself, as shine', 'Reflective movement: other as negation in itself, self-referring', 'Movement from nothing to nothing, back to itself', 'Being only as movement of nothingness to nothingness']
SET c.tags = ['negation', 'reflection', 'shine', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 14
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Essence is reflection, the movement of\nbecoming and transition that remains within itself,\nwherein that which is distinguished is determined\nsimply and solely as the negative in itself, as shine.\nIn the becoming of being, it is being which lies\nat the foundation of determinateness,\nand determinateness is reference to an other.\nReflective movement is by contrast\nthe other as negation in itself,\na negation which has being only as self-referring.\nOr, since this self-referring is\nprecisely this negating of negation,\nwhat we have is negation as negation,\nnegation that has its being in its being-negated, as shine.\nHere, therefore, the other is not\nbeing with negation or limit,\nbut negation with negation.\nBut the first over against this other,\nthe immediate or being,\nis only this self-equality itself of negation,\nthe negated negation, the absolute negativity.\nThis self-equality or immediacy, therefore, is\nnot a first from which the beginning is made\nand which would pass over into its negation;\nnor is there an existent substrate which would\ngo through the moves of reflection;\nimmediacy is rather just this movement itself.\n\nIn essence, therefore, the becoming,\nthe reflective movement of essence,\nis the movement from nothing to nothing\nand thereby back to itself.\nTransition or becoming sublates itself in its transition;\nthe other which comes to be in this transition is\nnot the non-being of a being but the nothingness of a nothingness,\nand this, to be the negation of a nothingness, constitutes being.\nBeing is only as the movement of nothingness to nothingness,\nand so it is essence;\nand this essence does not have this movement in itself,\nbut the movement is rather the absolute shine itself,\nthe pure negativity which has nothing outside it\nwhich it would negate but which rather negates only its negative,\nthe negative which is only in this negating.\nThis pure absolute reflection, which is the movement\nfrom nothing to nothing, further determines itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-2'})
MATCH (c:IntegratedChunk {id: 'ref-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-2:kp:1'})
SET kp.chunkId = 'ref-2'
SET kp.ordinal = 1
SET kp.text = 'Essence is reflection, movement of becoming that remains within itself';
MATCH (c:IntegratedChunk {id: 'ref-2'})
MATCH (kp:KeyPoint {id: 'ref-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-2:kp:2'})
SET kp.chunkId = 'ref-2'
SET kp.ordinal = 2
SET kp.text = 'Distinguished determined as negative in itself, as shine';
MATCH (c:IntegratedChunk {id: 'ref-2'})
MATCH (kp:KeyPoint {id: 'ref-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-2:kp:3'})
SET kp.chunkId = 'ref-2'
SET kp.ordinal = 3
SET kp.text = 'Reflective movement: other as negation in itself, self-referring';
MATCH (c:IntegratedChunk {id: 'ref-2'})
MATCH (kp:KeyPoint {id: 'ref-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-2:kp:4'})
SET kp.chunkId = 'ref-2'
SET kp.ordinal = 4
SET kp.text = 'Movement from nothing to nothing, back to itself';
MATCH (c:IntegratedChunk {id: 'ref-2'})
MATCH (kp:KeyPoint {id: 'ref-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-2:kp:5'})
SET kp.chunkId = 'ref-2'
SET kp.ordinal = 5
SET kp.text = 'Being only as movement of nothingness to nothingness';
MATCH (c:IntegratedChunk {id: 'ref-2'})
MATCH (kp:KeyPoint {id: 'ref-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-3'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 56
SET segment.lineEnd = 65
SET segment.text = 'It is, first, positing reflection.\n\nSecond, it takes as its starting point\nthe presupposed immediate,\nand then it is external reflection.\n\nThird, it sublates however this presupposition,\nand because in the sublating of the presupposition\nit presupposes at the same time,\nit is determining reflection.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-3'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-3'
SET topic.title = 'Three types of reflection'
SET topic.description = 'First: positing reflection. Second: external reflection (takes presupposed immediate as starting point). Third: determining reflection (sublates presupposition, presupposes at same time).'
SET topic.keyPoints = ['First: positing reflection', 'Second: external reflection (presupposed immediate as starting point)', 'Third: determining reflection (sublates presupposition, presupposes at same time)'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-3'})
MATCH (topic:Topic {id: 'topic:ref-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-3'})
SET c.title = 'Three types of reflection'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 56
SET c.lineEnd = 65
SET c.description = 'First: positing reflection. Second: external reflection (takes presupposed immediate as starting point). Third: determining reflection (sublates presupposition, presupposes at same time).'
SET c.keyPoints = ['First: positing reflection', 'Second: external reflection (presupposed immediate as starting point)', 'Third: determining reflection (sublates presupposition, presupposes at same time)']
SET c.tags = ['sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 15
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'It is, first, positing reflection.\n\nSecond, it takes as its starting point\nthe presupposed immediate,\nand then it is external reflection.\n\nThird, it sublates however this presupposition,\nand because in the sublating of the presupposition\nit presupposes at the same time,\nit is determining reflection.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-3'})
MATCH (c:IntegratedChunk {id: 'ref-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-3:kp:1'})
SET kp.chunkId = 'ref-3'
SET kp.ordinal = 1
SET kp.text = 'First: positing reflection';
MATCH (c:IntegratedChunk {id: 'ref-3'})
MATCH (kp:KeyPoint {id: 'ref-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-3:kp:2'})
SET kp.chunkId = 'ref-3'
SET kp.ordinal = 2
SET kp.text = 'Second: external reflection (presupposed immediate as starting point)';
MATCH (c:IntegratedChunk {id: 'ref-3'})
MATCH (kp:KeyPoint {id: 'ref-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-3:kp:3'})
SET kp.chunkId = 'ref-3'
SET kp.ordinal = 3
SET kp.text = 'Third: determining reflection (sublates presupposition, presupposes at same time)';
MATCH (c:IntegratedChunk {id: 'ref-3'})
MATCH (kp:KeyPoint {id: 'ref-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-4'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 69
SET segment.lineEnd = 84
SET segment.text = 'Shine is a nothingness or a lack of essence.\nBut a nothingness or that which is void of essence\ndoes not have its being in an other in which it shines,\nbut its being is its own equality with itself;\nthis conversion of the negative with itself has been determined\nas the absolute reflection of essence.\n\nThis self-referring negativity is\ntherefore the negating of itself.\nIt is thus just as much\nsublated negativity as it is negativity.\nOr again, it is itself the negative\nand the simple equality with itself or immediacy.\nIt consists, therefore, in being itself\nand not being itself,\nand the two in one unity.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-4'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-4'
SET topic.title = 'Positing reflection — introduction'
SET topic.description = 'Shine is nothingness or lack of essence. Nothingness has being as its own equality with itself. Conversion of negative with itself: absolute reflection. Self-referring negativity is negating of itself. Sublated negativity as it is negativity. Being itself and not being itself, two in one unity.'
SET topic.keyPoints = ['Shine is nothingness or lack of essence', 'Nothingness has being as its own equality with itself', 'Self-referring negativity is negating of itself', 'Being itself and not being itself, two in one unity'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-4'})
MATCH (topic:Topic {id: 'topic:ref-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-4'})
SET c.title = 'Positing reflection — introduction'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 69
SET c.lineEnd = 84
SET c.description = 'Shine is nothingness or lack of essence. Nothingness has being as its own equality with itself. Conversion of negative with itself: absolute reflection. Self-referring negativity is negating of itself. Sublated negativity as it is negativity. Being itself and not being itself, two in one unity.'
SET c.keyPoints = ['Shine is nothingness or lack of essence', 'Nothingness has being as its own equality with itself', 'Self-referring negativity is negating of itself', 'Being itself and not being itself, two in one unity']
SET c.tags = ['negation', 'sublation', 'reflection', 'shine', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 16
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Shine is a nothingness or a lack of essence.\nBut a nothingness or that which is void of essence\ndoes not have its being in an other in which it shines,\nbut its being is its own equality with itself;\nthis conversion of the negative with itself has been determined\nas the absolute reflection of essence.\n\nThis self-referring negativity is\ntherefore the negating of itself.\nIt is thus just as much\nsublated negativity as it is negativity.\nOr again, it is itself the negative\nand the simple equality with itself or immediacy.\nIt consists, therefore, in being itself\nand not being itself,\nand the two in one unity.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-4'})
MATCH (c:IntegratedChunk {id: 'ref-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-4:kp:1'})
SET kp.chunkId = 'ref-4'
SET kp.ordinal = 1
SET kp.text = 'Shine is nothingness or lack of essence';
MATCH (c:IntegratedChunk {id: 'ref-4'})
MATCH (kp:KeyPoint {id: 'ref-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-4:kp:2'})
SET kp.chunkId = 'ref-4'
SET kp.ordinal = 2
SET kp.text = 'Nothingness has being as its own equality with itself';
MATCH (c:IntegratedChunk {id: 'ref-4'})
MATCH (kp:KeyPoint {id: 'ref-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-4:kp:3'})
SET kp.chunkId = 'ref-4'
SET kp.ordinal = 3
SET kp.text = 'Self-referring negativity is negating of itself';
MATCH (c:IntegratedChunk {id: 'ref-4'})
MATCH (kp:KeyPoint {id: 'ref-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-4:kp:4'})
SET kp.chunkId = 'ref-4'
SET kp.ordinal = 4
SET kp.text = 'Being itself and not being itself, two in one unity';
MATCH (c:IntegratedChunk {id: 'ref-4'})
MATCH (kp:KeyPoint {id: 'ref-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-5'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 86
SET segment.lineEnd = 106
SET segment.text = 'Reflection is at first the movement of\nthe nothing to the nothing,\nand thus negation coinciding with itself.\nThis self-coinciding is in general\nsimple equality with itself, immediacy.\nBut this falling together is not\nthe transition of negation into equality\nas into a being other than it;\nreflection is transition rather\nas the sublating of transition,\nfor it is the immediate falling together\nof the negative with itself.\nAnd so this coinciding is, first,\nself-equality or immediacy;\nbut, second, this immediacy is\nthe self-equality of the negative,\nand hence self-negating equality,\nimmediacy which is in itself the negative,\nthe negative of itself:\nits being is to be what it is not.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-5'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-5'
SET topic.title = 'Positing reflection — movement from nothing to nothing'
SET topic.description = 'Reflection is movement from nothing to nothing. Negation coinciding with itself. Self-coinciding is simple equality with itself, immediacy. Not transition into equality as being other than it. Reflection is transition as sublating of transition. Immediacy is self-equality of negative, self-negating equality. Its being is to be what it is not.'
SET topic.keyPoints = ['Reflection is movement from nothing to nothing', 'Negation coinciding with itself', 'Self-coinciding is simple equality with itself, immediacy', 'Reflection is transition as sublating of transition', 'Its being is to be what it is not'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-5'})
MATCH (topic:Topic {id: 'topic:ref-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-5'})
SET c.title = 'Positing reflection — movement from nothing to nothing'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 86
SET c.lineEnd = 106
SET c.description = 'Reflection is movement from nothing to nothing. Negation coinciding with itself. Self-coinciding is simple equality with itself, immediacy. Not transition into equality as being other than it. Reflection is transition as sublating of transition. Immediacy is self-equality of negative, self-negating equality. Its being is to be what it is not.'
SET c.keyPoints = ['Reflection is movement from nothing to nothing', 'Negation coinciding with itself', 'Self-coinciding is simple equality with itself, immediacy', 'Reflection is transition as sublating of transition', 'Its being is to be what it is not']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 5
SET c.globalOrder = 17
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Reflection is at first the movement of\nthe nothing to the nothing,\nand thus negation coinciding with itself.\nThis self-coinciding is in general\nsimple equality with itself, immediacy.\nBut this falling together is not\nthe transition of negation into equality\nas into a being other than it;\nreflection is transition rather\nas the sublating of transition,\nfor it is the immediate falling together\nof the negative with itself.\nAnd so this coinciding is, first,\nself-equality or immediacy;\nbut, second, this immediacy is\nthe self-equality of the negative,\nand hence self-negating equality,\nimmediacy which is in itself the negative,\nthe negative of itself:\nits being is to be what it is not.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-5'})
MATCH (c:IntegratedChunk {id: 'ref-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-5:kp:1'})
SET kp.chunkId = 'ref-5'
SET kp.ordinal = 1
SET kp.text = 'Reflection is movement from nothing to nothing';
MATCH (c:IntegratedChunk {id: 'ref-5'})
MATCH (kp:KeyPoint {id: 'ref-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-5:kp:2'})
SET kp.chunkId = 'ref-5'
SET kp.ordinal = 2
SET kp.text = 'Negation coinciding with itself';
MATCH (c:IntegratedChunk {id: 'ref-5'})
MATCH (kp:KeyPoint {id: 'ref-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-5:kp:3'})
SET kp.chunkId = 'ref-5'
SET kp.ordinal = 3
SET kp.text = 'Self-coinciding is simple equality with itself, immediacy';
MATCH (c:IntegratedChunk {id: 'ref-5'})
MATCH (kp:KeyPoint {id: 'ref-5:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-5:kp:4'})
SET kp.chunkId = 'ref-5'
SET kp.ordinal = 4
SET kp.text = 'Reflection is transition as sublating of transition';
MATCH (c:IntegratedChunk {id: 'ref-5'})
MATCH (kp:KeyPoint {id: 'ref-5:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-5:kp:5'})
SET kp.chunkId = 'ref-5'
SET kp.ordinal = 5
SET kp.text = 'Its being is to be what it is not';
MATCH (c:IntegratedChunk {id: 'ref-5'})
MATCH (kp:KeyPoint {id: 'ref-5:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-6'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 107
SET segment.lineEnd = 126
SET segment.text = 'The self-reference of the negative is\ntherefore its turning back into itself;\nit is immediacy as the sublating of the negative,\nbut immediacy simply and solely as this reference\nor as turning back from a one,\nand hence as self-sublating immediacy.\nThis is positedness,\nimmediacy purely as determinateness\nor as self-reflecting.\nThis immediacy, which is only as\nthe turning back of the negative into itself,\nis the immediacy which constitutes the determinateness of shine,\nand from which the previous reflective movement seemed to begin.\nBut, far from being able to begin with this immediacy,\nthe latter first is rather as the turning back\nor as the reflection itself.\nReflection is therefore the movement which,\nsince it is the turning back,\nonly in this turning is that\nwhich starts out or returns.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-6'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-6'
SET topic.title = 'Positing reflection — turning back, positedness'
SET topic.description = 'Self-reference of negative is turning back into itself. Immediacy as sublating of negative, self-sublating immediacy. This is positedness, immediacy purely as determinateness. Immediacy constitutes determinateness of shine. Cannot begin with this immediacy; it is the turning back itself. Reflection only in turning is that which starts out or returns.'
SET topic.keyPoints = ['Self-reference of negative is turning back into itself', 'This is positedness, immediacy purely as determinateness', 'Immediacy constitutes determinateness of shine', 'Cannot begin with this immediacy; it is the turning back itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-6'})
MATCH (topic:Topic {id: 'topic:ref-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-6'})
SET c.title = 'Positing reflection — turning back, positedness'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 107
SET c.lineEnd = 126
SET c.description = 'Self-reference of negative is turning back into itself. Immediacy as sublating of negative, self-sublating immediacy. This is positedness, immediacy purely as determinateness. Immediacy constitutes determinateness of shine. Cannot begin with this immediacy; it is the turning back itself. Reflection only in turning is that which starts out or returns.'
SET c.keyPoints = ['Self-reference of negative is turning back into itself', 'This is positedness, immediacy purely as determinateness', 'Immediacy constitutes determinateness of shine', 'Cannot begin with this immediacy; it is the turning back itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'shine', 'citta']
SET c.orderInSource = 6
SET c.globalOrder = 18
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The self-reference of the negative is\ntherefore its turning back into itself;\nit is immediacy as the sublating of the negative,\nbut immediacy simply and solely as this reference\nor as turning back from a one,\nand hence as self-sublating immediacy.\nThis is positedness,\nimmediacy purely as determinateness\nor as self-reflecting.\nThis immediacy, which is only as\nthe turning back of the negative into itself,\nis the immediacy which constitutes the determinateness of shine,\nand from which the previous reflective movement seemed to begin.\nBut, far from being able to begin with this immediacy,\nthe latter first is rather as the turning back\nor as the reflection itself.\nReflection is therefore the movement which,\nsince it is the turning back,\nonly in this turning is that\nwhich starts out or returns.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-6'})
MATCH (c:IntegratedChunk {id: 'ref-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-6:kp:1'})
SET kp.chunkId = 'ref-6'
SET kp.ordinal = 1
SET kp.text = 'Self-reference of negative is turning back into itself';
MATCH (c:IntegratedChunk {id: 'ref-6'})
MATCH (kp:KeyPoint {id: 'ref-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-6:kp:2'})
SET kp.chunkId = 'ref-6'
SET kp.ordinal = 2
SET kp.text = 'This is positedness, immediacy purely as determinateness';
MATCH (c:IntegratedChunk {id: 'ref-6'})
MATCH (kp:KeyPoint {id: 'ref-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-6:kp:3'})
SET kp.chunkId = 'ref-6'
SET kp.ordinal = 3
SET kp.text = 'Immediacy constitutes determinateness of shine';
MATCH (c:IntegratedChunk {id: 'ref-6'})
MATCH (kp:KeyPoint {id: 'ref-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-6:kp:4'})
SET kp.chunkId = 'ref-6'
SET kp.ordinal = 4
SET kp.text = 'Cannot begin with this immediacy; it is the turning back itself';
MATCH (c:IntegratedChunk {id: 'ref-6'})
MATCH (kp:KeyPoint {id: 'ref-6:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-7'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 128
SET segment.lineEnd = 165
SET segment.text = 'It is a positing, inasmuch as it is\nimmediacy as a turning back;\nthat is to say, there is not an other beforehand,\none either from which or to which it would turn back;\nit is, therefore, only as a turning back\nor as the negative of itself.\nBut further, this immediacy is sublated negation\nand sublated return into itself.\nReflection, as the sublating of the negative, is\nthe sublating of its other, of the immediacy.\nBecause it is thus immediacy as a turning back,\nthe coinciding of the negative with itself,\nit is equally the negation of the negative as negative.\nAnd so it is presupposing.\nOr immediacy is as a turning back\nonly the negative of itself,\njust this, not to be immediacy;\nbut reflection is the sublating\nof the negative of itself,\ncoincidence with itself;\nit therefore sublates its positing,\nand inasmuch as it is in its positing\nthe sublating of positing, it is presupposing.\nIn presupposing, reflection determines the turning back\ninto itself as the negative of itself,\nas that of which essence is the sublating.\nIt is its relating to itself,\nbut to itself as to the negative of itself;\nonly so is it negativity which abides with itself,\nself-referring negativity.\nImmediacy comes on the scene simply and solely\nas a turning back and is that negative\nwhich is the semblance of a beginning,\nthe beginning which the return negates.\nThe turning back of essence is therefore its self-repulsion.\nOr inner directed reflection is essentially\nthe presupposing of that from which\nthe reflection is the turning back.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-7'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-7'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-7'
SET topic.title = 'Positing reflection — positing and presupposing'
SET topic.description = 'Positing: immediacy as turning back, no other beforehand. Only as turning back, negative of itself. Immediacy is sublated negation and sublated return. Sublating of negative is sublating of immediacy. Equally negation of negative as negative: presupposing. Presupposing: determines turning back as negative of itself. Self-repulsion, presupposing of that from which reflection turns back.'
SET topic.keyPoints = ['Positing: immediacy as turning back, no other beforehand', 'Sublating of negative is sublating of immediacy', 'Negation of negative as negative: presupposing', 'Self-repulsion, presupposing of that from which reflection turns back'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-7'})
MATCH (topic:Topic {id: 'topic:ref-7'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-7'})
SET c.title = 'Positing reflection — positing and presupposing'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 128
SET c.lineEnd = 165
SET c.description = 'Positing: immediacy as turning back, no other beforehand. Only as turning back, negative of itself. Immediacy is sublated negation and sublated return. Sublating of negative is sublating of immediacy. Equally negation of negative as negative: presupposing. Presupposing: determines turning back as negative of itself. Self-repulsion, presupposing of that from which reflection turns back.'
SET c.keyPoints = ['Positing: immediacy as turning back, no other beforehand', 'Sublating of negative is sublating of immediacy', 'Negation of negative as negative: presupposing', 'Self-repulsion, presupposing of that from which reflection turns back']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 7
SET c.globalOrder = 19
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'It is a positing, inasmuch as it is\nimmediacy as a turning back;\nthat is to say, there is not an other beforehand,\none either from which or to which it would turn back;\nit is, therefore, only as a turning back\nor as the negative of itself.\nBut further, this immediacy is sublated negation\nand sublated return into itself.\nReflection, as the sublating of the negative, is\nthe sublating of its other, of the immediacy.\nBecause it is thus immediacy as a turning back,\nthe coinciding of the negative with itself,\nit is equally the negation of the negative as negative.\nAnd so it is presupposing.\nOr immediacy is as a turning back\nonly the negative of itself,\njust this, not to be immediacy;\nbut reflection is the sublating\nof the negative of itself,\ncoincidence with itself;\nit therefore sublates its positing,\nand inasmuch as it is in its positing\nthe sublating of positing, it is presupposing.\nIn presupposing, reflection determines the turning back\ninto itself as the negative of itself,\nas that of which essence is the sublating.\nIt is its relating to itself,\nbut to itself as to the negative of itself;\nonly so is it negativity which abides with itself,\nself-referring negativity.\nImmediacy comes on the scene simply and solely\nas a turning back and is that negative\nwhich is the semblance of a beginning,\nthe beginning which the return negates.\nThe turning back of essence is therefore its self-repulsion.\nOr inner directed reflection is essentially\nthe presupposing of that from which\nthe reflection is the turning back.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-7'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-7'})
MATCH (c:IntegratedChunk {id: 'ref-7'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-7'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-7'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-7:kp:1'})
SET kp.chunkId = 'ref-7'
SET kp.ordinal = 1
SET kp.text = 'Positing: immediacy as turning back, no other beforehand';
MATCH (c:IntegratedChunk {id: 'ref-7'})
MATCH (kp:KeyPoint {id: 'ref-7:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-7:kp:2'})
SET kp.chunkId = 'ref-7'
SET kp.ordinal = 2
SET kp.text = 'Sublating of negative is sublating of immediacy';
MATCH (c:IntegratedChunk {id: 'ref-7'})
MATCH (kp:KeyPoint {id: 'ref-7:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-7:kp:3'})
SET kp.chunkId = 'ref-7'
SET kp.ordinal = 3
SET kp.text = 'Negation of negative as negative: presupposing';
MATCH (c:IntegratedChunk {id: 'ref-7'})
MATCH (kp:KeyPoint {id: 'ref-7:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-7:kp:4'})
SET kp.chunkId = 'ref-7'
SET kp.ordinal = 4
SET kp.text = 'Self-repulsion, presupposing of that from which reflection turns back';
MATCH (c:IntegratedChunk {id: 'ref-7'})
MATCH (kp:KeyPoint {id: 'ref-7:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-8'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 167
SET segment.lineEnd = 211
SET segment.text = 'It is only by virtue of the sublating of\nits equality with itself\nthat essence is equality with itself.\nEssence presupposes itself,\nand the sublating of this presupposing is essence itself;\ncontrariwise, this sublating of its presupposition is\nthe presupposition itself.\nReflection thus finds an immediate before it\nwhich it transcends and from which it is the turning back.\nBut this turning back is only the presupposing of\nwhat was antecedently found.\nThis antecedent comes to be only by being left behind;\nits immediacy is sublated immediacy.\nThe sublated immediacy is, contrariwise, the turning\nback into itself,\nessence that arrives at itself,\nsimple being equal to itself.\nThis arriving at itself is thus\nthe sublating of itself\nand self-repelling, presupposing reflection,\nand its repelling of itself from itself is\nthe arriving at itself.\n\nIt follows from these considerations that\nthe movement of reflection is to be taken as\nan absolute internal counter-repelling.\nFor the presupposition of\nthe turning back into itself\n[that from which essence arises,\nessence being only as this coming back]\nis only in the turning back itself.\nTranscending the immediate from which reflection begins\noccurs rather only through this transcending;\nand the transcending of the immediate is\nthe arriving at the immediate.\nThe movement, as forward movement, turns immediately\naround into itself and so is only self-movement:\na movement which comes from itself in so far as\npositing reflection is presupposing reflection, yet,\nas presupposing reflection, is simply positing reflection.\n\nThus is reflection itself and its non-being,\nand only is itself by being the negative of itself,\nfor only in this way is the sublating of the negative\nat the same time a coinciding with itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-8'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-8'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-8'
SET topic.title = 'Positing reflection — self-repulsion, absolute counter-repelling'
SET topic.description = 'Only by sublating equality with itself is essence equality with itself. Essence presupposes itself, sublating of presupposing is essence itself. Presupposition is only in turning back itself. Transcending immediate occurs only through transcending. Transcending immediate is arriving at immediate. Forward movement turns immediately around into itself: self-movement. Positing reflection is presupposing reflection, and vice versa. Reflection is itself and its non-being, negative of itself.'
SET topic.keyPoints = ['Only by sublating equality with itself is essence equality with itself', 'Essence presupposes itself, sublating of presupposing is essence itself', 'Transcending immediate is arriving at immediate', 'Forward movement turns immediately around into itself: self-movement', 'Reflection is itself and its non-being, negative of itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-8'})
MATCH (topic:Topic {id: 'topic:ref-8'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-8'})
SET c.title = 'Positing reflection — self-repulsion, absolute counter-repelling'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 167
SET c.lineEnd = 211
SET c.description = 'Only by sublating equality with itself is essence equality with itself. Essence presupposes itself, sublating of presupposing is essence itself. Presupposition is only in turning back itself. Transcending immediate occurs only through transcending. Transcending immediate is arriving at immediate. Forward movement turns immediately around into itself: self-movement. Positing reflection is presupposing reflection, and vice versa. Reflection is itself and its non-being, negative of itself.'
SET c.keyPoints = ['Only by sublating equality with itself is essence equality with itself', 'Essence presupposes itself, sublating of presupposing is essence itself', 'Transcending immediate is arriving at immediate', 'Forward movement turns immediately around into itself: self-movement', 'Reflection is itself and its non-being, negative of itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 8
SET c.globalOrder = 20
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'It is only by virtue of the sublating of\nits equality with itself\nthat essence is equality with itself.\nEssence presupposes itself,\nand the sublating of this presupposing is essence itself;\ncontrariwise, this sublating of its presupposition is\nthe presupposition itself.\nReflection thus finds an immediate before it\nwhich it transcends and from which it is the turning back.\nBut this turning back is only the presupposing of\nwhat was antecedently found.\nThis antecedent comes to be only by being left behind;\nits immediacy is sublated immediacy.\nThe sublated immediacy is, contrariwise, the turning\nback into itself,\nessence that arrives at itself,\nsimple being equal to itself.\nThis arriving at itself is thus\nthe sublating of itself\nand self-repelling, presupposing reflection,\nand its repelling of itself from itself is\nthe arriving at itself.\n\nIt follows from these considerations that\nthe movement of reflection is to be taken as\nan absolute internal counter-repelling.\nFor the presupposition of\nthe turning back into itself\n[that from which essence arises,\nessence being only as this coming back]\nis only in the turning back itself.\nTranscending the immediate from which reflection begins\noccurs rather only through this transcending;\nand the transcending of the immediate is\nthe arriving at the immediate.\nThe movement, as forward movement, turns immediately\naround into itself and so is only self-movement:\na movement which comes from itself in so far as\npositing reflection is presupposing reflection, yet,\nas presupposing reflection, is simply positing reflection.\n\nThus is reflection itself and its non-being,\nand only is itself by being the negative of itself,\nfor only in this way is the sublating of the negative\nat the same time a coinciding with itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-8'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-8'})
MATCH (c:IntegratedChunk {id: 'ref-8'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-8'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-8'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-8:kp:1'})
SET kp.chunkId = 'ref-8'
SET kp.ordinal = 1
SET kp.text = 'Only by sublating equality with itself is essence equality with itself';
MATCH (c:IntegratedChunk {id: 'ref-8'})
MATCH (kp:KeyPoint {id: 'ref-8:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-8:kp:2'})
SET kp.chunkId = 'ref-8'
SET kp.ordinal = 2
SET kp.text = 'Essence presupposes itself, sublating of presupposing is essence itself';
MATCH (c:IntegratedChunk {id: 'ref-8'})
MATCH (kp:KeyPoint {id: 'ref-8:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-8:kp:3'})
SET kp.chunkId = 'ref-8'
SET kp.ordinal = 3
SET kp.text = 'Transcending immediate is arriving at immediate';
MATCH (c:IntegratedChunk {id: 'ref-8'})
MATCH (kp:KeyPoint {id: 'ref-8:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-8:kp:4'})
SET kp.chunkId = 'ref-8'
SET kp.ordinal = 4
SET kp.text = 'Forward movement turns immediately around into itself: self-movement';
MATCH (c:IntegratedChunk {id: 'ref-8'})
MATCH (kp:KeyPoint {id: 'ref-8:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-8:kp:5'})
SET kp.chunkId = 'ref-8'
SET kp.ordinal = 5
SET kp.text = 'Reflection is itself and its non-being, negative of itself';
MATCH (c:IntegratedChunk {id: 'ref-8'})
MATCH (kp:KeyPoint {id: 'ref-8:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-9'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 213
SET segment.lineEnd = 228
SET segment.text = 'The immediacy which reflection,\nas a process of sublating,\npresupposes for itself is\nsimply and solely a positedness,\nsomething in itself sublated\nwhich is not diverse from\nreflection\'s turning back into itself\nbut is itself only this turning back.\nBut it is at the same time determined as a negative,\nas immediately in opposition to something,\nand hence to an other.\nAnd so is reflection determined.\nAccording to this determinateness,\nbecause reflection has a presupposition\nand takes its start from the immediate as its other,\nit is external reflection.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-9'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-9'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-9'
SET topic.title = 'Transition to external reflection'
SET topic.description = 'Immediacy presupposed is simply and solely positedness. Determined as negative, in opposition to other. Reflection has presupposition, takes start from immediate as other. External reflection.'
SET topic.keyPoints = ['Immediacy presupposed is simply and solely positedness', 'Determined as negative, in opposition to other', 'Reflection has presupposition, takes start from immediate as other'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-9'})
MATCH (topic:Topic {id: 'topic:ref-9'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-9'})
SET c.title = 'Transition to external reflection'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 213
SET c.lineEnd = 228
SET c.description = 'Immediacy presupposed is simply and solely positedness. Determined as negative, in opposition to other. Reflection has presupposition, takes start from immediate as other. External reflection.'
SET c.keyPoints = ['Immediacy presupposed is simply and solely positedness', 'Determined as negative, in opposition to other', 'Reflection has presupposition, takes start from immediate as other']
SET c.tags = ['negation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 9
SET c.globalOrder = 21
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The immediacy which reflection,\nas a process of sublating,\npresupposes for itself is\nsimply and solely a positedness,\nsomething in itself sublated\nwhich is not diverse from\nreflection\'s turning back into itself\nbut is itself only this turning back.\nBut it is at the same time determined as a negative,\nas immediately in opposition to something,\nand hence to an other.\nAnd so is reflection determined.\nAccording to this determinateness,\nbecause reflection has a presupposition\nand takes its start from the immediate as its other,\nit is external reflection.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-9'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-9'})
MATCH (c:IntegratedChunk {id: 'ref-9'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-9'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-9'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-9:kp:1'})
SET kp.chunkId = 'ref-9'
SET kp.ordinal = 1
SET kp.text = 'Immediacy presupposed is simply and solely positedness';
MATCH (c:IntegratedChunk {id: 'ref-9'})
MATCH (kp:KeyPoint {id: 'ref-9:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-9:kp:2'})
SET kp.chunkId = 'ref-9'
SET kp.ordinal = 2
SET kp.text = 'Determined as negative, in opposition to other';
MATCH (c:IntegratedChunk {id: 'ref-9'})
MATCH (kp:KeyPoint {id: 'ref-9:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-9:kp:3'})
SET kp.chunkId = 'ref-9'
SET kp.ordinal = 3
SET kp.text = 'Reflection has presupposition, takes start from immediate as other';
MATCH (c:IntegratedChunk {id: 'ref-9'})
MATCH (kp:KeyPoint {id: 'ref-9:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-10'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 232
SET segment.lineEnd = 247
SET segment.text = 'Reflection, as absolute reflection,\nis essence shining within,\nessence that posits only shine,\nonly positedness, for its presupposition;\nand as presupposing reflection,\nit is immediately only positing reflection.\nBut external or real reflection\npresupposes itself as sublated,\nas the negative of itself.\nIn this determination, it is doubled.\nAt one time it is as what is presupposed,\nor the reflection into itself which is the immediate.\nAt another time, it is as the reflection\nnegatively referring to itself;\nit refers itself to itself as\nto that its non-being.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-10'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-10'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-10'
SET topic.title = 'External reflection — doubled determination'
SET topic.description = 'Absolute reflection posits only shine, positedness, for presupposition. Presupposing reflection is immediately positing reflection. External reflection presupposes itself as sublated, negative of itself. Doubled: presupposed (reflection into itself as immediate) and reflection negatively referring to itself.'
SET topic.keyPoints = ['Absolute reflection posits only shine, positedness, for presupposition', 'External reflection presupposes itself as sublated, negative of itself', 'Doubled: presupposed and reflection negatively referring to itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-10'})
MATCH (topic:Topic {id: 'topic:ref-10'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-10'})
SET c.title = 'External reflection — doubled determination'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 232
SET c.lineEnd = 247
SET c.description = 'Absolute reflection posits only shine, positedness, for presupposition. Presupposing reflection is immediately positing reflection. External reflection presupposes itself as sublated, negative of itself. Doubled: presupposed (reflection into itself as immediate) and reflection negatively referring to itself.'
SET c.keyPoints = ['Absolute reflection posits only shine, positedness, for presupposition', 'External reflection presupposes itself as sublated, negative of itself', 'Doubled: presupposed and reflection negatively referring to itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 10
SET c.globalOrder = 22
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Reflection, as absolute reflection,\nis essence shining within,\nessence that posits only shine,\nonly positedness, for its presupposition;\nand as presupposing reflection,\nit is immediately only positing reflection.\nBut external or real reflection\npresupposes itself as sublated,\nas the negative of itself.\nIn this determination, it is doubled.\nAt one time it is as what is presupposed,\nor the reflection into itself which is the immediate.\nAt another time, it is as the reflection\nnegatively referring to itself;\nit refers itself to itself as\nto that its non-being.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-10'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-10'})
MATCH (c:IntegratedChunk {id: 'ref-10'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-10'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-10'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-10:kp:1'})
SET kp.chunkId = 'ref-10'
SET kp.ordinal = 1
SET kp.text = 'Absolute reflection posits only shine, positedness, for presupposition';
MATCH (c:IntegratedChunk {id: 'ref-10'})
MATCH (kp:KeyPoint {id: 'ref-10:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-10:kp:2'})
SET kp.chunkId = 'ref-10'
SET kp.ordinal = 2
SET kp.text = 'External reflection presupposes itself as sublated, negative of itself';
MATCH (c:IntegratedChunk {id: 'ref-10'})
MATCH (kp:KeyPoint {id: 'ref-10:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-10:kp:3'})
SET kp.chunkId = 'ref-10'
SET kp.ordinal = 3
SET kp.text = 'Doubled: presupposed and reflection negatively referring to itself';
MATCH (c:IntegratedChunk {id: 'ref-10'})
MATCH (kp:KeyPoint {id: 'ref-10:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-11'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 249
SET segment.lineEnd = 276
SET segment.text = 'External reflection thus presupposes a being,\nat first not in the sense that\nits immediacy is only positedness or moment,\nbut in the sense rather that\nthis immediacy refers to itself\nand the determinateness is only as moment.\nReflection refers to its presupposition in such a way\nthat the latter is its negative,\nbut this negative is thereby sublated as negative.\nReflection, in positing, immediately sublates its positing,\nand so it has an immediate presupposition.\nIt therefore finds this presupposition before it\nas something from which it starts,\nand from which it only makes its way back into itself,\nnegating it as its negative.\nBut that this presupposition is a negative\nor a positedness is not its concern;\nthis determinateness belongs only to positing reflection,\nwhereas in the presupposing positedness\nit is only as sublated.\nWhat external reflection determines and posits in the immediate\nare determinations which to that extent are external to it.\nIn the sphere of being, external reflection was the infinite;\nthe finite stands as the first,\nas the real from which the beginning is made\nas from a foundation that abides,\nwhereas the infinite is the reflection into itself\nstanding over against it.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-11'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-11'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-11'
SET topic.title = 'External reflection — presupposes being'
SET topic.description = 'Presupposes being, immediacy refers to itself. Determinateness only as moment. Reflection refers to presupposition as negative, sublated as negative. In positing, immediately sublates positing, has immediate presupposition. Finds presupposition before it, from which it starts. Presupposition as negative/positedness not its concern. In sphere of being, external reflection was infinite. Finite stands as first, infinite as reflection into itself.'
SET topic.keyPoints = ['Presupposes being, immediacy refers to itself', 'Reflection refers to presupposition as negative, sublated as negative', 'In sphere of being, external reflection was infinite', 'Finite stands as first, infinite as reflection into itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-11'})
MATCH (topic:Topic {id: 'topic:ref-11'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-11'})
SET c.title = 'External reflection — presupposes being'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 249
SET c.lineEnd = 276
SET c.description = 'Presupposes being, immediacy refers to itself. Determinateness only as moment. Reflection refers to presupposition as negative, sublated as negative. In positing, immediately sublates positing, has immediate presupposition. Finds presupposition before it, from which it starts. Presupposition as negative/positedness not its concern. In sphere of being, external reflection was infinite. Finite stands as first, infinite as reflection into itself.'
SET c.keyPoints = ['Presupposes being, immediacy refers to itself', 'Reflection refers to presupposition as negative, sublated as negative', 'In sphere of being, external reflection was infinite', 'Finite stands as first, infinite as reflection into itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 11
SET c.globalOrder = 23
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'External reflection thus presupposes a being,\nat first not in the sense that\nits immediacy is only positedness or moment,\nbut in the sense rather that\nthis immediacy refers to itself\nand the determinateness is only as moment.\nReflection refers to its presupposition in such a way\nthat the latter is its negative,\nbut this negative is thereby sublated as negative.\nReflection, in positing, immediately sublates its positing,\nand so it has an immediate presupposition.\nIt therefore finds this presupposition before it\nas something from which it starts,\nand from which it only makes its way back into itself,\nnegating it as its negative.\nBut that this presupposition is a negative\nor a positedness is not its concern;\nthis determinateness belongs only to positing reflection,\nwhereas in the presupposing positedness\nit is only as sublated.\nWhat external reflection determines and posits in the immediate\nare determinations which to that extent are external to it.\nIn the sphere of being, external reflection was the infinite;\nthe finite stands as the first,\nas the real from which the beginning is made\nas from a foundation that abides,\nwhereas the infinite is the reflection into itself\nstanding over against it.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-11'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-11'})
MATCH (c:IntegratedChunk {id: 'ref-11'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-11'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-11'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-11:kp:1'})
SET kp.chunkId = 'ref-11'
SET kp.ordinal = 1
SET kp.text = 'Presupposes being, immediacy refers to itself';
MATCH (c:IntegratedChunk {id: 'ref-11'})
MATCH (kp:KeyPoint {id: 'ref-11:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-11:kp:2'})
SET kp.chunkId = 'ref-11'
SET kp.ordinal = 2
SET kp.text = 'Reflection refers to presupposition as negative, sublated as negative';
MATCH (c:IntegratedChunk {id: 'ref-11'})
MATCH (kp:KeyPoint {id: 'ref-11:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-11:kp:3'})
SET kp.chunkId = 'ref-11'
SET kp.ordinal = 3
SET kp.text = 'In sphere of being, external reflection was infinite';
MATCH (c:IntegratedChunk {id: 'ref-11'})
MATCH (kp:KeyPoint {id: 'ref-11:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-11:kp:4'})
SET kp.chunkId = 'ref-11'
SET kp.ordinal = 4
SET kp.text = 'Finite stands as first, infinite as reflection into itself';
MATCH (c:IntegratedChunk {id: 'ref-11'})
MATCH (kp:KeyPoint {id: 'ref-11:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-12'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 278
SET segment.lineEnd = 284
SET segment.text = 'This external reflection is the syllogism\nin which the two extremes are\nthe immediate and the reflection into itself;\nthe middle term is the reference connecting the two,\nthe determinate immediate, so that one part of this connecting reference,\nthe immediate, falls to one extreme alone, and the other,\nthe determinateness or the negation, only to the other extreme.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-12'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-12'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-12'
SET topic.title = 'External reflection — syllogism structure'
SET topic.description = 'External reflection is syllogism. Two extremes: immediate and reflection into itself. Middle term: reference connecting two, determinate immediate. Immediate falls to one extreme, determinateness/negation to other.'
SET topic.keyPoints = ['External reflection is syllogism', 'Two extremes: immediate and reflection into itself', 'Middle term: reference connecting two, determinate immediate'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-12'})
MATCH (topic:Topic {id: 'topic:ref-12'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-12'})
SET c.title = 'External reflection — syllogism structure'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 278
SET c.lineEnd = 284
SET c.description = 'External reflection is syllogism. Two extremes: immediate and reflection into itself. Middle term: reference connecting two, determinate immediate. Immediate falls to one extreme, determinateness/negation to other.'
SET c.keyPoints = ['External reflection is syllogism', 'Two extremes: immediate and reflection into itself', 'Middle term: reference connecting two, determinate immediate']
SET c.tags = ['negation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 12
SET c.globalOrder = 24
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'This external reflection is the syllogism\nin which the two extremes are\nthe immediate and the reflection into itself;\nthe middle term is the reference connecting the two,\nthe determinate immediate, so that one part of this connecting reference,\nthe immediate, falls to one extreme alone, and the other,\nthe determinateness or the negation, only to the other extreme.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-12'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-12'})
MATCH (c:IntegratedChunk {id: 'ref-12'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-12'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-12'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-12:kp:1'})
SET kp.chunkId = 'ref-12'
SET kp.ordinal = 1
SET kp.text = 'External reflection is syllogism';
MATCH (c:IntegratedChunk {id: 'ref-12'})
MATCH (kp:KeyPoint {id: 'ref-12:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-12:kp:2'})
SET kp.chunkId = 'ref-12'
SET kp.ordinal = 2
SET kp.text = 'Two extremes: immediate and reflection into itself';
MATCH (c:IntegratedChunk {id: 'ref-12'})
MATCH (kp:KeyPoint {id: 'ref-12:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-12:kp:3'})
SET kp.chunkId = 'ref-12'
SET kp.ordinal = 3
SET kp.text = 'Middle term: reference connecting two, determinate immediate';
MATCH (c:IntegratedChunk {id: 'ref-12'})
MATCH (kp:KeyPoint {id: 'ref-12:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-13'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 286
SET segment.lineEnd = 313
SET segment.text = 'But if one takes a closer look at what the external reflection does,\nit turns out that it is, secondly, the positing of the immediate,\nan immediate which thus becomes the negative or the determined;\nbut it is immediately also the sublating of this positing,\nfor it presupposes the immediate;\nin negating, it is the negating of its negating.\nBut thereby it immediately is equally a positing,\nthe sublating of the immediate which is its negative;\nand this negative, from which it seemed to begin\nas from something alien,\nonly is in this its beginning.\nIn this way, the immediate is not only implicitly in itself\n(that is, for us or in external reflection)\nthe same as what reflection is,\nbut is posited as being the same.\nFor the immediate is determined by reflection as\nthe negative of the latter or as the other of it,\nbut it is reflection itself which negates this determining.\nThe externality of reflection vis-à-vis\nthe immediate is consequently sublated;\nits self-negating positing is its coinciding\nwith its negative, with the immediate,\nand this coinciding is the immediacy of essence itself.\nIt thus transpires that external reflection is not external\nbut is just as much the immanent reflection of immediacy itself;\nor that the result of positing reflection is\nessence existing in and for itself.\nExternal reflection is thus determining reflection.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-13'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-13'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-13'
SET topic.title = 'External reflection — positing and sublating'
SET topic.description = 'Positing of immediate, becomes negative/determined. Immediately sublates positing, presupposes immediate. Negating is negating of its negating, equally positing. Immediate posited as same as reflection. Externality sublated, coinciding with immediate is immediacy of essence. External reflection is immanent reflection of immediacy itself. Result of positing reflection is essence existing in and for itself. External reflection is determining reflection.'
SET topic.keyPoints = ['Positing of immediate, becomes negative/determined', 'Negating is negating of its negating, equally positing', 'Externality sublated, coinciding with immediate is immediacy of essence', 'External reflection is immanent reflection of immediacy itself', 'External reflection is determining reflection'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-13'})
MATCH (topic:Topic {id: 'topic:ref-13'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-13'})
SET c.title = 'External reflection — positing and sublating'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 286
SET c.lineEnd = 313
SET c.description = 'Positing of immediate, becomes negative/determined. Immediately sublates positing, presupposes immediate. Negating is negating of its negating, equally positing. Immediate posited as same as reflection. Externality sublated, coinciding with immediate is immediacy of essence. External reflection is immanent reflection of immediacy itself. Result of positing reflection is essence existing in and for itself. External reflection is determining reflection.'
SET c.keyPoints = ['Positing of immediate, becomes negative/determined', 'Negating is negating of its negating, equally positing', 'Externality sublated, coinciding with immediate is immediacy of essence', 'External reflection is immanent reflection of immediacy itself', 'External reflection is determining reflection']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 13
SET c.globalOrder = 25
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'But if one takes a closer look at what the external reflection does,\nit turns out that it is, secondly, the positing of the immediate,\nan immediate which thus becomes the negative or the determined;\nbut it is immediately also the sublating of this positing,\nfor it presupposes the immediate;\nin negating, it is the negating of its negating.\nBut thereby it immediately is equally a positing,\nthe sublating of the immediate which is its negative;\nand this negative, from which it seemed to begin\nas from something alien,\nonly is in this its beginning.\nIn this way, the immediate is not only implicitly in itself\n(that is, for us or in external reflection)\nthe same as what reflection is,\nbut is posited as being the same.\nFor the immediate is determined by reflection as\nthe negative of the latter or as the other of it,\nbut it is reflection itself which negates this determining.\nThe externality of reflection vis-à-vis\nthe immediate is consequently sublated;\nits self-negating positing is its coinciding\nwith its negative, with the immediate,\nand this coinciding is the immediacy of essence itself.\nIt thus transpires that external reflection is not external\nbut is just as much the immanent reflection of immediacy itself;\nor that the result of positing reflection is\nessence existing in and for itself.\nExternal reflection is thus determining reflection.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-13'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-13'})
MATCH (c:IntegratedChunk {id: 'ref-13'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-13'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-13'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-13:kp:1'})
SET kp.chunkId = 'ref-13'
SET kp.ordinal = 1
SET kp.text = 'Positing of immediate, becomes negative/determined';
MATCH (c:IntegratedChunk {id: 'ref-13'})
MATCH (kp:KeyPoint {id: 'ref-13:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-13:kp:2'})
SET kp.chunkId = 'ref-13'
SET kp.ordinal = 2
SET kp.text = 'Negating is negating of its negating, equally positing';
MATCH (c:IntegratedChunk {id: 'ref-13'})
MATCH (kp:KeyPoint {id: 'ref-13:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-13:kp:3'})
SET kp.chunkId = 'ref-13'
SET kp.ordinal = 3
SET kp.text = 'Externality sublated, coinciding with immediate is immediacy of essence';
MATCH (c:IntegratedChunk {id: 'ref-13'})
MATCH (kp:KeyPoint {id: 'ref-13:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-13:kp:4'})
SET kp.chunkId = 'ref-13'
SET kp.ordinal = 4
SET kp.text = 'External reflection is immanent reflection of immediacy itself';
MATCH (c:IntegratedChunk {id: 'ref-13'})
MATCH (kp:KeyPoint {id: 'ref-13:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-13:kp:5'})
SET kp.chunkId = 'ref-13'
SET kp.ordinal = 5
SET kp.text = 'External reflection is determining reflection';
MATCH (c:IntegratedChunk {id: 'ref-13'})
MATCH (kp:KeyPoint {id: 'ref-13:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-14'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 317
SET segment.lineEnd = 334
SET segment.text = 'Determining reflection is in general\nthe unity of positing and external reflection.\nThis is now to be examined more closely.\n\n1. External reflection begins from immediate being,\npositing reflection from nothing.\nIn its determining, external reflection posits another in the\nplace of the sublated being, but this other is essence;\nthe positing does not posit its determination in the place of an other;\nit has no presupposition.\nBut, precisely for this reason,\nit is not complete as determining reflection;\nthe determination which it posits is consequently only a posited;\nthis is an immediate, not however as equal to itself\nbut as self-negating;\nits connection with the turning back into itself is absolute;\nit is only in the reflection-into-itself\nbut is not this reflection itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-14'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-14'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-14'
SET topic.title = 'Determining reflection — unity of positing and external'
SET topic.description = 'Determining reflection is unity of positing and external reflection. External reflection begins from immediate being, positing from nothing. External reflection posits another (essence) in place of sublated being. Positing has no presupposition, not complete as determining reflection. Determination posited is only posited, immediate as self-negating. Connection with turning back into itself is absolute. Only in reflection-into-itself but not this reflection itself.'
SET topic.keyPoints = ['Determining reflection is unity of positing and external reflection', 'External reflection begins from immediate being, positing from nothing', 'Determination posited is only posited, immediate as self-negating', 'Connection with turning back into itself is absolute'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-14'})
MATCH (topic:Topic {id: 'topic:ref-14'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-14'})
SET c.title = 'Determining reflection — unity of positing and external'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 317
SET c.lineEnd = 334
SET c.description = 'Determining reflection is unity of positing and external reflection. External reflection begins from immediate being, positing from nothing. External reflection posits another (essence) in place of sublated being. Positing has no presupposition, not complete as determining reflection. Determination posited is only posited, immediate as self-negating. Connection with turning back into itself is absolute. Only in reflection-into-itself but not this reflection itself.'
SET c.keyPoints = ['Determining reflection is unity of positing and external reflection', 'External reflection begins from immediate being, positing from nothing', 'Determination posited is only posited, immediate as self-negating', 'Connection with turning back into itself is absolute']
SET c.tags = ['sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 14
SET c.globalOrder = 26
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Determining reflection is in general\nthe unity of positing and external reflection.\nThis is now to be examined more closely.\n\n1. External reflection begins from immediate being,\npositing reflection from nothing.\nIn its determining, external reflection posits another in the\nplace of the sublated being, but this other is essence;\nthe positing does not posit its determination in the place of an other;\nit has no presupposition.\nBut, precisely for this reason,\nit is not complete as determining reflection;\nthe determination which it posits is consequently only a posited;\nthis is an immediate, not however as equal to itself\nbut as self-negating;\nits connection with the turning back into itself is absolute;\nit is only in the reflection-into-itself\nbut is not this reflection itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-14'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-14'})
MATCH (c:IntegratedChunk {id: 'ref-14'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-14'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-14'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-14:kp:1'})
SET kp.chunkId = 'ref-14'
SET kp.ordinal = 1
SET kp.text = 'Determining reflection is unity of positing and external reflection';
MATCH (c:IntegratedChunk {id: 'ref-14'})
MATCH (kp:KeyPoint {id: 'ref-14:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-14:kp:2'})
SET kp.chunkId = 'ref-14'
SET kp.ordinal = 2
SET kp.text = 'External reflection begins from immediate being, positing from nothing';
MATCH (c:IntegratedChunk {id: 'ref-14'})
MATCH (kp:KeyPoint {id: 'ref-14:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-14:kp:3'})
SET kp.chunkId = 'ref-14'
SET kp.ordinal = 3
SET kp.text = 'Determination posited is only posited, immediate as self-negating';
MATCH (c:IntegratedChunk {id: 'ref-14'})
MATCH (kp:KeyPoint {id: 'ref-14:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-14:kp:4'})
SET kp.chunkId = 'ref-14'
SET kp.ordinal = 4
SET kp.text = 'Connection with turning back into itself is absolute';
MATCH (c:IntegratedChunk {id: 'ref-14'})
MATCH (kp:KeyPoint {id: 'ref-14:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-15'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 336
SET segment.lineEnd = 371
SET segment.text = 'The posited is therefore an other,\nbut in such a manner that the self-equality\nof reflection is retained;\nfor the posited is only as sublated,\nas reference to the turning back into itself.\nIn the sphere of being, existence was the being\nthat had negation in it, and being was the immediate ground\nand element of this negation which was,\ntherefore, itself immediate negation.\nIn the sphere of essence,\npositedness is what corresponds to existence.\nPositedness is equally an existence,\nbut its ground is being as essence\nor as pure negativity;\nit is a determinateness or a negation,\nnot as existent but immediately as sublated.\nExistence is only positedness;\nthis is the principle of the essence of existence.\nPositedness stands on the one side over against existence,\nand over against essence on the other:\nit is to be regarded as the means which conjoins\nexistence with essence and essence with existence.\nIf it is said, a determination is only a positedness,\nthe claim can thus have a twofold meaning,\naccording to whether the determination is such\nin opposition to existence or in opposition to essence.\nIn either meaning, existence is taken for\nsomething superior to positedness,\nwhich is attributed to external reflection, to the subjective.\nIn fact, however, positedness is the superior, because, as posited,\nexistence is what it is in itself something negative,\nsomething that refers simply and solely to the turning back into itself.\nFor this reason positedness is only a positedness\nwith respect to essence:\nit is the negation of this turning back\nas achieved return into itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-15'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-15'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-15'
SET topic.title = 'Determining reflection — positedness vs. existence'
SET topic.description = 'Posited is other, but self-equality of reflection retained. Posited only as sublated, reference to turning back. In sphere of being, existence had negation in it. In sphere of essence, positedness corresponds to existence. Positedness is existence, but ground is essence as pure negativity. Determinateness not as existent but immediately as sublated. Existence is only positedness: principle of essence of existence. Positedness conjoins existence with essence. Twofold meaning: in opposition to existence or to essence. Positedness is superior, existence is negative, refers to turning back.'
SET topic.keyPoints = ['In sphere of essence, positedness corresponds to existence', 'Positedness is existence, but ground is essence as pure negativity', 'Existence is only positedness: principle of essence of existence', 'Positedness conjoins existence with essence', 'Positedness is superior, existence is negative'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-15'})
MATCH (topic:Topic {id: 'topic:ref-15'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-15'})
SET c.title = 'Determining reflection — positedness vs. existence'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 336
SET c.lineEnd = 371
SET c.description = 'Posited is other, but self-equality of reflection retained. Posited only as sublated, reference to turning back. In sphere of being, existence had negation in it. In sphere of essence, positedness corresponds to existence. Positedness is existence, but ground is essence as pure negativity. Determinateness not as existent but immediately as sublated. Existence is only positedness: principle of essence of existence. Positedness conjoins existence with essence. Twofold meaning: in opposition to existence or to essence. Positedness is superior, existence is negative, refers to turning back.'
SET c.keyPoints = ['In sphere of essence, positedness corresponds to existence', 'Positedness is existence, but ground is essence as pure negativity', 'Existence is only positedness: principle of essence of existence', 'Positedness conjoins existence with essence', 'Positedness is superior, existence is negative']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 15
SET c.globalOrder = 27
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The posited is therefore an other,\nbut in such a manner that the self-equality\nof reflection is retained;\nfor the posited is only as sublated,\nas reference to the turning back into itself.\nIn the sphere of being, existence was the being\nthat had negation in it, and being was the immediate ground\nand element of this negation which was,\ntherefore, itself immediate negation.\nIn the sphere of essence,\npositedness is what corresponds to existence.\nPositedness is equally an existence,\nbut its ground is being as essence\nor as pure negativity;\nit is a determinateness or a negation,\nnot as existent but immediately as sublated.\nExistence is only positedness;\nthis is the principle of the essence of existence.\nPositedness stands on the one side over against existence,\nand over against essence on the other:\nit is to be regarded as the means which conjoins\nexistence with essence and essence with existence.\nIf it is said, a determination is only a positedness,\nthe claim can thus have a twofold meaning,\naccording to whether the determination is such\nin opposition to existence or in opposition to essence.\nIn either meaning, existence is taken for\nsomething superior to positedness,\nwhich is attributed to external reflection, to the subjective.\nIn fact, however, positedness is the superior, because, as posited,\nexistence is what it is in itself something negative,\nsomething that refers simply and solely to the turning back into itself.\nFor this reason positedness is only a positedness\nwith respect to essence:\nit is the negation of this turning back\nas achieved return into itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-15'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-15'})
MATCH (c:IntegratedChunk {id: 'ref-15'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-15'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-15'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-15:kp:1'})
SET kp.chunkId = 'ref-15'
SET kp.ordinal = 1
SET kp.text = 'In sphere of essence, positedness corresponds to existence';
MATCH (c:IntegratedChunk {id: 'ref-15'})
MATCH (kp:KeyPoint {id: 'ref-15:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-15:kp:2'})
SET kp.chunkId = 'ref-15'
SET kp.ordinal = 2
SET kp.text = 'Positedness is existence, but ground is essence as pure negativity';
MATCH (c:IntegratedChunk {id: 'ref-15'})
MATCH (kp:KeyPoint {id: 'ref-15:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-15:kp:3'})
SET kp.chunkId = 'ref-15'
SET kp.ordinal = 3
SET kp.text = 'Existence is only positedness: principle of essence of existence';
MATCH (c:IntegratedChunk {id: 'ref-15'})
MATCH (kp:KeyPoint {id: 'ref-15:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-15:kp:4'})
SET kp.chunkId = 'ref-15'
SET kp.ordinal = 4
SET kp.text = 'Positedness conjoins existence with essence';
MATCH (c:IntegratedChunk {id: 'ref-15'})
MATCH (kp:KeyPoint {id: 'ref-15:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-15:kp:5'})
SET kp.chunkId = 'ref-15'
SET kp.ordinal = 5
SET kp.text = 'Positedness is superior, existence is negative';
MATCH (c:IntegratedChunk {id: 'ref-15'})
MATCH (kp:KeyPoint {id: 'ref-15:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-16'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 373
SET segment.lineEnd = 408
SET segment.text = '2. Positedness is not yet a determination of reflection;\nit is only determinateness as negation in general.\nBut the positing is now united with external reflection;\nin this unity, the latter is absolute presupposing, that is,\nthe repelling of reflection from itself\nor the positing of determinateness as its own.\nAs posited, therefore, positedness is negation;\nbut as presupposed, it is reflected into itself.\nAnd in this way positedness is a determination of reflection.\n\nThe determination of reflection is distinct\nfrom the determinateness of being, of quality;\nthe latter is immediate reference to other in general;\npositedness also is reference to other,\nbut to immanently reflected being.\nNegation as quality is existent negation;\nbeing constitutes its ground and element.\nThe determination of reflection, on the contrary,\nhas for this ground immanent reflectedness.\nPositedness gets fixed in determination precisely\nbecause reflection is self-equality in its negatedness;\nthe latter is therefore itself reflection into itself.\nDetermination persists here, not by virtue of being\nbut because of its self-equality.\nSince the being which sustains quality is\nunequal to the negation, quality is\nconsequently unequal within itself,\nand hence a transient moment which disappears in the other.\nThe determination of reflection is\non the contrary positedness as negation,\nnegation which has negatedness for its ground,\nis therefore not unequal to itself within itself,\nand hence essential rather than transient determinateness.\nWhat gives subsistence to it is the self-equality of reflection\nwhich has the negative only as negative,\nas something sublated or posited.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-16'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-16'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-16'
SET topic.title = 'Determining reflection — determination of reflection'
SET topic.description = 'Positedness is determinateness as negation in general. Positing united with external reflection: absolute presupposing. Repelling of reflection from itself, positing determinateness as its own. As posited, positedness is negation; as presupposed, reflected into itself. Determination of reflection distinct from determinateness of being. Quality is immediate reference to other; positedness is reference to immanently reflected being. Negation as quality is existent negation; determination of reflection has immanent reflectedness as ground. Determination persists by self-equality, not by being. Quality is transient; determination of reflection is essential. Self-equality of reflection gives subsistence, negative only as sublated/posited.'
SET topic.keyPoints = ['Positedness is determinateness as negation in general', 'Determination of reflection distinct from determinateness of being', 'Quality is immediate reference to other; positedness is reference to immanently reflected being', 'Determination persists by self-equality, not by being', 'Quality is transient; determination of reflection is essential'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-16'})
MATCH (topic:Topic {id: 'topic:ref-16'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-16'})
SET c.title = 'Determining reflection — determination of reflection'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 373
SET c.lineEnd = 408
SET c.description = 'Positedness is determinateness as negation in general. Positing united with external reflection: absolute presupposing. Repelling of reflection from itself, positing determinateness as its own. As posited, positedness is negation; as presupposed, reflected into itself. Determination of reflection distinct from determinateness of being. Quality is immediate reference to other; positedness is reference to immanently reflected being. Negation as quality is existent negation; determination of reflection has immanent reflectedness as ground. Determination persists by self-equality, not by being. Quality is transient; determination of reflection is essential. Self-equality of reflection gives subsistence, negative only as sublated/posited.'
SET c.keyPoints = ['Positedness is determinateness as negation in general', 'Determination of reflection distinct from determinateness of being', 'Quality is immediate reference to other; positedness is reference to immanently reflected being', 'Determination persists by self-equality, not by being', 'Quality is transient; determination of reflection is essential']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 16
SET c.globalOrder = 28
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '2. Positedness is not yet a determination of reflection;\nit is only determinateness as negation in general.\nBut the positing is now united with external reflection;\nin this unity, the latter is absolute presupposing, that is,\nthe repelling of reflection from itself\nor the positing of determinateness as its own.\nAs posited, therefore, positedness is negation;\nbut as presupposed, it is reflected into itself.\nAnd in this way positedness is a determination of reflection.\n\nThe determination of reflection is distinct\nfrom the determinateness of being, of quality;\nthe latter is immediate reference to other in general;\npositedness also is reference to other,\nbut to immanently reflected being.\nNegation as quality is existent negation;\nbeing constitutes its ground and element.\nThe determination of reflection, on the contrary,\nhas for this ground immanent reflectedness.\nPositedness gets fixed in determination precisely\nbecause reflection is self-equality in its negatedness;\nthe latter is therefore itself reflection into itself.\nDetermination persists here, not by virtue of being\nbut because of its self-equality.\nSince the being which sustains quality is\nunequal to the negation, quality is\nconsequently unequal within itself,\nand hence a transient moment which disappears in the other.\nThe determination of reflection is\non the contrary positedness as negation,\nnegation which has negatedness for its ground,\nis therefore not unequal to itself within itself,\nand hence essential rather than transient determinateness.\nWhat gives subsistence to it is the self-equality of reflection\nwhich has the negative only as negative,\nas something sublated or posited.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-16'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-16'})
MATCH (c:IntegratedChunk {id: 'ref-16'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-16'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-16'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-16:kp:1'})
SET kp.chunkId = 'ref-16'
SET kp.ordinal = 1
SET kp.text = 'Positedness is determinateness as negation in general';
MATCH (c:IntegratedChunk {id: 'ref-16'})
MATCH (kp:KeyPoint {id: 'ref-16:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-16:kp:2'})
SET kp.chunkId = 'ref-16'
SET kp.ordinal = 2
SET kp.text = 'Determination of reflection distinct from determinateness of being';
MATCH (c:IntegratedChunk {id: 'ref-16'})
MATCH (kp:KeyPoint {id: 'ref-16:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-16:kp:3'})
SET kp.chunkId = 'ref-16'
SET kp.ordinal = 3
SET kp.text = 'Quality is immediate reference to other; positedness is reference to immanently reflected being';
MATCH (c:IntegratedChunk {id: 'ref-16'})
MATCH (kp:KeyPoint {id: 'ref-16:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-16:kp:4'})
SET kp.chunkId = 'ref-16'
SET kp.ordinal = 4
SET kp.text = 'Determination persists by self-equality, not by being';
MATCH (c:IntegratedChunk {id: 'ref-16'})
MATCH (kp:KeyPoint {id: 'ref-16:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-16:kp:5'})
SET kp.chunkId = 'ref-16'
SET kp.ordinal = 5
SET kp.text = 'Quality is transient; determination of reflection is essential';
MATCH (c:IntegratedChunk {id: 'ref-16'})
MATCH (kp:KeyPoint {id: 'ref-16:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-17'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 410
SET segment.lineEnd = 424
SET segment.text = 'Because of this reflection into themselves,\nthe determinations of reflection appear as\nfree essentialities, sublated in the void\nwithout reciprocal attraction or repulsion.\nIn them the determinateness has become entranced\nand infinitely fixed by virtue of the reference to itself.\nIt is the determinate which has subjugated its transitoriness\nand its mere positedness to itself, that is to say,\nhas deflected its reflection-into-other into reflection-into- itself.\nThese determinations hereby constitute the determinate shine\nas it is in essence, the essential shine.\nDetermining reflection is for this reason\nreflection that has exited from itself;\nthe equality of essence with itself is\nlost in the negation, and negation predominates.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-17'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-17'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-17'
SET topic.title = 'Determining reflection — free essentialities, essential shine'
SET topic.description = 'Determinations of reflection appear as free essentialities. Sublated in void, without reciprocal attraction or repulsion. Determinateness entranced, infinitely fixed by reference to itself. Deflected reflection-into-other into reflection-into-itself. Constitute determinate shine as it is in essence: essential shine. Determining reflection is reflection that has exited from itself. Equality of essence with itself lost in negation, negation predominates.'
SET topic.keyPoints = ['Determinations of reflection appear as free essentialities', 'Determinateness entranced, infinitely fixed by reference to itself', 'Constitute determinate shine as it is in essence: essential shine', 'Determining reflection is reflection that has exited from itself', 'Equality of essence with itself lost in negation, negation predominates'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-17'})
MATCH (topic:Topic {id: 'topic:ref-17'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-17'})
SET c.title = 'Determining reflection — free essentialities, essential shine'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 410
SET c.lineEnd = 424
SET c.description = 'Determinations of reflection appear as free essentialities. Sublated in void, without reciprocal attraction or repulsion. Determinateness entranced, infinitely fixed by reference to itself. Deflected reflection-into-other into reflection-into-itself. Constitute determinate shine as it is in essence: essential shine. Determining reflection is reflection that has exited from itself. Equality of essence with itself lost in negation, negation predominates.'
SET c.keyPoints = ['Determinations of reflection appear as free essentialities', 'Determinateness entranced, infinitely fixed by reference to itself', 'Constitute determinate shine as it is in essence: essential shine', 'Determining reflection is reflection that has exited from itself', 'Equality of essence with itself lost in negation, negation predominates']
SET c.tags = ['negation', 'sublation', 'reflection', 'shine', 'citta']
SET c.orderInSource = 17
SET c.globalOrder = 29
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Because of this reflection into themselves,\nthe determinations of reflection appear as\nfree essentialities, sublated in the void\nwithout reciprocal attraction or repulsion.\nIn them the determinateness has become entranced\nand infinitely fixed by virtue of the reference to itself.\nIt is the determinate which has subjugated its transitoriness\nand its mere positedness to itself, that is to say,\nhas deflected its reflection-into-other into reflection-into- itself.\nThese determinations hereby constitute the determinate shine\nas it is in essence, the essential shine.\nDetermining reflection is for this reason\nreflection that has exited from itself;\nthe equality of essence with itself is\nlost in the negation, and negation predominates.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-17'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-17'})
MATCH (c:IntegratedChunk {id: 'ref-17'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-17'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-17'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-17:kp:1'})
SET kp.chunkId = 'ref-17'
SET kp.ordinal = 1
SET kp.text = 'Determinations of reflection appear as free essentialities';
MATCH (c:IntegratedChunk {id: 'ref-17'})
MATCH (kp:KeyPoint {id: 'ref-17:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-17:kp:2'})
SET kp.chunkId = 'ref-17'
SET kp.ordinal = 2
SET kp.text = 'Determinateness entranced, infinitely fixed by reference to itself';
MATCH (c:IntegratedChunk {id: 'ref-17'})
MATCH (kp:KeyPoint {id: 'ref-17:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-17:kp:3'})
SET kp.chunkId = 'ref-17'
SET kp.ordinal = 3
SET kp.text = 'Constitute determinate shine as it is in essence: essential shine';
MATCH (c:IntegratedChunk {id: 'ref-17'})
MATCH (kp:KeyPoint {id: 'ref-17:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-17:kp:4'})
SET kp.chunkId = 'ref-17'
SET kp.ordinal = 4
SET kp.text = 'Determining reflection is reflection that has exited from itself';
MATCH (c:IntegratedChunk {id: 'ref-17'})
MATCH (kp:KeyPoint {id: 'ref-17:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-17:kp:5'})
SET kp.chunkId = 'ref-17'
SET kp.ordinal = 5
SET kp.text = 'Equality of essence with itself lost in negation, negation predominates';
MATCH (c:IntegratedChunk {id: 'ref-17'})
MATCH (kp:KeyPoint {id: 'ref-17:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-18'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 426
SET segment.lineEnd = 441
SET segment.text = 'Thus there are two distinct sides to the determination of reflection.\nFirst, reflection is positedness, negation as such;\nsecond, it is immanent reflection.\nAccording to the side of positedness,\nit is negation as negation,\nand this already is its unity with itself.\nBut it is this unity at first only implicitly or in itself,\nan immediate which sublates itself within, is the other of itself.\nTo this extent, reflection is a determining that abides in itself.\nIn it essence does not exit from itself;\nthe distinctions are solely posited,\ntaken back into essence.\nBut, from the other side, they are not posited\nbut are rather reflected into themselves;\nnegation as negation is equality with itself,\nnot in its other, not reflected into its non-being.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-18'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-18'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-18'
SET topic.title = 'Determining reflection — two sides: positedness and immanent reflection'
SET topic.description = 'Two distinct sides: positedness (negation as such) and immanent reflection. According to positedness: negation as negation, unity with itself (implicitly). Immediate which sublates itself within, other of itself. Reflection is determining that abides in itself, essence does not exit. Distinctions solely posited, taken back into essence. From other side: reflected into themselves. Negation as negation is equality with itself, not reflected into non-being.'
SET topic.keyPoints = ['Two distinct sides: positedness (negation as such) and immanent reflection', 'According to positedness: negation as negation, unity with itself (implicitly)', 'Reflection is determining that abides in itself, essence does not exit', 'Distinctions solely posited, taken back into essence', 'Negation as negation is equality with itself, not reflected into non-being'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-18'})
MATCH (topic:Topic {id: 'topic:ref-18'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-18'})
SET c.title = 'Determining reflection — two sides: positedness and immanent reflection'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 426
SET c.lineEnd = 441
SET c.description = 'Two distinct sides: positedness (negation as such) and immanent reflection. According to positedness: negation as negation, unity with itself (implicitly). Immediate which sublates itself within, other of itself. Reflection is determining that abides in itself, essence does not exit. Distinctions solely posited, taken back into essence. From other side: reflected into themselves. Negation as negation is equality with itself, not reflected into non-being.'
SET c.keyPoints = ['Two distinct sides: positedness (negation as such) and immanent reflection', 'According to positedness: negation as negation, unity with itself (implicitly)', 'Reflection is determining that abides in itself, essence does not exit', 'Distinctions solely posited, taken back into essence', 'Negation as negation is equality with itself, not reflected into non-being']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 18
SET c.globalOrder = 30
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Thus there are two distinct sides to the determination of reflection.\nFirst, reflection is positedness, negation as such;\nsecond, it is immanent reflection.\nAccording to the side of positedness,\nit is negation as negation,\nand this already is its unity with itself.\nBut it is this unity at first only implicitly or in itself,\nan immediate which sublates itself within, is the other of itself.\nTo this extent, reflection is a determining that abides in itself.\nIn it essence does not exit from itself;\nthe distinctions are solely posited,\ntaken back into essence.\nBut, from the other side, they are not posited\nbut are rather reflected into themselves;\nnegation as negation is equality with itself,\nnot in its other, not reflected into its non-being.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-18'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-18'})
MATCH (c:IntegratedChunk {id: 'ref-18'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-18'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-18'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-18:kp:1'})
SET kp.chunkId = 'ref-18'
SET kp.ordinal = 1
SET kp.text = 'Two distinct sides: positedness (negation as such) and immanent reflection';
MATCH (c:IntegratedChunk {id: 'ref-18'})
MATCH (kp:KeyPoint {id: 'ref-18:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-18:kp:2'})
SET kp.chunkId = 'ref-18'
SET kp.ordinal = 2
SET kp.text = 'According to positedness: negation as negation, unity with itself (implicitly)';
MATCH (c:IntegratedChunk {id: 'ref-18'})
MATCH (kp:KeyPoint {id: 'ref-18:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-18:kp:3'})
SET kp.chunkId = 'ref-18'
SET kp.ordinal = 3
SET kp.text = 'Reflection is determining that abides in itself, essence does not exit';
MATCH (c:IntegratedChunk {id: 'ref-18'})
MATCH (kp:KeyPoint {id: 'ref-18:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-18:kp:4'})
SET kp.chunkId = 'ref-18'
SET kp.ordinal = 4
SET kp.text = 'Distinctions solely posited, taken back into essence';
MATCH (c:IntegratedChunk {id: 'ref-18'})
MATCH (kp:KeyPoint {id: 'ref-18:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-18:kp:5'})
SET kp.chunkId = 'ref-18'
SET kp.ordinal = 5
SET kp.text = 'Negation as negation is equality with itself, not reflected into non-being';
MATCH (c:IntegratedChunk {id: 'ref-18'})
MATCH (kp:KeyPoint {id: 'ref-18:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ref-19'})
SET segment.sourceId = 'source-reflection'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET segment.lineStart = 443
SET segment.lineEnd = 482
SET segment.text = '3. Now keeping in mind that the determination of reflection is\nboth immanently reflected reference and positedness as well,\nits nature immediately becomes more transparent.\nFor, as positedness, the determination is negation as such,\na non-being as against another, namely,\nas against the absolute immanent reflection or as against essence.\nBut as self-reference, it is reflected within itself.\nThis, the reflection of the determination,\nand that positedness are distinct;\nits positedness is rather the sublatedness of the determination\nwhereas its immanent reflectedness is its subsisting.\nIn so far as now the positedness is\nat the same time immanent reflection,\nthe determinateness of the reflection is\nthe reference in it to its otherness.\nIt is not a determinateness that exists quiescent,\none which would be referred to an other\nin such a way that the referred term\nand its reference would be different,\neach something existing in itself,\neach a something that excludes its other\nand its reference to this other from itself.\nRather, the determination of reflection is\nwithin it the determinate side\nand the reference of this determinate side as determinate,\nthat is, the reference to its negation.\nQuality, through its reference, passes over into another;\nits alteration begins in its reference.\nThe determination of reflection, on the contrary,\nhas taken its otherness back into itself.\nIt is positedness, negation which has however deflected\nthe reference to another into itself,\nand negation which, equal to itself,\nis the unity of itself and its other,\nand only through this is an essentiality.\nIt is, therefore, positedness, negation,\nbut as reflection into itself it is at the same time\nthe sublatedness of this positedness,\ninfinite reference to itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (segment:ChunkSegment {id: 'chunk:ref-19'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ref-19'})
SET topic.sourceId = 'source-reflection'
SET topic.topicRef = 'ref-19'
SET topic.title = 'Determining reflection — nature as both'
SET topic.description = 'Determination of reflection is both immanently reflected reference and positedness. As positedness: negation as such, non-being as against essence. As self-reference: reflected within itself. Positedness is sublatedness of determination; immanent reflectedness is subsisting. Positedness at same time immanent reflection: determinateness is reference to otherness. Not quiescent determinateness, not separate from reference. Determination is determinate side and reference to its negation. Quality passes over into another; determination of reflection takes otherness back into itself. Positedness, negation deflected into itself, unity of itself and its other. Essentiality through this unity. Positedness, negation, but as reflection into itself is sublatedness of positedness. Infinite reference to itself.'
SET topic.keyPoints = ['Determination of reflection is both immanently reflected reference and positedness', 'Positedness is sublatedness of determination; immanent reflectedness is subsisting', 'Determination is determinate side and reference to its negation', 'Quality passes over into another; determination of reflection takes otherness back into itself', 'Positedness, negation deflected into itself, unity of itself and its other', 'Infinite reference to itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ref-19'})
MATCH (topic:Topic {id: 'topic:ref-19'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ref-19'})
SET c.title = 'Determining reflection — nature as both'
SET c.sourceId = 'source-reflection'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/reflection.txt'
SET c.lineStart = 443
SET c.lineEnd = 482
SET c.description = 'Determination of reflection is both immanently reflected reference and positedness. As positedness: negation as such, non-being as against essence. As self-reference: reflected within itself. Positedness is sublatedness of determination; immanent reflectedness is subsisting. Positedness at same time immanent reflection: determinateness is reference to otherness. Not quiescent determinateness, not separate from reference. Determination is determinate side and reference to its negation. Quality passes over into another; determination of reflection takes otherness back into itself. Positedness, negation deflected into itself, unity of itself and its other. Essentiality through this unity. Positedness, negation, but as reflection into itself is sublatedness of positedness. Infinite reference to itself.'
SET c.keyPoints = ['Determination of reflection is both immanently reflected reference and positedness', 'Positedness is sublatedness of determination; immanent reflectedness is subsisting', 'Determination is determinate side and reference to its negation', 'Quality passes over into another; determination of reflection takes otherness back into itself', 'Positedness, negation deflected into itself, unity of itself and its other', 'Infinite reference to itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 19
SET c.globalOrder = 31
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '3. Now keeping in mind that the determination of reflection is\nboth immanently reflected reference and positedness as well,\nits nature immediately becomes more transparent.\nFor, as positedness, the determination is negation as such,\na non-being as against another, namely,\nas against the absolute immanent reflection or as against essence.\nBut as self-reference, it is reflected within itself.\nThis, the reflection of the determination,\nand that positedness are distinct;\nits positedness is rather the sublatedness of the determination\nwhereas its immanent reflectedness is its subsisting.\nIn so far as now the positedness is\nat the same time immanent reflection,\nthe determinateness of the reflection is\nthe reference in it to its otherness.\nIt is not a determinateness that exists quiescent,\none which would be referred to an other\nin such a way that the referred term\nand its reference would be different,\neach something existing in itself,\neach a something that excludes its other\nand its reference to this other from itself.\nRather, the determination of reflection is\nwithin it the determinate side\nand the reference of this determinate side as determinate,\nthat is, the reference to its negation.\nQuality, through its reference, passes over into another;\nits alteration begins in its reference.\nThe determination of reflection, on the contrary,\nhas taken its otherness back into itself.\nIt is positedness, negation which has however deflected\nthe reference to another into itself,\nand negation which, equal to itself,\nis the unity of itself and its other,\nand only through this is an essentiality.\nIt is, therefore, positedness, negation,\nbut as reflection into itself it is at the same time\nthe sublatedness of this positedness,\ninfinite reference to itself.';
MATCH (s:SourceText {id: 'source-reflection'})
MATCH (c:IntegratedChunk {id: 'ref-19'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ref-19'})
MATCH (c:IntegratedChunk {id: 'ref-19'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ref-19'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ref-19'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ref-19:kp:1'})
SET kp.chunkId = 'ref-19'
SET kp.ordinal = 1
SET kp.text = 'Determination of reflection is both immanently reflected reference and positedness';
MATCH (c:IntegratedChunk {id: 'ref-19'})
MATCH (kp:KeyPoint {id: 'ref-19:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-19:kp:2'})
SET kp.chunkId = 'ref-19'
SET kp.ordinal = 2
SET kp.text = 'Positedness is sublatedness of determination; immanent reflectedness is subsisting';
MATCH (c:IntegratedChunk {id: 'ref-19'})
MATCH (kp:KeyPoint {id: 'ref-19:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-19:kp:3'})
SET kp.chunkId = 'ref-19'
SET kp.ordinal = 3
SET kp.text = 'Determination is determinate side and reference to its negation';
MATCH (c:IntegratedChunk {id: 'ref-19'})
MATCH (kp:KeyPoint {id: 'ref-19:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-19:kp:4'})
SET kp.chunkId = 'ref-19'
SET kp.ordinal = 4
SET kp.text = 'Quality passes over into another; determination of reflection takes otherness back into itself';
MATCH (c:IntegratedChunk {id: 'ref-19'})
MATCH (kp:KeyPoint {id: 'ref-19:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-19:kp:5'})
SET kp.chunkId = 'ref-19'
SET kp.ordinal = 5
SET kp.text = 'Positedness, negation deflected into itself, unity of itself and its other';
MATCH (c:IntegratedChunk {id: 'ref-19'})
MATCH (kp:KeyPoint {id: 'ref-19:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ref-19:kp:6'})
SET kp.chunkId = 'ref-19'
SET kp.ordinal = 6
SET kp.text = 'Infinite reference to itself';
MATCH (c:IntegratedChunk {id: 'ref-19'})
MATCH (kp:KeyPoint {id: 'ref-19:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
