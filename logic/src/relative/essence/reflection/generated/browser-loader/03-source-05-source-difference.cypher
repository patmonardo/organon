MERGE (s:SourceText {id: 'source-difference'})
SET s.title = 'Difference'
SET s.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET s.totalLines = 594;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-difference'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:diff-1'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 6
SET segment.lineEnd = 12
SET segment.text = 'Difference is the negativity that\nreflection possesses in itself,\nthe nothing which is said in identity discourse,\nthe essential moment of identity itself\nwhich, as the negativity of itself,\nat the same time determines itself\nand is differentiated from difference.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-1'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-1'
SET topic.title = 'Introduction — difference as negativity of reflection'
SET topic.description = 'Difference is negativity that reflection possesses in itself. Nothing which is said in identity discourse. Essential moment of identity itself. As negativity of itself, determines itself and is differentiated from difference.'
SET topic.keyPoints = ['Difference is negativity that reflection possesses in itself', 'Nothing which is said in identity discourse', 'Essential moment of identity itself', 'As negativity of itself, determines itself and is differentiated from difference'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-1'})
MATCH (topic:Topic {id: 'topic:diff-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-1'})
SET c.title = 'Introduction — difference as negativity of reflection'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 6
SET c.lineEnd = 12
SET c.description = 'Difference is negativity that reflection possesses in itself. Nothing which is said in identity discourse. Essential moment of identity itself. As negativity of itself, determines itself and is differentiated from difference.'
SET c.keyPoints = ['Difference is negativity that reflection possesses in itself', 'Nothing which is said in identity discourse', 'Essential moment of identity itself', 'As negativity of itself, determines itself and is differentiated from difference']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 48
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Difference is the negativity that\nreflection possesses in itself,\nthe nothing which is said in identity discourse,\nthe essential moment of identity itself\nwhich, as the negativity of itself,\nat the same time determines itself\nand is differentiated from difference.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-1'})
MATCH (c:IntegratedChunk {id: 'diff-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-1:kp:1'})
SET kp.chunkId = 'diff-1'
SET kp.ordinal = 1
SET kp.text = 'Difference is negativity that reflection possesses in itself';
MATCH (c:IntegratedChunk {id: 'diff-1'})
MATCH (kp:KeyPoint {id: 'diff-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-1:kp:2'})
SET kp.chunkId = 'diff-1'
SET kp.ordinal = 2
SET kp.text = 'Nothing which is said in identity discourse';
MATCH (c:IntegratedChunk {id: 'diff-1'})
MATCH (kp:KeyPoint {id: 'diff-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-1:kp:3'})
SET kp.chunkId = 'diff-1'
SET kp.ordinal = 3
SET kp.text = 'Essential moment of identity itself';
MATCH (c:IntegratedChunk {id: 'diff-1'})
MATCH (kp:KeyPoint {id: 'diff-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-1:kp:4'})
SET kp.chunkId = 'diff-1'
SET kp.ordinal = 4
SET kp.text = 'As negativity of itself, determines itself and is differentiated from difference';
MATCH (c:IntegratedChunk {id: 'diff-1'})
MATCH (kp:KeyPoint {id: 'diff-1:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-2'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 14
SET segment.lineEnd = 47
SET segment.text = '1. This difference is difference in and for itself,\nabsolute difference, the difference of essence.\nIt is difference in and for itself,\nnot difference through something external\nbut self-referring, hence simple, difference.\nIt is essential that we grasp absolute difference as simple.\nIn the absolute difference of A and not-A from each other,\nit is the simple “not” which, as such,\nconstitutes the difference.\nDifference itself is a simple concept.\n“In this,” so it is said, “two things differ, in that etc.”\n“In this,” that is, in one and the same respect,\nrelative to the same basis of determination.\nIt is the difference of reflection,\nnot the otherness of existence.\nOne existence and another existence are\nposited as lying outside each other;\neach of the two existences thus\ndetermined over against each other\nhas an immediate being for itself.\nThe other of essence, by contrast,\nis the other in and for itself,\nnot the other of some other\nwhich is to be found outside it;\nit is simple determinateness in itself.\nAlso in the sphere of existence\ndid otherness and determinateness\nprove to be of this nature,\nsimple determinateness, identical opposition;\nbut this identity showed itself only as\nthe transition of a determinateness into the other.\nHere, in the sphere of reflection,\ndifference comes in as reflected,\nso posited as it is in itself.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-2'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-2'
SET topic.title = 'Absolute difference as simple'
SET topic.description = 'Difference in and for itself, absolute difference, difference of essence. Not difference through something external but self-referring, simple difference. Essential to grasp absolute difference as simple. In absolute difference of A and not-A, simple \'not\' constitutes difference. Difference itself is simple concept. Difference of reflection, not otherness of existence. Other of essence is other in and for itself, simple determinateness in itself.'
SET topic.keyPoints = ['Absolute difference, difference of essence, self-referring, simple difference', 'Essential to grasp absolute difference as simple', 'In A and not-A, simple \'not\' constitutes difference', 'Difference of reflection, not otherness of existence', 'Other of essence is other in and for itself, simple determinateness in itself'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-2'})
MATCH (topic:Topic {id: 'topic:diff-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-2'})
SET c.title = 'Absolute difference as simple'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 14
SET c.lineEnd = 47
SET c.description = 'Difference in and for itself, absolute difference, difference of essence. Not difference through something external but self-referring, simple difference. Essential to grasp absolute difference as simple. In absolute difference of A and not-A, simple \'not\' constitutes difference. Difference itself is simple concept. Difference of reflection, not otherness of existence. Other of essence is other in and for itself, simple determinateness in itself.'
SET c.keyPoints = ['Absolute difference, difference of essence, self-referring, simple difference', 'Essential to grasp absolute difference as simple', 'In A and not-A, simple \'not\' constitutes difference', 'Difference of reflection, not otherness of existence', 'Other of essence is other in and for itself, simple determinateness in itself']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 49
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '1. This difference is difference in and for itself,\nabsolute difference, the difference of essence.\nIt is difference in and for itself,\nnot difference through something external\nbut self-referring, hence simple, difference.\nIt is essential that we grasp absolute difference as simple.\nIn the absolute difference of A and not-A from each other,\nit is the simple “not” which, as such,\nconstitutes the difference.\nDifference itself is a simple concept.\n“In this,” so it is said, “two things differ, in that etc.”\n“In this,” that is, in one and the same respect,\nrelative to the same basis of determination.\nIt is the difference of reflection,\nnot the otherness of existence.\nOne existence and another existence are\nposited as lying outside each other;\neach of the two existences thus\ndetermined over against each other\nhas an immediate being for itself.\nThe other of essence, by contrast,\nis the other in and for itself,\nnot the other of some other\nwhich is to be found outside it;\nit is simple determinateness in itself.\nAlso in the sphere of existence\ndid otherness and determinateness\nprove to be of this nature,\nsimple determinateness, identical opposition;\nbut this identity showed itself only as\nthe transition of a determinateness into the other.\nHere, in the sphere of reflection,\ndifference comes in as reflected,\nso posited as it is in itself.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-2'})
MATCH (c:IntegratedChunk {id: 'diff-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-2:kp:1'})
SET kp.chunkId = 'diff-2'
SET kp.ordinal = 1
SET kp.text = 'Absolute difference, difference of essence, self-referring, simple difference';
MATCH (c:IntegratedChunk {id: 'diff-2'})
MATCH (kp:KeyPoint {id: 'diff-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-2:kp:2'})
SET kp.chunkId = 'diff-2'
SET kp.ordinal = 2
SET kp.text = 'Essential to grasp absolute difference as simple';
MATCH (c:IntegratedChunk {id: 'diff-2'})
MATCH (kp:KeyPoint {id: 'diff-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-2:kp:3'})
SET kp.chunkId = 'diff-2'
SET kp.ordinal = 3
SET kp.text = 'In A and not-A, simple \'not\' constitutes difference';
MATCH (c:IntegratedChunk {id: 'diff-2'})
MATCH (kp:KeyPoint {id: 'diff-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-2:kp:4'})
SET kp.chunkId = 'diff-2'
SET kp.ordinal = 4
SET kp.text = 'Difference of reflection, not otherness of existence';
MATCH (c:IntegratedChunk {id: 'diff-2'})
MATCH (kp:KeyPoint {id: 'diff-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-2:kp:5'})
SET kp.chunkId = 'diff-2'
SET kp.ordinal = 5
SET kp.text = 'Other of essence is other in and for itself, simple determinateness in itself';
MATCH (c:IntegratedChunk {id: 'diff-2'})
MATCH (kp:KeyPoint {id: 'diff-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-3'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 49
SET segment.lineEnd = 82
SET segment.text = '2. Difference in itself is the difference\nthat refers itself to itself;\nthus it is the negativity of itself,\nthe difference not from another\nbut of itself from itself;\nit is not itself but its other.\nWhat is different from difference, however, is identity.\nDifference is, therefore, itself and identity.\nThe two together constitute difference;\ndifference is the whole and its moment.\nOne can also say that difference,\nas simple difference, is no difference;\nit is such only with reference to identity;\neven better, that as difference it entails\nitself and this reference equally.\nDifference is the whole and its own moment,\njust as identity equally is its whole and its moment.\nThis is to be regarded as\nthe essential nature of reflection\nand as the determined primordial origin\nof all activity and self-movement.\nBoth difference and identity make themselves\ninto moment or positedness\nbecause, as reflection, they are negative self-reference.\nDifference, thus as unity of itself and of identity,\nis internally determined difference.\nIt is not the transition into another,\nnot reference to another outside it;\nit has its other, identity, within,\nand in like manner identity,\nin being included in the determination of difference,\nhas not lost itself in it as its other\nbut retains itself therein is\nthe reflection-into-itself of difference, its moment.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-3'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-3'
SET topic.title = 'Difference in itself — unity with identity'
SET topic.description = 'Difference in itself refers itself to itself. Negativity of itself, difference not from another but of itself from itself. Not itself but its other. What is different from difference is identity. Difference is itself and identity. Two together constitute difference; difference is whole and its moment. Essential nature of reflection, determined primordial origin of all activity and self-movement. Difference as unity of itself and identity is internally determined difference.'
SET topic.keyPoints = ['Difference in itself refers itself to itself, difference of itself from itself', 'What is different from difference is identity', 'Difference is itself and identity, whole and its moment', 'Essential nature of reflection, primordial origin of all activity and self-movement', 'Difference as unity of itself and identity is internally determined difference'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-3'})
MATCH (topic:Topic {id: 'topic:diff-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-3'})
SET c.title = 'Difference in itself — unity with identity'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 49
SET c.lineEnd = 82
SET c.description = 'Difference in itself refers itself to itself. Negativity of itself, difference not from another but of itself from itself. Not itself but its other. What is different from difference is identity. Difference is itself and identity. Two together constitute difference; difference is whole and its moment. Essential nature of reflection, determined primordial origin of all activity and self-movement. Difference as unity of itself and identity is internally determined difference.'
SET c.keyPoints = ['Difference in itself refers itself to itself, difference of itself from itself', 'What is different from difference is identity', 'Difference is itself and identity, whole and its moment', 'Essential nature of reflection, primordial origin of all activity and self-movement', 'Difference as unity of itself and identity is internally determined difference']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 50
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '2. Difference in itself is the difference\nthat refers itself to itself;\nthus it is the negativity of itself,\nthe difference not from another\nbut of itself from itself;\nit is not itself but its other.\nWhat is different from difference, however, is identity.\nDifference is, therefore, itself and identity.\nThe two together constitute difference;\ndifference is the whole and its moment.\nOne can also say that difference,\nas simple difference, is no difference;\nit is such only with reference to identity;\neven better, that as difference it entails\nitself and this reference equally.\nDifference is the whole and its own moment,\njust as identity equally is its whole and its moment.\nThis is to be regarded as\nthe essential nature of reflection\nand as the determined primordial origin\nof all activity and self-movement.\nBoth difference and identity make themselves\ninto moment or positedness\nbecause, as reflection, they are negative self-reference.\nDifference, thus as unity of itself and of identity,\nis internally determined difference.\nIt is not the transition into another,\nnot reference to another outside it;\nit has its other, identity, within,\nand in like manner identity,\nin being included in the determination of difference,\nhas not lost itself in it as its other\nbut retains itself therein is\nthe reflection-into-itself of difference, its moment.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-3'})
MATCH (c:IntegratedChunk {id: 'diff-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-3:kp:1'})
SET kp.chunkId = 'diff-3'
SET kp.ordinal = 1
SET kp.text = 'Difference in itself refers itself to itself, difference of itself from itself';
MATCH (c:IntegratedChunk {id: 'diff-3'})
MATCH (kp:KeyPoint {id: 'diff-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-3:kp:2'})
SET kp.chunkId = 'diff-3'
SET kp.ordinal = 2
SET kp.text = 'What is different from difference is identity';
MATCH (c:IntegratedChunk {id: 'diff-3'})
MATCH (kp:KeyPoint {id: 'diff-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-3:kp:3'})
SET kp.chunkId = 'diff-3'
SET kp.ordinal = 3
SET kp.text = 'Difference is itself and identity, whole and its moment';
MATCH (c:IntegratedChunk {id: 'diff-3'})
MATCH (kp:KeyPoint {id: 'diff-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-3:kp:4'})
SET kp.chunkId = 'diff-3'
SET kp.ordinal = 4
SET kp.text = 'Essential nature of reflection, primordial origin of all activity and self-movement';
MATCH (c:IntegratedChunk {id: 'diff-3'})
MATCH (kp:KeyPoint {id: 'diff-3:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-3:kp:5'})
SET kp.chunkId = 'diff-3'
SET kp.ordinal = 5
SET kp.text = 'Difference as unity of itself and identity is internally determined difference';
MATCH (c:IntegratedChunk {id: 'diff-3'})
MATCH (kp:KeyPoint {id: 'diff-3:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-4'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 84
SET segment.lineEnd = 94
SET segment.text = '3. Difference has both these moments,\nidentity and difference;\nthus the two are both a positedness, determinateness.\nBut in this positedness each refers to itself.\nThe one, identity, is itself immediately\nthe moment of immanent reflection;\nbut no less is the other, difference,\ndifference in itself, reflected difference.\nDifference, inasmuch as it has two such moments\nwhich are themselves reflections into themselves,\nis diversity.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-4'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-4'
SET topic.title = 'Difference has both moments — diversity'
SET topic.description = 'Difference has both moments, identity and difference. Two are both positedness, determinateness. In this positedness each refers to itself. Identity is moment of immanent reflection. Difference is difference in itself, reflected difference. Difference, having two moments which are reflections into themselves, is diversity.'
SET topic.keyPoints = ['Difference has both moments, identity and difference', 'In positedness each refers to itself', 'Identity is moment of immanent reflection', 'Difference is difference in itself, reflected difference', 'Difference with two moments which are reflections into themselves is diversity'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-4'})
MATCH (topic:Topic {id: 'topic:diff-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-4'})
SET c.title = 'Difference has both moments — diversity'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 84
SET c.lineEnd = 94
SET c.description = 'Difference has both moments, identity and difference. Two are both positedness, determinateness. In this positedness each refers to itself. Identity is moment of immanent reflection. Difference is difference in itself, reflected difference. Difference, having two moments which are reflections into themselves, is diversity.'
SET c.keyPoints = ['Difference has both moments, identity and difference', 'In positedness each refers to itself', 'Identity is moment of immanent reflection', 'Difference is difference in itself, reflected difference', 'Difference with two moments which are reflections into themselves is diversity']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 51
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '3. Difference has both these moments,\nidentity and difference;\nthus the two are both a positedness, determinateness.\nBut in this positedness each refers to itself.\nThe one, identity, is itself immediately\nthe moment of immanent reflection;\nbut no less is the other, difference,\ndifference in itself, reflected difference.\nDifference, inasmuch as it has two such moments\nwhich are themselves reflections into themselves,\nis diversity.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-4'})
MATCH (c:IntegratedChunk {id: 'diff-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-4:kp:1'})
SET kp.chunkId = 'diff-4'
SET kp.ordinal = 1
SET kp.text = 'Difference has both moments, identity and difference';
MATCH (c:IntegratedChunk {id: 'diff-4'})
MATCH (kp:KeyPoint {id: 'diff-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-4:kp:2'})
SET kp.chunkId = 'diff-4'
SET kp.ordinal = 2
SET kp.text = 'In positedness each refers to itself';
MATCH (c:IntegratedChunk {id: 'diff-4'})
MATCH (kp:KeyPoint {id: 'diff-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-4:kp:3'})
SET kp.chunkId = 'diff-4'
SET kp.ordinal = 3
SET kp.text = 'Identity is moment of immanent reflection';
MATCH (c:IntegratedChunk {id: 'diff-4'})
MATCH (kp:KeyPoint {id: 'diff-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-4:kp:4'})
SET kp.chunkId = 'diff-4'
SET kp.ordinal = 4
SET kp.text = 'Difference is difference in itself, reflected difference';
MATCH (c:IntegratedChunk {id: 'diff-4'})
MATCH (kp:KeyPoint {id: 'diff-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-4:kp:5'})
SET kp.chunkId = 'diff-4'
SET kp.ordinal = 5
SET kp.text = 'Difference with two moments which are reflections into themselves is diversity';
MATCH (c:IntegratedChunk {id: 'diff-4'})
MATCH (kp:KeyPoint {id: 'diff-4:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-5'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 98
SET segment.lineEnd = 121
SET segment.text = '1. Identity internally breaks apart into diversity\nbecause, as absolute difference in itself,\nit posits itself as the negative of itself\nand these, its two moments\n(itself and the negative of itself),\nare reflections into themselves,\nare identical with themselves;\nor precisely because it itself\nimmediately sublates its negating\nand is in its determination reflected into itself.\nThe different subsists as diverse,\nindifferent to any other,\nbecause it is identical with itself,\nbecause identity constitutes its base and element;\nor, the diverse remains what it is\neven in its opposite, identity.\n\nDiversity constitutes the otherness\nas such of reflection.\nThe other of existence has immediate being,\nwhere negativity resides, for its foundation.\nBut in reflection it is self-identity,\nthe reflected immediacy, that constitutes\nthe subsistence of the negative and its indifference.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-5'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-5'
SET topic.title = 'Diversity — identity breaks apart'
SET topic.description = 'Identity internally breaks apart into diversity. As absolute difference in itself, posits itself as negative of itself. Two moments (itself and negative of itself) are reflections into themselves. Different subsists as diverse, indifferent to any other. Because identical with itself, identity constitutes base and element. Diverse remains what it is even in its opposite, identity. Diversity constitutes otherness as such of reflection.'
SET topic.keyPoints = ['Identity internally breaks apart into diversity', 'Posits itself as negative of itself', 'Different subsists as diverse, indifferent to any other', 'Identity constitutes base and element', 'Diversity constitutes otherness as such of reflection'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-5'})
MATCH (topic:Topic {id: 'topic:diff-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-5'})
SET c.title = 'Diversity — identity breaks apart'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 98
SET c.lineEnd = 121
SET c.description = 'Identity internally breaks apart into diversity. As absolute difference in itself, posits itself as negative of itself. Two moments (itself and negative of itself) are reflections into themselves. Different subsists as diverse, indifferent to any other. Because identical with itself, identity constitutes base and element. Diverse remains what it is even in its opposite, identity. Diversity constitutes otherness as such of reflection.'
SET c.keyPoints = ['Identity internally breaks apart into diversity', 'Posits itself as negative of itself', 'Different subsists as diverse, indifferent to any other', 'Identity constitutes base and element', 'Diversity constitutes otherness as such of reflection']
SET c.tags = ['negation', 'reflection', 'citta']
SET c.orderInSource = 5
SET c.globalOrder = 52
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '1. Identity internally breaks apart into diversity\nbecause, as absolute difference in itself,\nit posits itself as the negative of itself\nand these, its two moments\n(itself and the negative of itself),\nare reflections into themselves,\nare identical with themselves;\nor precisely because it itself\nimmediately sublates its negating\nand is in its determination reflected into itself.\nThe different subsists as diverse,\nindifferent to any other,\nbecause it is identical with itself,\nbecause identity constitutes its base and element;\nor, the diverse remains what it is\neven in its opposite, identity.\n\nDiversity constitutes the otherness\nas such of reflection.\nThe other of existence has immediate being,\nwhere negativity resides, for its foundation.\nBut in reflection it is self-identity,\nthe reflected immediacy, that constitutes\nthe subsistence of the negative and its indifference.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-5'})
MATCH (c:IntegratedChunk {id: 'diff-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-5:kp:1'})
SET kp.chunkId = 'diff-5'
SET kp.ordinal = 1
SET kp.text = 'Identity internally breaks apart into diversity';
MATCH (c:IntegratedChunk {id: 'diff-5'})
MATCH (kp:KeyPoint {id: 'diff-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-5:kp:2'})
SET kp.chunkId = 'diff-5'
SET kp.ordinal = 2
SET kp.text = 'Posits itself as negative of itself';
MATCH (c:IntegratedChunk {id: 'diff-5'})
MATCH (kp:KeyPoint {id: 'diff-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-5:kp:3'})
SET kp.chunkId = 'diff-5'
SET kp.ordinal = 3
SET kp.text = 'Different subsists as diverse, indifferent to any other';
MATCH (c:IntegratedChunk {id: 'diff-5'})
MATCH (kp:KeyPoint {id: 'diff-5:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-5:kp:4'})
SET kp.chunkId = 'diff-5'
SET kp.ordinal = 4
SET kp.text = 'Identity constitutes base and element';
MATCH (c:IntegratedChunk {id: 'diff-5'})
MATCH (kp:KeyPoint {id: 'diff-5:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-5:kp:5'})
SET kp.chunkId = 'diff-5'
SET kp.ordinal = 5
SET kp.text = 'Diversity constitutes otherness as such of reflection';
MATCH (c:IntegratedChunk {id: 'diff-5'})
MATCH (kp:KeyPoint {id: 'diff-5:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-6'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 123
SET segment.lineEnd = 140
SET segment.text = 'The moments of difference are identity and difference itself.\nThese moments are diverse when reflected into themselves,\nreferring themselves to themselves;\nthus, in the determination of identity,\nthey are only self-referring;\nidentity is not referred to difference,\nnor is difference referred to identity;\nhence, inasmuch as each of these moments is\nreferred only to itself, the two\nare not determined\nwith respect to each other.\nNow because in this way the two are not differentiated within,\nthe difference is external to them.\nThe diverse moments, therefore,\nconduct themselves with respect to each other,\nnot as identity and difference,\nbut only as moments different in general,\nindifferent to each other and to their determinateness.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-6'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-6'
SET topic.title = 'Diversity — moments not determined with respect to each other'
SET topic.description = 'Moments of difference are identity and difference itself. These moments are diverse when reflected into themselves. In determination of identity, only self-referring. Identity not referred to difference, difference not referred to identity. Each referred only to itself, two not determined with respect to each other. Because two not differentiated within, difference is external to them.'
SET topic.keyPoints = ['Moments of difference are identity and difference itself', 'Moments diverse when reflected into themselves', 'Each only self-referring, not referred to each other', 'Two not determined with respect to each other', 'Difference external to them'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-6'})
MATCH (topic:Topic {id: 'topic:diff-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-6'})
SET c.title = 'Diversity — moments not determined with respect to each other'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 123
SET c.lineEnd = 140
SET c.description = 'Moments of difference are identity and difference itself. These moments are diverse when reflected into themselves. In determination of identity, only self-referring. Identity not referred to difference, difference not referred to identity. Each referred only to itself, two not determined with respect to each other. Because two not differentiated within, difference is external to them.'
SET c.keyPoints = ['Moments of difference are identity and difference itself', 'Moments diverse when reflected into themselves', 'Each only self-referring, not referred to each other', 'Two not determined with respect to each other', 'Difference external to them']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 6
SET c.globalOrder = 53
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The moments of difference are identity and difference itself.\nThese moments are diverse when reflected into themselves,\nreferring themselves to themselves;\nthus, in the determination of identity,\nthey are only self-referring;\nidentity is not referred to difference,\nnor is difference referred to identity;\nhence, inasmuch as each of these moments is\nreferred only to itself, the two\nare not determined\nwith respect to each other.\nNow because in this way the two are not differentiated within,\nthe difference is external to them.\nThe diverse moments, therefore,\nconduct themselves with respect to each other,\nnot as identity and difference,\nbut only as moments different in general,\nindifferent to each other and to their determinateness.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-6'})
MATCH (c:IntegratedChunk {id: 'diff-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-6:kp:1'})
SET kp.chunkId = 'diff-6'
SET kp.ordinal = 1
SET kp.text = 'Moments of difference are identity and difference itself';
MATCH (c:IntegratedChunk {id: 'diff-6'})
MATCH (kp:KeyPoint {id: 'diff-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-6:kp:2'})
SET kp.chunkId = 'diff-6'
SET kp.ordinal = 2
SET kp.text = 'Moments diverse when reflected into themselves';
MATCH (c:IntegratedChunk {id: 'diff-6'})
MATCH (kp:KeyPoint {id: 'diff-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-6:kp:3'})
SET kp.chunkId = 'diff-6'
SET kp.ordinal = 3
SET kp.text = 'Each only self-referring, not referred to each other';
MATCH (c:IntegratedChunk {id: 'diff-6'})
MATCH (kp:KeyPoint {id: 'diff-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-6:kp:4'})
SET kp.chunkId = 'diff-6'
SET kp.ordinal = 4
SET kp.text = 'Two not determined with respect to each other';
MATCH (c:IntegratedChunk {id: 'diff-6'})
MATCH (kp:KeyPoint {id: 'diff-6:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-6:kp:5'})
SET kp.chunkId = 'diff-6'
SET kp.ordinal = 5
SET kp.text = 'Difference external to them';
MATCH (c:IntegratedChunk {id: 'diff-6'})
MATCH (kp:KeyPoint {id: 'diff-6:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-7'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 142
SET segment.lineEnd = 208
SET segment.text = '2. In diversity, as the indifference of difference,\nreflection has in general become external;\ndifference is only a positedness or as sublated,\nbut is itself the whole reflection.\nOn closer consideration, both, identity and difference\nare reflections, as we have just established;\neach is the unity of it and its other,\neach is the whole.\nBut the determinateness,\nto be only identity or only difference,\nis thus a sublated something.\nThey are not, therefore, qualities,\nsince their determinateness,\nbecause of the immanent reflection,\nis at the same time only as negation.\nWhat we have is therefore this duplicity,\nimmanent reflection as such\nand determinateness as negation or positedness.\nPositedness is the reflection that is external to itself;\nit is negation as negation\nand consequently, indeed in itself\nself-referring negation and immanent reflection,\nbut only in itself, implicitly;\nits reference is to a something external.\n\nReflection in itself and external reflection are\nthus the two determinations\nin which the moments of difference,\nidentity and difference, are posited.\nThey are these moments themselves\nas they have determined themselves at this point.\nImmanent reflection is identity,\nbut determined to be indifferent to difference,\nnot to have difference at all but to conduct\nitself towards difference as identical with itself;\nit is diversity.\nIt is identity that has so reflected itself into itself\nthat it truly is the one reflection of\nthe two moments into themselves;\nboth are immanent reflections.\nIdentity is this one reflection of the two,\nthe identity which has difference within it\nonly as an indifferent difference\nand is diversity in general.\nExternal reflection, on the contrary,\nis their determinate difference,\nnot as absolute immanent reflection,\nbut as a determination towards which\nthe implicitly present reflection is indifferent;\nits two moments, identity and difference themselves,\nare thus externally posited,\nare not determinations that\nexist in and for themselves.\n\nNow this external identity is likeness,\nand external difference is unlikeness.\nLikeness is indeed identity,\nbut only as a positedness,\nan identity which is not in and for itself.\nUnlikeness is equally difference,\nbut an external difference which is not, in and for itself,\nthe difference of the unlike itself.\nWhether something is like or unlike something else is\nnot the concern of either the like or the unlike;\neach refers only to itself, each is in and for itself what it is;\nidentity or non-identity, in the sense of likeness or unlikeness,\ndepend on the point of view of a third external to them.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-7'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-7'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-7'
SET topic.title = 'Diversity — reflection becomes external, likeness and unlikeness'
SET topic.description = 'In diversity, as indifference of difference, reflection has become external. Both identity and difference are reflections, each is whole. Determinateness to be only identity or only difference is sublated something. Duplicity: immanent reflection as such and determinateness as negation or positedness. Immanent reflection is identity, indifferent to difference, is diversity. External reflection is determinate difference. External identity is likeness, external difference is unlikeness. Whether like or unlike depends on point of view of third external to them.'
SET topic.keyPoints = ['In diversity, reflection has become external', 'Both identity and difference are reflections, each is whole', 'Duplicity: immanent reflection and determinateness as negation or positedness', 'External identity is likeness, external difference is unlikeness', 'Whether like or unlike depends on point of view of third external to them'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-7'})
MATCH (topic:Topic {id: 'topic:diff-7'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-7'})
SET c.title = 'Diversity — reflection becomes external, likeness and unlikeness'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 142
SET c.lineEnd = 208
SET c.description = 'In diversity, as indifference of difference, reflection has become external. Both identity and difference are reflections, each is whole. Determinateness to be only identity or only difference is sublated something. Duplicity: immanent reflection as such and determinateness as negation or positedness. Immanent reflection is identity, indifferent to difference, is diversity. External reflection is determinate difference. External identity is likeness, external difference is unlikeness. Whether like or unlike depends on point of view of third external to them.'
SET c.keyPoints = ['In diversity, reflection has become external', 'Both identity and difference are reflections, each is whole', 'Duplicity: immanent reflection and determinateness as negation or positedness', 'External identity is likeness, external difference is unlikeness', 'Whether like or unlike depends on point of view of third external to them']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 7
SET c.globalOrder = 54
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '2. In diversity, as the indifference of difference,\nreflection has in general become external;\ndifference is only a positedness or as sublated,\nbut is itself the whole reflection.\nOn closer consideration, both, identity and difference\nare reflections, as we have just established;\neach is the unity of it and its other,\neach is the whole.\nBut the determinateness,\nto be only identity or only difference,\nis thus a sublated something.\nThey are not, therefore, qualities,\nsince their determinateness,\nbecause of the immanent reflection,\nis at the same time only as negation.\nWhat we have is therefore this duplicity,\nimmanent reflection as such\nand determinateness as negation or positedness.\nPositedness is the reflection that is external to itself;\nit is negation as negation\nand consequently, indeed in itself\nself-referring negation and immanent reflection,\nbut only in itself, implicitly;\nits reference is to a something external.\n\nReflection in itself and external reflection are\nthus the two determinations\nin which the moments of difference,\nidentity and difference, are posited.\nThey are these moments themselves\nas they have determined themselves at this point.\nImmanent reflection is identity,\nbut determined to be indifferent to difference,\nnot to have difference at all but to conduct\nitself towards difference as identical with itself;\nit is diversity.\nIt is identity that has so reflected itself into itself\nthat it truly is the one reflection of\nthe two moments into themselves;\nboth are immanent reflections.\nIdentity is this one reflection of the two,\nthe identity which has difference within it\nonly as an indifferent difference\nand is diversity in general.\nExternal reflection, on the contrary,\nis their determinate difference,\nnot as absolute immanent reflection,\nbut as a determination towards which\nthe implicitly present reflection is indifferent;\nits two moments, identity and difference themselves,\nare thus externally posited,\nare not determinations that\nexist in and for themselves.\n\nNow this external identity is likeness,\nand external difference is unlikeness.\nLikeness is indeed identity,\nbut only as a positedness,\nan identity which is not in and for itself.\nUnlikeness is equally difference,\nbut an external difference which is not, in and for itself,\nthe difference of the unlike itself.\nWhether something is like or unlike something else is\nnot the concern of either the like or the unlike;\neach refers only to itself, each is in and for itself what it is;\nidentity or non-identity, in the sense of likeness or unlikeness,\ndepend on the point of view of a third external to them.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-7'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-7'})
MATCH (c:IntegratedChunk {id: 'diff-7'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-7'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-7'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-7:kp:1'})
SET kp.chunkId = 'diff-7'
SET kp.ordinal = 1
SET kp.text = 'In diversity, reflection has become external';
MATCH (c:IntegratedChunk {id: 'diff-7'})
MATCH (kp:KeyPoint {id: 'diff-7:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-7:kp:2'})
SET kp.chunkId = 'diff-7'
SET kp.ordinal = 2
SET kp.text = 'Both identity and difference are reflections, each is whole';
MATCH (c:IntegratedChunk {id: 'diff-7'})
MATCH (kp:KeyPoint {id: 'diff-7:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-7:kp:3'})
SET kp.chunkId = 'diff-7'
SET kp.ordinal = 3
SET kp.text = 'Duplicity: immanent reflection and determinateness as negation or positedness';
MATCH (c:IntegratedChunk {id: 'diff-7'})
MATCH (kp:KeyPoint {id: 'diff-7:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-7:kp:4'})
SET kp.chunkId = 'diff-7'
SET kp.ordinal = 4
SET kp.text = 'External identity is likeness, external difference is unlikeness';
MATCH (c:IntegratedChunk {id: 'diff-7'})
MATCH (kp:KeyPoint {id: 'diff-7:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-7:kp:5'})
SET kp.chunkId = 'diff-7'
SET kp.ordinal = 5
SET kp.text = 'Whether like or unlike depends on point of view of third external to them';
MATCH (c:IntegratedChunk {id: 'diff-7'})
MATCH (kp:KeyPoint {id: 'diff-7:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-8'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 210
SET segment.lineEnd = 231
SET segment.text = '3. External reflection connects diversity by\nreferring it to likeness and unlikeness.\nThis reference, which is a comparing,\nmoves back and forth from likeness\nto unlikeness and from unlikeness to likeness.\nBut this back and forth referring of\nlikeness and unlikeness is\nexternal to these determinations themselves;\nmoreover, they are not referred to each other,\nbut each, for itself, is referred to a third.\nIn this alternation,\neach immediately stands out on its own.\nExternal reflection is as such external to itself;\ndeterminate difference is negated absolute difference;\nit is not simple difference, therefore,\nnot an immanent reflection,\nbut has this reflection outside it;\nhence its moments come apart\nand both refer,\neach also outside the other,\nto the immanent reflection\nconfronting them.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-8'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-8'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-8'
SET topic.title = 'External reflection connects diversity — comparing'
SET topic.description = 'External reflection connects diversity by referring it to likeness and unlikeness. Reference is comparing, moves back and forth from likeness to unlikeness. Back and forth referring external to determinations themselves. Each, for itself, referred to third. External reflection external to itself. Determinate difference is negated absolute difference. Not simple difference, not immanent reflection, has reflection outside it.'
SET topic.keyPoints = ['External reflection connects diversity by referring to likeness and unlikeness', 'Reference is comparing, moves back and forth', 'Each referred to third', 'External reflection external to itself', 'Determinate difference is negated absolute difference'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-8'})
MATCH (topic:Topic {id: 'topic:diff-8'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-8'})
SET c.title = 'External reflection connects diversity — comparing'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 210
SET c.lineEnd = 231
SET c.description = 'External reflection connects diversity by referring it to likeness and unlikeness. Reference is comparing, moves back and forth from likeness to unlikeness. Back and forth referring external to determinations themselves. Each, for itself, referred to third. External reflection external to itself. Determinate difference is negated absolute difference. Not simple difference, not immanent reflection, has reflection outside it.'
SET c.keyPoints = ['External reflection connects diversity by referring to likeness and unlikeness', 'Reference is comparing, moves back and forth', 'Each referred to third', 'External reflection external to itself', 'Determinate difference is negated absolute difference']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 8
SET c.globalOrder = 55
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '3. External reflection connects diversity by\nreferring it to likeness and unlikeness.\nThis reference, which is a comparing,\nmoves back and forth from likeness\nto unlikeness and from unlikeness to likeness.\nBut this back and forth referring of\nlikeness and unlikeness is\nexternal to these determinations themselves;\nmoreover, they are not referred to each other,\nbut each, for itself, is referred to a third.\nIn this alternation,\neach immediately stands out on its own.\nExternal reflection is as such external to itself;\ndeterminate difference is negated absolute difference;\nit is not simple difference, therefore,\nnot an immanent reflection,\nbut has this reflection outside it;\nhence its moments come apart\nand both refer,\neach also outside the other,\nto the immanent reflection\nconfronting them.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-8'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-8'})
MATCH (c:IntegratedChunk {id: 'diff-8'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-8'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-8'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-8:kp:1'})
SET kp.chunkId = 'diff-8'
SET kp.ordinal = 1
SET kp.text = 'External reflection connects diversity by referring to likeness and unlikeness';
MATCH (c:IntegratedChunk {id: 'diff-8'})
MATCH (kp:KeyPoint {id: 'diff-8:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-8:kp:2'})
SET kp.chunkId = 'diff-8'
SET kp.ordinal = 2
SET kp.text = 'Reference is comparing, moves back and forth';
MATCH (c:IntegratedChunk {id: 'diff-8'})
MATCH (kp:KeyPoint {id: 'diff-8:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-8:kp:3'})
SET kp.chunkId = 'diff-8'
SET kp.ordinal = 3
SET kp.text = 'Each referred to third';
MATCH (c:IntegratedChunk {id: 'diff-8'})
MATCH (kp:KeyPoint {id: 'diff-8:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-8:kp:4'})
SET kp.chunkId = 'diff-8'
SET kp.ordinal = 4
SET kp.text = 'External reflection external to itself';
MATCH (c:IntegratedChunk {id: 'diff-8'})
MATCH (kp:KeyPoint {id: 'diff-8:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-8:kp:5'})
SET kp.chunkId = 'diff-8'
SET kp.ordinal = 5
SET kp.text = 'Determinate difference is negated absolute difference';
MATCH (c:IntegratedChunk {id: 'diff-8'})
MATCH (kp:KeyPoint {id: 'diff-8:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-9'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 233
SET segment.lineEnd = 274
SET segment.text = 'In reflection thus alienated from itself,\nlikeness and unlikeness present themselves,\ntherefore, as themselves unconnected,\nand reflection keeps them apart,\nfor it refers them to one and the same\nsomething by means of “in so far,”\n“from this side or that,”\nand “from this view or that.”\nThus diverse things that are one and the same,\nwhen likeness and unlikeness are said of them,\nare from one side like each other,\nbut from another side unlike,\nand in so far as they are alike,\nto that extent they are not unlike.\nLikeness thus refers only to itself,\nand unlikeness is equally only unlikeness.\n\nBecause of this separation from each other,\nthey sublate themselves.\nPrecisely that which should save them\nfrom contradiction and dissolution,\nnamely that something is like another in one respect\nbut unlike in another precisely this keeping of\nlikeness and unlikeness apart, is their destruction.\nFor both are determinations of difference;\nthey are references to each other,\neach intended to be what the other is not;\nthe like is not the unlike,\nand the unlike is not the like;\nboth have this connecting reference essentially,\nand have no meaning outside it;\nas determinations of difference,\neach is what it is as different from its other.\nBut because of their indifference to each other,\nthe likeness is referred to itself,\nand similarly is unlikeness a point of view of\nits own and a reflection unto itself;\neach, therefore, is like itself;\ndifference has vanished, since they have no\ndeterminateness to oppose them;\nin other words, each is consequently only likeness.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-9'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-9'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-9'
SET topic.title = 'Likeness and unlikeness — separation and self-sublation'
SET topic.description = 'In reflection alienated from itself, likeness and unlikeness present as unconnected. Reflection keeps them apart by \'in so far,\' \'from this side or that.\' Diverse things from one side like, from another unlike. Because of separation, they sublate themselves. Keeping likeness and unlikeness apart is their destruction. Both are determinations of difference, references to each other. But because of indifference, each referred to itself. Each is like itself; difference has vanished, each is only likeness.'
SET topic.keyPoints = ['Likeness and unlikeness present as unconnected', 'Reflection keeps them apart by \'in so far,\' \'from this side or that\'', 'Because of separation, they sublate themselves', 'Both are determinations of difference, references to each other', 'Each is like itself; difference has vanished, each is only likeness'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-9'})
MATCH (topic:Topic {id: 'topic:diff-9'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-9'})
SET c.title = 'Likeness and unlikeness — separation and self-sublation'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 233
SET c.lineEnd = 274
SET c.description = 'In reflection alienated from itself, likeness and unlikeness present as unconnected. Reflection keeps them apart by \'in so far,\' \'from this side or that.\' Diverse things from one side like, from another unlike. Because of separation, they sublate themselves. Keeping likeness and unlikeness apart is their destruction. Both are determinations of difference, references to each other. But because of indifference, each referred to itself. Each is like itself; difference has vanished, each is only likeness.'
SET c.keyPoints = ['Likeness and unlikeness present as unconnected', 'Reflection keeps them apart by \'in so far,\' \'from this side or that\'', 'Because of separation, they sublate themselves', 'Both are determinations of difference, references to each other', 'Each is like itself; difference has vanished, each is only likeness']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 9
SET c.globalOrder = 56
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'In reflection thus alienated from itself,\nlikeness and unlikeness present themselves,\ntherefore, as themselves unconnected,\nand reflection keeps them apart,\nfor it refers them to one and the same\nsomething by means of “in so far,”\n“from this side or that,”\nand “from this view or that.”\nThus diverse things that are one and the same,\nwhen likeness and unlikeness are said of them,\nare from one side like each other,\nbut from another side unlike,\nand in so far as they are alike,\nto that extent they are not unlike.\nLikeness thus refers only to itself,\nand unlikeness is equally only unlikeness.\n\nBecause of this separation from each other,\nthey sublate themselves.\nPrecisely that which should save them\nfrom contradiction and dissolution,\nnamely that something is like another in one respect\nbut unlike in another precisely this keeping of\nlikeness and unlikeness apart, is their destruction.\nFor both are determinations of difference;\nthey are references to each other,\neach intended to be what the other is not;\nthe like is not the unlike,\nand the unlike is not the like;\nboth have this connecting reference essentially,\nand have no meaning outside it;\nas determinations of difference,\neach is what it is as different from its other.\nBut because of their indifference to each other,\nthe likeness is referred to itself,\nand similarly is unlikeness a point of view of\nits own and a reflection unto itself;\neach, therefore, is like itself;\ndifference has vanished, since they have no\ndeterminateness to oppose them;\nin other words, each is consequently only likeness.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-9'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-9'})
MATCH (c:IntegratedChunk {id: 'diff-9'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-9'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-9'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-9:kp:1'})
SET kp.chunkId = 'diff-9'
SET kp.ordinal = 1
SET kp.text = 'Likeness and unlikeness present as unconnected';
MATCH (c:IntegratedChunk {id: 'diff-9'})
MATCH (kp:KeyPoint {id: 'diff-9:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-9:kp:2'})
SET kp.chunkId = 'diff-9'
SET kp.ordinal = 2
SET kp.text = 'Reflection keeps them apart by \'in so far,\' \'from this side or that\'';
MATCH (c:IntegratedChunk {id: 'diff-9'})
MATCH (kp:KeyPoint {id: 'diff-9:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-9:kp:3'})
SET kp.chunkId = 'diff-9'
SET kp.ordinal = 3
SET kp.text = 'Because of separation, they sublate themselves';
MATCH (c:IntegratedChunk {id: 'diff-9'})
MATCH (kp:KeyPoint {id: 'diff-9:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-9:kp:4'})
SET kp.chunkId = 'diff-9'
SET kp.ordinal = 4
SET kp.text = 'Both are determinations of difference, references to each other';
MATCH (c:IntegratedChunk {id: 'diff-9'})
MATCH (kp:KeyPoint {id: 'diff-9:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-9:kp:5'})
SET kp.chunkId = 'diff-9'
SET kp.ordinal = 5
SET kp.text = 'Each is like itself; difference has vanished, each is only likeness';
MATCH (c:IntegratedChunk {id: 'diff-9'})
MATCH (kp:KeyPoint {id: 'diff-9:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-10'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 275
SET segment.lineEnd = 347
SET segment.text = 'Accordingly, this indifferent viewpoint\nor the external difference sublates itself\nand it is in itself the negativity of itself.\nIt is the negativity which in comparing\nbelongs to that which does the comparing.\nThis latter oscillates from likeness\nto unlikeness and back again;\nhence it lets the one disappear into the other\nand is in fact the negative unity of both.\nThis negative unity transcends at first\nwhat is compared as well as\nthe moments of the comparing as\na subjective operation that falls outside them.\nBut the result is that this unity is\nin fact the nature of likeness and unlikeness themselves.\nEven the independent viewpoint\nthat each of these is,\nis rather the self-reference\nthat sublates their distinctness\nand so, too, themselves.\n\nFrom this side, as moments of external reflection\nand as external to themselves,\nlikeness and unlikeness disappear together into their likeness.\nBut this, their negative unity,\nis in addition also posited in them;\nfor their reflection implicitly exists\noutside them, that is, they are the likeness and\nunlikeness of a third,\nof another than they themselves are.\nThus the like is not the like of itself,\nand the unlike, as the unlike not of itself\nbut of an unlike to it,\nis itself the like.\nThe like and the unlike is\neach therefore the unlike of itself.\nEach is thereby this reflection:\nlikeness, that it is itself and the unlikeness;\nunlikeness, that it is itself and the likeness.\nLikeness and unlikeness constituted\nthe side of positedness as against\nwhat is being compared or the diverse\nwhich, as contrasted with them,\nhad determined itself as implicitly existent reflection.\nBut this positedness has consequently equally\nlost its determinateness as against this reflection.\n\nLikeness and unlikeness,\nthe determinations of external reflection,\nare precisely the merely\nimplicitly existent reflection\nwhich the diverse as such was supposed to be,\nits only indeterminate difference.\nImplicitly existent reflection is\nself-reference without negation,\nabstract self-identity\nand therefore positedness itself.\nThe merely diverse thus passes over\nthrough the positedness\ninto negative reflection.\nThe diverse is difference\nwhich is merely posited,\nhence a difference which is no difference,\nhence a negation that negates itself within.\nLikeness and unlikeness themselves, the positedness,\nthus return through indifference\nor through implicitly existing reflection\nback into negative unity with themselves,\ninto the reflection which is\nthe implicit difference of likeness and unlikeness.\nDiversity, the indifferent sides of which\nare just as much simply and solely\nmoments of a negative unity, is opposition.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-10'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-10'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-10'
SET topic.title = 'Negative unity — transition to opposition'
SET topic.description = 'Indifferent viewpoint or external difference sublates itself. Negativity which in comparing belongs to that which does comparing. Oscillates from likeness to unlikeness, lets one disappear into other. Negative unity of both. Unity is nature of likeness and unlikeness themselves. Like is not like of itself, unlike is like. Each is unlike of itself. Each is itself and its other (likeness/unlikeness). Merely diverse passes over through positedness into negative reflection. Diverse is difference merely posited, difference which is no difference. Negation that negates itself within. Diversity, indifferent sides of which are moments of negative unity, is opposition.'
SET topic.keyPoints = ['External difference sublates itself', 'Negative unity of likeness and unlikeness', 'Unity is nature of likeness and unlikeness themselves', 'Each is unlike of itself, itself and its other', 'Diversity, indifferent sides of which are moments of negative unity, is opposition'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-10'})
MATCH (topic:Topic {id: 'topic:diff-10'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-10'})
SET c.title = 'Negative unity — transition to opposition'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 275
SET c.lineEnd = 347
SET c.description = 'Indifferent viewpoint or external difference sublates itself. Negativity which in comparing belongs to that which does comparing. Oscillates from likeness to unlikeness, lets one disappear into other. Negative unity of both. Unity is nature of likeness and unlikeness themselves. Like is not like of itself, unlike is like. Each is unlike of itself. Each is itself and its other (likeness/unlikeness). Merely diverse passes over through positedness into negative reflection. Diverse is difference merely posited, difference which is no difference. Negation that negates itself within. Diversity, indifferent sides of which are moments of negative unity, is opposition.'
SET c.keyPoints = ['External difference sublates itself', 'Negative unity of likeness and unlikeness', 'Unity is nature of likeness and unlikeness themselves', 'Each is unlike of itself, itself and its other', 'Diversity, indifferent sides of which are moments of negative unity, is opposition']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 10
SET c.globalOrder = 57
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Accordingly, this indifferent viewpoint\nor the external difference sublates itself\nand it is in itself the negativity of itself.\nIt is the negativity which in comparing\nbelongs to that which does the comparing.\nThis latter oscillates from likeness\nto unlikeness and back again;\nhence it lets the one disappear into the other\nand is in fact the negative unity of both.\nThis negative unity transcends at first\nwhat is compared as well as\nthe moments of the comparing as\na subjective operation that falls outside them.\nBut the result is that this unity is\nin fact the nature of likeness and unlikeness themselves.\nEven the independent viewpoint\nthat each of these is,\nis rather the self-reference\nthat sublates their distinctness\nand so, too, themselves.\n\nFrom this side, as moments of external reflection\nand as external to themselves,\nlikeness and unlikeness disappear together into their likeness.\nBut this, their negative unity,\nis in addition also posited in them;\nfor their reflection implicitly exists\noutside them, that is, they are the likeness and\nunlikeness of a third,\nof another than they themselves are.\nThus the like is not the like of itself,\nand the unlike, as the unlike not of itself\nbut of an unlike to it,\nis itself the like.\nThe like and the unlike is\neach therefore the unlike of itself.\nEach is thereby this reflection:\nlikeness, that it is itself and the unlikeness;\nunlikeness, that it is itself and the likeness.\nLikeness and unlikeness constituted\nthe side of positedness as against\nwhat is being compared or the diverse\nwhich, as contrasted with them,\nhad determined itself as implicitly existent reflection.\nBut this positedness has consequently equally\nlost its determinateness as against this reflection.\n\nLikeness and unlikeness,\nthe determinations of external reflection,\nare precisely the merely\nimplicitly existent reflection\nwhich the diverse as such was supposed to be,\nits only indeterminate difference.\nImplicitly existent reflection is\nself-reference without negation,\nabstract self-identity\nand therefore positedness itself.\nThe merely diverse thus passes over\nthrough the positedness\ninto negative reflection.\nThe diverse is difference\nwhich is merely posited,\nhence a difference which is no difference,\nhence a negation that negates itself within.\nLikeness and unlikeness themselves, the positedness,\nthus return through indifference\nor through implicitly existing reflection\nback into negative unity with themselves,\ninto the reflection which is\nthe implicit difference of likeness and unlikeness.\nDiversity, the indifferent sides of which\nare just as much simply and solely\nmoments of a negative unity, is opposition.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-10'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-10'})
MATCH (c:IntegratedChunk {id: 'diff-10'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-10'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-10'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-10:kp:1'})
SET kp.chunkId = 'diff-10'
SET kp.ordinal = 1
SET kp.text = 'External difference sublates itself';
MATCH (c:IntegratedChunk {id: 'diff-10'})
MATCH (kp:KeyPoint {id: 'diff-10:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-10:kp:2'})
SET kp.chunkId = 'diff-10'
SET kp.ordinal = 2
SET kp.text = 'Negative unity of likeness and unlikeness';
MATCH (c:IntegratedChunk {id: 'diff-10'})
MATCH (kp:KeyPoint {id: 'diff-10:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-10:kp:3'})
SET kp.chunkId = 'diff-10'
SET kp.ordinal = 3
SET kp.text = 'Unity is nature of likeness and unlikeness themselves';
MATCH (c:IntegratedChunk {id: 'diff-10'})
MATCH (kp:KeyPoint {id: 'diff-10:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-10:kp:4'})
SET kp.chunkId = 'diff-10'
SET kp.ordinal = 4
SET kp.text = 'Each is unlike of itself, itself and its other';
MATCH (c:IntegratedChunk {id: 'diff-10'})
MATCH (kp:KeyPoint {id: 'diff-10:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-10:kp:5'})
SET kp.chunkId = 'diff-10'
SET kp.ordinal = 5
SET kp.text = 'Diversity, indifferent sides of which are moments of negative unity, is opposition';
MATCH (c:IntegratedChunk {id: 'diff-10'})
MATCH (kp:KeyPoint {id: 'diff-10:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-11'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 351
SET segment.lineEnd = 370
SET segment.text = 'In opposition, the determinate reflection,\ndifference, is brought to completion.\nOpposition is the unity of identity and diversity;\nits moments are diverse in one identity,\nand so they are opposites.\n\nIdentity and difference are the moments of\ndifference as held inside difference itself;\nthey are reflected moments of its unity.\nLikeness and unlikeness are instead\nthe externalized reflection;\ntheir self-identity is not only the indifference\nof each towards the other differentiated from it,\nbut towards being-in-and-for-itself as such;\ntheirs is a self-identity that contrasts with\nidentity reflected into itself,\nhence an immediacy which is not reflected into itself.\nThe positedness of the sides of\nexternal reflection is therefore a being,\njust as their non-positedness is a non-being.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-11'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-11'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-11'
SET topic.title = 'Opposition — introduction'
SET topic.description = 'In opposition, determinate reflection, difference, brought to completion. Opposition is unity of identity and diversity. Moments are diverse in one identity, so they are opposites. Identity and difference are moments of difference held inside difference itself. Reflected moments of its unity. Likeness and unlikeness are externalized reflection. Their self-identity contrasts with identity reflected into itself. Immediacy which is not reflected into itself.'
SET topic.keyPoints = ['In opposition, determinate reflection, difference, brought to completion', 'Opposition is unity of identity and diversity', 'Moments are diverse in one identity, so they are opposites', 'Likeness and unlikeness are externalized reflection', 'Immediacy which is not reflected into itself'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-11'})
MATCH (topic:Topic {id: 'topic:diff-11'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-11'})
SET c.title = 'Opposition — introduction'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 351
SET c.lineEnd = 370
SET c.description = 'In opposition, determinate reflection, difference, brought to completion. Opposition is unity of identity and diversity. Moments are diverse in one identity, so they are opposites. Identity and difference are moments of difference held inside difference itself. Reflected moments of its unity. Likeness and unlikeness are externalized reflection. Their self-identity contrasts with identity reflected into itself. Immediacy which is not reflected into itself.'
SET c.keyPoints = ['In opposition, determinate reflection, difference, brought to completion', 'Opposition is unity of identity and diversity', 'Moments are diverse in one identity, so they are opposites', 'Likeness and unlikeness are externalized reflection', 'Immediacy which is not reflected into itself']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 11
SET c.globalOrder = 58
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'In opposition, the determinate reflection,\ndifference, is brought to completion.\nOpposition is the unity of identity and diversity;\nits moments are diverse in one identity,\nand so they are opposites.\n\nIdentity and difference are the moments of\ndifference as held inside difference itself;\nthey are reflected moments of its unity.\nLikeness and unlikeness are instead\nthe externalized reflection;\ntheir self-identity is not only the indifference\nof each towards the other differentiated from it,\nbut towards being-in-and-for-itself as such;\ntheirs is a self-identity that contrasts with\nidentity reflected into itself,\nhence an immediacy which is not reflected into itself.\nThe positedness of the sides of\nexternal reflection is therefore a being,\njust as their non-positedness is a non-being.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-11'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-11'})
MATCH (c:IntegratedChunk {id: 'diff-11'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-11'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-11'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-11:kp:1'})
SET kp.chunkId = 'diff-11'
SET kp.ordinal = 1
SET kp.text = 'In opposition, determinate reflection, difference, brought to completion';
MATCH (c:IntegratedChunk {id: 'diff-11'})
MATCH (kp:KeyPoint {id: 'diff-11:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-11:kp:2'})
SET kp.chunkId = 'diff-11'
SET kp.ordinal = 2
SET kp.text = 'Opposition is unity of identity and diversity';
MATCH (c:IntegratedChunk {id: 'diff-11'})
MATCH (kp:KeyPoint {id: 'diff-11:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-11:kp:3'})
SET kp.chunkId = 'diff-11'
SET kp.ordinal = 3
SET kp.text = 'Moments are diverse in one identity, so they are opposites';
MATCH (c:IntegratedChunk {id: 'diff-11'})
MATCH (kp:KeyPoint {id: 'diff-11:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-11:kp:4'})
SET kp.chunkId = 'diff-11'
SET kp.ordinal = 4
SET kp.text = 'Likeness and unlikeness are externalized reflection';
MATCH (c:IntegratedChunk {id: 'diff-11'})
MATCH (kp:KeyPoint {id: 'diff-11:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-11:kp:5'})
SET kp.chunkId = 'diff-11'
SET kp.ordinal = 5
SET kp.text = 'Immediacy which is not reflected into itself';
MATCH (c:IntegratedChunk {id: 'diff-11'})
MATCH (kp:KeyPoint {id: 'diff-11:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-12'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 372
SET segment.lineEnd = 392
SET segment.text = 'On closer consideration, the moments of opposition are\npositedness reflected into itself\nor determination in general.\nPositedness is likeness and unlikeness;\nthese two, reflected into themselves,\nconstitute the determinations of opposition.\nTheir immanent reflection consists in that\neach is within it the unity of likeness and unlikeness.\nLikeness is only in a reflection\nwhich compares according to the unlikeness\nand is therefore mediated by its\nother indifferent moment; similarly,\nunlikeness is only in the same\nreflective reference in which likeness is.\nEach of these moments, in its determinateness,\nis therefore the whole.\nIt is the whole because it also contains its other moment;\nbut this, its other, is an indifferent existent;\nthus each contains a reference to its non-being,\nand it is reflection-into-itself, or the whole,\nonly as essentially referring to its non-being.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-12'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-12'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-12'
SET topic.title = 'Moments of opposition — positedness reflected into itself'
SET topic.description = 'Moments of opposition are positedness reflected into itself or determination in general. Positedness is likeness and unlikeness. These two, reflected into themselves, constitute determinations of opposition. Their immanent reflection: each is within it unity of likeness and unlikeness. Likeness only in reflection which compares according to unlikeness. Each, in its determinateness, is whole. Whole because contains its other moment. But other is indifferent existent. Each contains reference to its non-being. Reflection-into-itself, or whole, only as essentially referring to its non-being.'
SET topic.keyPoints = ['Moments of opposition are positedness reflected into itself', 'Positedness is likeness and unlikeness, reflected into themselves', 'Each is within it unity of likeness and unlikeness', 'Each, in its determinateness, is whole', 'Reflection-into-itself only as essentially referring to its non-being'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-12'})
MATCH (topic:Topic {id: 'topic:diff-12'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-12'})
SET c.title = 'Moments of opposition — positedness reflected into itself'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 372
SET c.lineEnd = 392
SET c.description = 'Moments of opposition are positedness reflected into itself or determination in general. Positedness is likeness and unlikeness. These two, reflected into themselves, constitute determinations of opposition. Their immanent reflection: each is within it unity of likeness and unlikeness. Likeness only in reflection which compares according to unlikeness. Each, in its determinateness, is whole. Whole because contains its other moment. But other is indifferent existent. Each contains reference to its non-being. Reflection-into-itself, or whole, only as essentially referring to its non-being.'
SET c.keyPoints = ['Moments of opposition are positedness reflected into itself', 'Positedness is likeness and unlikeness, reflected into themselves', 'Each is within it unity of likeness and unlikeness', 'Each, in its determinateness, is whole', 'Reflection-into-itself only as essentially referring to its non-being']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 12
SET c.globalOrder = 59
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'On closer consideration, the moments of opposition are\npositedness reflected into itself\nor determination in general.\nPositedness is likeness and unlikeness;\nthese two, reflected into themselves,\nconstitute the determinations of opposition.\nTheir immanent reflection consists in that\neach is within it the unity of likeness and unlikeness.\nLikeness is only in a reflection\nwhich compares according to the unlikeness\nand is therefore mediated by its\nother indifferent moment; similarly,\nunlikeness is only in the same\nreflective reference in which likeness is.\nEach of these moments, in its determinateness,\nis therefore the whole.\nIt is the whole because it also contains its other moment;\nbut this, its other, is an indifferent existent;\nthus each contains a reference to its non-being,\nand it is reflection-into-itself, or the whole,\nonly as essentially referring to its non-being.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-12'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-12'})
MATCH (c:IntegratedChunk {id: 'diff-12'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-12'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-12'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-12:kp:1'})
SET kp.chunkId = 'diff-12'
SET kp.ordinal = 1
SET kp.text = 'Moments of opposition are positedness reflected into itself';
MATCH (c:IntegratedChunk {id: 'diff-12'})
MATCH (kp:KeyPoint {id: 'diff-12:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-12:kp:2'})
SET kp.chunkId = 'diff-12'
SET kp.ordinal = 2
SET kp.text = 'Positedness is likeness and unlikeness, reflected into themselves';
MATCH (c:IntegratedChunk {id: 'diff-12'})
MATCH (kp:KeyPoint {id: 'diff-12:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-12:kp:3'})
SET kp.chunkId = 'diff-12'
SET kp.ordinal = 3
SET kp.text = 'Each is within it unity of likeness and unlikeness';
MATCH (c:IntegratedChunk {id: 'diff-12'})
MATCH (kp:KeyPoint {id: 'diff-12:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-12:kp:4'})
SET kp.chunkId = 'diff-12'
SET kp.ordinal = 4
SET kp.text = 'Each, in its determinateness, is whole';
MATCH (c:IntegratedChunk {id: 'diff-12'})
MATCH (kp:KeyPoint {id: 'diff-12:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-12:kp:5'})
SET kp.chunkId = 'diff-12'
SET kp.ordinal = 5
SET kp.text = 'Reflection-into-itself only as essentially referring to its non-being';
MATCH (c:IntegratedChunk {id: 'diff-12'})
MATCH (kp:KeyPoint {id: 'diff-12:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-13'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 394
SET segment.lineEnd = 420
SET segment.text = 'This self-likeness, reflected into itself\nand containing the reference to\nunlikeness within it, is the positive;\nand the unlikeness that contains within itself\nthe reference to its non-being,\nto likeness, is the negative.\nOr again, both are positedness;\nnow in so far as the differentiated determinateness is\ntaken as a differentiated determinate reference of\npositedness to itself, opposition is, on the one hand,\npositedness reflected into its likeness with itself;\nand, on the other hand, it is the same positedness\nreflected into its inequality with itself:\nthe positive and the negative.\nThe positive is positedness as reflected into self-likeness;\nbut what is reflected is positedness, that is,\nthe negation as negation,\nand so this immanent reflection has\nthe reference to the other for its determination.\nThe negative is positedness as reflected into unlikeness;\nbut positedness is the unlikeness itself,\nand so this reflection is therefore\nthe identity of unlikeness with itself\nand absolute self-reference.\nEach, therefore, equally has the other in it:\npositedness reflected into self-likeness has the unlikeness;\nand positedness reflected into self-unlikeness, the likeness.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-13'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-13'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-13'
SET topic.title = 'Positive and negative — determinations of opposition'
SET topic.description = 'Self-likeness, reflected into itself, containing reference to unlikeness within it, is positive. Unlikeness containing reference to its non-being, to likeness, is negative. Both are positedness. Opposition: positedness reflected into likeness with itself, and into inequality with itself. Positive and negative. Positive is positedness as reflected into self-likeness. Negative is positedness as reflected into unlikeness. Each equally has other in it: positive has unlikeness, negative has likeness.'
SET topic.keyPoints = ['Self-likeness containing reference to unlikeness is positive', 'Unlikeness containing reference to likeness is negative', 'Both are positedness', 'Positive is positedness reflected into self-likeness', 'Negative is positedness reflected into unlikeness', 'Each equally has other in it'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-13'})
MATCH (topic:Topic {id: 'topic:diff-13'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-13'})
SET c.title = 'Positive and negative — determinations of opposition'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 394
SET c.lineEnd = 420
SET c.description = 'Self-likeness, reflected into itself, containing reference to unlikeness within it, is positive. Unlikeness containing reference to its non-being, to likeness, is negative. Both are positedness. Opposition: positedness reflected into likeness with itself, and into inequality with itself. Positive and negative. Positive is positedness as reflected into self-likeness. Negative is positedness as reflected into unlikeness. Each equally has other in it: positive has unlikeness, negative has likeness.'
SET c.keyPoints = ['Self-likeness containing reference to unlikeness is positive', 'Unlikeness containing reference to likeness is negative', 'Both are positedness', 'Positive is positedness reflected into self-likeness', 'Negative is positedness reflected into unlikeness', 'Each equally has other in it']
SET c.tags = ['negation', 'reflection', 'citta']
SET c.orderInSource = 13
SET c.globalOrder = 60
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'This self-likeness, reflected into itself\nand containing the reference to\nunlikeness within it, is the positive;\nand the unlikeness that contains within itself\nthe reference to its non-being,\nto likeness, is the negative.\nOr again, both are positedness;\nnow in so far as the differentiated determinateness is\ntaken as a differentiated determinate reference of\npositedness to itself, opposition is, on the one hand,\npositedness reflected into its likeness with itself;\nand, on the other hand, it is the same positedness\nreflected into its inequality with itself:\nthe positive and the negative.\nThe positive is positedness as reflected into self-likeness;\nbut what is reflected is positedness, that is,\nthe negation as negation,\nand so this immanent reflection has\nthe reference to the other for its determination.\nThe negative is positedness as reflected into unlikeness;\nbut positedness is the unlikeness itself,\nand so this reflection is therefore\nthe identity of unlikeness with itself\nand absolute self-reference.\nEach, therefore, equally has the other in it:\npositedness reflected into self-likeness has the unlikeness;\nand positedness reflected into self-unlikeness, the likeness.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-13'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-13'})
MATCH (c:IntegratedChunk {id: 'diff-13'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-13'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-13'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-13:kp:1'})
SET kp.chunkId = 'diff-13'
SET kp.ordinal = 1
SET kp.text = 'Self-likeness containing reference to unlikeness is positive';
MATCH (c:IntegratedChunk {id: 'diff-13'})
MATCH (kp:KeyPoint {id: 'diff-13:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-13:kp:2'})
SET kp.chunkId = 'diff-13'
SET kp.ordinal = 2
SET kp.text = 'Unlikeness containing reference to likeness is negative';
MATCH (c:IntegratedChunk {id: 'diff-13'})
MATCH (kp:KeyPoint {id: 'diff-13:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-13:kp:3'})
SET kp.chunkId = 'diff-13'
SET kp.ordinal = 3
SET kp.text = 'Both are positedness';
MATCH (c:IntegratedChunk {id: 'diff-13'})
MATCH (kp:KeyPoint {id: 'diff-13:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-13:kp:4'})
SET kp.chunkId = 'diff-13'
SET kp.ordinal = 4
SET kp.text = 'Positive is positedness reflected into self-likeness';
MATCH (c:IntegratedChunk {id: 'diff-13'})
MATCH (kp:KeyPoint {id: 'diff-13:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-13:kp:5'})
SET kp.chunkId = 'diff-13'
SET kp.ordinal = 5
SET kp.text = 'Negative is positedness reflected into unlikeness';
MATCH (c:IntegratedChunk {id: 'diff-13'})
MATCH (kp:KeyPoint {id: 'diff-13:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-13:kp:6'})
SET kp.chunkId = 'diff-13'
SET kp.ordinal = 6
SET kp.text = 'Each equally has other in it';
MATCH (c:IntegratedChunk {id: 'diff-13'})
MATCH (kp:KeyPoint {id: 'diff-13:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-14'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 422
SET segment.lineEnd = 448
SET segment.text = 'The positive and the negative are thus\nthe sides of opposition that have become self-subsisting.\nThey are self-subsisting because they are\nthe reflection of the whole into itself,\nand they belong to opposition in so far\nas the latter is determinateness\nwhich, as the whole, is reflected into itself.\nBecause of their self-subsistence,\nthe opposition which they constitute is\nimplicitly determinate.\nEach is itself and its other;\nfor this reason, each has its determinateness\nnot in an other but within.\nEach refers itself to itself\nonly as referring itself to its other.\nThis has a twofold aspect.\nEach is the reference to its non-being as\nthe sublating of this otherness in itself;\nits non-being is thus only a moment in it.\nBut, on the other hand, here positedness\nhas become a being, an indifferent subsistence;\nthe other of itself which each contains is\ntherefore also the non-being of that in which\nit should be contained only as a moment.\nEach is, therefore, only to the\nextent that its non-being is,\nthe two in an identical reference.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-14'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-14'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-14'
SET topic.title = 'Positive and negative as self-subsisting'
SET topic.description = 'Positive and negative are sides of opposition that have become self-subsisting. Self-subsisting because reflection of whole into itself. Belong to opposition as determinateness reflected into itself. Because of self-subsistence, opposition implicitly determinate. Each is itself and its other. Each has determinateness not in other but within. Each refers itself to itself only as referring itself to its other. Twofold aspect: reference to non-being as sublating otherness in itself. But positedness has become being, indifferent subsistence. Each only to extent that its non-being is, two in identical reference.'
SET topic.keyPoints = ['Positive and negative are sides of opposition that have become self-subsisting', 'Self-subsisting because reflection of whole into itself', 'Each is itself and its other', 'Each has determinateness not in other but within', 'Each only to extent that its non-being is, two in identical reference'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-14'})
MATCH (topic:Topic {id: 'topic:diff-14'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-14'})
SET c.title = 'Positive and negative as self-subsisting'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 422
SET c.lineEnd = 448
SET c.description = 'Positive and negative are sides of opposition that have become self-subsisting. Self-subsisting because reflection of whole into itself. Belong to opposition as determinateness reflected into itself. Because of self-subsistence, opposition implicitly determinate. Each is itself and its other. Each has determinateness not in other but within. Each refers itself to itself only as referring itself to its other. Twofold aspect: reference to non-being as sublating otherness in itself. But positedness has become being, indifferent subsistence. Each only to extent that its non-being is, two in identical reference.'
SET c.keyPoints = ['Positive and negative are sides of opposition that have become self-subsisting', 'Self-subsisting because reflection of whole into itself', 'Each is itself and its other', 'Each has determinateness not in other but within', 'Each only to extent that its non-being is, two in identical reference']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 14
SET c.globalOrder = 61
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The positive and the negative are thus\nthe sides of opposition that have become self-subsisting.\nThey are self-subsisting because they are\nthe reflection of the whole into itself,\nand they belong to opposition in so far\nas the latter is determinateness\nwhich, as the whole, is reflected into itself.\nBecause of their self-subsistence,\nthe opposition which they constitute is\nimplicitly determinate.\nEach is itself and its other;\nfor this reason, each has its determinateness\nnot in an other but within.\nEach refers itself to itself\nonly as referring itself to its other.\nThis has a twofold aspect.\nEach is the reference to its non-being as\nthe sublating of this otherness in itself;\nits non-being is thus only a moment in it.\nBut, on the other hand, here positedness\nhas become a being, an indifferent subsistence;\nthe other of itself which each contains is\ntherefore also the non-being of that in which\nit should be contained only as a moment.\nEach is, therefore, only to the\nextent that its non-being is,\nthe two in an identical reference.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-14'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-14'})
MATCH (c:IntegratedChunk {id: 'diff-14'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-14'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-14'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-14:kp:1'})
SET kp.chunkId = 'diff-14'
SET kp.ordinal = 1
SET kp.text = 'Positive and negative are sides of opposition that have become self-subsisting';
MATCH (c:IntegratedChunk {id: 'diff-14'})
MATCH (kp:KeyPoint {id: 'diff-14:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-14:kp:2'})
SET kp.chunkId = 'diff-14'
SET kp.ordinal = 2
SET kp.text = 'Self-subsisting because reflection of whole into itself';
MATCH (c:IntegratedChunk {id: 'diff-14'})
MATCH (kp:KeyPoint {id: 'diff-14:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-14:kp:3'})
SET kp.chunkId = 'diff-14'
SET kp.ordinal = 3
SET kp.text = 'Each is itself and its other';
MATCH (c:IntegratedChunk {id: 'diff-14'})
MATCH (kp:KeyPoint {id: 'diff-14:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-14:kp:4'})
SET kp.chunkId = 'diff-14'
SET kp.ordinal = 4
SET kp.text = 'Each has determinateness not in other but within';
MATCH (c:IntegratedChunk {id: 'diff-14'})
MATCH (kp:KeyPoint {id: 'diff-14:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-14:kp:5'})
SET kp.chunkId = 'diff-14'
SET kp.ordinal = 5
SET kp.text = 'Each only to extent that its non-being is, two in identical reference';
MATCH (c:IntegratedChunk {id: 'diff-14'})
MATCH (kp:KeyPoint {id: 'diff-14:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:diff-15'})
SET segment.sourceId = 'source-difference'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET segment.lineStart = 450
SET segment.lineEnd = 594
SET segment.text = 'The determinations which constitute\nthe positive and the negative consist,\ntherefore, in that the positive and the negative are,\nfirst, absolute moments of opposition;\ntheir subsistence is indivisibly one reflection;\nit is one mediation in which each is\nby virtue of the non-being of its other,\nhence by virtue of its other\nor its own non-being.\nThus they are simply opposites;\nor each is only the opposite of the other;\nthe one is not yet the positive\nand the other not yet the negative,\nbut both are negative with\nrespect to each other.\nEach, therefore, simply is,\nfirst, to the extent that the other is;\nit is what it is by virtue of the other,\nby virtue of its own non-being;\nit is only positedness.\nSecond, it is to the extent that\nthe other is not; it is what it is\nby virtue of the non-being of the other;\nit is reflection into itself.\nThe two, however, are both\nthe one mediation of opposition as such\nin which they simply are only posited moments.\n\nMoreover, this mere positedness is\nreflected into itself in general\nand, according to this moment of external reflection,\nthe positive and the negative are indifferent towards\nthis first identity where they are only moments;\nor again, because that first reflection is\nthe positive\'s and the negative\'s own\nreflection into itself,\neach is indifferent towards its reflection\ninto its non-being, towards its own positedness.\nThe two sides are thus merely diverse,\nand because their determinateness\nthat they are positive or negative\nconstitutes their positedness as against each other,\neach is not specifically so determined internally\nbut is only determinateness in general;\nto each side, therefore, there belongs indeed\none of the two determinacies,\nthe positive or the negative;\nbut the two can be interchanged,\nand each side is such as\ncan be taken equally as positive or negative.\n\nBut, in third place, the positive and the negative are\nnot only a posited being,\nnor are they something merely indifferent,\nbut their positedness,\nor the reference to the other in the one unity\nwhich they themselves are not,\nis rather taken back into each.\nEach is itself positive and negative within;\nthe positive and the negative are\nthe determination of reflection in and for itself;\nonly in this reflection of the opposite into itself is\nthe opposite either positive or negative.\nThe positive has within it the reference to\nthe other in which the determinateness of the positive consists.\nAnd the same applies to the negative:\nit is not negative as contrasted with another\nbut has the determinateness by which it is negative within.\n\nEach is thus self-subsistent unity existing for itself.\nThe positive is indeed a positedness,\nbut in such a way that the positedness is\nfor it posited being as sublated.\nIt is the non-opposed, the sublated opposition,\nbut as the side of the opposition itself.\nAs positive, it is indeed a something\nwhich is determined with reference to an otherness,\nbut in such a way that its nature\nis not to be something posited;\nit is the immanent reflection\nthat negates otherness.\nBut its other, the negative,\nis itself no longer positedness or a moment\nbut itself a self-subsisting being\nand so the negating reflection\nof the positive is internally\ndetermined to exclude this being,\nwhich is its non-being, from itself.\n\nThus the negative, as absolute reflection,\nis not the immediate negative\nbut is the negative as sublated positedness,\nthe negative in and for itself\nwhich positively rests upon itself.\nAs immanent reflection,\nit negates its reference to its other;\nits other is the positive,\na self-subsisting being\nhence its negative reference\nto this positive is\nthe excluding of it from itself.\nThe negative is the independently existing opposite,\nover against the positive\nwhich is the determination of the sublated opposition,\nthe whole opposition resting upon itself,\nopposed to the self-identical positedness.\n\nThe positive and the negative are such, therefore,\nnot just in themselves, but in and for themselves.\nThey are in themselves positive and negative\nwhen they are abstracted from their excluding\nreference to the other\nand are taken only in accordance\nwith their determination.\nSomething is in itself positive or negative\nwhen it is not supposed to be\ndetermined as positive or negative\nmerely in contrast with the other.\nBut the positive and the negative,\ntaken not as a positedness\nand hence not as opposed,\nare each an immediate,\nbeing and non-being.\nThey are, however, moments of opposition:\ntheir in-itself constitutes only\nthe form of their immanent reflectedness.\nSomething is said to be positive in itself,\noutside the reference to something negative,\nand something negative in itself,\noutside the reference to something negative:\nin this determination, merely the abstract moment of\nthis reflectedness is held on to.\nHowever, to say that the positive and the negative\nexist in themselves essentially implies that\nto be opposed is not a mere moment,\nnor that it is just a matter of comparison,\nbut that it is the determination of the sides\nthemselves of the opposition.\nThe sides, as positive or negative in themselves,\nare not, therefore, outside the reference to the other;\non the contrary, this reference, precisely as exclusive,\nconstitutes their determination or their in-itselfness;\nin this, therefore, they are at the same time\nin and for themselves.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (segment:ChunkSegment {id: 'chunk:diff-15'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:diff-15'})
SET topic.sourceId = 'source-difference'
SET topic.topicRef = 'diff-15'
SET topic.title = 'Three aspects: moments, diverse, in and for themselves'
SET topic.description = 'First: positive and negative are absolute moments of opposition. Subsistence indivisibly one reflection, one mediation. Each by virtue of non-being of its other. Simply opposites, each only opposite of other. Second: mere positedness reflected into itself. Two sides merely diverse, can be interchanged, each can be taken equally as positive or negative. Third: positive and negative not only posited being, nor merely indifferent. Their positedness, reference to other in unity, taken back into each. Each itself positive and negative within. Each is self-subsistent unity existing for itself. Positive: positedness for it posited being as sublated, immanent reflection that negates otherness. Negative: absolute reflection, negative as sublated positedness, negatively rests upon itself, excludes positive from itself. Positive and negative in themselves and in and for themselves. Reference, precisely as exclusive, constitutes their determination or in-itselfness.'
SET topic.keyPoints = ['First: absolute moments of opposition, one mediation', 'Second: mere positedness, two sides diverse, can be interchanged', 'Third: each itself positive and negative within', 'Each is self-subsistent unity existing for itself', 'Positive: immanent reflection that negates otherness', 'Negative: absolute reflection, excludes positive from itself', 'Reference, precisely as exclusive, constitutes their determination or in-itselfness'];
MATCH (segment:ChunkSegment {id: 'chunk:diff-15'})
MATCH (topic:Topic {id: 'topic:diff-15'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'diff-15'})
SET c.title = 'Three aspects: moments, diverse, in and for themselves'
SET c.sourceId = 'source-difference'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/difference.txt'
SET c.lineStart = 450
SET c.lineEnd = 594
SET c.description = 'First: positive and negative are absolute moments of opposition. Subsistence indivisibly one reflection, one mediation. Each by virtue of non-being of its other. Simply opposites, each only opposite of other. Second: mere positedness reflected into itself. Two sides merely diverse, can be interchanged, each can be taken equally as positive or negative. Third: positive and negative not only posited being, nor merely indifferent. Their positedness, reference to other in unity, taken back into each. Each itself positive and negative within. Each is self-subsistent unity existing for itself. Positive: positedness for it posited being as sublated, immanent reflection that negates otherness. Negative: absolute reflection, negative as sublated positedness, negatively rests upon itself, excludes positive from itself. Positive and negative in themselves and in and for themselves. Reference, precisely as exclusive, constitutes their determination or in-itselfness.'
SET c.keyPoints = ['First: absolute moments of opposition, one mediation', 'Second: mere positedness, two sides diverse, can be interchanged', 'Third: each itself positive and negative within', 'Each is self-subsistent unity existing for itself', 'Positive: immanent reflection that negates otherness', 'Negative: absolute reflection, excludes positive from itself', 'Reference, precisely as exclusive, constitutes their determination or in-itselfness']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 15
SET c.globalOrder = 62
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The determinations which constitute\nthe positive and the negative consist,\ntherefore, in that the positive and the negative are,\nfirst, absolute moments of opposition;\ntheir subsistence is indivisibly one reflection;\nit is one mediation in which each is\nby virtue of the non-being of its other,\nhence by virtue of its other\nor its own non-being.\nThus they are simply opposites;\nor each is only the opposite of the other;\nthe one is not yet the positive\nand the other not yet the negative,\nbut both are negative with\nrespect to each other.\nEach, therefore, simply is,\nfirst, to the extent that the other is;\nit is what it is by virtue of the other,\nby virtue of its own non-being;\nit is only positedness.\nSecond, it is to the extent that\nthe other is not; it is what it is\nby virtue of the non-being of the other;\nit is reflection into itself.\nThe two, however, are both\nthe one mediation of opposition as such\nin which they simply are only posited moments.\n\nMoreover, this mere positedness is\nreflected into itself in general\nand, according to this moment of external reflection,\nthe positive and the negative are indifferent towards\nthis first identity where they are only moments;\nor again, because that first reflection is\nthe positive\'s and the negative\'s own\nreflection into itself,\neach is indifferent towards its reflection\ninto its non-being, towards its own positedness.\nThe two sides are thus merely diverse,\nand because their determinateness\nthat they are positive or negative\nconstitutes their positedness as against each other,\neach is not specifically so determined internally\nbut is only determinateness in general;\nto each side, therefore, there belongs indeed\none of the two determinacies,\nthe positive or the negative;\nbut the two can be interchanged,\nand each side is such as\ncan be taken equally as positive or negative.\n\nBut, in third place, the positive and the negative are\nnot only a posited being,\nnor are they something merely indifferent,\nbut their positedness,\nor the reference to the other in the one unity\nwhich they themselves are not,\nis rather taken back into each.\nEach is itself positive and negative within;\nthe positive and the negative are\nthe determination of reflection in and for itself;\nonly in this reflection of the opposite into itself is\nthe opposite either positive or negative.\nThe positive has within it the reference to\nthe other in which the determinateness of the positive consists.\nAnd the same applies to the negative:\nit is not negative as contrasted with another\nbut has the determinateness by which it is negative within.\n\nEach is thus self-subsistent unity existing for itself.\nThe positive is indeed a positedness,\nbut in such a way that the positedness is\nfor it posited being as sublated.\nIt is the non-opposed, the sublated opposition,\nbut as the side of the opposition itself.\nAs positive, it is indeed a something\nwhich is determined with reference to an otherness,\nbut in such a way that its nature\nis not to be something posited;\nit is the immanent reflection\nthat negates otherness.\nBut its other, the negative,\nis itself no longer positedness or a moment\nbut itself a self-subsisting being\nand so the negating reflection\nof the positive is internally\ndetermined to exclude this being,\nwhich is its non-being, from itself.\n\nThus the negative, as absolute reflection,\nis not the immediate negative\nbut is the negative as sublated positedness,\nthe negative in and for itself\nwhich positively rests upon itself.\nAs immanent reflection,\nit negates its reference to its other;\nits other is the positive,\na self-subsisting being\nhence its negative reference\nto this positive is\nthe excluding of it from itself.\nThe negative is the independently existing opposite,\nover against the positive\nwhich is the determination of the sublated opposition,\nthe whole opposition resting upon itself,\nopposed to the self-identical positedness.\n\nThe positive and the negative are such, therefore,\nnot just in themselves, but in and for themselves.\nThey are in themselves positive and negative\nwhen they are abstracted from their excluding\nreference to the other\nand are taken only in accordance\nwith their determination.\nSomething is in itself positive or negative\nwhen it is not supposed to be\ndetermined as positive or negative\nmerely in contrast with the other.\nBut the positive and the negative,\ntaken not as a positedness\nand hence not as opposed,\nare each an immediate,\nbeing and non-being.\nThey are, however, moments of opposition:\ntheir in-itself constitutes only\nthe form of their immanent reflectedness.\nSomething is said to be positive in itself,\noutside the reference to something negative,\nand something negative in itself,\noutside the reference to something negative:\nin this determination, merely the abstract moment of\nthis reflectedness is held on to.\nHowever, to say that the positive and the negative\nexist in themselves essentially implies that\nto be opposed is not a mere moment,\nnor that it is just a matter of comparison,\nbut that it is the determination of the sides\nthemselves of the opposition.\nThe sides, as positive or negative in themselves,\nare not, therefore, outside the reference to the other;\non the contrary, this reference, precisely as exclusive,\nconstitutes their determination or their in-itselfness;\nin this, therefore, they are at the same time\nin and for themselves.';
MATCH (s:SourceText {id: 'source-difference'})
MATCH (c:IntegratedChunk {id: 'diff-15'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:diff-15'})
MATCH (c:IntegratedChunk {id: 'diff-15'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'diff-15:kp:1'})
SET kp.chunkId = 'diff-15'
SET kp.ordinal = 1
SET kp.text = 'First: absolute moments of opposition, one mediation';
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (kp:KeyPoint {id: 'diff-15:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-15:kp:2'})
SET kp.chunkId = 'diff-15'
SET kp.ordinal = 2
SET kp.text = 'Second: mere positedness, two sides diverse, can be interchanged';
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (kp:KeyPoint {id: 'diff-15:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-15:kp:3'})
SET kp.chunkId = 'diff-15'
SET kp.ordinal = 3
SET kp.text = 'Third: each itself positive and negative within';
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (kp:KeyPoint {id: 'diff-15:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-15:kp:4'})
SET kp.chunkId = 'diff-15'
SET kp.ordinal = 4
SET kp.text = 'Each is self-subsistent unity existing for itself';
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (kp:KeyPoint {id: 'diff-15:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-15:kp:5'})
SET kp.chunkId = 'diff-15'
SET kp.ordinal = 5
SET kp.text = 'Positive: immanent reflection that negates otherness';
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (kp:KeyPoint {id: 'diff-15:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-15:kp:6'})
SET kp.chunkId = 'diff-15'
SET kp.ordinal = 6
SET kp.text = 'Negative: absolute reflection, excludes positive from itself';
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (kp:KeyPoint {id: 'diff-15:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'diff-15:kp:7'})
SET kp.chunkId = 'diff-15'
SET kp.ordinal = 7
SET kp.text = 'Reference, precisely as exclusive, constitutes their determination or in-itselfness';
MATCH (c:IntegratedChunk {id: 'diff-15'})
MATCH (kp:KeyPoint {id: 'diff-15:kp:7'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
