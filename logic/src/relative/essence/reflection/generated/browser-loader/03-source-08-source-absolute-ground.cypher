MERGE (s:SourceText {id: 'source-absolute-ground'})
SET s.title = 'Absolute Ground'
SET s.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET s.totalLines = 536;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-absolute-ground'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:abs-1'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 5
SET segment.lineEnd = 43
SET segment.text = 'The determination of reflection,\ninasmuch as this determination returns into ground,\nis a first immediate existence in general\nfrom which the beginning is made.\nBut existence still has only the meaning of positedness\nand essentially presupposes a ground,\nin the sense that it does not really posit a ground;\nthat the positing is a sublating of itself;\nthat it is rather the immediate that is posited,\nand the ground the non-posited.\nAs we have seen, this presupposing is the positing\nthat rebounds on that which posits;\nas sublated determinate being, the ground is not an indeterminate\nbut is rather essence determined through itself,\nbut determined as indeterminate or as sublated positedness.\nIt is essence that in its negativity is identical with itself.\n\nThe determinateness of essence as ground is thus twofold:\nit is the determinateness of the ground and of the grounded.\nIt is, first, essence as ground,\nessence determined to be essence\nas against positedness, as non-positedness.\nSecond, it is that which is grounded,\nthe immediate that, however, is not anything in and for itself:\nis positedness as positedness.\nConsequently, this positedness is equally identical with itself,\nbut in an identity which is that of the negative with itself.\nThe self-identical negative and the self-identical positive\nare now one and the same identity.\nFor the ground is the self-identity\nof the positive or even also of positedness;\nthe grounded is positedness as positedness,\nbut this its reflection-into-itself is the identity of the ground.\nThis simple identity, therefore, is not itself ground,\nfor the ground is essence posited as\nthe non-posited as against positedness.\nAs the unity of this determinate identity (the ground)\nand of the negative identity (the grounded),\nit is essence in general distinct from its mediation.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-1'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-1'
SET topic.title = 'a. Form and essence — ground and grounded'
SET topic.description = 'Determination of reflection returns into ground. First immediate existence from which beginning is made. Existence has meaning of positedness, presupposes ground. Positing is sublating of itself, immediate is posited, ground is non-posited. Ground is essence determined through itself, as indeterminate or sublated positedness. Essence in its negativity identical with itself. Determinateness of essence as ground is twofold: ground and grounded. Essence as ground: essence determined to be essence as against positedness, as non-positedness. Grounded: immediate that is not anything in and for itself, positedness as positedness. Self-identical negative and self-identical positive are one and same identity.'
SET topic.keyPoints = ['Determination of reflection returns into ground', 'Existence has meaning of positedness, presupposes ground', 'Ground is essence determined through itself, as indeterminate or sublated positedness', 'Determinateness of essence as ground is twofold: ground and grounded', 'Self-identical negative and self-identical positive are one and same identity'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-1'})
MATCH (topic:Topic {id: 'topic:abs-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-1'})
SET c.title = 'a. Form and essence — ground and grounded'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 5
SET c.lineEnd = 43
SET c.description = 'Determination of reflection returns into ground. First immediate existence from which beginning is made. Existence has meaning of positedness, presupposes ground. Positing is sublating of itself, immediate is posited, ground is non-posited. Ground is essence determined through itself, as indeterminate or sublated positedness. Essence in its negativity identical with itself. Determinateness of essence as ground is twofold: ground and grounded. Essence as ground: essence determined to be essence as against positedness, as non-positedness. Grounded: immediate that is not anything in and for itself, positedness as positedness. Self-identical negative and self-identical positive are one and same identity.'
SET c.keyPoints = ['Determination of reflection returns into ground', 'Existence has meaning of positedness, presupposes ground', 'Ground is essence determined through itself, as indeterminate or sublated positedness', 'Determinateness of essence as ground is twofold: ground and grounded', 'Self-identical negative and self-identical positive are one and same identity']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 87
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The determination of reflection,\ninasmuch as this determination returns into ground,\nis a first immediate existence in general\nfrom which the beginning is made.\nBut existence still has only the meaning of positedness\nand essentially presupposes a ground,\nin the sense that it does not really posit a ground;\nthat the positing is a sublating of itself;\nthat it is rather the immediate that is posited,\nand the ground the non-posited.\nAs we have seen, this presupposing is the positing\nthat rebounds on that which posits;\nas sublated determinate being, the ground is not an indeterminate\nbut is rather essence determined through itself,\nbut determined as indeterminate or as sublated positedness.\nIt is essence that in its negativity is identical with itself.\n\nThe determinateness of essence as ground is thus twofold:\nit is the determinateness of the ground and of the grounded.\nIt is, first, essence as ground,\nessence determined to be essence\nas against positedness, as non-positedness.\nSecond, it is that which is grounded,\nthe immediate that, however, is not anything in and for itself:\nis positedness as positedness.\nConsequently, this positedness is equally identical with itself,\nbut in an identity which is that of the negative with itself.\nThe self-identical negative and the self-identical positive\nare now one and the same identity.\nFor the ground is the self-identity\nof the positive or even also of positedness;\nthe grounded is positedness as positedness,\nbut this its reflection-into-itself is the identity of the ground.\nThis simple identity, therefore, is not itself ground,\nfor the ground is essence posited as\nthe non-posited as against positedness.\nAs the unity of this determinate identity (the ground)\nand of the negative identity (the grounded),\nit is essence in general distinct from its mediation.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-1'})
MATCH (c:IntegratedChunk {id: 'abs-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-1:kp:1'})
SET kp.chunkId = 'abs-1'
SET kp.ordinal = 1
SET kp.text = 'Determination of reflection returns into ground';
MATCH (c:IntegratedChunk {id: 'abs-1'})
MATCH (kp:KeyPoint {id: 'abs-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-1:kp:2'})
SET kp.chunkId = 'abs-1'
SET kp.ordinal = 2
SET kp.text = 'Existence has meaning of positedness, presupposes ground';
MATCH (c:IntegratedChunk {id: 'abs-1'})
MATCH (kp:KeyPoint {id: 'abs-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-1:kp:3'})
SET kp.chunkId = 'abs-1'
SET kp.ordinal = 3
SET kp.text = 'Ground is essence determined through itself, as indeterminate or sublated positedness';
MATCH (c:IntegratedChunk {id: 'abs-1'})
MATCH (kp:KeyPoint {id: 'abs-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-1:kp:4'})
SET kp.chunkId = 'abs-1'
SET kp.ordinal = 4
SET kp.text = 'Determinateness of essence as ground is twofold: ground and grounded';
MATCH (c:IntegratedChunk {id: 'abs-1'})
MATCH (kp:KeyPoint {id: 'abs-1:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-1:kp:5'})
SET kp.chunkId = 'abs-1'
SET kp.ordinal = 5
SET kp.text = 'Self-identical negative and self-identical positive are one and same identity';
MATCH (c:IntegratedChunk {id: 'abs-1'})
MATCH (kp:KeyPoint {id: 'abs-1:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-2'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 45
SET segment.lineEnd = 69
SET segment.text = 'For one thing, this mediation,\ncompared with the preceding reflections\nfrom which it derives,\nis not pure reflection,\nwhich is undistinguished from essence\nand still does not have the negative in it,\nconsequently also does not as yet contain\nthe self-subsistence of the determinations.\nThese have their subsistence, rather,\nin the ground understood as sublated reflection.\nAnd it is also not the determining reflection\nwhose determinations have essential self-subsistence,\nfor that reflection has foundered, has sunk to the ground,\nand in the unity of the latter\nthe determinations are only posited determinations.\nThis mediation of the ground is thus\nthe unity of pure reflection and determining reflection;\ntheir determinations or that which is posited has self-subsistence,\nand conversely the self-subsistence of\nthe determinations is a posited subsistence.\nSince this subsistence of the determinations is\nitself posited or has determinateness,\nthe determinations are consequently distinguished\nfrom their simple identity,\nand they constitute the form as against essence.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-2'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-2'
SET topic.title = 'Mediation of ground — unity of pure and determining reflection'
SET topic.description = 'Mediation compared with preceding reflections. Not pure reflection: undistinguished from essence, no negative, no self-subsistence. Not determining reflection: determinations have essential self-subsistence, but reflection foundered, sunk to ground. In unity of ground, determinations are only posited determinations. Mediation of ground is unity of pure reflection and determining reflection. Determinations or posited have self-subsistence. Self-subsistence of determinations is posited subsistence. Since subsistence is itself posited or has determinateness, determinations distinguished from simple identity. They constitute form as against essence.'
SET topic.keyPoints = ['Not pure reflection: no negative, no self-subsistence', 'Not determining reflection: foundered, sunk to ground', 'Mediation of ground is unity of pure reflection and determining reflection', 'Self-subsistence of determinations is posited subsistence', 'Determinations constitute form as against essence'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-2'})
MATCH (topic:Topic {id: 'topic:abs-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-2'})
SET c.title = 'Mediation of ground — unity of pure and determining reflection'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 45
SET c.lineEnd = 69
SET c.description = 'Mediation compared with preceding reflections. Not pure reflection: undistinguished from essence, no negative, no self-subsistence. Not determining reflection: determinations have essential self-subsistence, but reflection foundered, sunk to ground. In unity of ground, determinations are only posited determinations. Mediation of ground is unity of pure reflection and determining reflection. Determinations or posited have self-subsistence. Self-subsistence of determinations is posited subsistence. Since subsistence is itself posited or has determinateness, determinations distinguished from simple identity. They constitute form as against essence.'
SET c.keyPoints = ['Not pure reflection: no negative, no self-subsistence', 'Not determining reflection: foundered, sunk to ground', 'Mediation of ground is unity of pure reflection and determining reflection', 'Self-subsistence of determinations is posited subsistence', 'Determinations constitute form as against essence']
SET c.tags = ['negation', 'reflection', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 88
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'For one thing, this mediation,\ncompared with the preceding reflections\nfrom which it derives,\nis not pure reflection,\nwhich is undistinguished from essence\nand still does not have the negative in it,\nconsequently also does not as yet contain\nthe self-subsistence of the determinations.\nThese have their subsistence, rather,\nin the ground understood as sublated reflection.\nAnd it is also not the determining reflection\nwhose determinations have essential self-subsistence,\nfor that reflection has foundered, has sunk to the ground,\nand in the unity of the latter\nthe determinations are only posited determinations.\nThis mediation of the ground is thus\nthe unity of pure reflection and determining reflection;\ntheir determinations or that which is posited has self-subsistence,\nand conversely the self-subsistence of\nthe determinations is a posited subsistence.\nSince this subsistence of the determinations is\nitself posited or has determinateness,\nthe determinations are consequently distinguished\nfrom their simple identity,\nand they constitute the form as against essence.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-2'})
MATCH (c:IntegratedChunk {id: 'abs-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-2:kp:1'})
SET kp.chunkId = 'abs-2'
SET kp.ordinal = 1
SET kp.text = 'Not pure reflection: no negative, no self-subsistence';
MATCH (c:IntegratedChunk {id: 'abs-2'})
MATCH (kp:KeyPoint {id: 'abs-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-2:kp:2'})
SET kp.chunkId = 'abs-2'
SET kp.ordinal = 2
SET kp.text = 'Not determining reflection: foundered, sunk to ground';
MATCH (c:IntegratedChunk {id: 'abs-2'})
MATCH (kp:KeyPoint {id: 'abs-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-2:kp:3'})
SET kp.chunkId = 'abs-2'
SET kp.ordinal = 3
SET kp.text = 'Mediation of ground is unity of pure reflection and determining reflection';
MATCH (c:IntegratedChunk {id: 'abs-2'})
MATCH (kp:KeyPoint {id: 'abs-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-2:kp:4'})
SET kp.chunkId = 'abs-2'
SET kp.ordinal = 4
SET kp.text = 'Self-subsistence of determinations is posited subsistence';
MATCH (c:IntegratedChunk {id: 'abs-2'})
MATCH (kp:KeyPoint {id: 'abs-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-2:kp:5'})
SET kp.chunkId = 'abs-2'
SET kp.ordinal = 5
SET kp.text = 'Determinations constitute form as against essence';
MATCH (c:IntegratedChunk {id: 'abs-2'})
MATCH (kp:KeyPoint {id: 'abs-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-3'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 71
SET segment.lineEnd = 101
SET segment.text = 'Essence has a form and determinations of this form.\nOnly as ground does it have a fixed immediacy or is substrate.\nEssence as such is one with its reflection,\ninseparable from its movement.\nIt is not essence, therefore, through which\nthis movement runs its reflective course;\nnor is essence that from which the movement begins,\nas from a starting point.\nIt is this circumstance that above all makes\nthe exposition of reflection especially difficult,\nfor strictly speaking one cannot say\nthat essence returns into itself,\nthat essence shines in itself,\nfor essence is neither before its movement nor in the movement:\nthis movement has no substrate on which it runs its course.\nA term of reference arises in the ground only following upon the\nmoment of sublated reflection.\nBut essence as the referred-to term is determinate essence,\nand by virtue of this positedness it has form as essence.\nThe determinations of form, on the contrary,\nare now determinations in the essence;\nthe latter lies at their foundation\nas an indeterminate which in its determination\nis indifferent to them;\nin it, they are reflected into themselves.\nThe determinations of reflection should have\ntheir subsistence in them and be self-subsistent.\nBut their self-subsistence is their dissolution,\nwhich they thus have in an other;\nbut this dissolution is itself this self-identity\nor the ground of the subsistence that they give to themselves.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-3'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-3'
SET topic.title = 'Essence has form — substrate and movement'
SET topic.description = 'Essence has form and determinations of this form. Only as ground does it have fixed immediacy or is substrate. Essence as such is one with its reflection, inseparable from its movement. Not essence through which movement runs its reflective course. Nor essence that from which movement begins. Cannot say essence returns into itself, essence shines in itself. Essence is neither before its movement nor in the movement. Movement has no substrate on which it runs its course. Term of reference arises in ground only following moment of sublated reflection. Essence as referred-to term is determinate essence. By virtue of positedness it has form as essence. Determinations of form are determinations in essence. Essence lies at their foundation as indeterminate, indifferent to them.'
SET topic.keyPoints = ['Only as ground does essence have fixed immediacy or is substrate', 'Essence is one with its reflection, inseparable from its movement', 'Essence is neither before its movement nor in the movement', 'Term of reference arises in ground only following moment of sublated reflection', 'Essence lies at foundation as indeterminate, indifferent to determinations'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-3'})
MATCH (topic:Topic {id: 'topic:abs-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-3'})
SET c.title = 'Essence has form — substrate and movement'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 71
SET c.lineEnd = 101
SET c.description = 'Essence has form and determinations of this form. Only as ground does it have fixed immediacy or is substrate. Essence as such is one with its reflection, inseparable from its movement. Not essence through which movement runs its reflective course. Nor essence that from which movement begins. Cannot say essence returns into itself, essence shines in itself. Essence is neither before its movement nor in the movement. Movement has no substrate on which it runs its course. Term of reference arises in ground only following moment of sublated reflection. Essence as referred-to term is determinate essence. By virtue of positedness it has form as essence. Determinations of form are determinations in essence. Essence lies at their foundation as indeterminate, indifferent to them.'
SET c.keyPoints = ['Only as ground does essence have fixed immediacy or is substrate', 'Essence is one with its reflection, inseparable from its movement', 'Essence is neither before its movement nor in the movement', 'Term of reference arises in ground only following moment of sublated reflection', 'Essence lies at foundation as indeterminate, indifferent to determinations']
SET c.tags = ['sublation', 'reflection', 'shine', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 89
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Essence has a form and determinations of this form.\nOnly as ground does it have a fixed immediacy or is substrate.\nEssence as such is one with its reflection,\ninseparable from its movement.\nIt is not essence, therefore, through which\nthis movement runs its reflective course;\nnor is essence that from which the movement begins,\nas from a starting point.\nIt is this circumstance that above all makes\nthe exposition of reflection especially difficult,\nfor strictly speaking one cannot say\nthat essence returns into itself,\nthat essence shines in itself,\nfor essence is neither before its movement nor in the movement:\nthis movement has no substrate on which it runs its course.\nA term of reference arises in the ground only following upon the\nmoment of sublated reflection.\nBut essence as the referred-to term is determinate essence,\nand by virtue of this positedness it has form as essence.\nThe determinations of form, on the contrary,\nare now determinations in the essence;\nthe latter lies at their foundation\nas an indeterminate which in its determination\nis indifferent to them;\nin it, they are reflected into themselves.\nThe determinations of reflection should have\ntheir subsistence in them and be self-subsistent.\nBut their self-subsistence is their dissolution,\nwhich they thus have in an other;\nbut this dissolution is itself this self-identity\nor the ground of the subsistence that they give to themselves.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-3'})
MATCH (c:IntegratedChunk {id: 'abs-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-3:kp:1'})
SET kp.chunkId = 'abs-3'
SET kp.ordinal = 1
SET kp.text = 'Only as ground does essence have fixed immediacy or is substrate';
MATCH (c:IntegratedChunk {id: 'abs-3'})
MATCH (kp:KeyPoint {id: 'abs-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-3:kp:2'})
SET kp.chunkId = 'abs-3'
SET kp.ordinal = 2
SET kp.text = 'Essence is one with its reflection, inseparable from its movement';
MATCH (c:IntegratedChunk {id: 'abs-3'})
MATCH (kp:KeyPoint {id: 'abs-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-3:kp:3'})
SET kp.chunkId = 'abs-3'
SET kp.ordinal = 3
SET kp.text = 'Essence is neither before its movement nor in the movement';
MATCH (c:IntegratedChunk {id: 'abs-3'})
MATCH (kp:KeyPoint {id: 'abs-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-3:kp:4'})
SET kp.chunkId = 'abs-3'
SET kp.ordinal = 4
SET kp.text = 'Term of reference arises in ground only following moment of sublated reflection';
MATCH (c:IntegratedChunk {id: 'abs-3'})
MATCH (kp:KeyPoint {id: 'abs-3:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-3:kp:5'})
SET kp.chunkId = 'abs-3'
SET kp.ordinal = 5
SET kp.text = 'Essence lies at foundation as indeterminate, indifferent to determinations';
MATCH (c:IntegratedChunk {id: 'abs-3'})
MATCH (kp:KeyPoint {id: 'abs-3:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-4'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 103
SET segment.lineEnd = 137
SET segment.text = 'Everything determinate belongs in general to form;\nit is a form determination inasmuch as it is something posited\nand hence distinguished from that of which it is the form.\nAs quality, determinateness is one with its substrate, being;\nbeing is the immediate determinate,\nnot yet distinct from its determinateness\nor, in this determinateness,\nstill unreflected into itself,\njust as the determinateness is, therefore,\nan existent determinateness,\nnot yet one that is posited.\nMoreover, the form determinations of essence are,\nin their more specific determinateness,\nthe previously considered moments of reflections:\nidentity and difference, the latter as both\ndiversity and opposition.\nBut also the ground-connection belongs among\nthese form determinations of essence,\nbecause through it, though itself the\nsublated determination of reflection,\nessence is at the same time as posited.\nBy contrast, the identity that has the ground immanent in it\ndoes not pertain to form, because positedness,\nas sublated and as such (as ground and grounded),\nis one reflection, and this reflection constitutes\nessence as simple substrate which is the subsistence of form.\nBut in ground this subsistence is posited,\nor this essence is itself essentially as determinate and, consequently,\nis in turn also the moment of the ground-connection and form.\nThis is the absolute reciprocal connecting reference of form and essence:\nessence is the simple unity of ground and grounded\nbut, in this unity, is itself determined, or is a negative,\nand it distinguishes itself as substrate from form,\nbut at the same time it thereby becomes itself\nground and moment of form.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-4'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-4'
SET topic.title = 'Form determinations — everything determinate belongs to form'
SET topic.description = 'Everything determinate belongs in general to form. Form determination is something posited, distinguished from that of which it is form. Form determinations of essence are previously considered moments of reflections: identity and difference, latter as diversity and opposition. Ground-connection belongs among form determinations. Identity that has ground immanent in it does not pertain to form. Positedness as sublated (as ground and grounded) is one reflection. This reflection constitutes essence as simple substrate which is subsistence of form. But in ground this subsistence is posited. Essence is itself essentially as determinate, is moment of ground-connection and form. Absolute reciprocal connecting reference of form and essence.'
SET topic.keyPoints = ['Everything determinate belongs in general to form', 'Form determinations are previously considered moments of reflections', 'Ground-connection belongs among form determinations', 'Essence is simple substrate which is subsistence of form', 'Absolute reciprocal connecting reference of form and essence'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-4'})
MATCH (topic:Topic {id: 'topic:abs-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-4'})
SET c.title = 'Form determinations — everything determinate belongs to form'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 103
SET c.lineEnd = 137
SET c.description = 'Everything determinate belongs in general to form. Form determination is something posited, distinguished from that of which it is form. Form determinations of essence are previously considered moments of reflections: identity and difference, latter as diversity and opposition. Ground-connection belongs among form determinations. Identity that has ground immanent in it does not pertain to form. Positedness as sublated (as ground and grounded) is one reflection. This reflection constitutes essence as simple substrate which is subsistence of form. But in ground this subsistence is posited. Essence is itself essentially as determinate, is moment of ground-connection and form. Absolute reciprocal connecting reference of form and essence.'
SET c.keyPoints = ['Everything determinate belongs in general to form', 'Form determinations are previously considered moments of reflections', 'Ground-connection belongs among form determinations', 'Essence is simple substrate which is subsistence of form', 'Absolute reciprocal connecting reference of form and essence']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 90
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Everything determinate belongs in general to form;\nit is a form determination inasmuch as it is something posited\nand hence distinguished from that of which it is the form.\nAs quality, determinateness is one with its substrate, being;\nbeing is the immediate determinate,\nnot yet distinct from its determinateness\nor, in this determinateness,\nstill unreflected into itself,\njust as the determinateness is, therefore,\nan existent determinateness,\nnot yet one that is posited.\nMoreover, the form determinations of essence are,\nin their more specific determinateness,\nthe previously considered moments of reflections:\nidentity and difference, the latter as both\ndiversity and opposition.\nBut also the ground-connection belongs among\nthese form determinations of essence,\nbecause through it, though itself the\nsublated determination of reflection,\nessence is at the same time as posited.\nBy contrast, the identity that has the ground immanent in it\ndoes not pertain to form, because positedness,\nas sublated and as such (as ground and grounded),\nis one reflection, and this reflection constitutes\nessence as simple substrate which is the subsistence of form.\nBut in ground this subsistence is posited,\nor this essence is itself essentially as determinate and, consequently,\nis in turn also the moment of the ground-connection and form.\nThis is the absolute reciprocal connecting reference of form and essence:\nessence is the simple unity of ground and grounded\nbut, in this unity, is itself determined, or is a negative,\nand it distinguishes itself as substrate from form,\nbut at the same time it thereby becomes itself\nground and moment of form.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-4'})
MATCH (c:IntegratedChunk {id: 'abs-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-4:kp:1'})
SET kp.chunkId = 'abs-4'
SET kp.ordinal = 1
SET kp.text = 'Everything determinate belongs in general to form';
MATCH (c:IntegratedChunk {id: 'abs-4'})
MATCH (kp:KeyPoint {id: 'abs-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-4:kp:2'})
SET kp.chunkId = 'abs-4'
SET kp.ordinal = 2
SET kp.text = 'Form determinations are previously considered moments of reflections';
MATCH (c:IntegratedChunk {id: 'abs-4'})
MATCH (kp:KeyPoint {id: 'abs-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-4:kp:3'})
SET kp.chunkId = 'abs-4'
SET kp.ordinal = 3
SET kp.text = 'Ground-connection belongs among form determinations';
MATCH (c:IntegratedChunk {id: 'abs-4'})
MATCH (kp:KeyPoint {id: 'abs-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-4:kp:4'})
SET kp.chunkId = 'abs-4'
SET kp.ordinal = 4
SET kp.text = 'Essence is simple substrate which is subsistence of form';
MATCH (c:IntegratedChunk {id: 'abs-4'})
MATCH (kp:KeyPoint {id: 'abs-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-4:kp:5'})
SET kp.chunkId = 'abs-4'
SET kp.ordinal = 5
SET kp.text = 'Absolute reciprocal connecting reference of form and essence';
MATCH (c:IntegratedChunk {id: 'abs-4'})
MATCH (kp:KeyPoint {id: 'abs-4:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-5'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 139
SET segment.lineEnd = 189
SET segment.text = 'Form is therefore the completed whole of reflection;\nit also contains this determination of reflection, that it is sublated;\njust like reflection, therefore, it is one unity of its determining,\nand it is also referred to its sublatedness,\nto another that is not itself form but in which the form is.\nAs essential self-referring negativity,\nin contrast with that simple negative,\nform is positing and determining;\nsimple essence, on the contrary, is indeterminate and inert\nsubstrate in which the determinations of form have their subsistence\nor their reflection into themselves.\nExternal reflection normally halts at\nthis distinction of essence and form;\nthe distinction is necessary,\nbut the distinguishing itself of the two is their unity,\njust as this unity of ground is essence repelling itself from itself\nand making itself into positedness.\nForm is absolute negativity itself\nor the negative absolute self-identity\nby virtue of which essence is indeed not being but essence.\nThis identity, taken abstractly, is essence as against form,\njust as negativity, taken abstractly as positedness,\nis the one determination of form.\nBut this determination has shown itself to be in truth\nthe whole self-referring negativity\nwhich within, as this identity, thus is simple essence.\nConsequently, form has essence in its own identity,\njust as essence has absolute form in its negative nature.\nOne cannot therefore ask, how form comes to essence,\nfor form is only the internal reflective shining of essence,\nits own reflection inhabiting it.\nForm equally is, within it,\nthe reflection turning back into itself\nor the identical essence;\nin its determining, form makes the determination\ninto positedness as positedness.\n\nForm, therefore, does not determine essence,\nas if it were truly presupposed, separate from essence,\nfor it would then be the unessential,\nconstantly foundering determination of reflection;\nhere it rather is itself the ground of its sublating\nor the identical reference of its determinations.\nThat the form determines the essence means, therefore,\nthat in its distinguishing form sublates this very distinguishing\nand is the self-identity that essence is\nas the subsistence of the determinations;\nform is the contradiction of being sublated in its positedness\nand yet having subsistence in this sublatedness;\nit is accordingly ground as essence which\nis self-identical in being determined or negated.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-5'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-5'
SET topic.title = 'Form as completed whole — absolute negativity'
SET topic.description = 'Form is completed whole of reflection. Contains determination of reflection that it is sublated. Referred to its sublatedness, to another that is not itself form but in which form is. As essential self-referring negativity, form is positing and determining. Simple essence is indeterminate and inert substrate. Form is absolute negativity itself or negative absolute self-identity. By virtue of which essence is not being but essence. Form has essence in its own identity, essence has absolute form in its negative nature. Cannot ask how form comes to essence. Form is only internal reflective shining of essence, its own reflection inhabiting it. Form does not determine essence as if truly presupposed, separate from essence. Form is itself ground of its sublating or identical reference of its determinations. Form is contradiction of being sublated in positedness and yet having subsistence in sublatedness.'
SET topic.keyPoints = ['Form is completed whole of reflection', 'Form is absolute negativity itself or negative absolute self-identity', 'Form is only internal reflective shining of essence', 'Form is itself ground of its sublating', 'Form is contradiction of being sublated in positedness and yet having subsistence'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-5'})
MATCH (topic:Topic {id: 'topic:abs-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-5'})
SET c.title = 'Form as completed whole — absolute negativity'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 139
SET c.lineEnd = 189
SET c.description = 'Form is completed whole of reflection. Contains determination of reflection that it is sublated. Referred to its sublatedness, to another that is not itself form but in which form is. As essential self-referring negativity, form is positing and determining. Simple essence is indeterminate and inert substrate. Form is absolute negativity itself or negative absolute self-identity. By virtue of which essence is not being but essence. Form has essence in its own identity, essence has absolute form in its negative nature. Cannot ask how form comes to essence. Form is only internal reflective shining of essence, its own reflection inhabiting it. Form does not determine essence as if truly presupposed, separate from essence. Form is itself ground of its sublating or identical reference of its determinations. Form is contradiction of being sublated in positedness and yet having subsistence in sublatedness.'
SET c.keyPoints = ['Form is completed whole of reflection', 'Form is absolute negativity itself or negative absolute self-identity', 'Form is only internal reflective shining of essence', 'Form is itself ground of its sublating', 'Form is contradiction of being sublated in positedness and yet having subsistence']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 5
SET c.globalOrder = 91
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Form is therefore the completed whole of reflection;\nit also contains this determination of reflection, that it is sublated;\njust like reflection, therefore, it is one unity of its determining,\nand it is also referred to its sublatedness,\nto another that is not itself form but in which the form is.\nAs essential self-referring negativity,\nin contrast with that simple negative,\nform is positing and determining;\nsimple essence, on the contrary, is indeterminate and inert\nsubstrate in which the determinations of form have their subsistence\nor their reflection into themselves.\nExternal reflection normally halts at\nthis distinction of essence and form;\nthe distinction is necessary,\nbut the distinguishing itself of the two is their unity,\njust as this unity of ground is essence repelling itself from itself\nand making itself into positedness.\nForm is absolute negativity itself\nor the negative absolute self-identity\nby virtue of which essence is indeed not being but essence.\nThis identity, taken abstractly, is essence as against form,\njust as negativity, taken abstractly as positedness,\nis the one determination of form.\nBut this determination has shown itself to be in truth\nthe whole self-referring negativity\nwhich within, as this identity, thus is simple essence.\nConsequently, form has essence in its own identity,\njust as essence has absolute form in its negative nature.\nOne cannot therefore ask, how form comes to essence,\nfor form is only the internal reflective shining of essence,\nits own reflection inhabiting it.\nForm equally is, within it,\nthe reflection turning back into itself\nor the identical essence;\nin its determining, form makes the determination\ninto positedness as positedness.\n\nForm, therefore, does not determine essence,\nas if it were truly presupposed, separate from essence,\nfor it would then be the unessential,\nconstantly foundering determination of reflection;\nhere it rather is itself the ground of its sublating\nor the identical reference of its determinations.\nThat the form determines the essence means, therefore,\nthat in its distinguishing form sublates this very distinguishing\nand is the self-identity that essence is\nas the subsistence of the determinations;\nform is the contradiction of being sublated in its positedness\nand yet having subsistence in this sublatedness;\nit is accordingly ground as essence which\nis self-identical in being determined or negated.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-5'})
MATCH (c:IntegratedChunk {id: 'abs-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-5:kp:1'})
SET kp.chunkId = 'abs-5'
SET kp.ordinal = 1
SET kp.text = 'Form is completed whole of reflection';
MATCH (c:IntegratedChunk {id: 'abs-5'})
MATCH (kp:KeyPoint {id: 'abs-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-5:kp:2'})
SET kp.chunkId = 'abs-5'
SET kp.ordinal = 2
SET kp.text = 'Form is absolute negativity itself or negative absolute self-identity';
MATCH (c:IntegratedChunk {id: 'abs-5'})
MATCH (kp:KeyPoint {id: 'abs-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-5:kp:3'})
SET kp.chunkId = 'abs-5'
SET kp.ordinal = 3
SET kp.text = 'Form is only internal reflective shining of essence';
MATCH (c:IntegratedChunk {id: 'abs-5'})
MATCH (kp:KeyPoint {id: 'abs-5:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-5:kp:4'})
SET kp.chunkId = 'abs-5'
SET kp.ordinal = 4
SET kp.text = 'Form is itself ground of its sublating';
MATCH (c:IntegratedChunk {id: 'abs-5'})
MATCH (kp:KeyPoint {id: 'abs-5:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-5:kp:5'})
SET kp.chunkId = 'abs-5'
SET kp.ordinal = 5
SET kp.text = 'Form is contradiction of being sublated in positedness and yet having subsistence';
MATCH (c:IntegratedChunk {id: 'abs-5'})
MATCH (kp:KeyPoint {id: 'abs-5:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-6'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 191
SET segment.lineEnd = 202
SET segment.text = 'These distinctions, of form and of essence,\nare therefore only moments of the simple reference of form itself.\nBut they must be examined and fixed more closely.\nDetermining form refers itself to itself as sublated positedness;\nit thereby refers itself to its identity as to another.\nIt posits itself as sublated;\nit therefore presupposes its identity;\naccording to this moment, essence is the indeterminate\nto which form is an other.\nIt is not the essence which is absolute reflection within,\nbut essence determined as formless identity:\nit is matter.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-6'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-6'
SET topic.title = 'Form determines essence — matter'
SET topic.description = 'Distinctions of form and essence are only moments of simple reference of form itself. Determining form refers itself to itself as sublated positedness. Refers itself to its identity as to another. Posits itself as sublated, therefore presupposes its identity. According to this moment, essence is indeterminate to which form is other. Not essence which is absolute reflection within. But essence determined as formless identity: it is matter.'
SET topic.keyPoints = ['Distinctions of form and essence are only moments of simple reference of form itself', 'Determining form refers itself to its identity as to another', 'Essence is indeterminate to which form is other', 'Essence determined as formless identity: it is matter'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-6'})
MATCH (topic:Topic {id: 'topic:abs-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-6'})
SET c.title = 'Form determines essence — matter'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 191
SET c.lineEnd = 202
SET c.description = 'Distinctions of form and essence are only moments of simple reference of form itself. Determining form refers itself to itself as sublated positedness. Refers itself to its identity as to another. Posits itself as sublated, therefore presupposes its identity. According to this moment, essence is indeterminate to which form is other. Not essence which is absolute reflection within. But essence determined as formless identity: it is matter.'
SET c.keyPoints = ['Distinctions of form and essence are only moments of simple reference of form itself', 'Determining form refers itself to its identity as to another', 'Essence is indeterminate to which form is other', 'Essence determined as formless identity: it is matter']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 6
SET c.globalOrder = 92
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'These distinctions, of form and of essence,\nare therefore only moments of the simple reference of form itself.\nBut they must be examined and fixed more closely.\nDetermining form refers itself to itself as sublated positedness;\nit thereby refers itself to its identity as to another.\nIt posits itself as sublated;\nit therefore presupposes its identity;\naccording to this moment, essence is the indeterminate\nto which form is an other.\nIt is not the essence which is absolute reflection within,\nbut essence determined as formless identity:\nit is matter.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-6'})
MATCH (c:IntegratedChunk {id: 'abs-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-6:kp:1'})
SET kp.chunkId = 'abs-6'
SET kp.ordinal = 1
SET kp.text = 'Distinctions of form and essence are only moments of simple reference of form itself';
MATCH (c:IntegratedChunk {id: 'abs-6'})
MATCH (kp:KeyPoint {id: 'abs-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-6:kp:2'})
SET kp.chunkId = 'abs-6'
SET kp.ordinal = 2
SET kp.text = 'Determining form refers itself to its identity as to another';
MATCH (c:IntegratedChunk {id: 'abs-6'})
MATCH (kp:KeyPoint {id: 'abs-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-6:kp:3'})
SET kp.chunkId = 'abs-6'
SET kp.ordinal = 3
SET kp.text = 'Essence is indeterminate to which form is other';
MATCH (c:IntegratedChunk {id: 'abs-6'})
MATCH (kp:KeyPoint {id: 'abs-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-6:kp:4'})
SET kp.chunkId = 'abs-6'
SET kp.ordinal = 4
SET kp.text = 'Essence determined as formless identity: it is matter';
MATCH (c:IntegratedChunk {id: 'abs-6'})
MATCH (kp:KeyPoint {id: 'abs-6:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-7'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 206
SET segment.lineEnd = 228
SET segment.text = 'Essence becomes matter in that its reflection is\ndetermined as relating itself\nto essence as to the formless indeterminate.\nMatter, therefore, is the simple identity,\nvoid of distinction, that essence is,\nwith the determination that it is the other of form.\nHence it is the proper base or substrate of form,\nsince it constitutes the immanent reflection\nof the determinations of form,\nor the self-subsistent term,\nto which such determinations refer\nas to their positive subsistence.\n\nIf abstraction is made from every determination,\nfrom every form of a something, matter is what is left over.\nMatter is the absolutely abstract.\n(One cannot see, feel, etc. matter;\nwhat one sees or feels is a determinate matter,\nthat is, a unity of matter and form.)\nThis abstraction from which matter derives is not, however,\nan external removal and sublation of form;\nit is rather the form itself which, as we have just seen,\nreduces itself by virtue of itself to this simple identity.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-7'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-7'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-7'
SET topic.title = 'b. Form and matter — essence becomes matter'
SET topic.description = 'Essence becomes matter in that its reflection is determined as relating itself to essence as to formless indeterminate. Matter is simple identity, void of distinction, that essence is. With determination that it is other of form. Proper base or substrate of form. Constitutes immanent reflection of determinations of form. Self-subsistent term to which determinations refer as to their positive subsistence. If abstraction made from every determination, from every form, matter is what is left over. Matter is absolutely abstract. Abstraction from which matter derives is not external removal and sublation of form. Form itself reduces itself by virtue of itself to this simple identity.'
SET topic.keyPoints = ['Essence becomes matter in that its reflection relates itself to essence as to formless indeterminate', 'Matter is simple identity, void of distinction, other of form', 'Matter is absolutely abstract', 'Abstraction from which matter derives is not external removal of form', 'Form itself reduces itself to this simple identity'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-7'})
MATCH (topic:Topic {id: 'topic:abs-7'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-7'})
SET c.title = 'b. Form and matter — essence becomes matter'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 206
SET c.lineEnd = 228
SET c.description = 'Essence becomes matter in that its reflection is determined as relating itself to essence as to formless indeterminate. Matter is simple identity, void of distinction, that essence is. With determination that it is other of form. Proper base or substrate of form. Constitutes immanent reflection of determinations of form. Self-subsistent term to which determinations refer as to their positive subsistence. If abstraction made from every determination, from every form, matter is what is left over. Matter is absolutely abstract. Abstraction from which matter derives is not external removal and sublation of form. Form itself reduces itself by virtue of itself to this simple identity.'
SET c.keyPoints = ['Essence becomes matter in that its reflection relates itself to essence as to formless indeterminate', 'Matter is simple identity, void of distinction, other of form', 'Matter is absolutely abstract', 'Abstraction from which matter derives is not external removal of form', 'Form itself reduces itself to this simple identity']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 7
SET c.globalOrder = 93
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Essence becomes matter in that its reflection is\ndetermined as relating itself\nto essence as to the formless indeterminate.\nMatter, therefore, is the simple identity,\nvoid of distinction, that essence is,\nwith the determination that it is the other of form.\nHence it is the proper base or substrate of form,\nsince it constitutes the immanent reflection\nof the determinations of form,\nor the self-subsistent term,\nto which such determinations refer\nas to their positive subsistence.\n\nIf abstraction is made from every determination,\nfrom every form of a something, matter is what is left over.\nMatter is the absolutely abstract.\n(One cannot see, feel, etc. matter;\nwhat one sees or feels is a determinate matter,\nthat is, a unity of matter and form.)\nThis abstraction from which matter derives is not, however,\nan external removal and sublation of form;\nit is rather the form itself which, as we have just seen,\nreduces itself by virtue of itself to this simple identity.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-7'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-7'})
MATCH (c:IntegratedChunk {id: 'abs-7'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-7'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-7'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-7:kp:1'})
SET kp.chunkId = 'abs-7'
SET kp.ordinal = 1
SET kp.text = 'Essence becomes matter in that its reflection relates itself to essence as to formless indeterminate';
MATCH (c:IntegratedChunk {id: 'abs-7'})
MATCH (kp:KeyPoint {id: 'abs-7:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-7:kp:2'})
SET kp.chunkId = 'abs-7'
SET kp.ordinal = 2
SET kp.text = 'Matter is simple identity, void of distinction, other of form';
MATCH (c:IntegratedChunk {id: 'abs-7'})
MATCH (kp:KeyPoint {id: 'abs-7:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-7:kp:3'})
SET kp.chunkId = 'abs-7'
SET kp.ordinal = 3
SET kp.text = 'Matter is absolutely abstract';
MATCH (c:IntegratedChunk {id: 'abs-7'})
MATCH (kp:KeyPoint {id: 'abs-7:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-7:kp:4'})
SET kp.chunkId = 'abs-7'
SET kp.ordinal = 4
SET kp.text = 'Abstraction from which matter derives is not external removal of form';
MATCH (c:IntegratedChunk {id: 'abs-7'})
MATCH (kp:KeyPoint {id: 'abs-7:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-7:kp:5'})
SET kp.chunkId = 'abs-7'
SET kp.ordinal = 5
SET kp.text = 'Form itself reduces itself to this simple identity';
MATCH (c:IntegratedChunk {id: 'abs-7'})
MATCH (kp:KeyPoint {id: 'abs-7:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-8'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 230
SET segment.lineEnd = 296
SET segment.text = 'Further, form presupposes a matter to which it refers.\nBut for this reason the two do not find themselves\nconfronting each other externally and accidentally;\nneither matter nor form derives from itself, is a se,\nor, in other words, is eternal.\nMatter is indifferent with respect to form,\nbut this indifference is the determinateness\nof self-identity to which\nform returns as to its substrate.\nForm presupposes matter for the very reason\nthat it posits itself as a sublated,\nhence refers to this, its identity,\nas to something other.\nContrariwise, form is presupposed by matter;\nfor matter is not simple essence,\nwhich immediately is itself absolute reflection,\nbut is essence determined as something positive,\nthat is to say, which only is as sublated negation.\nBut, on the other hand, since form posits itself\nas matter only in sublating itself,\nhence in presupposing matter,\nmatter is also determined as groundless subsistence.\nEqually so, matter is not determined as the ground of form;\nbut rather, inasmuch as matter posits itself as\nthe abstract identity of the sublated determination of form,\nit is not that identity as ground,\nand form is therefore groundless with respect to it.\nForm and matter are consequently alike determined as\nnot to be posited each by the other,\neach not to be the ground of the other.\nMatter is rather the identity of\nthe ground and the grounded,\nas the substrate that stands over\nagainst this reference of form.\nThis determination of indifference that\nthe two have in common is\nthe determination of matter as such\nand also constitutes their reciprocal reference.\nThe determination of form, that it is\nthe connection of the two as distinct,\nequally is also the other moment of\nthe relating of the two to each other.\nMatter, determined as indifferent,\nis the passive as contrasted to form,\nwhich is determined as the active.\nThis latter, as self-referring negative,\nis inherently contradiction, self-dissolving,\nself-repelling, and self-determining.\nIt refers to matter, and it is posited to refer to this matter,\nwhich is its subsistence, as to another.\nMatter is posited, on the contrary,\nas referring only to itself\nand as indifferent to the other;\nbut, implicitly, it does refer to the form,\nfor it contains the sublated negativity\nand is matter only by virtue of this determination.\nIt refers to it as an other only\nbecause form is not posited in it,\nbecause it is form only implicitly.\nIt contains form locked up inside it,\nand it is an absolute receptivity for form\nonly because it has the latter within it absolutely,\nbecause to be form is its implicit vocation.\nHence matter must be informed,\nand form must materialize itself;\nit must give itself self-identity\nor subsistence in matter.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-8'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-8'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-8'
SET topic.title = 'Form and matter presuppose each other — reciprocal reference'
SET topic.description = 'Form presupposes matter to which it refers. Two do not find themselves confronting each other externally and accidentally. Neither matter nor form derives from itself, is a se, is eternal. Matter is indifferent with respect to form. Form presupposes matter because posits itself as sublated, refers to its identity as to something other. Matter is not simple essence but essence determined as something positive, only is as sublated negation. Matter is determined as groundless subsistence. Matter is not determined as ground of form. Form and matter alike determined as not to be posited each by other, each not to be ground of other. Matter is identity of ground and grounded, as substrate. Matter, determined as indifferent, is passive as contrasted to form, which is active. Form contains form locked up inside it. Matter must be informed, form must materialize itself.'
SET topic.keyPoints = ['Form and matter presuppose each other', 'Neither matter nor form derives from itself, is eternal', 'Matter is not determined as ground of form', 'Matter is passive, form is active', 'Matter contains form locked up inside it, must be informed'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-8'})
MATCH (topic:Topic {id: 'topic:abs-8'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-8'})
SET c.title = 'Form and matter presuppose each other — reciprocal reference'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 230
SET c.lineEnd = 296
SET c.description = 'Form presupposes matter to which it refers. Two do not find themselves confronting each other externally and accidentally. Neither matter nor form derives from itself, is a se, is eternal. Matter is indifferent with respect to form. Form presupposes matter because posits itself as sublated, refers to its identity as to something other. Matter is not simple essence but essence determined as something positive, only is as sublated negation. Matter is determined as groundless subsistence. Matter is not determined as ground of form. Form and matter alike determined as not to be posited each by other, each not to be ground of other. Matter is identity of ground and grounded, as substrate. Matter, determined as indifferent, is passive as contrasted to form, which is active. Form contains form locked up inside it. Matter must be informed, form must materialize itself.'
SET c.keyPoints = ['Form and matter presuppose each other', 'Neither matter nor form derives from itself, is eternal', 'Matter is not determined as ground of form', 'Matter is passive, form is active', 'Matter contains form locked up inside it, must be informed']
SET c.tags = ['negation', 'sublation', 'citta']
SET c.orderInSource = 8
SET c.globalOrder = 94
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Further, form presupposes a matter to which it refers.\nBut for this reason the two do not find themselves\nconfronting each other externally and accidentally;\nneither matter nor form derives from itself, is a se,\nor, in other words, is eternal.\nMatter is indifferent with respect to form,\nbut this indifference is the determinateness\nof self-identity to which\nform returns as to its substrate.\nForm presupposes matter for the very reason\nthat it posits itself as a sublated,\nhence refers to this, its identity,\nas to something other.\nContrariwise, form is presupposed by matter;\nfor matter is not simple essence,\nwhich immediately is itself absolute reflection,\nbut is essence determined as something positive,\nthat is to say, which only is as sublated negation.\nBut, on the other hand, since form posits itself\nas matter only in sublating itself,\nhence in presupposing matter,\nmatter is also determined as groundless subsistence.\nEqually so, matter is not determined as the ground of form;\nbut rather, inasmuch as matter posits itself as\nthe abstract identity of the sublated determination of form,\nit is not that identity as ground,\nand form is therefore groundless with respect to it.\nForm and matter are consequently alike determined as\nnot to be posited each by the other,\neach not to be the ground of the other.\nMatter is rather the identity of\nthe ground and the grounded,\nas the substrate that stands over\nagainst this reference of form.\nThis determination of indifference that\nthe two have in common is\nthe determination of matter as such\nand also constitutes their reciprocal reference.\nThe determination of form, that it is\nthe connection of the two as distinct,\nequally is also the other moment of\nthe relating of the two to each other.\nMatter, determined as indifferent,\nis the passive as contrasted to form,\nwhich is determined as the active.\nThis latter, as self-referring negative,\nis inherently contradiction, self-dissolving,\nself-repelling, and self-determining.\nIt refers to matter, and it is posited to refer to this matter,\nwhich is its subsistence, as to another.\nMatter is posited, on the contrary,\nas referring only to itself\nand as indifferent to the other;\nbut, implicitly, it does refer to the form,\nfor it contains the sublated negativity\nand is matter only by virtue of this determination.\nIt refers to it as an other only\nbecause form is not posited in it,\nbecause it is form only implicitly.\nIt contains form locked up inside it,\nand it is an absolute receptivity for form\nonly because it has the latter within it absolutely,\nbecause to be form is its implicit vocation.\nHence matter must be informed,\nand form must materialize itself;\nit must give itself self-identity\nor subsistence in matter.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-8'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-8'})
MATCH (c:IntegratedChunk {id: 'abs-8'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-8'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-8'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-8:kp:1'})
SET kp.chunkId = 'abs-8'
SET kp.ordinal = 1
SET kp.text = 'Form and matter presuppose each other';
MATCH (c:IntegratedChunk {id: 'abs-8'})
MATCH (kp:KeyPoint {id: 'abs-8:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-8:kp:2'})
SET kp.chunkId = 'abs-8'
SET kp.ordinal = 2
SET kp.text = 'Neither matter nor form derives from itself, is eternal';
MATCH (c:IntegratedChunk {id: 'abs-8'})
MATCH (kp:KeyPoint {id: 'abs-8:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-8:kp:3'})
SET kp.chunkId = 'abs-8'
SET kp.ordinal = 3
SET kp.text = 'Matter is not determined as ground of form';
MATCH (c:IntegratedChunk {id: 'abs-8'})
MATCH (kp:KeyPoint {id: 'abs-8:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-8:kp:4'})
SET kp.chunkId = 'abs-8'
SET kp.ordinal = 4
SET kp.text = 'Matter is passive, form is active';
MATCH (c:IntegratedChunk {id: 'abs-8'})
MATCH (kp:KeyPoint {id: 'abs-8:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-8:kp:5'})
SET kp.chunkId = 'abs-8'
SET kp.ordinal = 5
SET kp.text = 'Matter contains form locked up inside it, must be informed';
MATCH (c:IntegratedChunk {id: 'abs-8'})
MATCH (kp:KeyPoint {id: 'abs-8:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-9'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 298
SET segment.lineEnd = 311
SET segment.text = '2. Consequently, form determines matter,\nand matter is determined by form.\nBecause form is itself absolute self-identity\nand hence implicitly contains matter;\nand equally because matter in its pure abstraction\nor absolute negativity possesses form within it,\nthe activity of the form on the matter\nand the reception by the latter of the form determination is only\nthe sublating of the semblance of their indifference and distinctness.\nThus the determination referring each to the other is\nthe self-mediation of each through its own non-being.\nBut the two mediations are one movement,\nand the restoration of their original identity is\nthe inner recollection of their exteriorization.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-9'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-9'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-9'
SET topic.title = 'Form determines matter — self-mediation'
SET topic.description = 'Form determines matter, matter is determined by form. Because form is absolute self-identity and implicitly contains matter. And equally because matter in pure abstraction or absolute negativity possesses form within it. Activity of form on matter and reception by matter is only sublating of semblance of their indifference and distinctness. Determination referring each to other is self-mediation of each through its own non-being. Two mediations are one movement. Restoration of their original identity is inner recollection of their exteriorization.'
SET topic.keyPoints = ['Form determines matter, matter is determined by form', 'Form implicitly contains matter, matter possesses form within it', 'Activity is only sublating of semblance of indifference and distinctness', 'Self-mediation of each through its own non-being', 'Restoration of original identity is inner recollection of exteriorization'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-9'})
MATCH (topic:Topic {id: 'topic:abs-9'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-9'})
SET c.title = 'Form determines matter — self-mediation'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 298
SET c.lineEnd = 311
SET c.description = 'Form determines matter, matter is determined by form. Because form is absolute self-identity and implicitly contains matter. And equally because matter in pure abstraction or absolute negativity possesses form within it. Activity of form on matter and reception by matter is only sublating of semblance of their indifference and distinctness. Determination referring each to other is self-mediation of each through its own non-being. Two mediations are one movement. Restoration of their original identity is inner recollection of their exteriorization.'
SET c.keyPoints = ['Form determines matter, matter is determined by form', 'Form implicitly contains matter, matter possesses form within it', 'Activity is only sublating of semblance of indifference and distinctness', 'Self-mediation of each through its own non-being', 'Restoration of original identity is inner recollection of exteriorization']
SET c.tags = ['sublation', 'citta']
SET c.orderInSource = 9
SET c.globalOrder = 95
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '2. Consequently, form determines matter,\nand matter is determined by form.\nBecause form is itself absolute self-identity\nand hence implicitly contains matter;\nand equally because matter in its pure abstraction\nor absolute negativity possesses form within it,\nthe activity of the form on the matter\nand the reception by the latter of the form determination is only\nthe sublating of the semblance of their indifference and distinctness.\nThus the determination referring each to the other is\nthe self-mediation of each through its own non-being.\nBut the two mediations are one movement,\nand the restoration of their original identity is\nthe inner recollection of their exteriorization.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-9'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-9'})
MATCH (c:IntegratedChunk {id: 'abs-9'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-9'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-9'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-9:kp:1'})
SET kp.chunkId = 'abs-9'
SET kp.ordinal = 1
SET kp.text = 'Form determines matter, matter is determined by form';
MATCH (c:IntegratedChunk {id: 'abs-9'})
MATCH (kp:KeyPoint {id: 'abs-9:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-9:kp:2'})
SET kp.chunkId = 'abs-9'
SET kp.ordinal = 2
SET kp.text = 'Form implicitly contains matter, matter possesses form within it';
MATCH (c:IntegratedChunk {id: 'abs-9'})
MATCH (kp:KeyPoint {id: 'abs-9:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-9:kp:3'})
SET kp.chunkId = 'abs-9'
SET kp.ordinal = 3
SET kp.text = 'Activity is only sublating of semblance of indifference and distinctness';
MATCH (c:IntegratedChunk {id: 'abs-9'})
MATCH (kp:KeyPoint {id: 'abs-9:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-9:kp:4'})
SET kp.chunkId = 'abs-9'
SET kp.ordinal = 4
SET kp.text = 'Self-mediation of each through its own non-being';
MATCH (c:IntegratedChunk {id: 'abs-9'})
MATCH (kp:KeyPoint {id: 'abs-9:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-9:kp:5'})
SET kp.chunkId = 'abs-9'
SET kp.ordinal = 5
SET kp.text = 'Restoration of original identity is inner recollection of exteriorization';
MATCH (c:IntegratedChunk {id: 'abs-9'})
MATCH (kp:KeyPoint {id: 'abs-9:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-10'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 313
SET segment.lineEnd = 323
SET segment.text = 'First, form and matter presuppose each other.\nAs we have seen, this only means that the one essential unity is\nnegative self-reference, and that it therefore splits,\ndetermined as an indifferent substrate in the essential identity,\nand as determining form in essential distinction or negativity.\nThat unity of essence and form, the two opposed to each other as\nform and matter, is the absolute self-determining ground.\nInasmuch as this unity differentiates itself,\nthe reference connecting the two diverse terms,\nbecause of the unity that underlies them,\nbecomes a reference of reciprocal presupposition.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-10'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-10'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-10'
SET topic.title = 'Form and matter presuppose each other — absolute ground'
SET topic.description = 'Form and matter presuppose each other. This only means one essential unity is negative self-reference. Therefore splits, determined as indifferent substrate in essential identity. And as determining form in essential distinction or negativity. That unity of essence and form, two opposed as form and matter, is absolute self-determining ground. Inasmuch as unity differentiates itself, reference connecting two diverse terms becomes reference of reciprocal presupposition.'
SET topic.keyPoints = ['One essential unity is negative self-reference', 'Splits into indifferent substrate and determining form', 'Unity of essence and form is absolute self-determining ground', 'Reference becomes reference of reciprocal presupposition'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-10'})
MATCH (topic:Topic {id: 'topic:abs-10'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-10'})
SET c.title = 'Form and matter presuppose each other — absolute ground'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 313
SET c.lineEnd = 323
SET c.description = 'Form and matter presuppose each other. This only means one essential unity is negative self-reference. Therefore splits, determined as indifferent substrate in essential identity. And as determining form in essential distinction or negativity. That unity of essence and form, two opposed as form and matter, is absolute self-determining ground. Inasmuch as unity differentiates itself, reference connecting two diverse terms becomes reference of reciprocal presupposition.'
SET c.keyPoints = ['One essential unity is negative self-reference', 'Splits into indifferent substrate and determining form', 'Unity of essence and form is absolute self-determining ground', 'Reference becomes reference of reciprocal presupposition']
SET c.tags = ['negation', 'citta']
SET c.orderInSource = 10
SET c.globalOrder = 96
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'First, form and matter presuppose each other.\nAs we have seen, this only means that the one essential unity is\nnegative self-reference, and that it therefore splits,\ndetermined as an indifferent substrate in the essential identity,\nand as determining form in essential distinction or negativity.\nThat unity of essence and form, the two opposed to each other as\nform and matter, is the absolute self-determining ground.\nInasmuch as this unity differentiates itself,\nthe reference connecting the two diverse terms,\nbecause of the unity that underlies them,\nbecomes a reference of reciprocal presupposition.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-10'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-10'})
MATCH (c:IntegratedChunk {id: 'abs-10'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-10'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-10'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-10:kp:1'})
SET kp.chunkId = 'abs-10'
SET kp.ordinal = 1
SET kp.text = 'One essential unity is negative self-reference';
MATCH (c:IntegratedChunk {id: 'abs-10'})
MATCH (kp:KeyPoint {id: 'abs-10:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-10:kp:2'})
SET kp.chunkId = 'abs-10'
SET kp.ordinal = 2
SET kp.text = 'Splits into indifferent substrate and determining form';
MATCH (c:IntegratedChunk {id: 'abs-10'})
MATCH (kp:KeyPoint {id: 'abs-10:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-10:kp:3'})
SET kp.chunkId = 'abs-10'
SET kp.ordinal = 3
SET kp.text = 'Unity of essence and form is absolute self-determining ground';
MATCH (c:IntegratedChunk {id: 'abs-10'})
MATCH (kp:KeyPoint {id: 'abs-10:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-10:kp:4'})
SET kp.chunkId = 'abs-10'
SET kp.ordinal = 4
SET kp.text = 'Reference becomes reference of reciprocal presupposition';
MATCH (c:IntegratedChunk {id: 'abs-10'})
MATCH (kp:KeyPoint {id: 'abs-10:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-11'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 325
SET segment.lineEnd = 349
SET segment.text = 'Second, the form already is, as self-subsisting,\nself-sublating contradiction;\nbut it is also posited as in this way self-sublating,\nfor it is self-subsisting and at the same time\nessentially referred to another,\nand consequently it sublates itself.\nSince it is itself two-sided, its sublating also has two sides.\nFor one, form sublates its self-subsistence\nand transforms itself into something posited,\nsomething that exists in an other,\nand this other is in its case matter.\nFor the other, form sublates its determinateness vis-à-vis matter,\nsublates its reference to it, consequently its positedness,\nand it thereby gives itself subsistence.\nIts reflection in thus sublating its positedness is\nits own identity into which it passes over.\nBut since form at the same time externalizes this identity\nand posits it over against itself as matter,\nthat reflection of the positedness into itself is\na union with a matter in which it obtains subsistence.\nIn this union, therefore, it is equally both:\nis united with matter as with something other\n(in accordance with the first side, viz. in that it makes\nitself into a positedness),\nand, in this other, is united with its own identity.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-11'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-11'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-11'
SET topic.title = 'Form self-sublating — two sides'
SET topic.description = 'Form already is, as self-subsisting, self-sublating contradiction. Also posited as in this way self-sublating. Since two-sided, sublating also has two sides. For one, form sublates its self-subsistence and transforms itself into something posited, something that exists in other, and this other is matter. For other, form sublates its determinateness vis-à-vis matter, sublates its reference to it, consequently its positedness, thereby gives itself subsistence. Reflection in sublating positedness is its own identity into which it passes over. But since form at same time externalizes this identity and posits it over against itself as matter, reflection of positedness into itself is union with matter in which it obtains subsistence.'
SET topic.keyPoints = ['Form is self-sublating contradiction', 'Sublating has two sides', 'Form transforms itself into something posited in matter', 'Form sublates its determinateness vis-à-vis matter, gives itself subsistence', 'Union with matter in which form obtains subsistence'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-11'})
MATCH (topic:Topic {id: 'topic:abs-11'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-11'})
SET c.title = 'Form self-sublating — two sides'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 325
SET c.lineEnd = 349
SET c.description = 'Form already is, as self-subsisting, self-sublating contradiction. Also posited as in this way self-sublating. Since two-sided, sublating also has two sides. For one, form sublates its self-subsistence and transforms itself into something posited, something that exists in other, and this other is matter. For other, form sublates its determinateness vis-à-vis matter, sublates its reference to it, consequently its positedness, thereby gives itself subsistence. Reflection in sublating positedness is its own identity into which it passes over. But since form at same time externalizes this identity and posits it over against itself as matter, reflection of positedness into itself is union with matter in which it obtains subsistence.'
SET c.keyPoints = ['Form is self-sublating contradiction', 'Sublating has two sides', 'Form transforms itself into something posited in matter', 'Form sublates its determinateness vis-à-vis matter, gives itself subsistence', 'Union with matter in which form obtains subsistence']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 11
SET c.globalOrder = 97
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Second, the form already is, as self-subsisting,\nself-sublating contradiction;\nbut it is also posited as in this way self-sublating,\nfor it is self-subsisting and at the same time\nessentially referred to another,\nand consequently it sublates itself.\nSince it is itself two-sided, its sublating also has two sides.\nFor one, form sublates its self-subsistence\nand transforms itself into something posited,\nsomething that exists in an other,\nand this other is in its case matter.\nFor the other, form sublates its determinateness vis-à-vis matter,\nsublates its reference to it, consequently its positedness,\nand it thereby gives itself subsistence.\nIts reflection in thus sublating its positedness is\nits own identity into which it passes over.\nBut since form at the same time externalizes this identity\nand posits it over against itself as matter,\nthat reflection of the positedness into itself is\na union with a matter in which it obtains subsistence.\nIn this union, therefore, it is equally both:\nis united with matter as with something other\n(in accordance with the first side, viz. in that it makes\nitself into a positedness),\nand, in this other, is united with its own identity.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-11'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-11'})
MATCH (c:IntegratedChunk {id: 'abs-11'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-11'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-11'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-11:kp:1'})
SET kp.chunkId = 'abs-11'
SET kp.ordinal = 1
SET kp.text = 'Form is self-sublating contradiction';
MATCH (c:IntegratedChunk {id: 'abs-11'})
MATCH (kp:KeyPoint {id: 'abs-11:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-11:kp:2'})
SET kp.chunkId = 'abs-11'
SET kp.ordinal = 2
SET kp.text = 'Sublating has two sides';
MATCH (c:IntegratedChunk {id: 'abs-11'})
MATCH (kp:KeyPoint {id: 'abs-11:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-11:kp:3'})
SET kp.chunkId = 'abs-11'
SET kp.ordinal = 3
SET kp.text = 'Form transforms itself into something posited in matter';
MATCH (c:IntegratedChunk {id: 'abs-11'})
MATCH (kp:KeyPoint {id: 'abs-11:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-11:kp:4'})
SET kp.chunkId = 'abs-11'
SET kp.ordinal = 4
SET kp.text = 'Form sublates its determinateness vis-à-vis matter, gives itself subsistence';
MATCH (c:IntegratedChunk {id: 'abs-11'})
MATCH (kp:KeyPoint {id: 'abs-11:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-11:kp:5'})
SET kp.chunkId = 'abs-11'
SET kp.ordinal = 5
SET kp.text = 'Union with matter in which form obtains subsistence';
MATCH (c:IntegratedChunk {id: 'abs-11'})
MATCH (kp:KeyPoint {id: 'abs-11:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-12'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 351
SET segment.lineEnd = 393
SET segment.text = 'The activity of form by which matter is determined consists,\ntherefore, in a negative relating of the form to itself.\nBut, conversely, form thereby negatively relates itself to matter also;\nthe movement, however, by which matter becomes determined is\njust as much the form\'s own movement.\nForm is free of matter, but it sublates its self-subsistence;\nbut this, its self-subsistence, is matter itself,\nfor it is in this matter that it has its essential identity.\nIt makes itself into a positedness, but this is one and the same\nas making matter into something determinate.\nBut, considered from the other side,\nthe form\'s own identity is at the same time externalized,\nand matter is its other;\nfor this reason, because form sublates its own self-subsistence,\nmatter is also not determined.\nBut matter only subsists vis-à-vis form;\nas the negative sublates itself, so does the positive also.\nAnd as the form sublates itself, the determinateness of matter\nthat the latter has vis-à-vis form also falls away\nthe determinateness, namely, of being the indeterminate subsistence.\n\nWhat appears here as the activity of form is, moreover,\njust as much the movement that belongs to matter itself.\nThe determination that implicitly exists in matter,\nwhat matter is supposed to be, is its absolute negativity.\nThrough it matter does not just refer to form simply as to an other,\nbut this external other is the form rather that\nmatter itself contains locked up within itself.\nMatter is in itself the same contradiction that form contains,\nand this contradiction, like its resolution, is only one.\nBut matter is thus in itself self-contradictory because,\nas indeterminate self-identity,\nit is at the same time absolute negativity;\nit sublates itself within:\nits identity disintegrates in its negativity\nwhile the latter obtains in it its subsistence.\nSince matter is therefore determined by form as by something external,\nit thereby attains its determination,\nand the externality of the relating, for both form and matter,\nconsists in that each, or rather in that the original unity of each,\nin positing is at the same time presupposing:\nthe result is that self-reference is at the same time\na reference to the self as sublated or is reference to its other.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-12'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-12'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-12'
SET topic.title = 'Activity of form — matter\'s movement'
SET topic.description = 'Activity of form by which matter is determined consists in negative relating of form to itself. Movement by which matter becomes determined is just as much form\'s own movement. Form is free of matter, but sublates its self-subsistence. Its self-subsistence is matter itself, for in matter it has its essential identity. Makes itself into positedness, but this is same as making matter into something determinate. What appears as activity of form is just as much movement that belongs to matter itself. Matter is in itself same contradiction that form contains. Matter is in itself self-contradictory because, as indeterminate self-identity, is at same time absolute negativity. Sublates itself within: identity disintegrates in negativity while latter obtains in it its subsistence.'
SET topic.keyPoints = ['Activity of form consists in negative relating of form to itself', 'Movement of matter is just as much form\'s own movement', 'Self-subsistence of form is matter itself', 'Matter is in itself same contradiction that form contains', 'Matter sublates itself within: identity disintegrates in negativity'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-12'})
MATCH (topic:Topic {id: 'topic:abs-12'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-12'})
SET c.title = 'Activity of form — matter\'s movement'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 351
SET c.lineEnd = 393
SET c.description = 'Activity of form by which matter is determined consists in negative relating of form to itself. Movement by which matter becomes determined is just as much form\'s own movement. Form is free of matter, but sublates its self-subsistence. Its self-subsistence is matter itself, for in matter it has its essential identity. Makes itself into positedness, but this is same as making matter into something determinate. What appears as activity of form is just as much movement that belongs to matter itself. Matter is in itself same contradiction that form contains. Matter is in itself self-contradictory because, as indeterminate self-identity, is at same time absolute negativity. Sublates itself within: identity disintegrates in negativity while latter obtains in it its subsistence.'
SET c.keyPoints = ['Activity of form consists in negative relating of form to itself', 'Movement of matter is just as much form\'s own movement', 'Self-subsistence of form is matter itself', 'Matter is in itself same contradiction that form contains', 'Matter sublates itself within: identity disintegrates in negativity']
SET c.tags = ['negation', 'sublation', 'citta']
SET c.orderInSource = 12
SET c.globalOrder = 98
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The activity of form by which matter is determined consists,\ntherefore, in a negative relating of the form to itself.\nBut, conversely, form thereby negatively relates itself to matter also;\nthe movement, however, by which matter becomes determined is\njust as much the form\'s own movement.\nForm is free of matter, but it sublates its self-subsistence;\nbut this, its self-subsistence, is matter itself,\nfor it is in this matter that it has its essential identity.\nIt makes itself into a positedness, but this is one and the same\nas making matter into something determinate.\nBut, considered from the other side,\nthe form\'s own identity is at the same time externalized,\nand matter is its other;\nfor this reason, because form sublates its own self-subsistence,\nmatter is also not determined.\nBut matter only subsists vis-à-vis form;\nas the negative sublates itself, so does the positive also.\nAnd as the form sublates itself, the determinateness of matter\nthat the latter has vis-à-vis form also falls away\nthe determinateness, namely, of being the indeterminate subsistence.\n\nWhat appears here as the activity of form is, moreover,\njust as much the movement that belongs to matter itself.\nThe determination that implicitly exists in matter,\nwhat matter is supposed to be, is its absolute negativity.\nThrough it matter does not just refer to form simply as to an other,\nbut this external other is the form rather that\nmatter itself contains locked up within itself.\nMatter is in itself the same contradiction that form contains,\nand this contradiction, like its resolution, is only one.\nBut matter is thus in itself self-contradictory because,\nas indeterminate self-identity,\nit is at the same time absolute negativity;\nit sublates itself within:\nits identity disintegrates in its negativity\nwhile the latter obtains in it its subsistence.\nSince matter is therefore determined by form as by something external,\nit thereby attains its determination,\nand the externality of the relating, for both form and matter,\nconsists in that each, or rather in that the original unity of each,\nin positing is at the same time presupposing:\nthe result is that self-reference is at the same time\na reference to the self as sublated or is reference to its other.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-12'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-12'})
MATCH (c:IntegratedChunk {id: 'abs-12'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-12'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-12'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-12:kp:1'})
SET kp.chunkId = 'abs-12'
SET kp.ordinal = 1
SET kp.text = 'Activity of form consists in negative relating of form to itself';
MATCH (c:IntegratedChunk {id: 'abs-12'})
MATCH (kp:KeyPoint {id: 'abs-12:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-12:kp:2'})
SET kp.chunkId = 'abs-12'
SET kp.ordinal = 2
SET kp.text = 'Movement of matter is just as much form\'s own movement';
MATCH (c:IntegratedChunk {id: 'abs-12'})
MATCH (kp:KeyPoint {id: 'abs-12:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-12:kp:3'})
SET kp.chunkId = 'abs-12'
SET kp.ordinal = 3
SET kp.text = 'Self-subsistence of form is matter itself';
MATCH (c:IntegratedChunk {id: 'abs-12'})
MATCH (kp:KeyPoint {id: 'abs-12:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-12:kp:4'})
SET kp.chunkId = 'abs-12'
SET kp.ordinal = 4
SET kp.text = 'Matter is in itself same contradiction that form contains';
MATCH (c:IntegratedChunk {id: 'abs-12'})
MATCH (kp:KeyPoint {id: 'abs-12:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-12:kp:5'})
SET kp.chunkId = 'abs-12'
SET kp.ordinal = 5
SET kp.text = 'Matter sublates itself within: identity disintegrates in negativity';
MATCH (c:IntegratedChunk {id: 'abs-12'})
MATCH (kp:KeyPoint {id: 'abs-12:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-13'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 395
SET segment.lineEnd = 457
SET segment.text = 'Third, through this movement of form and matter,\nthe original unity of the two is, on the one hand, restored;\non the other hand, it is henceforth a posited unity.\nMatter is just as much a self-determining as\nthis determining is for it an activity of form external to it;\ncontrariwise, form determines only itself,\nor has the matter that it determines within it,\njust much as in its determining it relates itself to another;\nand both, the activity of form and the movement of matter,\nare one and the same thing, only that the former is an activity,\nthat is, it is the negativity as posited,\nwhile the latter is movement or becoming,\nthe negativity as determination existing in itself.\nThe result, therefore, is the unity of the in-itself and positedness.\nMatter is as such determined or necessarily has a form,\nand form is simply material, subsistent form.\n\nInasmuch as form presupposes a matter as its other, it is finite.\nIt is not a ground but only the active factor.\nEqually so, matter, inasmuch as it presupposes\nform as its non-being, is finite matter;\nit is not the ground of its unity with form\nbut is for the latter only the substrate.\nBut neither this finite matter nor the finite form have any truth;\neach refers to the other, or only their unity is their truth.\nThe two determinations return to\nthis unity and there they sublate their self-subsistence;\nthe unity thereby proves to be their ground.\nConsequently, matter is the ground\nof its form determination not as matter\nbut only inasmuch as it is the absolute unity of essence and form;\nsimilarly, form is the ground of the subsistence of its determinations\nonly to the extent that it is that same one unity.\nBut this one unity, as absolute negativity,\nand more specifically as exclusive unity,\nis, in its reflection, a presupposing;\nor again, that unity is one act,\nof preserving itself as positedness in positing,\nand of repelling itself from itself;\nof referring itself to itself as itself\nand to itself as to another.\nOr, the act by which matter is determined by form\nis the self-mediation of essence as ground, in one unity:\nthrough itself and through the negation of itself.\n\nInformed matter or form that possesses subsistence is now,\nnot only this absolute unity of ground with itself,\nbut also unity as posited.\nThe movement just considered is the one\nin which the absolute ground has exhibited\nits moments at once as self-sublating\nand consequently as posited.\nOr the restored unity, in withdrawing into itself,\nhas repelled itself from itself and\nhas determined itself;\nfor its unity has been established through negation\nand is, therefore, also negative unity.\nIt is, therefore, the unity of form and matter,\nas the substrate of both, but a substrate which is determinate:\nit is formed matter, but matter at the same time\nindifferent to form and matter,\nindifferent to them because sublated and unessential.\nThis is content.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-13'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-13'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-13'
SET topic.title = 'Original unity restored — content'
SET topic.description = 'Through movement of form and matter, original unity restored. On other hand, henceforth posited unity. Matter is just as much self-determining as determining is for it activity of form external to it. Activity of form and movement of matter are one and same thing. Result is unity of in-itself and positedness. Matter is as such determined or necessarily has form. Form is simply material, subsistent form. Inasmuch as form presupposes matter as its other, it is finite. Neither finite matter nor finite form have truth. Each refers to other, or only their unity is their truth. Unity proves to be their ground. Matter is ground only as absolute unity of essence and form. One unity, as absolute negativity, exclusive unity, is in its reflection a presupposing. Act by which matter determined by form is self-mediation of essence as ground, through itself and through negation of itself. Informed matter or form that possesses subsistence is absolute unity of ground with itself, but also unity as posited. Unity of form and matter, as substrate of both, but substrate which is determinate. Formed matter, but matter at same time indifferent to form and matter, because sublated and unessential. This is content.'
SET topic.keyPoints = ['Original unity restored, henceforth posited unity', 'Activity of form and movement of matter are one and same thing', 'Neither finite matter nor finite form have truth, only their unity is truth', 'Unity proves to be their ground', 'Informed matter or form is content'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-13'})
MATCH (topic:Topic {id: 'topic:abs-13'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-13'})
SET c.title = 'Original unity restored — content'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 395
SET c.lineEnd = 457
SET c.description = 'Through movement of form and matter, original unity restored. On other hand, henceforth posited unity. Matter is just as much self-determining as determining is for it activity of form external to it. Activity of form and movement of matter are one and same thing. Result is unity of in-itself and positedness. Matter is as such determined or necessarily has form. Form is simply material, subsistent form. Inasmuch as form presupposes matter as its other, it is finite. Neither finite matter nor finite form have truth. Each refers to other, or only their unity is their truth. Unity proves to be their ground. Matter is ground only as absolute unity of essence and form. One unity, as absolute negativity, exclusive unity, is in its reflection a presupposing. Act by which matter determined by form is self-mediation of essence as ground, through itself and through negation of itself. Informed matter or form that possesses subsistence is absolute unity of ground with itself, but also unity as posited. Unity of form and matter, as substrate of both, but substrate which is determinate. Formed matter, but matter at same time indifferent to form and matter, because sublated and unessential. This is content.'
SET c.keyPoints = ['Original unity restored, henceforth posited unity', 'Activity of form and movement of matter are one and same thing', 'Neither finite matter nor finite form have truth, only their unity is truth', 'Unity proves to be their ground', 'Informed matter or form is content']
SET c.tags = ['negation', 'sublation', 'reflection', 'citta']
SET c.orderInSource = 13
SET c.globalOrder = 99
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Third, through this movement of form and matter,\nthe original unity of the two is, on the one hand, restored;\non the other hand, it is henceforth a posited unity.\nMatter is just as much a self-determining as\nthis determining is for it an activity of form external to it;\ncontrariwise, form determines only itself,\nor has the matter that it determines within it,\njust much as in its determining it relates itself to another;\nand both, the activity of form and the movement of matter,\nare one and the same thing, only that the former is an activity,\nthat is, it is the negativity as posited,\nwhile the latter is movement or becoming,\nthe negativity as determination existing in itself.\nThe result, therefore, is the unity of the in-itself and positedness.\nMatter is as such determined or necessarily has a form,\nand form is simply material, subsistent form.\n\nInasmuch as form presupposes a matter as its other, it is finite.\nIt is not a ground but only the active factor.\nEqually so, matter, inasmuch as it presupposes\nform as its non-being, is finite matter;\nit is not the ground of its unity with form\nbut is for the latter only the substrate.\nBut neither this finite matter nor the finite form have any truth;\neach refers to the other, or only their unity is their truth.\nThe two determinations return to\nthis unity and there they sublate their self-subsistence;\nthe unity thereby proves to be their ground.\nConsequently, matter is the ground\nof its form determination not as matter\nbut only inasmuch as it is the absolute unity of essence and form;\nsimilarly, form is the ground of the subsistence of its determinations\nonly to the extent that it is that same one unity.\nBut this one unity, as absolute negativity,\nand more specifically as exclusive unity,\nis, in its reflection, a presupposing;\nor again, that unity is one act,\nof preserving itself as positedness in positing,\nand of repelling itself from itself;\nof referring itself to itself as itself\nand to itself as to another.\nOr, the act by which matter is determined by form\nis the self-mediation of essence as ground, in one unity:\nthrough itself and through the negation of itself.\n\nInformed matter or form that possesses subsistence is now,\nnot only this absolute unity of ground with itself,\nbut also unity as posited.\nThe movement just considered is the one\nin which the absolute ground has exhibited\nits moments at once as self-sublating\nand consequently as posited.\nOr the restored unity, in withdrawing into itself,\nhas repelled itself from itself and\nhas determined itself;\nfor its unity has been established through negation\nand is, therefore, also negative unity.\nIt is, therefore, the unity of form and matter,\nas the substrate of both, but a substrate which is determinate:\nit is formed matter, but matter at the same time\nindifferent to form and matter,\nindifferent to them because sublated and unessential.\nThis is content.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-13'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-13'})
MATCH (c:IntegratedChunk {id: 'abs-13'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-13'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-13'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-13:kp:1'})
SET kp.chunkId = 'abs-13'
SET kp.ordinal = 1
SET kp.text = 'Original unity restored, henceforth posited unity';
MATCH (c:IntegratedChunk {id: 'abs-13'})
MATCH (kp:KeyPoint {id: 'abs-13:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-13:kp:2'})
SET kp.chunkId = 'abs-13'
SET kp.ordinal = 2
SET kp.text = 'Activity of form and movement of matter are one and same thing';
MATCH (c:IntegratedChunk {id: 'abs-13'})
MATCH (kp:KeyPoint {id: 'abs-13:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-13:kp:3'})
SET kp.chunkId = 'abs-13'
SET kp.ordinal = 3
SET kp.text = 'Neither finite matter nor finite form have truth, only their unity is truth';
MATCH (c:IntegratedChunk {id: 'abs-13'})
MATCH (kp:KeyPoint {id: 'abs-13:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-13:kp:4'})
SET kp.chunkId = 'abs-13'
SET kp.ordinal = 4
SET kp.text = 'Unity proves to be their ground';
MATCH (c:IntegratedChunk {id: 'abs-13'})
MATCH (kp:KeyPoint {id: 'abs-13:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-13:kp:5'})
SET kp.chunkId = 'abs-13'
SET kp.ordinal = 5
SET kp.text = 'Informed matter or form is content';
MATCH (c:IntegratedChunk {id: 'abs-13'})
MATCH (kp:KeyPoint {id: 'abs-13:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-14'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 461
SET segment.lineEnd = 475
SET segment.text = 'Form stands at first over against essence;\nit is then the ground-connection in general,\nand its determinations are the ground and the grounded.\nIt then stands over against matter,\nand so it is determining reflection,\nand its determinations are the determination of reflection itself\nand the subsistence of the latter.\nFinally, it stands over against content,\nand then its determinations are again itself and matter.\nWhat was previously the self-identical\nat first the ground,\nthen subsistence in general,\nand finally matter\nnow passes under the dominion of form\nand is once more one of its determinations.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-14'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-14'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-14'
SET topic.title = 'c. Form and content — form stands over against content'
SET topic.description = 'Form stands at first over against essence. It is then ground-connection in general, determinations are ground and grounded. It then stands over against matter. So it is determining reflection, determinations are determination of reflection itself and subsistence of latter. Finally, it stands over against content. Then its determinations are again itself and matter. What was previously self-identical: at first ground, then subsistence in general, finally matter. Now passes under dominion of form and is once more one of its determinations.'
SET topic.keyPoints = ['Form stands over against essence, then matter, finally content', 'Form is ground-connection, then determining reflection', 'What was self-identical now passes under dominion of form', 'Is once more one of form\'s determinations'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-14'})
MATCH (topic:Topic {id: 'topic:abs-14'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-14'})
SET c.title = 'c. Form and content — form stands over against content'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 461
SET c.lineEnd = 475
SET c.description = 'Form stands at first over against essence. It is then ground-connection in general, determinations are ground and grounded. It then stands over against matter. So it is determining reflection, determinations are determination of reflection itself and subsistence of latter. Finally, it stands over against content. Then its determinations are again itself and matter. What was previously self-identical: at first ground, then subsistence in general, finally matter. Now passes under dominion of form and is once more one of its determinations.'
SET c.keyPoints = ['Form stands over against essence, then matter, finally content', 'Form is ground-connection, then determining reflection', 'What was self-identical now passes under dominion of form', 'Is once more one of form\'s determinations']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 14
SET c.globalOrder = 100
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Form stands at first over against essence;\nit is then the ground-connection in general,\nand its determinations are the ground and the grounded.\nIt then stands over against matter,\nand so it is determining reflection,\nand its determinations are the determination of reflection itself\nand the subsistence of the latter.\nFinally, it stands over against content,\nand then its determinations are again itself and matter.\nWhat was previously the self-identical\nat first the ground,\nthen subsistence in general,\nand finally matter\nnow passes under the dominion of form\nand is once more one of its determinations.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-14'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-14'})
MATCH (c:IntegratedChunk {id: 'abs-14'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-14'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-14'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-14:kp:1'})
SET kp.chunkId = 'abs-14'
SET kp.ordinal = 1
SET kp.text = 'Form stands over against essence, then matter, finally content';
MATCH (c:IntegratedChunk {id: 'abs-14'})
MATCH (kp:KeyPoint {id: 'abs-14:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-14:kp:2'})
SET kp.chunkId = 'abs-14'
SET kp.ordinal = 2
SET kp.text = 'Form is ground-connection, then determining reflection';
MATCH (c:IntegratedChunk {id: 'abs-14'})
MATCH (kp:KeyPoint {id: 'abs-14:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-14:kp:3'})
SET kp.chunkId = 'abs-14'
SET kp.ordinal = 3
SET kp.text = 'What was self-identical now passes under dominion of form';
MATCH (c:IntegratedChunk {id: 'abs-14'})
MATCH (kp:KeyPoint {id: 'abs-14:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-14:kp:4'})
SET kp.chunkId = 'abs-14'
SET kp.ordinal = 4
SET kp.text = 'Is once more one of form\'s determinations';
MATCH (c:IntegratedChunk {id: 'abs-14'})
MATCH (kp:KeyPoint {id: 'abs-14:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-15'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 477
SET segment.lineEnd = 502
SET segment.text = 'Content has, first, a form and a matter that\nbelong to it essentially; it is their unity.\nBut, because this unity is at the same time determinate\nor posited unity, content stands over against form;\nthe latter constitutes the positedness\nand is the unessential over against content.\nThe latter is therefore indifferent towards form;\nform embraces both the form as such as well as the matter,\nand content therefore has a form and a matter,\nof which it constitutes the substrate\nand which are to it mere positedness.\n\nContent is, second, what is identical in form and matter,\nso that these would be only indifferent external determinations.\nThey are positedness in general, but a positedness\nthat has returned in the content to its unity or its ground.\nThe identity of the content with itself is,\ntherefore, in one respect that identity which is indifferent to form,\nbut in another the identity of ground.\nThe ground has at first disappeared into content;\nbut content is at the same time the negative reflection of\nthe form determinations into themselves;\nits unity, at first only the unity indifferent to form, is\ntherefore also the formal unity or the ground-connection as such.\nContent, therefore, has this ground-connection as its essential form,\nand, contrariwise, the ground has a content.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-15'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-15'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-15'
SET topic.title = 'Content has form and matter — unity'
SET topic.description = 'Content has, first, form and matter that belong to it essentially; it is their unity. But because unity is at same time determinate or posited unity, content stands over against form. Latter constitutes positedness and is unessential over against content. Content indifferent towards form. Content is, second, what is identical in form and matter. So that these would be only indifferent external determinations. They are positedness in general, but positedness that has returned in content to its unity or its ground. Identity of content with itself is, in one respect, identity indifferent to form. But in another, identity of ground. Content is at same time negative reflection of form determinations into themselves. Its unity is therefore also formal unity or ground-connection as such. Content has this ground-connection as its essential form. Contrariwise, ground has a content.'
SET topic.keyPoints = ['Content has form and matter that belong to it essentially; it is their unity', 'Content stands over against form', 'Content is what is identical in form and matter', 'Content\'s unity is formal unity or ground-connection as such', 'Content has ground-connection as its essential form'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-15'})
MATCH (topic:Topic {id: 'topic:abs-15'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-15'})
SET c.title = 'Content has form and matter — unity'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 477
SET c.lineEnd = 502
SET c.description = 'Content has, first, form and matter that belong to it essentially; it is their unity. But because unity is at same time determinate or posited unity, content stands over against form. Latter constitutes positedness and is unessential over against content. Content indifferent towards form. Content is, second, what is identical in form and matter. So that these would be only indifferent external determinations. They are positedness in general, but positedness that has returned in content to its unity or its ground. Identity of content with itself is, in one respect, identity indifferent to form. But in another, identity of ground. Content is at same time negative reflection of form determinations into themselves. Its unity is therefore also formal unity or ground-connection as such. Content has this ground-connection as its essential form. Contrariwise, ground has a content.'
SET c.keyPoints = ['Content has form and matter that belong to it essentially; it is their unity', 'Content stands over against form', 'Content is what is identical in form and matter', 'Content\'s unity is formal unity or ground-connection as such', 'Content has ground-connection as its essential form']
SET c.tags = ['negation', 'reflection', 'citta']
SET c.orderInSource = 15
SET c.globalOrder = 101
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Content has, first, a form and a matter that\nbelong to it essentially; it is their unity.\nBut, because this unity is at the same time determinate\nor posited unity, content stands over against form;\nthe latter constitutes the positedness\nand is the unessential over against content.\nThe latter is therefore indifferent towards form;\nform embraces both the form as such as well as the matter,\nand content therefore has a form and a matter,\nof which it constitutes the substrate\nand which are to it mere positedness.\n\nContent is, second, what is identical in form and matter,\nso that these would be only indifferent external determinations.\nThey are positedness in general, but a positedness\nthat has returned in the content to its unity or its ground.\nThe identity of the content with itself is,\ntherefore, in one respect that identity which is indifferent to form,\nbut in another the identity of ground.\nThe ground has at first disappeared into content;\nbut content is at the same time the negative reflection of\nthe form determinations into themselves;\nits unity, at first only the unity indifferent to form, is\ntherefore also the formal unity or the ground-connection as such.\nContent, therefore, has this ground-connection as its essential form,\nand, contrariwise, the ground has a content.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-15'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-15'})
MATCH (c:IntegratedChunk {id: 'abs-15'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-15'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-15'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-15:kp:1'})
SET kp.chunkId = 'abs-15'
SET kp.ordinal = 1
SET kp.text = 'Content has form and matter that belong to it essentially; it is their unity';
MATCH (c:IntegratedChunk {id: 'abs-15'})
MATCH (kp:KeyPoint {id: 'abs-15:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-15:kp:2'})
SET kp.chunkId = 'abs-15'
SET kp.ordinal = 2
SET kp.text = 'Content stands over against form';
MATCH (c:IntegratedChunk {id: 'abs-15'})
MATCH (kp:KeyPoint {id: 'abs-15:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-15:kp:3'})
SET kp.chunkId = 'abs-15'
SET kp.ordinal = 3
SET kp.text = 'Content is what is identical in form and matter';
MATCH (c:IntegratedChunk {id: 'abs-15'})
MATCH (kp:KeyPoint {id: 'abs-15:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-15:kp:4'})
SET kp.chunkId = 'abs-15'
SET kp.ordinal = 4
SET kp.text = 'Content\'s unity is formal unity or ground-connection as such';
MATCH (c:IntegratedChunk {id: 'abs-15'})
MATCH (kp:KeyPoint {id: 'abs-15:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-15:kp:5'})
SET kp.chunkId = 'abs-15'
SET kp.ordinal = 5
SET kp.text = 'Content has ground-connection as its essential form';
MATCH (c:IntegratedChunk {id: 'abs-15'})
MATCH (kp:KeyPoint {id: 'abs-15:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:abs-16'})
SET segment.sourceId = 'source-absolute-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET segment.lineStart = 504
SET segment.lineEnd = 535
SET segment.text = 'The content of the ground is therefore the ground\nthat has returned into its unity with itself;\nthe ground is at first the essence that in its\npositedness is identical with itself;\nas diverse from and indifferent to its positedness,\nthe ground is indeterminate matter;\nbut as content it is at the same time informed identity,\nand this form becomes for this reason a ground-connection,\nsince the determinations of its oppositions are posited\nin the content also as negated.\nContent is further determined within,\nnot like matter as an indifferent in general,\nbut like informed matter,\nso that the determinations of form have\na material, indifferent subsistence.\nOn the one hand, content is the essential self-identity\nof the ground in its positedness;\non the other hand, it is posited identity\nas against the ground-connection;\nthis positedness, which is in this identity as determination of form,\nstands over against the free positedness, that is to say,\nover against the form as the whole connection of ground and grounded.\nThis form is the total positedness returning into itself;\nthe other form, therefore, is only the positedness as immediate,\nthe determinateness as such.\n\nThe ground has thus made itself into a determinate ground in general,\nand the determinateness is itself twofold:\nof form first, and of content second.\nThe former is its determinateness of being external to the content as such,\nthe content that remains indifferent to this external reference.\nThe latter is the determinateness of the content that the ground has.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:abs-16'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:abs-16'})
SET topic.sourceId = 'source-absolute-ground'
SET topic.topicRef = 'abs-16'
SET topic.title = 'Content of ground — determinate ground'
SET topic.description = 'Content of ground is ground that has returned into its unity with itself. Ground is at first essence that in its positedness is identical with itself. As diverse from and indifferent to its positedness, ground is indeterminate matter. But as content it is at same time informed identity. This form becomes ground-connection. Content is further determined within, not like matter as indifferent in general. But like informed matter, so that determinations of form have material, indifferent subsistence. On one hand, content is essential self-identity of ground in its positedness. On other hand, it is posited identity as against ground-connection. This positedness stands over against free positedness, form as whole connection of ground and grounded. Ground has made itself into determinate ground in general. Determinateness is itself twofold: of form first, and of content second.'
SET topic.keyPoints = ['Content of ground is ground that has returned into its unity with itself', 'As content, ground is informed identity', 'Content is like informed matter, determinations of form have material subsistence', 'Content is essential self-identity of ground in its positedness', 'Ground has made itself into determinate ground, determinateness twofold: of form and of content'];
MATCH (segment:ChunkSegment {id: 'chunk:abs-16'})
MATCH (topic:Topic {id: 'topic:abs-16'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'abs-16'})
SET c.title = 'Content of ground — determinate ground'
SET c.sourceId = 'source-absolute-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/absolute.txt'
SET c.lineStart = 504
SET c.lineEnd = 535
SET c.description = 'Content of ground is ground that has returned into its unity with itself. Ground is at first essence that in its positedness is identical with itself. As diverse from and indifferent to its positedness, ground is indeterminate matter. But as content it is at same time informed identity. This form becomes ground-connection. Content is further determined within, not like matter as indifferent in general. But like informed matter, so that determinations of form have material, indifferent subsistence. On one hand, content is essential self-identity of ground in its positedness. On other hand, it is posited identity as against ground-connection. This positedness stands over against free positedness, form as whole connection of ground and grounded. Ground has made itself into determinate ground in general. Determinateness is itself twofold: of form first, and of content second.'
SET c.keyPoints = ['Content of ground is ground that has returned into its unity with itself', 'As content, ground is informed identity', 'Content is like informed matter, determinations of form have material subsistence', 'Content is essential self-identity of ground in its positedness', 'Ground has made itself into determinate ground, determinateness twofold: of form and of content']
SET c.tags = ['citta']
SET c.orderInSource = 16
SET c.globalOrder = 102
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The content of the ground is therefore the ground\nthat has returned into its unity with itself;\nthe ground is at first the essence that in its\npositedness is identical with itself;\nas diverse from and indifferent to its positedness,\nthe ground is indeterminate matter;\nbut as content it is at the same time informed identity,\nand this form becomes for this reason a ground-connection,\nsince the determinations of its oppositions are posited\nin the content also as negated.\nContent is further determined within,\nnot like matter as an indifferent in general,\nbut like informed matter,\nso that the determinations of form have\na material, indifferent subsistence.\nOn the one hand, content is the essential self-identity\nof the ground in its positedness;\non the other hand, it is posited identity\nas against the ground-connection;\nthis positedness, which is in this identity as determination of form,\nstands over against the free positedness, that is to say,\nover against the form as the whole connection of ground and grounded.\nThis form is the total positedness returning into itself;\nthe other form, therefore, is only the positedness as immediate,\nthe determinateness as such.\n\nThe ground has thus made itself into a determinate ground in general,\nand the determinateness is itself twofold:\nof form first, and of content second.\nThe former is its determinateness of being external to the content as such,\nthe content that remains indifferent to this external reference.\nThe latter is the determinateness of the content that the ground has.';
MATCH (s:SourceText {id: 'source-absolute-ground'})
MATCH (c:IntegratedChunk {id: 'abs-16'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:abs-16'})
MATCH (c:IntegratedChunk {id: 'abs-16'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'abs-16'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'abs-16'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'abs-16:kp:1'})
SET kp.chunkId = 'abs-16'
SET kp.ordinal = 1
SET kp.text = 'Content of ground is ground that has returned into its unity with itself';
MATCH (c:IntegratedChunk {id: 'abs-16'})
MATCH (kp:KeyPoint {id: 'abs-16:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-16:kp:2'})
SET kp.chunkId = 'abs-16'
SET kp.ordinal = 2
SET kp.text = 'As content, ground is informed identity';
MATCH (c:IntegratedChunk {id: 'abs-16'})
MATCH (kp:KeyPoint {id: 'abs-16:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-16:kp:3'})
SET kp.chunkId = 'abs-16'
SET kp.ordinal = 3
SET kp.text = 'Content is like informed matter, determinations of form have material subsistence';
MATCH (c:IntegratedChunk {id: 'abs-16'})
MATCH (kp:KeyPoint {id: 'abs-16:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-16:kp:4'})
SET kp.chunkId = 'abs-16'
SET kp.ordinal = 4
SET kp.text = 'Content is essential self-identity of ground in its positedness';
MATCH (c:IntegratedChunk {id: 'abs-16'})
MATCH (kp:KeyPoint {id: 'abs-16:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'abs-16:kp:5'})
SET kp.chunkId = 'abs-16'
SET kp.ordinal = 5
SET kp.text = 'Ground has made itself into determinate ground, determinateness twofold: of form and of content';
MATCH (c:IntegratedChunk {id: 'abs-16'})
MATCH (kp:KeyPoint {id: 'abs-16:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
