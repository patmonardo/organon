MERGE (s:SourceText {id: 'source-contradiction'})
SET s.title = 'Contradiction'
SET s.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET s.totalLines = 294;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-contradiction'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-1'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 4
SET segment.lineEnd = 39
SET segment.text = '1. Difference in general contains both its sides as moments;\nin diversity, these sides fall apart as indifferent to each other;\nand in opposition as such, they are the moments of difference,\neach determined by the other and hence only moments.\nBut in opposition these moments are equally determined within,\nindifferent to each other and mutually exclusive,\nself-subsisting determinations of reflection.\n\nOne is the positive and the other the negative,\nbut the former as a positive which is such within,\nand the latter as a negative which is such within.\nEach has indifferent self-subsistence for itself\nby virtue of having the reference to\nits other moment within it;\neach moment is thus the whole self-contained opposition.\nAs this whole, each moment is self-mediated\nthrough its other and contains this other.\nBut it is also self-mediated\nthrough the non-being of its other\nand is, therefore, a unity existing for itself\nand excluding the other from itself.\n\nSince the self-subsisting determination of reflection\nexcludes the other in the same respect as it contains it\nand is self-subsisting for precisely this reason,\nin its self-subsistence the determination excludes\nits own self-subsistence from itself.\nFor this self-subsistence consists in\nthat it contains the determination\nwhich is other than it in itself\nand does not refer to anything external\nfor just this reason;\nbut no less immediately in that\nit is itself and excludes from itself\nthe determination that negates it.\nAnd so it is contradiction.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-1'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-1'
SET topic.title = 'Opposition as self-subsisting — contradiction'
SET topic.description = 'Difference contains both sides as moments. In diversity, sides fall apart as indifferent. In opposition, moments determined by other, only moments. But in opposition moments equally determined within, mutually exclusive, self-subsisting. Positive and negative, each such within. Each has indifferent self-subsistence by virtue of reference to other moment within it. Each moment is whole self-contained opposition. Self-mediated through other and through non-being of other. Unity existing for itself and excluding other from itself. Self-subsisting determination excludes other in same respect as it contains it. In self-subsistence, determination excludes its own self-subsistence from itself. And so it is contradiction.'
SET topic.keyPoints = ['In opposition, moments equally determined within, mutually exclusive, self-subsisting', 'Each moment is whole self-contained opposition', 'Self-mediated through other and through non-being of other', 'Self-subsisting determination excludes other in same respect as it contains it', 'In self-subsistence, determination excludes its own self-subsistence from itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-1'})
MATCH (topic:Topic {id: 'topic:ctr-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-1'})
SET c.title = 'Opposition as self-subsisting — contradiction'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 4
SET c.lineEnd = 39
SET c.description = 'Difference contains both sides as moments. In diversity, sides fall apart as indifferent. In opposition, moments determined by other, only moments. But in opposition moments equally determined within, mutually exclusive, self-subsisting. Positive and negative, each such within. Each has indifferent self-subsistence by virtue of reference to other moment within it. Each moment is whole self-contained opposition. Self-mediated through other and through non-being of other. Unity existing for itself and excluding other from itself. Self-subsisting determination excludes other in same respect as it contains it. In self-subsistence, determination excludes its own self-subsistence from itself. And so it is contradiction.'
SET c.keyPoints = ['In opposition, moments equally determined within, mutually exclusive, self-subsisting', 'Each moment is whole self-contained opposition', 'Self-mediated through other and through non-being of other', 'Self-subsisting determination excludes other in same respect as it contains it', 'In self-subsistence, determination excludes its own self-subsistence from itself']
SET c.tags = ['negation', 'mediation', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 63
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '1. Difference in general contains both its sides as moments;\nin diversity, these sides fall apart as indifferent to each other;\nand in opposition as such, they are the moments of difference,\neach determined by the other and hence only moments.\nBut in opposition these moments are equally determined within,\nindifferent to each other and mutually exclusive,\nself-subsisting determinations of reflection.\n\nOne is the positive and the other the negative,\nbut the former as a positive which is such within,\nand the latter as a negative which is such within.\nEach has indifferent self-subsistence for itself\nby virtue of having the reference to\nits other moment within it;\neach moment is thus the whole self-contained opposition.\nAs this whole, each moment is self-mediated\nthrough its other and contains this other.\nBut it is also self-mediated\nthrough the non-being of its other\nand is, therefore, a unity existing for itself\nand excluding the other from itself.\n\nSince the self-subsisting determination of reflection\nexcludes the other in the same respect as it contains it\nand is self-subsisting for precisely this reason,\nin its self-subsistence the determination excludes\nits own self-subsistence from itself.\nFor this self-subsistence consists in\nthat it contains the determination\nwhich is other than it in itself\nand does not refer to anything external\nfor just this reason;\nbut no less immediately in that\nit is itself and excludes from itself\nthe determination that negates it.\nAnd so it is contradiction.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-1'})
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-1:kp:1'})
SET kp.chunkId = 'ctr-1'
SET kp.ordinal = 1
SET kp.text = 'In opposition, moments equally determined within, mutually exclusive, self-subsisting';
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MATCH (kp:KeyPoint {id: 'ctr-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-1:kp:2'})
SET kp.chunkId = 'ctr-1'
SET kp.ordinal = 2
SET kp.text = 'Each moment is whole self-contained opposition';
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MATCH (kp:KeyPoint {id: 'ctr-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-1:kp:3'})
SET kp.chunkId = 'ctr-1'
SET kp.ordinal = 3
SET kp.text = 'Self-mediated through other and through non-being of other';
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MATCH (kp:KeyPoint {id: 'ctr-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-1:kp:4'})
SET kp.chunkId = 'ctr-1'
SET kp.ordinal = 4
SET kp.text = 'Self-subsisting determination excludes other in same respect as it contains it';
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MATCH (kp:KeyPoint {id: 'ctr-1:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-1:kp:5'})
SET kp.chunkId = 'ctr-1'
SET kp.ordinal = 5
SET kp.text = 'In self-subsistence, determination excludes its own self-subsistence from itself';
MATCH (c:IntegratedChunk {id: 'ctr-1'})
MATCH (kp:KeyPoint {id: 'ctr-1:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-2'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 41
SET segment.lineEnd = 57
SET segment.text = 'Difference as such is already implicitly contradiction;\nfor it is the unity of beings which are,\nonly in so far as they are not one\nand it is the separation of beings which are,\nonly in so far as they are separated\nin the same reference connecting them.\nThe positive and the negative, however,\nare the posited contradiction,\nfor, as negative unities,\nthey are precisely their self-positing\nand therein each the sublating of itself\nand the positing of its opposite.\nThey constitute determining reflection as exclusive;\nfor the excluding is one act of distinguishing\nand each of the distinguished beings,\nas exclusive, is itself the whole act of excluding,\nand so each excludes itself internally.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-2'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-2'
SET topic.title = 'Difference implicitly contradiction — posited contradiction'
SET topic.description = 'Difference as such already implicitly contradiction. Unity of beings which are, only in so far as they are not one. Separation of beings which are, only in so far as they are separated in same reference connecting them. Positive and negative are posited contradiction. As negative unities, precisely their self-positing. Each sublating of itself and positing of its opposite. Constitute determining reflection as exclusive. Each excludes itself internally.'
SET topic.keyPoints = ['Difference as such already implicitly contradiction', 'Unity of beings which are, only in so far as they are not one', 'Positive and negative are posited contradiction', 'As negative unities, precisely their self-positing', 'Each excludes itself internally'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-2'})
MATCH (topic:Topic {id: 'topic:ctr-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-2'})
SET c.title = 'Difference implicitly contradiction — posited contradiction'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 41
SET c.lineEnd = 57
SET c.description = 'Difference as such already implicitly contradiction. Unity of beings which are, only in so far as they are not one. Separation of beings which are, only in so far as they are separated in same reference connecting them. Positive and negative are posited contradiction. As negative unities, precisely their self-positing. Each sublating of itself and positing of its opposite. Constitute determining reflection as exclusive. Each excludes itself internally.'
SET c.keyPoints = ['Difference as such already implicitly contradiction', 'Unity of beings which are, only in so far as they are not one', 'Positive and negative are posited contradiction', 'As negative unities, precisely their self-positing', 'Each excludes itself internally']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 64
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Difference as such is already implicitly contradiction;\nfor it is the unity of beings which are,\nonly in so far as they are not one\nand it is the separation of beings which are,\nonly in so far as they are separated\nin the same reference connecting them.\nThe positive and the negative, however,\nare the posited contradiction,\nfor, as negative unities,\nthey are precisely their self-positing\nand therein each the sublating of itself\nand the positing of its opposite.\nThey constitute determining reflection as exclusive;\nfor the excluding is one act of distinguishing\nand each of the distinguished beings,\nas exclusive, is itself the whole act of excluding,\nand so each excludes itself internally.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-2'})
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-2:kp:1'})
SET kp.chunkId = 'ctr-2'
SET kp.ordinal = 1
SET kp.text = 'Difference as such already implicitly contradiction';
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MATCH (kp:KeyPoint {id: 'ctr-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-2:kp:2'})
SET kp.chunkId = 'ctr-2'
SET kp.ordinal = 2
SET kp.text = 'Unity of beings which are, only in so far as they are not one';
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MATCH (kp:KeyPoint {id: 'ctr-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-2:kp:3'})
SET kp.chunkId = 'ctr-2'
SET kp.ordinal = 3
SET kp.text = 'Positive and negative are posited contradiction';
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MATCH (kp:KeyPoint {id: 'ctr-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-2:kp:4'})
SET kp.chunkId = 'ctr-2'
SET kp.ordinal = 4
SET kp.text = 'As negative unities, precisely their self-positing';
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MATCH (kp:KeyPoint {id: 'ctr-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-2:kp:5'})
SET kp.chunkId = 'ctr-2'
SET kp.ordinal = 5
SET kp.text = 'Each excludes itself internally';
MATCH (c:IntegratedChunk {id: 'ctr-2'})
MATCH (kp:KeyPoint {id: 'ctr-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-3'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 59
SET segment.lineEnd = 80
SET segment.text = 'If we look at the two self-subsisting\ndeterminations of reflection on their own,\nthe positive is positedness as reflected\ninto likeness with itself\npositedness which is not reference to another,\nhence subsistence inasmuch as\nthe positedness is sublated and excluded.\nBut with this the positive makes itself\ninto the reference of a non-being into a positedness.\nIn this way the positive is contradiction in that,\nas the positing of self-identity by the\nexcluding of the negative,\nit makes itself into a negative,\nhence into the other\nwhich it excludes from itself.\nThis last, as excluded, is posited\nfree of the one that excludes;\nhence, as reflected into itself and itself as excluding.\nThe reflection that excludes is thus\nthe positing of the positive as excluding the other,\nso that this positing immediately is\nthe positing of its other which excludes it.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-3'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-3'
SET topic.title = 'Positive as contradiction'
SET topic.description = 'Positive is positedness as reflected into likeness with itself. Positedness which is not reference to another. Subsistence inasmuch as positedness is sublated and excluded. But positive makes itself into reference of non-being into positedness. Positive is contradiction: positing self-identity by excluding negative, makes itself into negative. Makes itself into other which it excludes from itself. Reflection that excludes is positing of positive as excluding other. This positing immediately is positing of its other which excludes it.'
SET topic.keyPoints = ['Positive is positedness as reflected into likeness with itself', 'Positive makes itself into reference of non-being into positedness', 'Positing self-identity by excluding negative, makes itself into negative', 'Makes itself into other which it excludes from itself', 'Positing of positive as excluding other is positing of its other which excludes it'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-3'})
MATCH (topic:Topic {id: 'topic:ctr-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-3'})
SET c.title = 'Positive as contradiction'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 59
SET c.lineEnd = 80
SET c.description = 'Positive is positedness as reflected into likeness with itself. Positedness which is not reference to another. Subsistence inasmuch as positedness is sublated and excluded. But positive makes itself into reference of non-being into positedness. Positive is contradiction: positing self-identity by excluding negative, makes itself into negative. Makes itself into other which it excludes from itself. Reflection that excludes is positing of positive as excluding other. This positing immediately is positing of its other which excludes it.'
SET c.keyPoints = ['Positive is positedness as reflected into likeness with itself', 'Positive makes itself into reference of non-being into positedness', 'Positing self-identity by excluding negative, makes itself into negative', 'Makes itself into other which it excludes from itself', 'Positing of positive as excluding other is positing of its other which excludes it']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 65
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'If we look at the two self-subsisting\ndeterminations of reflection on their own,\nthe positive is positedness as reflected\ninto likeness with itself\npositedness which is not reference to another,\nhence subsistence inasmuch as\nthe positedness is sublated and excluded.\nBut with this the positive makes itself\ninto the reference of a non-being into a positedness.\nIn this way the positive is contradiction in that,\nas the positing of self-identity by the\nexcluding of the negative,\nit makes itself into a negative,\nhence into the other\nwhich it excludes from itself.\nThis last, as excluded, is posited\nfree of the one that excludes;\nhence, as reflected into itself and itself as excluding.\nThe reflection that excludes is thus\nthe positing of the positive as excluding the other,\nso that this positing immediately is\nthe positing of its other which excludes it.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-3'})
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-3:kp:1'})
SET kp.chunkId = 'ctr-3'
SET kp.ordinal = 1
SET kp.text = 'Positive is positedness as reflected into likeness with itself';
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MATCH (kp:KeyPoint {id: 'ctr-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-3:kp:2'})
SET kp.chunkId = 'ctr-3'
SET kp.ordinal = 2
SET kp.text = 'Positive makes itself into reference of non-being into positedness';
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MATCH (kp:KeyPoint {id: 'ctr-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-3:kp:3'})
SET kp.chunkId = 'ctr-3'
SET kp.ordinal = 3
SET kp.text = 'Positing self-identity by excluding negative, makes itself into negative';
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MATCH (kp:KeyPoint {id: 'ctr-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-3:kp:4'})
SET kp.chunkId = 'ctr-3'
SET kp.ordinal = 4
SET kp.text = 'Makes itself into other which it excludes from itself';
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MATCH (kp:KeyPoint {id: 'ctr-3:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-3:kp:5'})
SET kp.chunkId = 'ctr-3'
SET kp.ordinal = 5
SET kp.text = 'Positing of positive as excluding other is positing of its other which excludes it';
MATCH (c:IntegratedChunk {id: 'ctr-3'})
MATCH (kp:KeyPoint {id: 'ctr-3:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-4'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 82
SET segment.lineEnd = 130
SET segment.text = 'This is the absolute contradiction of the positive;\nbut it is immediately the absolute contradiction of the negative;\nthe positing of both in one reflection.\nConsidered in itself as against the positive,\nthe negative is positedness as reflected into unlikeness to itself,\nthe negative as negative.\nBut the negative is itself the unlike,\nthe non-being of another;\nconsequently, reflection is in its unlikeness\nits reference rather to itself.\nNegation in general is the negative\nas quality or immediate determinateness;\nbut taken as negative, it is referred to\nthe negative of itself, to its other.\nIf this second negative is taken only\nas identical with the first,\nthen it is also only immediate,\njust like the first;\nthey are not taken, therefore,\nas each the other of the other,\nhence not as negatives:\nthe negative is not at all an immediate.\nBut now, since each is moreover equally\nthe same as what the other is,\nthis reference connecting them as unequal\nis just as much their identical connection.\n\nThis is therefore the same contradiction which the positive is,\nnamely positedness or negation as self-reference.\nBut the positive is only implicitly this contradiction,\nis contradiction only in itself;\nthe negative, on the contrary, is the posited contradiction;\nfor in its reflection into itself,\nas a negative which is in and for itself\nor a negative which is identical with itself,\nits determination is to be the not-identical,\nthe exclusion of identity.\nThe negative is this,\nto be identical with itself over against identity,\nand consequently, because of this excluding reflection,\nto exclude itself from itself.\n\nThe negative is therefore the whole opposition\nthe opposition which, as opposition, rests upon itself;\ndistinction that absolutely does not refer itself to another;\ndistinction which, as opposition, excludes identity from itself,\nbut thereby also excludes itself,\nfor as reference to itself it determines itself\nas the very identity which it excludes.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-4'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-4'
SET topic.title = 'Absolute contradiction of positive and negative'
SET topic.description = 'Absolute contradiction of positive, immediately absolute contradiction of negative. Negative is positedness as reflected into unlikeness to itself. Negative is itself unlike, non-being of another. Reflection in its unlikeness is reference rather to itself. Negative is not immediate. Same contradiction as positive: positedness or negation as self-reference. Positive only implicitly contradiction, contradiction only in itself. Negative is posited contradiction. Negative identical with itself, determination is to be not-identical, exclusion of identity. To be identical with itself over against identity, excludes itself from itself. Negative is whole opposition, excludes identity from itself, but thereby excludes itself. As reference to itself determines itself as very identity which it excludes.'
SET topic.keyPoints = ['Absolute contradiction of positive and negative', 'Negative is posited contradiction', 'Negative identical with itself, determination is to be not-identical', 'Excludes itself from itself', 'As reference to itself determines itself as very identity which it excludes'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-4'})
MATCH (topic:Topic {id: 'topic:ctr-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-4'})
SET c.title = 'Absolute contradiction of positive and negative'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 82
SET c.lineEnd = 130
SET c.description = 'Absolute contradiction of positive, immediately absolute contradiction of negative. Negative is positedness as reflected into unlikeness to itself. Negative is itself unlike, non-being of another. Reflection in its unlikeness is reference rather to itself. Negative is not immediate. Same contradiction as positive: positedness or negation as self-reference. Positive only implicitly contradiction, contradiction only in itself. Negative is posited contradiction. Negative identical with itself, determination is to be not-identical, exclusion of identity. To be identical with itself over against identity, excludes itself from itself. Negative is whole opposition, excludes identity from itself, but thereby excludes itself. As reference to itself determines itself as very identity which it excludes.'
SET c.keyPoints = ['Absolute contradiction of positive and negative', 'Negative is posited contradiction', 'Negative identical with itself, determination is to be not-identical', 'Excludes itself from itself', 'As reference to itself determines itself as very identity which it excludes']
SET c.tags = ['negation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 66
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'This is the absolute contradiction of the positive;\nbut it is immediately the absolute contradiction of the negative;\nthe positing of both in one reflection.\nConsidered in itself as against the positive,\nthe negative is positedness as reflected into unlikeness to itself,\nthe negative as negative.\nBut the negative is itself the unlike,\nthe non-being of another;\nconsequently, reflection is in its unlikeness\nits reference rather to itself.\nNegation in general is the negative\nas quality or immediate determinateness;\nbut taken as negative, it is referred to\nthe negative of itself, to its other.\nIf this second negative is taken only\nas identical with the first,\nthen it is also only immediate,\njust like the first;\nthey are not taken, therefore,\nas each the other of the other,\nhence not as negatives:\nthe negative is not at all an immediate.\nBut now, since each is moreover equally\nthe same as what the other is,\nthis reference connecting them as unequal\nis just as much their identical connection.\n\nThis is therefore the same contradiction which the positive is,\nnamely positedness or negation as self-reference.\nBut the positive is only implicitly this contradiction,\nis contradiction only in itself;\nthe negative, on the contrary, is the posited contradiction;\nfor in its reflection into itself,\nas a negative which is in and for itself\nor a negative which is identical with itself,\nits determination is to be the not-identical,\nthe exclusion of identity.\nThe negative is this,\nto be identical with itself over against identity,\nand consequently, because of this excluding reflection,\nto exclude itself from itself.\n\nThe negative is therefore the whole opposition\nthe opposition which, as opposition, rests upon itself;\ndistinction that absolutely does not refer itself to another;\ndistinction which, as opposition, excludes identity from itself,\nbut thereby also excludes itself,\nfor as reference to itself it determines itself\nas the very identity which it excludes.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-4'})
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-4:kp:1'})
SET kp.chunkId = 'ctr-4'
SET kp.ordinal = 1
SET kp.text = 'Absolute contradiction of positive and negative';
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MATCH (kp:KeyPoint {id: 'ctr-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-4:kp:2'})
SET kp.chunkId = 'ctr-4'
SET kp.ordinal = 2
SET kp.text = 'Negative is posited contradiction';
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MATCH (kp:KeyPoint {id: 'ctr-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-4:kp:3'})
SET kp.chunkId = 'ctr-4'
SET kp.ordinal = 3
SET kp.text = 'Negative identical with itself, determination is to be not-identical';
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MATCH (kp:KeyPoint {id: 'ctr-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-4:kp:4'})
SET kp.chunkId = 'ctr-4'
SET kp.ordinal = 4
SET kp.text = 'Excludes itself from itself';
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MATCH (kp:KeyPoint {id: 'ctr-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-4:kp:5'})
SET kp.chunkId = 'ctr-4'
SET kp.ordinal = 5
SET kp.text = 'As reference to itself determines itself as very identity which it excludes';
MATCH (c:IntegratedChunk {id: 'ctr-4'})
MATCH (kp:KeyPoint {id: 'ctr-4:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-5'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 132
SET segment.lineEnd = 154
SET segment.text = '2. Contradiction resolves itself.\n\nIn the self-excluding reflection\nwe have just considered,\nthe positive and the negative,\neach in its self-subsistence,\nsublates itself;\neach is simply the passing over,\nor rather the self-translating of itself into its opposite.\nThis internal ceaseless vanishing of the opposites is\nthe first unity that arises by virtue of contradiction;\nit is the null.\n\nBut contradiction does not contain merely the negative;\nit also contains the positive;\nor the self-excluding reflection is\nat the same time positing reflection;\nthe result of contradiction is not only the null.\nThe positive and the negative constitute\nthe positedness of the self-subsistence;\ntheir own self-negation sublates it.\nIt is this positedness which in truth\nfounders to the ground in contradiction.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-5'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-5'
SET topic.title = 'Contradiction resolves itself — null and positive'
SET topic.description = 'In self-excluding reflection, positive and negative, each in self-subsistence, sublates itself. Each is passing over, self-translating of itself into its opposite. Internal ceaseless vanishing of opposites is first unity that arises by virtue of contradiction. It is the null. But contradiction does not contain merely negative, also contains positive. Self-excluding reflection is at same time positing reflection. Result of contradiction is not only null. Positive and negative constitute positedness of self-subsistence. Their own self-negation sublates it. This positedness founders to ground in contradiction.'
SET topic.keyPoints = ['Each is self-translating of itself into its opposite', 'Internal ceaseless vanishing of opposites is first unity: the null', 'Contradiction also contains positive, self-excluding reflection is positing reflection', 'Result of contradiction is not only null', 'Positedness founders to ground in contradiction'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-5'})
MATCH (topic:Topic {id: 'topic:ctr-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-5'})
SET c.title = 'Contradiction resolves itself — null and positive'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 132
SET c.lineEnd = 154
SET c.description = 'In self-excluding reflection, positive and negative, each in self-subsistence, sublates itself. Each is passing over, self-translating of itself into its opposite. Internal ceaseless vanishing of opposites is first unity that arises by virtue of contradiction. It is the null. But contradiction does not contain merely negative, also contains positive. Self-excluding reflection is at same time positing reflection. Result of contradiction is not only null. Positive and negative constitute positedness of self-subsistence. Their own self-negation sublates it. This positedness founders to ground in contradiction.'
SET c.keyPoints = ['Each is self-translating of itself into its opposite', 'Internal ceaseless vanishing of opposites is first unity: the null', 'Contradiction also contains positive, self-excluding reflection is positing reflection', 'Result of contradiction is not only null', 'Positedness founders to ground in contradiction']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 5
SET c.globalOrder = 67
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '2. Contradiction resolves itself.\n\nIn the self-excluding reflection\nwe have just considered,\nthe positive and the negative,\neach in its self-subsistence,\nsublates itself;\neach is simply the passing over,\nor rather the self-translating of itself into its opposite.\nThis internal ceaseless vanishing of the opposites is\nthe first unity that arises by virtue of contradiction;\nit is the null.\n\nBut contradiction does not contain merely the negative;\nit also contains the positive;\nor the self-excluding reflection is\nat the same time positing reflection;\nthe result of contradiction is not only the null.\nThe positive and the negative constitute\nthe positedness of the self-subsistence;\ntheir own self-negation sublates it.\nIt is this positedness which in truth\nfounders to the ground in contradiction.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-5'})
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-5:kp:1'})
SET kp.chunkId = 'ctr-5'
SET kp.ordinal = 1
SET kp.text = 'Each is self-translating of itself into its opposite';
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MATCH (kp:KeyPoint {id: 'ctr-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-5:kp:2'})
SET kp.chunkId = 'ctr-5'
SET kp.ordinal = 2
SET kp.text = 'Internal ceaseless vanishing of opposites is first unity: the null';
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MATCH (kp:KeyPoint {id: 'ctr-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-5:kp:3'})
SET kp.chunkId = 'ctr-5'
SET kp.ordinal = 3
SET kp.text = 'Contradiction also contains positive, self-excluding reflection is positing reflection';
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MATCH (kp:KeyPoint {id: 'ctr-5:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-5:kp:4'})
SET kp.chunkId = 'ctr-5'
SET kp.ordinal = 4
SET kp.text = 'Result of contradiction is not only null';
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MATCH (kp:KeyPoint {id: 'ctr-5:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-5:kp:5'})
SET kp.chunkId = 'ctr-5'
SET kp.ordinal = 5
SET kp.text = 'Positedness founders to ground in contradiction';
MATCH (c:IntegratedChunk {id: 'ctr-5'})
MATCH (kp:KeyPoint {id: 'ctr-5:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-6'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 156
SET segment.lineEnd = 176
SET segment.text = 'The immanent reflection by virtue of which\nthe sides of opposition are turned into\nself-subsistent self-references is,\nfirst of all, their self-subsistence as distinct moments;\nthus they are this self-subsistence only in themselves,\nfor they are still opposites,\nand that they are in themselves self-subsistent\nconstitutes their positedness.\nBut their excluding reflection\nsublates this positedness,\nturns them into self-subsistent beings\nexisting in and for themselves,\nsuch as are self-subsistent not only in themselves\nbut by virtue of their negative reference to their other;\nin this way, their self-subsistence is also posited.\nBut, further, by thus being posited as self-subsistent,\nthey make themselves into a positedness.\nThey fate themselves to founder,\nsince they determine themselves as self-identical,\nyet in their self-identity they are rather the negative,\na self-identity which is reference-to-other.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-6'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-6'
SET topic.title = 'Immanent reflection — self-subsistence and positedness'
SET topic.description = 'Immanent reflection turns sides of opposition into self-subsistent self-references. First, their self-subsistence as distinct moments. They are self-subsistence only in themselves, for they are still opposites. That they are in themselves self-subsistent constitutes their positedness. But excluding reflection sublates this positedness. Turns them into self-subsistent beings existing in and for themselves. Self-subsistent by virtue of negative reference to their other. Their self-subsistence also posited. But by being posited as self-subsistent, they make themselves into positedness. They fate themselves to founder. Determine themselves as self-identical, yet in self-identity are rather negative. Self-identity which is reference-to-other.'
SET topic.keyPoints = ['Immanent reflection turns sides into self-subsistent self-references', 'They are self-subsistence only in themselves, for they are still opposites', 'Excluding reflection sublates positedness, turns them into self-subsistent beings', 'By being posited as self-subsistent, they make themselves into positedness', 'Determine themselves as self-identical, yet in self-identity are rather negative'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-6'})
MATCH (topic:Topic {id: 'topic:ctr-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-6'})
SET c.title = 'Immanent reflection — self-subsistence and positedness'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 156
SET c.lineEnd = 176
SET c.description = 'Immanent reflection turns sides of opposition into self-subsistent self-references. First, their self-subsistence as distinct moments. They are self-subsistence only in themselves, for they are still opposites. That they are in themselves self-subsistent constitutes their positedness. But excluding reflection sublates this positedness. Turns them into self-subsistent beings existing in and for themselves. Self-subsistent by virtue of negative reference to their other. Their self-subsistence also posited. But by being posited as self-subsistent, they make themselves into positedness. They fate themselves to founder. Determine themselves as self-identical, yet in self-identity are rather negative. Self-identity which is reference-to-other.'
SET c.keyPoints = ['Immanent reflection turns sides into self-subsistent self-references', 'They are self-subsistence only in themselves, for they are still opposites', 'Excluding reflection sublates positedness, turns them into self-subsistent beings', 'By being posited as self-subsistent, they make themselves into positedness', 'Determine themselves as self-identical, yet in self-identity are rather negative']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 6
SET c.globalOrder = 68
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The immanent reflection by virtue of which\nthe sides of opposition are turned into\nself-subsistent self-references is,\nfirst of all, their self-subsistence as distinct moments;\nthus they are this self-subsistence only in themselves,\nfor they are still opposites,\nand that they are in themselves self-subsistent\nconstitutes their positedness.\nBut their excluding reflection\nsublates this positedness,\nturns them into self-subsistent beings\nexisting in and for themselves,\nsuch as are self-subsistent not only in themselves\nbut by virtue of their negative reference to their other;\nin this way, their self-subsistence is also posited.\nBut, further, by thus being posited as self-subsistent,\nthey make themselves into a positedness.\nThey fate themselves to founder,\nsince they determine themselves as self-identical,\nyet in their self-identity they are rather the negative,\na self-identity which is reference-to-other.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-6'})
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-6:kp:1'})
SET kp.chunkId = 'ctr-6'
SET kp.ordinal = 1
SET kp.text = 'Immanent reflection turns sides into self-subsistent self-references';
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MATCH (kp:KeyPoint {id: 'ctr-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-6:kp:2'})
SET kp.chunkId = 'ctr-6'
SET kp.ordinal = 2
SET kp.text = 'They are self-subsistence only in themselves, for they are still opposites';
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MATCH (kp:KeyPoint {id: 'ctr-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-6:kp:3'})
SET kp.chunkId = 'ctr-6'
SET kp.ordinal = 3
SET kp.text = 'Excluding reflection sublates positedness, turns them into self-subsistent beings';
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MATCH (kp:KeyPoint {id: 'ctr-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-6:kp:4'})
SET kp.chunkId = 'ctr-6'
SET kp.ordinal = 4
SET kp.text = 'By being posited as self-subsistent, they make themselves into positedness';
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MATCH (kp:KeyPoint {id: 'ctr-6:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-6:kp:5'})
SET kp.chunkId = 'ctr-6'
SET kp.ordinal = 5
SET kp.text = 'Determine themselves as self-identical, yet in self-identity are rather negative';
MATCH (c:IntegratedChunk {id: 'ctr-6'})
MATCH (kp:KeyPoint {id: 'ctr-6:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-7'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 178
SET segment.lineEnd = 210
SET segment.text = 'However, on closer examination,\nthis excluding reflection is not only\nthis formal determination.\nIt is self-subsistence existing in itself,\nand the sublating of this positedness\nis only through this sublating a unity that\nexists for itself and is in fact self-subsistent.\nOf course, through the sublating of otherness or positedness,\npositedness or the negative of an other is indeed present again.\nBut in fact, this negation is not just a return to the first\nimmediate reference to the other,\nis not positedness as sublated immediacy,\nbut positedness as sublated positedness.\nThe excluding reflection of self-subsistence,\nsince it is excluding,\nmakes itself a positedness but is just as much\nthe sublation of its positedness.\nIt is sublating reference to itself;\nin that reference, it first sublates the negative\nand it secondly posits itself as a negative,\nand it is only this posited negative that it sublates;\nin sublating the negative,\nit both posits and sublates it at the same time.\nIn this way the exclusive determination is\nitself that other of itself of which it is the negation;\nthe sublation of this positedness is not, therefore,\nonce more positedness as the negative of an other,\nbut is self-withdrawal, positive self-unity.\nSelf-subsistence is thus unity that turns back into itself\nby virtue of its own negation,\nfor it turns into itself through the negation of its positedness.\nIt is the unity of essence to be identical with itself\nthrough the negation not of an other, but of itself.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-7'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-7'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-7'
SET topic.title = 'Excluding reflection — self-withdrawal, positive self-unity'
SET topic.description = 'Excluding reflection is not only formal determination. It is self-subsistence existing in itself. Sublating of positedness is only through this sublating a unity that exists for itself and is self-subsistent. This negation is not return to first immediate reference to other. Not positedness as sublated immediacy, but positedness as sublated positedness. Excluding reflection makes itself positedness but is sublation of its positedness. Sublating reference to itself. First sublates negative, secondly posits itself as negative, only this posited negative that it sublates. In sublating negative, both posits and sublates it at same time. Exclusive determination is itself that other of itself of which it is negation. Sublation of positedness is self-withdrawal, positive self-unity. Self-subsistence is unity that turns back into itself by virtue of its own negation. Unity of essence to be identical with itself through negation not of other, but of itself.'
SET topic.keyPoints = ['Excluding reflection is self-subsistence existing in itself', 'Not positedness as sublated immediacy, but positedness as sublated positedness', 'In sublating negative, both posits and sublates it at same time', 'Sublation of positedness is self-withdrawal, positive self-unity', 'Unity of essence to be identical with itself through negation not of other, but of itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-7'})
MATCH (topic:Topic {id: 'topic:ctr-7'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-7'})
SET c.title = 'Excluding reflection — self-withdrawal, positive self-unity'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 178
SET c.lineEnd = 210
SET c.description = 'Excluding reflection is not only formal determination. It is self-subsistence existing in itself. Sublating of positedness is only through this sublating a unity that exists for itself and is self-subsistent. This negation is not return to first immediate reference to other. Not positedness as sublated immediacy, but positedness as sublated positedness. Excluding reflection makes itself positedness but is sublation of its positedness. Sublating reference to itself. First sublates negative, secondly posits itself as negative, only this posited negative that it sublates. In sublating negative, both posits and sublates it at same time. Exclusive determination is itself that other of itself of which it is negation. Sublation of positedness is self-withdrawal, positive self-unity. Self-subsistence is unity that turns back into itself by virtue of its own negation. Unity of essence to be identical with itself through negation not of other, but of itself.'
SET c.keyPoints = ['Excluding reflection is self-subsistence existing in itself', 'Not positedness as sublated immediacy, but positedness as sublated positedness', 'In sublating negative, both posits and sublates it at same time', 'Sublation of positedness is self-withdrawal, positive self-unity', 'Unity of essence to be identical with itself through negation not of other, but of itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 7
SET c.globalOrder = 69
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'However, on closer examination,\nthis excluding reflection is not only\nthis formal determination.\nIt is self-subsistence existing in itself,\nand the sublating of this positedness\nis only through this sublating a unity that\nexists for itself and is in fact self-subsistent.\nOf course, through the sublating of otherness or positedness,\npositedness or the negative of an other is indeed present again.\nBut in fact, this negation is not just a return to the first\nimmediate reference to the other,\nis not positedness as sublated immediacy,\nbut positedness as sublated positedness.\nThe excluding reflection of self-subsistence,\nsince it is excluding,\nmakes itself a positedness but is just as much\nthe sublation of its positedness.\nIt is sublating reference to itself;\nin that reference, it first sublates the negative\nand it secondly posits itself as a negative,\nand it is only this posited negative that it sublates;\nin sublating the negative,\nit both posits and sublates it at the same time.\nIn this way the exclusive determination is\nitself that other of itself of which it is the negation;\nthe sublation of this positedness is not, therefore,\nonce more positedness as the negative of an other,\nbut is self-withdrawal, positive self-unity.\nSelf-subsistence is thus unity that turns back into itself\nby virtue of its own negation,\nfor it turns into itself through the negation of its positedness.\nIt is the unity of essence to be identical with itself\nthrough the negation not of an other, but of itself.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-7'})
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-7:kp:1'})
SET kp.chunkId = 'ctr-7'
SET kp.ordinal = 1
SET kp.text = 'Excluding reflection is self-subsistence existing in itself';
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MATCH (kp:KeyPoint {id: 'ctr-7:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-7:kp:2'})
SET kp.chunkId = 'ctr-7'
SET kp.ordinal = 2
SET kp.text = 'Not positedness as sublated immediacy, but positedness as sublated positedness';
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MATCH (kp:KeyPoint {id: 'ctr-7:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-7:kp:3'})
SET kp.chunkId = 'ctr-7'
SET kp.ordinal = 3
SET kp.text = 'In sublating negative, both posits and sublates it at same time';
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MATCH (kp:KeyPoint {id: 'ctr-7:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-7:kp:4'})
SET kp.chunkId = 'ctr-7'
SET kp.ordinal = 4
SET kp.text = 'Sublation of positedness is self-withdrawal, positive self-unity';
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MATCH (kp:KeyPoint {id: 'ctr-7:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-7:kp:5'})
SET kp.chunkId = 'ctr-7'
SET kp.ordinal = 5
SET kp.text = 'Unity of essence to be identical with itself through negation not of other, but of itself';
MATCH (c:IntegratedChunk {id: 'ctr-7'})
MATCH (kp:KeyPoint {id: 'ctr-7:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-8'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 212
SET segment.lineEnd = 236
SET segment.text = '3. According to this positive side,\nsince self-subsistence in opposition,\nas excluding reflection,\nmakes itself into a positedness\nand equally sublates this positedness,\nnot only has opposition foundered\nbut in foundering it has gone back\nto its foundation, to its ground.\nThe excluding reflection of\nthe self-subsisting opposition turns it\ninto a negative, something only posited;\nit thereby reduces its formerly self-subsisting determinations,\nthe positive and the negative,\nto determinations which are only determinations;\nand the positedness, since it is now made into positedness,\nhas simply gone back to its unity with itself;\nit is simple essence, but essence as ground.\nThrough the sublating of the determinations of essence,\nwhich are in themselves self-contradictory,\nessence is restored,\nbut restored in the determination of\nan exclusive, reflective unity\na simple unity which determines itself as negation,\nbut in this positedness is immediately like itself\nand withdrawn into itself.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-8'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-8'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-8'
SET topic.title = 'Transition to ground — opposition foundered'
SET topic.description = 'Since self-subsistence in opposition, as excluding reflection, makes itself into positedness and equally sublates it. Not only has opposition foundered but in foundering has gone back to foundation, to ground. Excluding reflection turns self-subsisting opposition into negative, something only posited. Reduces formerly self-subsisting determinations (positive and negative) to determinations which are only determinations. Positedness has gone back to its unity with itself. Simple essence, but essence as ground. Through sublating of determinations of essence, which are in themselves self-contradictory, essence is restored. Restored in determination of exclusive, reflective unity. Simple unity which determines itself as negation, but in positedness is immediately like itself and withdrawn into itself.'
SET topic.keyPoints = ['Opposition foundered, gone back to foundation, to ground', 'Excluding reflection turns opposition into negative, something only posited', 'Positedness has gone back to its unity with itself', 'Essence is restored in determination of exclusive, reflective unity', 'Simple unity which determines itself as negation, but in positedness is like itself'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-8'})
MATCH (topic:Topic {id: 'topic:ctr-8'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-8'})
SET c.title = 'Transition to ground — opposition foundered'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 212
SET c.lineEnd = 236
SET c.description = 'Since self-subsistence in opposition, as excluding reflection, makes itself into positedness and equally sublates it. Not only has opposition foundered but in foundering has gone back to foundation, to ground. Excluding reflection turns self-subsisting opposition into negative, something only posited. Reduces formerly self-subsisting determinations (positive and negative) to determinations which are only determinations. Positedness has gone back to its unity with itself. Simple essence, but essence as ground. Through sublating of determinations of essence, which are in themselves self-contradictory, essence is restored. Restored in determination of exclusive, reflective unity. Simple unity which determines itself as negation, but in positedness is immediately like itself and withdrawn into itself.'
SET c.keyPoints = ['Opposition foundered, gone back to foundation, to ground', 'Excluding reflection turns opposition into negative, something only posited', 'Positedness has gone back to its unity with itself', 'Essence is restored in determination of exclusive, reflective unity', 'Simple unity which determines itself as negation, but in positedness is like itself']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 8
SET c.globalOrder = 70
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '3. According to this positive side,\nsince self-subsistence in opposition,\nas excluding reflection,\nmakes itself into a positedness\nand equally sublates this positedness,\nnot only has opposition foundered\nbut in foundering it has gone back\nto its foundation, to its ground.\nThe excluding reflection of\nthe self-subsisting opposition turns it\ninto a negative, something only posited;\nit thereby reduces its formerly self-subsisting determinations,\nthe positive and the negative,\nto determinations which are only determinations;\nand the positedness, since it is now made into positedness,\nhas simply gone back to its unity with itself;\nit is simple essence, but essence as ground.\nThrough the sublating of the determinations of essence,\nwhich are in themselves self-contradictory,\nessence is restored,\nbut restored in the determination of\nan exclusive, reflective unity\na simple unity which determines itself as negation,\nbut in this positedness is immediately like itself\nand withdrawn into itself.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-8'})
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-8:kp:1'})
SET kp.chunkId = 'ctr-8'
SET kp.ordinal = 1
SET kp.text = 'Opposition foundered, gone back to foundation, to ground';
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MATCH (kp:KeyPoint {id: 'ctr-8:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-8:kp:2'})
SET kp.chunkId = 'ctr-8'
SET kp.ordinal = 2
SET kp.text = 'Excluding reflection turns opposition into negative, something only posited';
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MATCH (kp:KeyPoint {id: 'ctr-8:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-8:kp:3'})
SET kp.chunkId = 'ctr-8'
SET kp.ordinal = 3
SET kp.text = 'Positedness has gone back to its unity with itself';
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MATCH (kp:KeyPoint {id: 'ctr-8:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-8:kp:4'})
SET kp.chunkId = 'ctr-8'
SET kp.ordinal = 4
SET kp.text = 'Essence is restored in determination of exclusive, reflective unity';
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MATCH (kp:KeyPoint {id: 'ctr-8:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-8:kp:5'})
SET kp.chunkId = 'ctr-8'
SET kp.ordinal = 5
SET kp.text = 'Simple unity which determines itself as negation, but in positedness is like itself';
MATCH (c:IntegratedChunk {id: 'ctr-8'})
MATCH (kp:KeyPoint {id: 'ctr-8:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-9'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 238
SET segment.lineEnd = 265
SET segment.text = 'In the first place, therefore, because of its contradiction,\nthe self-subsisting opposition goes back into a ground;\nthis opposition is what comes first,\nthe immediate from which the beginning is made,\nwhile the sublated opposition\nor the sublated positedness is itself a positedness.\nAccordingly, essence is as ground a positedness,\nsomething that has become.\nBut conversely, only this has been posited,\nnamely that the opposition or the positedness is\nsomething sublated, only is as positedness.\nAs ground, therefore, essence is excluding reflection\nbecause it makes itself into a positedness;\nbecause the opposition from which the start\nwas just now made and was the immediate is\nthe merely posited determinate self-subsistence of essence;\nbecause opposition only sublates itself within,\nwhereas essence is in its determinateness reflected into itself.\nAs ground, therefore, essence excludes\nitself from itself, it posits itself;\nits positedness which is what is excluded\nis only as positedness,\nas identity of the negative with itself.\nThis self-subsistent is the negative\nposited as the negative,\nsomething self-contradictory\nwhich, consequently, remains in\nthe essence as in its ground.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-9'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-9'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-9'
SET topic.title = 'Essence as ground — positedness'
SET topic.description = 'Because of contradiction, self-subsisting opposition goes back into ground. Opposition is what comes first, immediate from which beginning is made. Sublated opposition or sublated positedness is itself positedness. Essence is as ground positedness, something that has become. But conversely, only this has been posited: opposition or positedness is something sublated, only is as positedness. As ground, essence is excluding reflection because makes itself into positedness. Opposition from which start was made is merely posited determinate self-subsistence of essence. Opposition only sublates itself within, whereas essence is in determinateness reflected into itself. As ground, essence excludes itself from itself, posits itself. Positedness which is excluded is only as positedness, as identity of negative with itself. Self-subsistent is negative posited as negative, something self-contradictory. Remains in essence as in its ground.'
SET topic.keyPoints = ['Self-subsisting opposition goes back into ground', 'Essence is as ground positedness, something that has become', 'As ground, essence is excluding reflection because makes itself into positedness', 'Essence excludes itself from itself, posits itself', 'Self-subsistent is negative posited as negative, remains in essence as in its ground'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-9'})
MATCH (topic:Topic {id: 'topic:ctr-9'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-9'})
SET c.title = 'Essence as ground — positedness'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 238
SET c.lineEnd = 265
SET c.description = 'Because of contradiction, self-subsisting opposition goes back into ground. Opposition is what comes first, immediate from which beginning is made. Sublated opposition or sublated positedness is itself positedness. Essence is as ground positedness, something that has become. But conversely, only this has been posited: opposition or positedness is something sublated, only is as positedness. As ground, essence is excluding reflection because makes itself into positedness. Opposition from which start was made is merely posited determinate self-subsistence of essence. Opposition only sublates itself within, whereas essence is in determinateness reflected into itself. As ground, essence excludes itself from itself, posits itself. Positedness which is excluded is only as positedness, as identity of negative with itself. Self-subsistent is negative posited as negative, something self-contradictory. Remains in essence as in its ground.'
SET c.keyPoints = ['Self-subsisting opposition goes back into ground', 'Essence is as ground positedness, something that has become', 'As ground, essence is excluding reflection because makes itself into positedness', 'Essence excludes itself from itself, posits itself', 'Self-subsistent is negative posited as negative, remains in essence as in its ground']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 9
SET c.globalOrder = 71
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'In the first place, therefore, because of its contradiction,\nthe self-subsisting opposition goes back into a ground;\nthis opposition is what comes first,\nthe immediate from which the beginning is made,\nwhile the sublated opposition\nor the sublated positedness is itself a positedness.\nAccordingly, essence is as ground a positedness,\nsomething that has become.\nBut conversely, only this has been posited,\nnamely that the opposition or the positedness is\nsomething sublated, only is as positedness.\nAs ground, therefore, essence is excluding reflection\nbecause it makes itself into a positedness;\nbecause the opposition from which the start\nwas just now made and was the immediate is\nthe merely posited determinate self-subsistence of essence;\nbecause opposition only sublates itself within,\nwhereas essence is in its determinateness reflected into itself.\nAs ground, therefore, essence excludes\nitself from itself, it posits itself;\nits positedness which is what is excluded\nis only as positedness,\nas identity of the negative with itself.\nThis self-subsistent is the negative\nposited as the negative,\nsomething self-contradictory\nwhich, consequently, remains in\nthe essence as in its ground.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-9'})
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-9:kp:1'})
SET kp.chunkId = 'ctr-9'
SET kp.ordinal = 1
SET kp.text = 'Self-subsisting opposition goes back into ground';
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MATCH (kp:KeyPoint {id: 'ctr-9:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-9:kp:2'})
SET kp.chunkId = 'ctr-9'
SET kp.ordinal = 2
SET kp.text = 'Essence is as ground positedness, something that has become';
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MATCH (kp:KeyPoint {id: 'ctr-9:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-9:kp:3'})
SET kp.chunkId = 'ctr-9'
SET kp.ordinal = 3
SET kp.text = 'As ground, essence is excluding reflection because makes itself into positedness';
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MATCH (kp:KeyPoint {id: 'ctr-9:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-9:kp:4'})
SET kp.chunkId = 'ctr-9'
SET kp.ordinal = 4
SET kp.text = 'Essence excludes itself from itself, posits itself';
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MATCH (kp:KeyPoint {id: 'ctr-9:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-9:kp:5'})
SET kp.chunkId = 'ctr-9'
SET kp.ordinal = 5
SET kp.text = 'Self-subsistent is negative posited as negative, remains in essence as in its ground';
MATCH (c:IntegratedChunk {id: 'ctr-9'})
MATCH (kp:KeyPoint {id: 'ctr-9:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:ctr-10'})
SET segment.sourceId = 'source-contradiction'
SET segment.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET segment.lineStart = 267
SET segment.lineEnd = 293
SET segment.text = 'The resolved contradiction is therefore ground,\nessence as unity of the positive and the negative.\nIn opposition, determinateness has progressed to self-subsistence;\nbut ground is this self-subsistence as completed;\nin it, the negative is self-subsistent essence, but as negative;\nand, as self-identical in this negativity,\nground is thus equally the positive.\nIn ground, therefore, opposition and its contradiction\nare just as much removed as preserved.\nGround is essence as positive self-identity\nwhich, however, at the same time\nrefers itself to itself as negativity\nand therefore determines itself,\nmaking itself into an excluded positedness;\nbut this positedness is the whole self-subsisting essence,\nand essence is ground, self-identical in its negation and positive.\nThe self-contradictory self-subsistent opposition\nwas itself, therefore, already ground;\nall that was added to it was the determination of self-unity\nwhich emerges as each of the self-subsisting opposites\nsublates itself and makes itself into its other,\nthereby founders and sinks to the ground\nbut therein also reunites itself with itself;\nthus in this foundering, that is,\nin its positedness or in the negation,\nit rather is for the first time the essence\nthat is reflected into itself and self-identical.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (segment:ChunkSegment {id: 'chunk:ctr-10'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:ctr-10'})
SET topic.sourceId = 'source-contradiction'
SET topic.topicRef = 'ctr-10'
SET topic.title = 'Resolved contradiction is ground — unity of positive and negative'
SET topic.description = 'Resolved contradiction is ground, essence as unity of positive and negative. In opposition, determinateness has progressed to self-subsistence. Ground is this self-subsistence as completed. In it, negative is self-subsistent essence, but as negative. As self-identical in this negativity, ground is equally positive. In ground, opposition and its contradiction just as much removed as preserved. Ground is essence as positive self-identity which refers itself to itself as negativity. Determines itself, making itself into excluded positedness. But positedness is whole self-subsisting essence. Essence is ground, self-identical in its negation and positive. Self-contradictory self-subsistent opposition was itself already ground. All that was added was determination of self-unity. Which emerges as each self-subsisting opposite sublates itself and makes itself into its other. Founders and sinks to ground but therein reunites itself with itself. In foundering, in positedness or in negation, rather is for first time essence that is reflected into itself and self-identical.'
SET topic.keyPoints = ['Resolved contradiction is ground, essence as unity of positive and negative', 'Ground is self-subsistence as completed', 'In ground, opposition and its contradiction just as much removed as preserved', 'Ground is essence as positive self-identity which refers itself to itself as negativity', 'Self-contradictory opposition was itself already ground', 'In foundering, essence is for first time reflected into itself and self-identical'];
MATCH (segment:ChunkSegment {id: 'chunk:ctr-10'})
MATCH (topic:Topic {id: 'topic:ctr-10'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'ctr-10'})
SET c.title = 'Resolved contradiction is ground — unity of positive and negative'
SET c.sourceId = 'source-contradiction'
SET c.sourceFile = 'relative/essence/reflection/foundation/sources/contradiction.txt'
SET c.lineStart = 267
SET c.lineEnd = 293
SET c.description = 'Resolved contradiction is ground, essence as unity of positive and negative. In opposition, determinateness has progressed to self-subsistence. Ground is this self-subsistence as completed. In it, negative is self-subsistent essence, but as negative. As self-identical in this negativity, ground is equally positive. In ground, opposition and its contradiction just as much removed as preserved. Ground is essence as positive self-identity which refers itself to itself as negativity. Determines itself, making itself into excluded positedness. But positedness is whole self-subsisting essence. Essence is ground, self-identical in its negation and positive. Self-contradictory self-subsistent opposition was itself already ground. All that was added was determination of self-unity. Which emerges as each self-subsisting opposite sublates itself and makes itself into its other. Founders and sinks to ground but therein reunites itself with itself. In foundering, in positedness or in negation, rather is for first time essence that is reflected into itself and self-identical.'
SET c.keyPoints = ['Resolved contradiction is ground, essence as unity of positive and negative', 'Ground is self-subsistence as completed', 'In ground, opposition and its contradiction just as much removed as preserved', 'Ground is essence as positive self-identity which refers itself to itself as negativity', 'Self-contradictory opposition was itself already ground', 'In foundering, essence is for first time reflected into itself and self-identical']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 10
SET c.globalOrder = 72
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The resolved contradiction is therefore ground,\nessence as unity of the positive and the negative.\nIn opposition, determinateness has progressed to self-subsistence;\nbut ground is this self-subsistence as completed;\nin it, the negative is self-subsistent essence, but as negative;\nand, as self-identical in this negativity,\nground is thus equally the positive.\nIn ground, therefore, opposition and its contradiction\nare just as much removed as preserved.\nGround is essence as positive self-identity\nwhich, however, at the same time\nrefers itself to itself as negativity\nand therefore determines itself,\nmaking itself into an excluded positedness;\nbut this positedness is the whole self-subsisting essence,\nand essence is ground, self-identical in its negation and positive.\nThe self-contradictory self-subsistent opposition\nwas itself, therefore, already ground;\nall that was added to it was the determination of self-unity\nwhich emerges as each of the self-subsisting opposites\nsublates itself and makes itself into its other,\nthereby founders and sinks to the ground\nbut therein also reunites itself with itself;\nthus in this foundering, that is,\nin its positedness or in the negation,\nit rather is for the first time the essence\nthat is reflected into itself and self-identical.';
MATCH (s:SourceText {id: 'source-contradiction'})
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:ctr-10'})
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'ctr-10:kp:1'})
SET kp.chunkId = 'ctr-10'
SET kp.ordinal = 1
SET kp.text = 'Resolved contradiction is ground, essence as unity of positive and negative';
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MATCH (kp:KeyPoint {id: 'ctr-10:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-10:kp:2'})
SET kp.chunkId = 'ctr-10'
SET kp.ordinal = 2
SET kp.text = 'Ground is self-subsistence as completed';
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MATCH (kp:KeyPoint {id: 'ctr-10:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-10:kp:3'})
SET kp.chunkId = 'ctr-10'
SET kp.ordinal = 3
SET kp.text = 'In ground, opposition and its contradiction just as much removed as preserved';
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MATCH (kp:KeyPoint {id: 'ctr-10:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-10:kp:4'})
SET kp.chunkId = 'ctr-10'
SET kp.ordinal = 4
SET kp.text = 'Ground is essence as positive self-identity which refers itself to itself as negativity';
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MATCH (kp:KeyPoint {id: 'ctr-10:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-10:kp:5'})
SET kp.chunkId = 'ctr-10'
SET kp.ordinal = 5
SET kp.text = 'Self-contradictory opposition was itself already ground';
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MATCH (kp:KeyPoint {id: 'ctr-10:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'ctr-10:kp:6'})
SET kp.chunkId = 'ctr-10'
SET kp.ordinal = 6
SET kp.text = 'In foundering, essence is for first time reflected into itself and self-identical';
MATCH (c:IntegratedChunk {id: 'ctr-10'})
MATCH (kp:KeyPoint {id: 'ctr-10:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
