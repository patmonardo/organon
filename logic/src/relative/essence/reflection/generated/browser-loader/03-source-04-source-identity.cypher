MERGE (s:SourceText {id: 'source-identity'})
SET s.title = 'Identity'
SET s.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET s.totalLines = 64;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-identity'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:idn-1'})
SET segment.sourceId = 'source-identity'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET segment.lineStart = 2
SET segment.lineEnd = 19
SET segment.text = 'CHAPTER 2\n\nFoundation\n\nThe essentialities or the determinations of reflection\n\nReflection is determined reflection;\naccordingly, essence is determined essence, or it is essentiality.\n\nReflection is the shining of essence within itself.\n\nEssence, as infinite immanent turning back is\nnot immediate simplicity, but negative simplicity;\nit is a movement across moments that are distinct,\nis absolute mediation with itself.\nBut in these moments it shines;\nthe moments are, therefore, themselves\ndeterminations reflected into themselves.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (segment:ChunkSegment {id: 'chunk:idn-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:idn-1'})
SET topic.sourceId = 'source-identity'
SET topic.topicRef = 'idn-1'
SET topic.title = 'Chapter 2 introduction — Foundation, essentialities'
SET topic.description = 'Foundation: The essentialities or the determinations of reflection. Reflection is determined reflection; essence is determined essence, essentiality. Reflection is shining of essence within itself. Essence as infinite immanent turning back. Not immediate simplicity, but negative simplicity. Movement across moments that are distinct, absolute mediation with itself. In these moments it shines; moments are determinations reflected into themselves.'
SET topic.keyPoints = ['Foundation: The essentialities or the determinations of reflection', 'Reflection is determined reflection; essence is determined essence, essentiality', 'Essence as infinite immanent turning back, not immediate simplicity but negative simplicity', 'Movement across moments that are distinct, absolute mediation with itself', 'Moments are determinations reflected into themselves'];
MATCH (segment:ChunkSegment {id: 'chunk:idn-1'})
MATCH (topic:Topic {id: 'topic:idn-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'idn-1'})
SET c.title = 'Chapter 2 introduction — Foundation, essentialities'
SET c.sourceId = 'source-identity'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET c.lineStart = 2
SET c.lineEnd = 19
SET c.description = 'Foundation: The essentialities or the determinations of reflection. Reflection is determined reflection; essence is determined essence, essentiality. Reflection is shining of essence within itself. Essence as infinite immanent turning back. Not immediate simplicity, but negative simplicity. Movement across moments that are distinct, absolute mediation with itself. In these moments it shines; moments are determinations reflected into themselves.'
SET c.keyPoints = ['Foundation: The essentialities or the determinations of reflection', 'Reflection is determined reflection; essence is determined essence, essentiality', 'Essence as infinite immanent turning back, not immediate simplicity but negative simplicity', 'Movement across moments that are distinct, absolute mediation with itself', 'Moments are determinations reflected into themselves']
SET c.tags = ['negation', 'reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 42
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'CHAPTER 2\n\nFoundation\n\nThe essentialities or the determinations of reflection\n\nReflection is determined reflection;\naccordingly, essence is determined essence, or it is essentiality.\n\nReflection is the shining of essence within itself.\n\nEssence, as infinite immanent turning back is\nnot immediate simplicity, but negative simplicity;\nit is a movement across moments that are distinct,\nis absolute mediation with itself.\nBut in these moments it shines;\nthe moments are, therefore, themselves\ndeterminations reflected into themselves.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (c:IntegratedChunk {id: 'idn-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:idn-1'})
MATCH (c:IntegratedChunk {id: 'idn-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'idn-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'idn-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'idn-1:kp:1'})
SET kp.chunkId = 'idn-1'
SET kp.ordinal = 1
SET kp.text = 'Foundation: The essentialities or the determinations of reflection';
MATCH (c:IntegratedChunk {id: 'idn-1'})
MATCH (kp:KeyPoint {id: 'idn-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-1:kp:2'})
SET kp.chunkId = 'idn-1'
SET kp.ordinal = 2
SET kp.text = 'Reflection is determined reflection; essence is determined essence, essentiality';
MATCH (c:IntegratedChunk {id: 'idn-1'})
MATCH (kp:KeyPoint {id: 'idn-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-1:kp:3'})
SET kp.chunkId = 'idn-1'
SET kp.ordinal = 3
SET kp.text = 'Essence as infinite immanent turning back, not immediate simplicity but negative simplicity';
MATCH (c:IntegratedChunk {id: 'idn-1'})
MATCH (kp:KeyPoint {id: 'idn-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-1:kp:4'})
SET kp.chunkId = 'idn-1'
SET kp.ordinal = 4
SET kp.text = 'Movement across moments that are distinct, absolute mediation with itself';
MATCH (c:IntegratedChunk {id: 'idn-1'})
MATCH (kp:KeyPoint {id: 'idn-1:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-1:kp:5'})
SET kp.chunkId = 'idn-1'
SET kp.ordinal = 5
SET kp.text = 'Moments are determinations reflected into themselves';
MATCH (c:IntegratedChunk {id: 'idn-1'})
MATCH (kp:KeyPoint {id: 'idn-1:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:idn-2'})
SET segment.sourceId = 'source-identity'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET segment.lineStart = 21
SET segment.lineEnd = 30
SET segment.text = 'First, essence is simple self-reference, pure identity.\nThis is its determination, one by which it is rather\nthe absence of determination.\n\nSecond, the specifying determination is difference,\ndifference which is either external or indefinite,\ndiversity in general, or opposed diversity or opposition.\n\nThird, as contradiction this opposition is reflected into itself\nand returns to its foundation.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (segment:ChunkSegment {id: 'chunk:idn-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:idn-2'})
SET topic.sourceId = 'source-identity'
SET topic.topicRef = 'idn-2'
SET topic.title = 'Three moments: identity, difference, contradiction'
SET topic.description = 'First: essence is simple self-reference, pure identity. This determination is absence of determination. Second: specifying determination is difference. Difference: external or indefinite, diversity in general, or opposed diversity or opposition. Third: as contradiction, opposition reflected into itself and returns to foundation.'
SET topic.keyPoints = ['First: essence is simple self-reference, pure identity', 'This determination is absence of determination', 'Second: specifying determination is difference', 'Difference: external or indefinite, diversity, or opposed diversity or opposition', 'Third: as contradiction, opposition reflected into itself and returns to foundation'];
MATCH (segment:ChunkSegment {id: 'chunk:idn-2'})
MATCH (topic:Topic {id: 'topic:idn-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'idn-2'})
SET c.title = 'Three moments: identity, difference, contradiction'
SET c.sourceId = 'source-identity'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET c.lineStart = 21
SET c.lineEnd = 30
SET c.description = 'First: essence is simple self-reference, pure identity. This determination is absence of determination. Second: specifying determination is difference. Difference: external or indefinite, diversity in general, or opposed diversity or opposition. Third: as contradiction, opposition reflected into itself and returns to foundation.'
SET c.keyPoints = ['First: essence is simple self-reference, pure identity', 'This determination is absence of determination', 'Second: specifying determination is difference', 'Difference: external or indefinite, diversity, or opposed diversity or opposition', 'Third: as contradiction, opposition reflected into itself and returns to foundation']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 43
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'First, essence is simple self-reference, pure identity.\nThis is its determination, one by which it is rather\nthe absence of determination.\n\nSecond, the specifying determination is difference,\ndifference which is either external or indefinite,\ndiversity in general, or opposed diversity or opposition.\n\nThird, as contradiction this opposition is reflected into itself\nand returns to its foundation.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (c:IntegratedChunk {id: 'idn-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:idn-2'})
MATCH (c:IntegratedChunk {id: 'idn-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'idn-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'idn-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'idn-2:kp:1'})
SET kp.chunkId = 'idn-2'
SET kp.ordinal = 1
SET kp.text = 'First: essence is simple self-reference, pure identity';
MATCH (c:IntegratedChunk {id: 'idn-2'})
MATCH (kp:KeyPoint {id: 'idn-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-2:kp:2'})
SET kp.chunkId = 'idn-2'
SET kp.ordinal = 2
SET kp.text = 'This determination is absence of determination';
MATCH (c:IntegratedChunk {id: 'idn-2'})
MATCH (kp:KeyPoint {id: 'idn-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-2:kp:3'})
SET kp.chunkId = 'idn-2'
SET kp.ordinal = 3
SET kp.text = 'Second: specifying determination is difference';
MATCH (c:IntegratedChunk {id: 'idn-2'})
MATCH (kp:KeyPoint {id: 'idn-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-2:kp:4'})
SET kp.chunkId = 'idn-2'
SET kp.ordinal = 4
SET kp.text = 'Difference: external or indefinite, diversity, or opposed diversity or opposition';
MATCH (c:IntegratedChunk {id: 'idn-2'})
MATCH (kp:KeyPoint {id: 'idn-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-2:kp:5'})
SET kp.chunkId = 'idn-2'
SET kp.ordinal = 5
SET kp.text = 'Third: as contradiction, opposition reflected into itself and returns to foundation';
MATCH (c:IntegratedChunk {id: 'idn-2'})
MATCH (kp:KeyPoint {id: 'idn-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:idn-3'})
SET segment.sourceId = 'source-identity'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET segment.lineStart = 32
SET segment.lineEnd = 39
SET segment.text = 'A. IDENTITY\n\n1. Essence is simple immediacy as sublated immediacy.\nIts negativity is its being;\nit is equal to itself in its absolute negativity\nby virtue of which otherness and reference to other\nhave as such simply disappeared into pure self-equality.\nEssence is therefore simple self-identity.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (segment:ChunkSegment {id: 'chunk:idn-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:idn-3'})
SET topic.sourceId = 'source-identity'
SET topic.topicRef = 'idn-3'
SET topic.title = 'A. IDENTITY — essence as simple self-identity'
SET topic.description = 'Essence is simple immediacy as sublated immediacy. Its negativity is its being. Equal to itself in absolute negativity. By virtue of which otherness and reference to other have disappeared into pure self-equality. Essence is therefore simple self-identity.'
SET topic.keyPoints = ['Essence is simple immediacy as sublated immediacy', 'Its negativity is its being', 'Equal to itself in absolute negativity', 'Otherness and reference to other have disappeared into pure self-equality', 'Essence is therefore simple self-identity'];
MATCH (segment:ChunkSegment {id: 'chunk:idn-3'})
MATCH (topic:Topic {id: 'topic:idn-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'idn-3'})
SET c.title = 'A. IDENTITY — essence as simple self-identity'
SET c.sourceId = 'source-identity'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET c.lineStart = 32
SET c.lineEnd = 39
SET c.description = 'Essence is simple immediacy as sublated immediacy. Its negativity is its being. Equal to itself in absolute negativity. By virtue of which otherness and reference to other have disappeared into pure self-equality. Essence is therefore simple self-identity.'
SET c.keyPoints = ['Essence is simple immediacy as sublated immediacy', 'Its negativity is its being', 'Equal to itself in absolute negativity', 'Otherness and reference to other have disappeared into pure self-equality', 'Essence is therefore simple self-identity']
SET c.tags = ['sublation', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 44
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'A. IDENTITY\n\n1. Essence is simple immediacy as sublated immediacy.\nIts negativity is its being;\nit is equal to itself in its absolute negativity\nby virtue of which otherness and reference to other\nhave as such simply disappeared into pure self-equality.\nEssence is therefore simple self-identity.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (c:IntegratedChunk {id: 'idn-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:idn-3'})
MATCH (c:IntegratedChunk {id: 'idn-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'idn-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'idn-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'idn-3:kp:1'})
SET kp.chunkId = 'idn-3'
SET kp.ordinal = 1
SET kp.text = 'Essence is simple immediacy as sublated immediacy';
MATCH (c:IntegratedChunk {id: 'idn-3'})
MATCH (kp:KeyPoint {id: 'idn-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-3:kp:2'})
SET kp.chunkId = 'idn-3'
SET kp.ordinal = 2
SET kp.text = 'Its negativity is its being';
MATCH (c:IntegratedChunk {id: 'idn-3'})
MATCH (kp:KeyPoint {id: 'idn-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-3:kp:3'})
SET kp.chunkId = 'idn-3'
SET kp.ordinal = 3
SET kp.text = 'Equal to itself in absolute negativity';
MATCH (c:IntegratedChunk {id: 'idn-3'})
MATCH (kp:KeyPoint {id: 'idn-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-3:kp:4'})
SET kp.chunkId = 'idn-3'
SET kp.ordinal = 4
SET kp.text = 'Otherness and reference to other have disappeared into pure self-equality';
MATCH (c:IntegratedChunk {id: 'idn-3'})
MATCH (kp:KeyPoint {id: 'idn-3:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-3:kp:5'})
SET kp.chunkId = 'idn-3'
SET kp.ordinal = 5
SET kp.text = 'Essence is therefore simple self-identity';
MATCH (c:IntegratedChunk {id: 'idn-3'})
MATCH (kp:KeyPoint {id: 'idn-3:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:idn-4'})
SET segment.sourceId = 'source-identity'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET segment.lineStart = 41
SET segment.lineEnd = 46
SET segment.text = 'This self-identity is the immediacy of reflection.\nIt is not that self-equality which being is, or also nothing,\nbut a self-equality which, in producing itself as unity,\ndoes not produce itself over again, as from another,\nbut is a pure production, from itself and in itself,\nessential identity.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (segment:ChunkSegment {id: 'chunk:idn-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:idn-4'})
SET topic.sourceId = 'source-identity'
SET topic.topicRef = 'idn-4'
SET topic.title = 'Self-identity as immediacy of reflection — essential identity'
SET topic.description = 'This self-identity is immediacy of reflection. Not self-equality which being is, or also nothing. Self-equality which, in producing itself as unity, does not produce itself over again, as from another. Pure production, from itself and in itself. Essential identity.'
SET topic.keyPoints = ['Self-identity is immediacy of reflection', 'Not self-equality which being is, or also nothing', 'Self-equality producing itself as unity, not from another', 'Pure production, from itself and in itself', 'Essential identity'];
MATCH (segment:ChunkSegment {id: 'chunk:idn-4'})
MATCH (topic:Topic {id: 'topic:idn-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'idn-4'})
SET c.title = 'Self-identity as immediacy of reflection — essential identity'
SET c.sourceId = 'source-identity'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET c.lineStart = 41
SET c.lineEnd = 46
SET c.description = 'This self-identity is immediacy of reflection. Not self-equality which being is, or also nothing. Self-equality which, in producing itself as unity, does not produce itself over again, as from another. Pure production, from itself and in itself. Essential identity.'
SET c.keyPoints = ['Self-identity is immediacy of reflection', 'Not self-equality which being is, or also nothing', 'Self-equality producing itself as unity, not from another', 'Pure production, from itself and in itself', 'Essential identity']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 45
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'This self-identity is the immediacy of reflection.\nIt is not that self-equality which being is, or also nothing,\nbut a self-equality which, in producing itself as unity,\ndoes not produce itself over again, as from another,\nbut is a pure production, from itself and in itself,\nessential identity.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (c:IntegratedChunk {id: 'idn-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:idn-4'})
MATCH (c:IntegratedChunk {id: 'idn-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'idn-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'idn-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'idn-4:kp:1'})
SET kp.chunkId = 'idn-4'
SET kp.ordinal = 1
SET kp.text = 'Self-identity is immediacy of reflection';
MATCH (c:IntegratedChunk {id: 'idn-4'})
MATCH (kp:KeyPoint {id: 'idn-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-4:kp:2'})
SET kp.chunkId = 'idn-4'
SET kp.ordinal = 2
SET kp.text = 'Not self-equality which being is, or also nothing';
MATCH (c:IntegratedChunk {id: 'idn-4'})
MATCH (kp:KeyPoint {id: 'idn-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-4:kp:3'})
SET kp.chunkId = 'idn-4'
SET kp.ordinal = 3
SET kp.text = 'Self-equality producing itself as unity, not from another';
MATCH (c:IntegratedChunk {id: 'idn-4'})
MATCH (kp:KeyPoint {id: 'idn-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-4:kp:4'})
SET kp.chunkId = 'idn-4'
SET kp.ordinal = 4
SET kp.text = 'Pure production, from itself and in itself';
MATCH (c:IntegratedChunk {id: 'idn-4'})
MATCH (kp:KeyPoint {id: 'idn-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-4:kp:5'})
SET kp.chunkId = 'idn-4'
SET kp.ordinal = 5
SET kp.text = 'Essential identity';
MATCH (c:IntegratedChunk {id: 'idn-4'})
MATCH (kp:KeyPoint {id: 'idn-4:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:idn-5'})
SET segment.sourceId = 'source-identity'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET segment.lineStart = 48
SET segment.lineEnd = 54
SET segment.text = 'It is not, therefore, abstract identity\nor an identity which is the result\nof a relative negation preceding it,\none that separates indeed\nwhat it distinguishes from it\nbut, for the rest, leaves it existing outside it,\nthe same after as before.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (segment:ChunkSegment {id: 'chunk:idn-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:idn-5'})
SET topic.sourceId = 'source-identity'
SET topic.topicRef = 'idn-5'
SET topic.title = 'Not abstract identity — relative negation'
SET topic.description = 'Not abstract identity. Not identity which is result of relative negation preceding it. Relative negation separates what it distinguishes but leaves it existing outside it. Same after as before.'
SET topic.keyPoints = ['Not abstract identity', 'Not identity which is result of relative negation preceding it', 'Relative negation separates what it distinguishes but leaves it existing outside it', 'Same after as before'];
MATCH (segment:ChunkSegment {id: 'chunk:idn-5'})
MATCH (topic:Topic {id: 'topic:idn-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'idn-5'})
SET c.title = 'Not abstract identity — relative negation'
SET c.sourceId = 'source-identity'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET c.lineStart = 48
SET c.lineEnd = 54
SET c.description = 'Not abstract identity. Not identity which is result of relative negation preceding it. Relative negation separates what it distinguishes but leaves it existing outside it. Same after as before.'
SET c.keyPoints = ['Not abstract identity', 'Not identity which is result of relative negation preceding it', 'Relative negation separates what it distinguishes but leaves it existing outside it', 'Same after as before']
SET c.tags = ['negation', 'citta']
SET c.orderInSource = 5
SET c.globalOrder = 46
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'It is not, therefore, abstract identity\nor an identity which is the result\nof a relative negation preceding it,\none that separates indeed\nwhat it distinguishes from it\nbut, for the rest, leaves it existing outside it,\nthe same after as before.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (c:IntegratedChunk {id: 'idn-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:idn-5'})
MATCH (c:IntegratedChunk {id: 'idn-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'idn-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'idn-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'idn-5:kp:1'})
SET kp.chunkId = 'idn-5'
SET kp.ordinal = 1
SET kp.text = 'Not abstract identity';
MATCH (c:IntegratedChunk {id: 'idn-5'})
MATCH (kp:KeyPoint {id: 'idn-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-5:kp:2'})
SET kp.chunkId = 'idn-5'
SET kp.ordinal = 2
SET kp.text = 'Not identity which is result of relative negation preceding it';
MATCH (c:IntegratedChunk {id: 'idn-5'})
MATCH (kp:KeyPoint {id: 'idn-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-5:kp:3'})
SET kp.chunkId = 'idn-5'
SET kp.ordinal = 3
SET kp.text = 'Relative negation separates what it distinguishes but leaves it existing outside it';
MATCH (c:IntegratedChunk {id: 'idn-5'})
MATCH (kp:KeyPoint {id: 'idn-5:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-5:kp:4'})
SET kp.chunkId = 'idn-5'
SET kp.ordinal = 4
SET kp.text = 'Same after as before';
MATCH (c:IntegratedChunk {id: 'idn-5'})
MATCH (kp:KeyPoint {id: 'idn-5:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:idn-6'})
SET segment.sourceId = 'source-identity'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET segment.lineStart = 56
SET segment.lineEnd = 63
SET segment.text = 'Being, and every determinateness of being,\nhas rather sublated itself not relatively,\nbut in itself, and this simple negativity,\nthe negativity of being in itself,\nis the identity itself.\n\nIn general, therefore,\nit is still the same as essence.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (segment:ChunkSegment {id: 'chunk:idn-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:idn-6'})
SET topic.sourceId = 'source-identity'
SET topic.topicRef = 'idn-6'
SET topic.title = 'Being\'s negativity is identity itself'
SET topic.description = 'Being, and every determinateness of being, has sublated itself not relatively, but in itself. This simple negativity, negativity of being in itself, is the identity itself. In general, therefore, it is still the same as essence.'
SET topic.keyPoints = ['Being, and every determinateness of being, has sublated itself not relatively, but in itself', 'This simple negativity, negativity of being in itself, is the identity itself', 'In general, therefore, it is still the same as essence'];
MATCH (segment:ChunkSegment {id: 'chunk:idn-6'})
MATCH (topic:Topic {id: 'topic:idn-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'idn-6'})
SET c.title = 'Being\'s negativity is identity itself'
SET c.sourceId = 'source-identity'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/identity.txt'
SET c.lineStart = 56
SET c.lineEnd = 63
SET c.description = 'Being, and every determinateness of being, has sublated itself not relatively, but in itself. This simple negativity, negativity of being in itself, is the identity itself. In general, therefore, it is still the same as essence.'
SET c.keyPoints = ['Being, and every determinateness of being, has sublated itself not relatively, but in itself', 'This simple negativity, negativity of being in itself, is the identity itself', 'In general, therefore, it is still the same as essence']
SET c.tags = ['sublation', 'citta']
SET c.orderInSource = 6
SET c.globalOrder = 47
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Being, and every determinateness of being,\nhas rather sublated itself not relatively,\nbut in itself, and this simple negativity,\nthe negativity of being in itself,\nis the identity itself.\n\nIn general, therefore,\nit is still the same as essence.';
MATCH (s:SourceText {id: 'source-identity'})
MATCH (c:IntegratedChunk {id: 'idn-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:idn-6'})
MATCH (c:IntegratedChunk {id: 'idn-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'idn-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'idn-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'idn-6:kp:1'})
SET kp.chunkId = 'idn-6'
SET kp.ordinal = 1
SET kp.text = 'Being, and every determinateness of being, has sublated itself not relatively, but in itself';
MATCH (c:IntegratedChunk {id: 'idn-6'})
MATCH (kp:KeyPoint {id: 'idn-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-6:kp:2'})
SET kp.chunkId = 'idn-6'
SET kp.ordinal = 2
SET kp.text = 'This simple negativity, negativity of being in itself, is the identity itself';
MATCH (c:IntegratedChunk {id: 'idn-6'})
MATCH (kp:KeyPoint {id: 'idn-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'idn-6:kp:3'})
SET kp.chunkId = 'idn-6'
SET kp.ordinal = 3
SET kp.text = 'In general, therefore, it is still the same as essence';
MATCH (c:IntegratedChunk {id: 'idn-6'})
MATCH (kp:KeyPoint {id: 'idn-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
