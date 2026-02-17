MERGE (s:SourceText {id: 'source-determinate-ground'})
SET s.title = 'Determinate Ground'
SET s.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET s.totalLines = 397;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-determinate-ground'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:det-1'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 6
SET segment.lineEnd = 17
SET segment.text = 'The ground has a determinate content.\nFor the form, as we have seen,\nthe determinateness of content is the substrate,\nthe simple immediate as against the mediation of form.\nThe ground is negatively self-referring identity which,\nfor this reason, makes itself into a positedness;\nit negatively refers to itself because in its negativity\nit is identical with itself;\nthis identity is the substrate or the content\nwhich thus constitutes the indifferent\nor positive unity of the ground-connection\nand, in this connection, is the mediating factor.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-1'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-1'
SET topic.title = 'a. Formal ground — determinate content'
SET topic.description = 'Ground has determinate content. For form, determinateness of content is substrate, simple immediate as against mediation of form. Ground is negatively self-referring identity. Makes itself into positedness. Negatively refers to itself because in its negativity identical with itself. This identity is substrate or content. Constitutes indifferent or positive unity of ground-connection. In this connection, is mediating factor.'
SET topic.keyPoints = ['Ground has determinate content', 'Determinateness of content is substrate, simple immediate', 'Ground is negatively self-referring identity', 'This identity is substrate or content', 'Constitutes indifferent or positive unity of ground-connection'];
MATCH (segment:ChunkSegment {id: 'chunk:det-1'})
MATCH (topic:Topic {id: 'topic:det-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-1'})
SET c.title = 'a. Formal ground — determinate content'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 6
SET c.lineEnd = 17
SET c.description = 'Ground has determinate content. For form, determinateness of content is substrate, simple immediate as against mediation of form. Ground is negatively self-referring identity. Makes itself into positedness. Negatively refers to itself because in its negativity identical with itself. This identity is substrate or content. Constitutes indifferent or positive unity of ground-connection. In this connection, is mediating factor.'
SET c.keyPoints = ['Ground has determinate content', 'Determinateness of content is substrate, simple immediate', 'Ground is negatively self-referring identity', 'This identity is substrate or content', 'Constitutes indifferent or positive unity of ground-connection']
SET c.tags = ['negation', 'mediation', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 73
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The ground has a determinate content.\nFor the form, as we have seen,\nthe determinateness of content is the substrate,\nthe simple immediate as against the mediation of form.\nThe ground is negatively self-referring identity which,\nfor this reason, makes itself into a positedness;\nit negatively refers to itself because in its negativity\nit is identical with itself;\nthis identity is the substrate or the content\nwhich thus constitutes the indifferent\nor positive unity of the ground-connection\nand, in this connection, is the mediating factor.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-1'})
MATCH (c:IntegratedChunk {id: 'det-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-1:kp:1'})
SET kp.chunkId = 'det-1'
SET kp.ordinal = 1
SET kp.text = 'Ground has determinate content';
MATCH (c:IntegratedChunk {id: 'det-1'})
MATCH (kp:KeyPoint {id: 'det-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-1:kp:2'})
SET kp.chunkId = 'det-1'
SET kp.ordinal = 2
SET kp.text = 'Determinateness of content is substrate, simple immediate';
MATCH (c:IntegratedChunk {id: 'det-1'})
MATCH (kp:KeyPoint {id: 'det-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-1:kp:3'})
SET kp.chunkId = 'det-1'
SET kp.ordinal = 3
SET kp.text = 'Ground is negatively self-referring identity';
MATCH (c:IntegratedChunk {id: 'det-1'})
MATCH (kp:KeyPoint {id: 'det-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-1:kp:4'})
SET kp.chunkId = 'det-1'
SET kp.ordinal = 4
SET kp.text = 'This identity is substrate or content';
MATCH (c:IntegratedChunk {id: 'det-1'})
MATCH (kp:KeyPoint {id: 'det-1:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-1:kp:5'})
SET kp.chunkId = 'det-1'
SET kp.ordinal = 5
SET kp.text = 'Constitutes indifferent or positive unity of ground-connection';
MATCH (c:IntegratedChunk {id: 'det-1'})
MATCH (kp:KeyPoint {id: 'det-1:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-2'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 19
SET segment.lineEnd = 44
SET segment.text = 'In this content, the determinateness that\nthe ground and the grounded have over\nagainst one another has at first disappeared.\nThe mediation, however, is also negative unity.\nThe negative implicit in that indifferent substrate is\nthis substrate\'s immediate determinateness through which\nthe ground has a determinate content.\nBut then, the negative is the negative reference of form to itself.\nWhat has been posited sublates itself on its side\nand returns to its ground;\nthe ground, however, the essential self-subsistence,\nrefers negatively to itself and makes itself into a positedness.\nThis negative mediation of ground and grounded is\nthe mediation that belongs to form as such, formal mediation.\nNow both sides of form, because each passes over into the other,\nthereby mutually posit themselves into one identity as sublated;\nin this, they presuppose the identity.\nThe latter is the determinate content\nto which the formal mediation thus refers itself\nthrough itself as to the positive mediating factor.\nThat content is the identical element of both,\nand because the two are distinct,\nyet in their distinction each is\nthe reference to the other,\nit is their subsistence,\nthe subsistence of each as the whole itself.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-2'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-2'
SET topic.title = 'Formal mediation — negative and positive unity'
SET topic.description = 'In content, determinateness that ground and grounded have over against one another has at first disappeared. Mediation is also negative unity. Negative implicit in indifferent substrate is substrate\'s immediate determinateness. Through which ground has determinate content. Negative is negative reference of form to itself. Negative mediation of ground and grounded is formal mediation. Both sides of form mutually posit themselves into one identity as sublated. Presuppose identity. Latter is determinate content to which formal mediation refers itself as to positive mediating factor. Content is identical element of both, their subsistence, subsistence of each as whole itself.'
SET topic.keyPoints = ['Determinateness of ground and grounded has at first disappeared', 'Mediation is also negative unity', 'Negative mediation is formal mediation', 'Both sides mutually posit themselves into one identity as sublated', 'Content is identical element of both, subsistence of each as whole itself'];
MATCH (segment:ChunkSegment {id: 'chunk:det-2'})
MATCH (topic:Topic {id: 'topic:det-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-2'})
SET c.title = 'Formal mediation — negative and positive unity'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 19
SET c.lineEnd = 44
SET c.description = 'In content, determinateness that ground and grounded have over against one another has at first disappeared. Mediation is also negative unity. Negative implicit in indifferent substrate is substrate\'s immediate determinateness. Through which ground has determinate content. Negative is negative reference of form to itself. Negative mediation of ground and grounded is formal mediation. Both sides of form mutually posit themselves into one identity as sublated. Presuppose identity. Latter is determinate content to which formal mediation refers itself as to positive mediating factor. Content is identical element of both, their subsistence, subsistence of each as whole itself.'
SET c.keyPoints = ['Determinateness of ground and grounded has at first disappeared', 'Mediation is also negative unity', 'Negative mediation is formal mediation', 'Both sides mutually posit themselves into one identity as sublated', 'Content is identical element of both, subsistence of each as whole itself']
SET c.tags = ['negation', 'sublation', 'mediation', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 74
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'In this content, the determinateness that\nthe ground and the grounded have over\nagainst one another has at first disappeared.\nThe mediation, however, is also negative unity.\nThe negative implicit in that indifferent substrate is\nthis substrate\'s immediate determinateness through which\nthe ground has a determinate content.\nBut then, the negative is the negative reference of form to itself.\nWhat has been posited sublates itself on its side\nand returns to its ground;\nthe ground, however, the essential self-subsistence,\nrefers negatively to itself and makes itself into a positedness.\nThis negative mediation of ground and grounded is\nthe mediation that belongs to form as such, formal mediation.\nNow both sides of form, because each passes over into the other,\nthereby mutually posit themselves into one identity as sublated;\nin this, they presuppose the identity.\nThe latter is the determinate content\nto which the formal mediation thus refers itself\nthrough itself as to the positive mediating factor.\nThat content is the identical element of both,\nand because the two are distinct,\nyet in their distinction each is\nthe reference to the other,\nit is their subsistence,\nthe subsistence of each as the whole itself.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-2'})
MATCH (c:IntegratedChunk {id: 'det-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-2:kp:1'})
SET kp.chunkId = 'det-2'
SET kp.ordinal = 1
SET kp.text = 'Determinateness of ground and grounded has at first disappeared';
MATCH (c:IntegratedChunk {id: 'det-2'})
MATCH (kp:KeyPoint {id: 'det-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-2:kp:2'})
SET kp.chunkId = 'det-2'
SET kp.ordinal = 2
SET kp.text = 'Mediation is also negative unity';
MATCH (c:IntegratedChunk {id: 'det-2'})
MATCH (kp:KeyPoint {id: 'det-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-2:kp:3'})
SET kp.chunkId = 'det-2'
SET kp.ordinal = 3
SET kp.text = 'Negative mediation is formal mediation';
MATCH (c:IntegratedChunk {id: 'det-2'})
MATCH (kp:KeyPoint {id: 'det-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-2:kp:4'})
SET kp.chunkId = 'det-2'
SET kp.ordinal = 4
SET kp.text = 'Both sides mutually posit themselves into one identity as sublated';
MATCH (c:IntegratedChunk {id: 'det-2'})
MATCH (kp:KeyPoint {id: 'det-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-2:kp:5'})
SET kp.chunkId = 'det-2'
SET kp.ordinal = 5
SET kp.text = 'Content is identical element of both, subsistence of each as whole itself';
MATCH (c:IntegratedChunk {id: 'det-2'})
MATCH (kp:KeyPoint {id: 'det-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-3'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 46
SET segment.lineEnd = 76
SET segment.text = 'Accordingly, the result is that\nin the determinate ground we have the following.\nFirst, a determinate content is considered from two sides,\nonce in so far as it is ground,\nthen again in so far as it is grounded.\nThe content itself is indifferent to these forms;\nit is in each simply and solely one determination.\nSecond, the ground is itself just as much a moment of form\nas what is posited by it; this is its identity according to form.\nIt is a matter of indifference which of\nthe two determinations is made the first,\nwhether the transition is\nfrom the one as posited to the other as ground\nor from the one as ground to the other as posited.\nThe grounded, considered for itself, is the sublating of itself;\nit thereby makes itself on the one side into a posited,\nand is at the same time the positing of the ground.\nThe same movement is the ground as such;\nit makes itself into something posited,\nand thereby becomes the ground of something,\nthat is to say, is present therein\nboth as a posited and also first as ground.\nThat there be a ground, of that the posited is the ground,\nand, conversely, the ground is thereby the posited.\nThe mediation begins just as much from the one as from the other;\neach side is just as much ground as posited,\nand each is the whole mediation or the whole form.\nFurther, this whole form is itself, as self-identical,\nthe substrate of the two determinations\nthat constitute the two sides of the ground and the grounded;\nform and content are thus themselves one and the same identity.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-3'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-3'
SET topic.title = 'Determinate ground — ground and grounded'
SET topic.description = 'In determinate ground: first, determinate content considered from two sides. Once as ground, then as grounded. Content indifferent to these forms, in each simply one determination. Second, ground is itself just as much moment of form as what is posited by it. Matter of indifference which determination is made first. Grounded is sublating of itself, makes itself into posited, and is positing of ground. Same movement is ground as such. Makes itself into posited, thereby becomes ground. Each side is just as much ground as posited, and each is whole mediation or whole form. Form and content are one and same identity.'
SET topic.keyPoints = ['Determinate content considered from two sides: as ground and as grounded', 'Ground is itself moment of form as what is posited by it', 'Each side is just as much ground as posited', 'Each is whole mediation or whole form', 'Form and content are one and same identity'];
MATCH (segment:ChunkSegment {id: 'chunk:det-3'})
MATCH (topic:Topic {id: 'topic:det-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-3'})
SET c.title = 'Determinate ground — ground and grounded'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 46
SET c.lineEnd = 76
SET c.description = 'In determinate ground: first, determinate content considered from two sides. Once as ground, then as grounded. Content indifferent to these forms, in each simply one determination. Second, ground is itself just as much moment of form as what is posited by it. Matter of indifference which determination is made first. Grounded is sublating of itself, makes itself into posited, and is positing of ground. Same movement is ground as such. Makes itself into posited, thereby becomes ground. Each side is just as much ground as posited, and each is whole mediation or whole form. Form and content are one and same identity.'
SET c.keyPoints = ['Determinate content considered from two sides: as ground and as grounded', 'Ground is itself moment of form as what is posited by it', 'Each side is just as much ground as posited', 'Each is whole mediation or whole form', 'Form and content are one and same identity']
SET c.tags = ['sublation', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 75
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Accordingly, the result is that\nin the determinate ground we have the following.\nFirst, a determinate content is considered from two sides,\nonce in so far as it is ground,\nthen again in so far as it is grounded.\nThe content itself is indifferent to these forms;\nit is in each simply and solely one determination.\nSecond, the ground is itself just as much a moment of form\nas what is posited by it; this is its identity according to form.\nIt is a matter of indifference which of\nthe two determinations is made the first,\nwhether the transition is\nfrom the one as posited to the other as ground\nor from the one as ground to the other as posited.\nThe grounded, considered for itself, is the sublating of itself;\nit thereby makes itself on the one side into a posited,\nand is at the same time the positing of the ground.\nThe same movement is the ground as such;\nit makes itself into something posited,\nand thereby becomes the ground of something,\nthat is to say, is present therein\nboth as a posited and also first as ground.\nThat there be a ground, of that the posited is the ground,\nand, conversely, the ground is thereby the posited.\nThe mediation begins just as much from the one as from the other;\neach side is just as much ground as posited,\nand each is the whole mediation or the whole form.\nFurther, this whole form is itself, as self-identical,\nthe substrate of the two determinations\nthat constitute the two sides of the ground and the grounded;\nform and content are thus themselves one and the same identity.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-3'})
MATCH (c:IntegratedChunk {id: 'det-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-3:kp:1'})
SET kp.chunkId = 'det-3'
SET kp.ordinal = 1
SET kp.text = 'Determinate content considered from two sides: as ground and as grounded';
MATCH (c:IntegratedChunk {id: 'det-3'})
MATCH (kp:KeyPoint {id: 'det-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-3:kp:2'})
SET kp.chunkId = 'det-3'
SET kp.ordinal = 2
SET kp.text = 'Ground is itself moment of form as what is posited by it';
MATCH (c:IntegratedChunk {id: 'det-3'})
MATCH (kp:KeyPoint {id: 'det-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-3:kp:3'})
SET kp.chunkId = 'det-3'
SET kp.ordinal = 3
SET kp.text = 'Each side is just as much ground as posited';
MATCH (c:IntegratedChunk {id: 'det-3'})
MATCH (kp:KeyPoint {id: 'det-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-3:kp:4'})
SET kp.chunkId = 'det-3'
SET kp.ordinal = 4
SET kp.text = 'Each is whole mediation or whole form';
MATCH (c:IntegratedChunk {id: 'det-3'})
MATCH (kp:KeyPoint {id: 'det-3:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-3:kp:5'})
SET kp.chunkId = 'det-3'
SET kp.ordinal = 5
SET kp.text = 'Form and content are one and same identity';
MATCH (c:IntegratedChunk {id: 'det-3'})
MATCH (kp:KeyPoint {id: 'det-3:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-4'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 78
SET segment.lineEnd = 103
SET segment.text = 'Because of this identity of the ground and the grounded,\naccording both to content and form,\nthe ground is sufficient\n(the sufficiency being limited to this relation);\nthere is nothing in the grounded which is not in the ground.\nWhenever one asks for a ground,\none expects to see the same determination\nwhich is the content doubled,\nonce in the form of that which is posited,\nand again in the form of existence\nreflected into itself, of essentiality.\n\nNow inasmuch as in the determined ground,\nthe ground and the grounded are each the whole form, and their content,\nthough determinate, is nevertheless one and the same,\nthe two sides of the ground do not as yet have a real determination,\ndo not have a different content;\nthe determinateness is only one simple determinateness\nthat has yet to pass over into the two sides;\nthe determinate ground is present\nonly in its pure form, as formal ground.\nBecause the content is only this simple determinateness,\none that does not have in it the form of the ground-connection,\nthe determinateness is a self-identical content indifferent to form,\nand the form is external to it;\nthe content is other than the form.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-4'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-4'
SET topic.title = 'Ground is sufficient — formal ground'
SET topic.description = 'Because of identity of ground and grounded, according both to content and form, ground is sufficient. Nothing in grounded which is not in ground. When one asks for ground, expects to see same determination doubled: once as posited, again as existence reflected into itself. In determined ground, ground and grounded are each whole form, content is one and same. Two sides do not have real determination, do not have different content. Determinateness is only one simple determinateness. Determinate ground present only in pure form, as formal ground. Content is self-identical content indifferent to form. Form is external to it, content is other than form.'
SET topic.keyPoints = ['Ground is sufficient, nothing in grounded which is not in ground', 'Same determination doubled: as posited and as existence reflected into itself', 'Two sides do not have real determination, do not have different content', 'Determinate ground present only in pure form, as formal ground', 'Content is self-identical content indifferent to form'];
MATCH (segment:ChunkSegment {id: 'chunk:det-4'})
MATCH (topic:Topic {id: 'topic:det-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-4'})
SET c.title = 'Ground is sufficient — formal ground'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 78
SET c.lineEnd = 103
SET c.description = 'Because of identity of ground and grounded, according both to content and form, ground is sufficient. Nothing in grounded which is not in ground. When one asks for ground, expects to see same determination doubled: once as posited, again as existence reflected into itself. In determined ground, ground and grounded are each whole form, content is one and same. Two sides do not have real determination, do not have different content. Determinateness is only one simple determinateness. Determinate ground present only in pure form, as formal ground. Content is self-identical content indifferent to form. Form is external to it, content is other than form.'
SET c.keyPoints = ['Ground is sufficient, nothing in grounded which is not in ground', 'Same determination doubled: as posited and as existence reflected into itself', 'Two sides do not have real determination, do not have different content', 'Determinate ground present only in pure form, as formal ground', 'Content is self-identical content indifferent to form']
SET c.tags = ['reflection', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 76
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Because of this identity of the ground and the grounded,\naccording both to content and form,\nthe ground is sufficient\n(the sufficiency being limited to this relation);\nthere is nothing in the grounded which is not in the ground.\nWhenever one asks for a ground,\none expects to see the same determination\nwhich is the content doubled,\nonce in the form of that which is posited,\nand again in the form of existence\nreflected into itself, of essentiality.\n\nNow inasmuch as in the determined ground,\nthe ground and the grounded are each the whole form, and their content,\nthough determinate, is nevertheless one and the same,\nthe two sides of the ground do not as yet have a real determination,\ndo not have a different content;\nthe determinateness is only one simple determinateness\nthat has yet to pass over into the two sides;\nthe determinate ground is present\nonly in its pure form, as formal ground.\nBecause the content is only this simple determinateness,\none that does not have in it the form of the ground-connection,\nthe determinateness is a self-identical content indifferent to form,\nand the form is external to it;\nthe content is other than the form.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-4'})
MATCH (c:IntegratedChunk {id: 'det-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-4:kp:1'})
SET kp.chunkId = 'det-4'
SET kp.ordinal = 1
SET kp.text = 'Ground is sufficient, nothing in grounded which is not in ground';
MATCH (c:IntegratedChunk {id: 'det-4'})
MATCH (kp:KeyPoint {id: 'det-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-4:kp:2'})
SET kp.chunkId = 'det-4'
SET kp.ordinal = 2
SET kp.text = 'Same determination doubled: as posited and as existence reflected into itself';
MATCH (c:IntegratedChunk {id: 'det-4'})
MATCH (kp:KeyPoint {id: 'det-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-4:kp:3'})
SET kp.chunkId = 'det-4'
SET kp.ordinal = 3
SET kp.text = 'Two sides do not have real determination, do not have different content';
MATCH (c:IntegratedChunk {id: 'det-4'})
MATCH (kp:KeyPoint {id: 'det-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-4:kp:4'})
SET kp.chunkId = 'det-4'
SET kp.ordinal = 4
SET kp.text = 'Determinate ground present only in pure form, as formal ground';
MATCH (c:IntegratedChunk {id: 'det-4'})
MATCH (kp:KeyPoint {id: 'det-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-4:kp:5'})
SET kp.chunkId = 'det-4'
SET kp.ordinal = 5
SET kp.text = 'Content is self-identical content indifferent to form';
MATCH (c:IntegratedChunk {id: 'det-4'})
MATCH (kp:KeyPoint {id: 'det-4:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-5'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 107
SET segment.lineEnd = 133
SET segment.text = 'The determinateness of ground is, as we have seen,\non the one hand determinateness of the substrate\nor content determination;\non the other hand,\nit is the otherness in the ground-connection itself,\nnamely the distinctness of its content and the form;\nthe connection of ground and grounded strays\nin the content as an external form,\nand the content is indifferent to these determinations.\nBut in fact the two are not external to each other;\nfor this is what the content is:\nto be the identity of the ground\nwith itself in the grounded,\nand of the grounded in the ground.\nThe side of the ground\nhas shown itself to be itself a posited,\nand the side of the grounded to be\nitself ground;\neach side is this identity of the whole within it.\nBut since they equally belong to form\nand constitute its determinate difference,\neach is in its determinateness the identity of the whole with itself.\nConsequently, each has a diverse content as against the other.\nOr, considering the matter from the side of the content,\nsince the latter is the self-identity of the ground-connection,\nit essentially possesses this difference of form within,\nand is as ground something other than what it is as grounded.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-5'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-5'
SET topic.title = 'b. Real ground — diverse content'
SET topic.description = 'Determinateness of ground is determinateness of substrate or content determination, and otherness in ground-connection itself, distinctness of content and form. Connection strays in content as external form, content indifferent to determinations. But two are not external to each other. Content is: identity of ground with itself in grounded, and of grounded in ground. Each side is identity of whole within it. But since they belong to form and constitute its determinate difference, each is in its determinateness identity of whole with itself. Consequently, each has diverse content as against other. Content essentially possesses difference of form within. Is as ground something other than what it is as grounded.'
SET topic.keyPoints = ['Determinateness of ground is determinateness of substrate and otherness in ground-connection', 'Content is identity of ground with itself in grounded', 'Each side is identity of whole within it', 'Each has diverse content as against other', 'Content essentially possesses difference of form within'];
MATCH (segment:ChunkSegment {id: 'chunk:det-5'})
MATCH (topic:Topic {id: 'topic:det-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-5'})
SET c.title = 'b. Real ground — diverse content'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 107
SET c.lineEnd = 133
SET c.description = 'Determinateness of ground is determinateness of substrate or content determination, and otherness in ground-connection itself, distinctness of content and form. Connection strays in content as external form, content indifferent to determinations. But two are not external to each other. Content is: identity of ground with itself in grounded, and of grounded in ground. Each side is identity of whole within it. But since they belong to form and constitute its determinate difference, each is in its determinateness identity of whole with itself. Consequently, each has diverse content as against other. Content essentially possesses difference of form within. Is as ground something other than what it is as grounded.'
SET c.keyPoints = ['Determinateness of ground is determinateness of substrate and otherness in ground-connection', 'Content is identity of ground with itself in grounded', 'Each side is identity of whole within it', 'Each has diverse content as against other', 'Content essentially possesses difference of form within']
SET c.tags = ['citta']
SET c.orderInSource = 5
SET c.globalOrder = 77
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The determinateness of ground is, as we have seen,\non the one hand determinateness of the substrate\nor content determination;\non the other hand,\nit is the otherness in the ground-connection itself,\nnamely the distinctness of its content and the form;\nthe connection of ground and grounded strays\nin the content as an external form,\nand the content is indifferent to these determinations.\nBut in fact the two are not external to each other;\nfor this is what the content is:\nto be the identity of the ground\nwith itself in the grounded,\nand of the grounded in the ground.\nThe side of the ground\nhas shown itself to be itself a posited,\nand the side of the grounded to be\nitself ground;\neach side is this identity of the whole within it.\nBut since they equally belong to form\nand constitute its determinate difference,\neach is in its determinateness the identity of the whole with itself.\nConsequently, each has a diverse content as against the other.\nOr, considering the matter from the side of the content,\nsince the latter is the self-identity of the ground-connection,\nit essentially possesses this difference of form within,\nand is as ground something other than what it is as grounded.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-5'})
MATCH (c:IntegratedChunk {id: 'det-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-5:kp:1'})
SET kp.chunkId = 'det-5'
SET kp.ordinal = 1
SET kp.text = 'Determinateness of ground is determinateness of substrate and otherness in ground-connection';
MATCH (c:IntegratedChunk {id: 'det-5'})
MATCH (kp:KeyPoint {id: 'det-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-5:kp:2'})
SET kp.chunkId = 'det-5'
SET kp.ordinal = 2
SET kp.text = 'Content is identity of ground with itself in grounded';
MATCH (c:IntegratedChunk {id: 'det-5'})
MATCH (kp:KeyPoint {id: 'det-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-5:kp:3'})
SET kp.chunkId = 'det-5'
SET kp.ordinal = 3
SET kp.text = 'Each side is identity of whole within it';
MATCH (c:IntegratedChunk {id: 'det-5'})
MATCH (kp:KeyPoint {id: 'det-5:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-5:kp:4'})
SET kp.chunkId = 'det-5'
SET kp.ordinal = 4
SET kp.text = 'Each has diverse content as against other';
MATCH (c:IntegratedChunk {id: 'det-5'})
MATCH (kp:KeyPoint {id: 'det-5:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-5:kp:5'})
SET kp.chunkId = 'det-5'
SET kp.ordinal = 5
SET kp.text = 'Content essentially possesses difference of form within';
MATCH (c:IntegratedChunk {id: 'det-5'})
MATCH (kp:KeyPoint {id: 'det-5:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-6'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 135
SET segment.lineEnd = 142
SET segment.text = 'Now the moment ground and grounded have a diverse content,\nthe ground-connection has ceased to be a formal one;\nthe turning back to the ground and\nthe procession forward from ground to posited\nis no longer a tautology; the ground is realized.\nHenceforth, whenever we ask for a ground,\nwe actually demand another content determination for it\nthan the determination of the content whose ground we are asking for.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-6'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-6'
SET topic.title = 'Ground realized — no longer tautology'
SET topic.description = 'Moment ground and grounded have diverse content, ground-connection has ceased to be formal one. Turning back to ground and procession forward from ground to posited is no longer tautology. Ground is realized. Henceforth, when we ask for ground, we actually demand another content determination for it than determination of content whose ground we are asking for.'
SET topic.keyPoints = ['Ground and grounded have diverse content', 'Ground-connection has ceased to be formal', 'No longer tautology, ground is realized', 'We demand another content determination for ground'];
MATCH (segment:ChunkSegment {id: 'chunk:det-6'})
MATCH (topic:Topic {id: 'topic:det-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-6'})
SET c.title = 'Ground realized — no longer tautology'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 135
SET c.lineEnd = 142
SET c.description = 'Moment ground and grounded have diverse content, ground-connection has ceased to be formal one. Turning back to ground and procession forward from ground to posited is no longer tautology. Ground is realized. Henceforth, when we ask for ground, we actually demand another content determination for it than determination of content whose ground we are asking for.'
SET c.keyPoints = ['Ground and grounded have diverse content', 'Ground-connection has ceased to be formal', 'No longer tautology, ground is realized', 'We demand another content determination for ground']
SET c.tags = ['citta']
SET c.orderInSource = 6
SET c.globalOrder = 78
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Now the moment ground and grounded have a diverse content,\nthe ground-connection has ceased to be a formal one;\nthe turning back to the ground and\nthe procession forward from ground to posited\nis no longer a tautology; the ground is realized.\nHenceforth, whenever we ask for a ground,\nwe actually demand another content determination for it\nthan the determination of the content whose ground we are asking for.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-6'})
MATCH (c:IntegratedChunk {id: 'det-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-6:kp:1'})
SET kp.chunkId = 'det-6'
SET kp.ordinal = 1
SET kp.text = 'Ground and grounded have diverse content';
MATCH (c:IntegratedChunk {id: 'det-6'})
MATCH (kp:KeyPoint {id: 'det-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-6:kp:2'})
SET kp.chunkId = 'det-6'
SET kp.ordinal = 2
SET kp.text = 'Ground-connection has ceased to be formal';
MATCH (c:IntegratedChunk {id: 'det-6'})
MATCH (kp:KeyPoint {id: 'det-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-6:kp:3'})
SET kp.chunkId = 'det-6'
SET kp.ordinal = 3
SET kp.text = 'No longer tautology, ground is realized';
MATCH (c:IntegratedChunk {id: 'det-6'})
MATCH (kp:KeyPoint {id: 'det-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-6:kp:4'})
SET kp.chunkId = 'det-6'
SET kp.ordinal = 4
SET kp.text = 'We demand another content determination for ground';
MATCH (c:IntegratedChunk {id: 'det-6'})
MATCH (kp:KeyPoint {id: 'det-6:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-7'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 144
SET segment.lineEnd = 163
SET segment.text = 'This connection now determines itself further.\nFor inasmuch as its two sides are of different content,\nthey are indifferent to each other;\neach is an immediate, self-identical determination.\nMoreover, as referred to each other as ground and grounded,\nthe ground reflects itself in the other,\nas in something posited by it, back to itself;\nthe content on the side of the ground,\ntherefore, is equally in the grounded;\nthe latter, as the posited, has its\nself-identity and subsistence only in the ground.\nBut besides this content of the ground,\nthe grounded also now possesses a content of its own\nand is accordingly the unity of a twofold content.\nNow this unity, as the unity of sides that are different,\nis indeed their negative unity;\nbut since the two determinations of content are indifferent to each other,\nthat unity is only their empty reference to each other,\nin itself void of content, and not their mediation;\nit is a one or a something externally holding them together.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-7'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-7'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-7'
SET topic.title = 'Connection determines itself — twofold content'
SET topic.description = 'Connection determines itself further. Two sides are of different content, indifferent to each other. Each is immediate, self-identical determination. Ground reflects itself in other, back to itself. Content on side of ground is equally in grounded. Latter has self-identity and subsistence only in ground. But grounded also possesses content of its own. Is unity of twofold content. Unity is negative unity, but since determinations are indifferent, unity is only empty reference to each other, void of content, not their mediation. It is something externally holding them together.'
SET topic.keyPoints = ['Two sides are of different content, indifferent to each other', 'Ground reflects itself in other, back to itself', 'Grounded possesses content of its own, is unity of twofold content', 'Unity is only empty reference, void of content, not their mediation', 'Something externally holding them together'];
MATCH (segment:ChunkSegment {id: 'chunk:det-7'})
MATCH (topic:Topic {id: 'topic:det-7'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-7'})
SET c.title = 'Connection determines itself — twofold content'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 144
SET c.lineEnd = 163
SET c.description = 'Connection determines itself further. Two sides are of different content, indifferent to each other. Each is immediate, self-identical determination. Ground reflects itself in other, back to itself. Content on side of ground is equally in grounded. Latter has self-identity and subsistence only in ground. But grounded also possesses content of its own. Is unity of twofold content. Unity is negative unity, but since determinations are indifferent, unity is only empty reference to each other, void of content, not their mediation. It is something externally holding them together.'
SET c.keyPoints = ['Two sides are of different content, indifferent to each other', 'Ground reflects itself in other, back to itself', 'Grounded possesses content of its own, is unity of twofold content', 'Unity is only empty reference, void of content, not their mediation', 'Something externally holding them together']
SET c.tags = ['negation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 7
SET c.globalOrder = 79
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'This connection now determines itself further.\nFor inasmuch as its two sides are of different content,\nthey are indifferent to each other;\neach is an immediate, self-identical determination.\nMoreover, as referred to each other as ground and grounded,\nthe ground reflects itself in the other,\nas in something posited by it, back to itself;\nthe content on the side of the ground,\ntherefore, is equally in the grounded;\nthe latter, as the posited, has its\nself-identity and subsistence only in the ground.\nBut besides this content of the ground,\nthe grounded also now possesses a content of its own\nand is accordingly the unity of a twofold content.\nNow this unity, as the unity of sides that are different,\nis indeed their negative unity;\nbut since the two determinations of content are indifferent to each other,\nthat unity is only their empty reference to each other,\nin itself void of content, and not their mediation;\nit is a one or a something externally holding them together.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-7'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-7'})
MATCH (c:IntegratedChunk {id: 'det-7'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-7'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-7'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-7:kp:1'})
SET kp.chunkId = 'det-7'
SET kp.ordinal = 1
SET kp.text = 'Two sides are of different content, indifferent to each other';
MATCH (c:IntegratedChunk {id: 'det-7'})
MATCH (kp:KeyPoint {id: 'det-7:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-7:kp:2'})
SET kp.chunkId = 'det-7'
SET kp.ordinal = 2
SET kp.text = 'Ground reflects itself in other, back to itself';
MATCH (c:IntegratedChunk {id: 'det-7'})
MATCH (kp:KeyPoint {id: 'det-7:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-7:kp:3'})
SET kp.chunkId = 'det-7'
SET kp.ordinal = 3
SET kp.text = 'Grounded possesses content of its own, is unity of twofold content';
MATCH (c:IntegratedChunk {id: 'det-7'})
MATCH (kp:KeyPoint {id: 'det-7:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-7:kp:4'})
SET kp.chunkId = 'det-7'
SET kp.ordinal = 4
SET kp.text = 'Unity is only empty reference, void of content, not their mediation';
MATCH (c:IntegratedChunk {id: 'det-7'})
MATCH (kp:KeyPoint {id: 'det-7:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-7:kp:5'})
SET kp.chunkId = 'det-7'
SET kp.ordinal = 5
SET kp.text = 'Something externally holding them together';
MATCH (c:IntegratedChunk {id: 'det-7'})
MATCH (kp:KeyPoint {id: 'det-7:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-8'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 165
SET segment.lineEnd = 205
SET segment.text = 'In the real grounding connection\nthere is present, therefore, a twofold.\nFor one thing, the content determination which is ground\nextends continuously into the positedness,\nso that it constitutes the simple identity\nof the ground and the grounded;\nthe grounded thus contains the ground\nfully within itself;\ntheir connection is one of\nundifferentiated essential compactness.\nAnything else in the grounded\nadded to this simple essence is,\ntherefore, only an unessential form,\nexternal determinations of the content\nthat, as such, are free from the ground\nand constitute an immediate manifold.\nOf this unessential more, therefore,\nthe essential is not the ground,\nnor is it the ground of any connection\nbetween it and the unessential in the grounded.\nThe unessential is a positively identical element\nthat resides in the grounded but does not posit itself\nthere in any distinctive form;\nas self-referring content, it is rather\nan indifferent positive substrate.\nFor another thing, that which in the something is\nlinked with this substrate is an indifferent content,\nbut as the unessential side.\nThe main thing is the connection of the substrate\nand the unessential manifold.\nBut this connection, since the determinations\nthat it connects are an indifferent content,\nis also not a ground;\ntrue, one determination is determined as essential content\nand the other as only unessential or as posited;\nbut this form is to each, as a self-referring content, an external one.\nThe one of the something that constitutes their connection is\nfor this reason not a reference of form,\nbut only an external tie that does not hold\nthe unessential manifold content as posited;\nit too is therefore likewise only a substrate.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-8'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-8'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-8'
SET topic.title = 'Real grounding connection — twofold'
SET topic.description = 'In real grounding connection there is present a twofold. For one thing, content determination which is ground extends continuously into positedness. Constitutes simple identity of ground and grounded. Grounded contains ground fully within itself. Connection is undifferentiated essential compactness. Anything else in grounded added is only unessential form, external determinations free from ground, immediate manifold. Unessential is positively identical element, indifferent positive substrate. For another thing, that linked with substrate is indifferent content, but as unessential side. Main thing is connection of substrate and unessential manifold. But this connection is also not ground. One of something that constitutes connection is not reference of form but only external tie. It too is only substrate.'
SET topic.keyPoints = ['Content determination which is ground extends continuously into positedness', 'Grounded contains ground fully within itself', 'Anything else is only unessential form, immediate manifold', 'Unessential is indifferent positive substrate', 'Connection is not ground, only external tie, only substrate'];
MATCH (segment:ChunkSegment {id: 'chunk:det-8'})
MATCH (topic:Topic {id: 'topic:det-8'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-8'})
SET c.title = 'Real grounding connection — twofold'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 165
SET c.lineEnd = 205
SET c.description = 'In real grounding connection there is present a twofold. For one thing, content determination which is ground extends continuously into positedness. Constitutes simple identity of ground and grounded. Grounded contains ground fully within itself. Connection is undifferentiated essential compactness. Anything else in grounded added is only unessential form, external determinations free from ground, immediate manifold. Unessential is positively identical element, indifferent positive substrate. For another thing, that linked with substrate is indifferent content, but as unessential side. Main thing is connection of substrate and unessential manifold. But this connection is also not ground. One of something that constitutes connection is not reference of form but only external tie. It too is only substrate.'
SET c.keyPoints = ['Content determination which is ground extends continuously into positedness', 'Grounded contains ground fully within itself', 'Anything else is only unessential form, immediate manifold', 'Unessential is indifferent positive substrate', 'Connection is not ground, only external tie, only substrate']
SET c.tags = ['mediation', 'citta']
SET c.orderInSource = 8
SET c.globalOrder = 80
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'In the real grounding connection\nthere is present, therefore, a twofold.\nFor one thing, the content determination which is ground\nextends continuously into the positedness,\nso that it constitutes the simple identity\nof the ground and the grounded;\nthe grounded thus contains the ground\nfully within itself;\ntheir connection is one of\nundifferentiated essential compactness.\nAnything else in the grounded\nadded to this simple essence is,\ntherefore, only an unessential form,\nexternal determinations of the content\nthat, as such, are free from the ground\nand constitute an immediate manifold.\nOf this unessential more, therefore,\nthe essential is not the ground,\nnor is it the ground of any connection\nbetween it and the unessential in the grounded.\nThe unessential is a positively identical element\nthat resides in the grounded but does not posit itself\nthere in any distinctive form;\nas self-referring content, it is rather\nan indifferent positive substrate.\nFor another thing, that which in the something is\nlinked with this substrate is an indifferent content,\nbut as the unessential side.\nThe main thing is the connection of the substrate\nand the unessential manifold.\nBut this connection, since the determinations\nthat it connects are an indifferent content,\nis also not a ground;\ntrue, one determination is determined as essential content\nand the other as only unessential or as posited;\nbut this form is to each, as a self-referring content, an external one.\nThe one of the something that constitutes their connection is\nfor this reason not a reference of form,\nbut only an external tie that does not hold\nthe unessential manifold content as posited;\nit too is therefore likewise only a substrate.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-8'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-8'})
MATCH (c:IntegratedChunk {id: 'det-8'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-8'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-8'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-8:kp:1'})
SET kp.chunkId = 'det-8'
SET kp.ordinal = 1
SET kp.text = 'Content determination which is ground extends continuously into positedness';
MATCH (c:IntegratedChunk {id: 'det-8'})
MATCH (kp:KeyPoint {id: 'det-8:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-8:kp:2'})
SET kp.chunkId = 'det-8'
SET kp.ordinal = 2
SET kp.text = 'Grounded contains ground fully within itself';
MATCH (c:IntegratedChunk {id: 'det-8'})
MATCH (kp:KeyPoint {id: 'det-8:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-8:kp:3'})
SET kp.chunkId = 'det-8'
SET kp.ordinal = 3
SET kp.text = 'Anything else is only unessential form, immediate manifold';
MATCH (c:IntegratedChunk {id: 'det-8'})
MATCH (kp:KeyPoint {id: 'det-8:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-8:kp:4'})
SET kp.chunkId = 'det-8'
SET kp.ordinal = 4
SET kp.text = 'Unessential is indifferent positive substrate';
MATCH (c:IntegratedChunk {id: 'det-8'})
MATCH (kp:KeyPoint {id: 'det-8:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-8:kp:5'})
SET kp.chunkId = 'det-8'
SET kp.ordinal = 5
SET kp.text = 'Connection is not ground, only external tie, only substrate';
MATCH (c:IntegratedChunk {id: 'det-8'})
MATCH (kp:KeyPoint {id: 'det-8:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-9'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 207
SET segment.lineEnd = 229
SET segment.text = 'Ground, in determining itself as real,\nbecause of the diversity of the content\nthat constitutes its reality,\nthus breaks down into external determinations.\nThe two connections of the essential reality content,\nas the simple immediate identity of ground and grounded;\nand then the something connecting distinct contents\nare two different substrates.\nThe self-identical form of ground,\naccording to which one and the same thing\nis at one time the essential\nand at another the posited, has vanished.\nThe ground-connection has thus become external to itself.\n\nConsequently, it is an external ground that now\nholds together a diversified content\nand determines what is ground and what is posited by it;\nthis determination is not to be found in the two-sided content itself.\nThe real ground is therefore the reference to another,\non the one hand, of a content to another content\nand, on the other, of the ground-connection itself\n(the form) to another, namely to an immediate,\nto something not posited by it.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-9'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-9'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-9'
SET topic.title = 'Ground breaks down — external ground'
SET topic.description = 'Ground, in determining itself as real, because of diversity of content, breaks down into external determinations. Two connections: essential reality content as simple immediate identity, and something connecting distinct contents are two different substrates. Self-identical form of ground has vanished. Ground-connection has become external to itself. External ground holds together diversified content. Determines what is ground and what is posited by it. Determination not found in two-sided content itself. Real ground is reference to another: of content to another content, and of ground-connection (form) to immediate, to something not posited by it.'
SET topic.keyPoints = ['Ground breaks down into external determinations', 'Two connections are two different substrates', 'Self-identical form of ground has vanished', 'External ground holds together diversified content', 'Real ground is reference to another'];
MATCH (segment:ChunkSegment {id: 'chunk:det-9'})
MATCH (topic:Topic {id: 'topic:det-9'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-9'})
SET c.title = 'Ground breaks down — external ground'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 207
SET c.lineEnd = 229
SET c.description = 'Ground, in determining itself as real, because of diversity of content, breaks down into external determinations. Two connections: essential reality content as simple immediate identity, and something connecting distinct contents are two different substrates. Self-identical form of ground has vanished. Ground-connection has become external to itself. External ground holds together diversified content. Determines what is ground and what is posited by it. Determination not found in two-sided content itself. Real ground is reference to another: of content to another content, and of ground-connection (form) to immediate, to something not posited by it.'
SET c.keyPoints = ['Ground breaks down into external determinations', 'Two connections are two different substrates', 'Self-identical form of ground has vanished', 'External ground holds together diversified content', 'Real ground is reference to another']
SET c.tags = ['mediation', 'citta']
SET c.orderInSource = 9
SET c.globalOrder = 81
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Ground, in determining itself as real,\nbecause of the diversity of the content\nthat constitutes its reality,\nthus breaks down into external determinations.\nThe two connections of the essential reality content,\nas the simple immediate identity of ground and grounded;\nand then the something connecting distinct contents\nare two different substrates.\nThe self-identical form of ground,\naccording to which one and the same thing\nis at one time the essential\nand at another the posited, has vanished.\nThe ground-connection has thus become external to itself.\n\nConsequently, it is an external ground that now\nholds together a diversified content\nand determines what is ground and what is posited by it;\nthis determination is not to be found in the two-sided content itself.\nThe real ground is therefore the reference to another,\non the one hand, of a content to another content\nand, on the other, of the ground-connection itself\n(the form) to another, namely to an immediate,\nto something not posited by it.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-9'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-9'})
MATCH (c:IntegratedChunk {id: 'det-9'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-9'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-9'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-9:kp:1'})
SET kp.chunkId = 'det-9'
SET kp.ordinal = 1
SET kp.text = 'Ground breaks down into external determinations';
MATCH (c:IntegratedChunk {id: 'det-9'})
MATCH (kp:KeyPoint {id: 'det-9:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-9:kp:2'})
SET kp.chunkId = 'det-9'
SET kp.ordinal = 2
SET kp.text = 'Two connections are two different substrates';
MATCH (c:IntegratedChunk {id: 'det-9'})
MATCH (kp:KeyPoint {id: 'det-9:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-9:kp:3'})
SET kp.chunkId = 'det-9'
SET kp.ordinal = 3
SET kp.text = 'Self-identical form of ground has vanished';
MATCH (c:IntegratedChunk {id: 'det-9'})
MATCH (kp:KeyPoint {id: 'det-9:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-9:kp:4'})
SET kp.chunkId = 'det-9'
SET kp.ordinal = 4
SET kp.text = 'External ground holds together diversified content';
MATCH (c:IntegratedChunk {id: 'det-9'})
MATCH (kp:KeyPoint {id: 'det-9:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-9:kp:5'})
SET kp.chunkId = 'det-9'
SET kp.ordinal = 5
SET kp.text = 'Real ground is reference to another';
MATCH (c:IntegratedChunk {id: 'det-9'})
MATCH (kp:KeyPoint {id: 'det-9:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-10'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 233
SET segment.lineEnd = 254
SET segment.text = '1. In real ground, ground as content\nand ground as connection are only substrates.\nThe former is only posited as essential and as ground;\nthe connection is what the grounded immediately is\nas the indeterminate substrate of a diversified content,\na linking of this content which is not the content\'s own reflection\nbut is rather external and consequently a reflection which is only posited.\nThe real ground-connection is ground, therefore, rather as sublated;\nconsequently, it rather makes up the side of\nthe grounded or of the positedness.\nAs positedness, however, the ground itself\nhas now returned to its ground;\nit is now something grounded: it has another ground.\nThis ground will therefore be so determined that,\nfirst, it is identical with the ground by which it is grounded;\nboth sides have in this determination one and the same content;\nthe two content determinations and their linkage in\na something are equally to be found in the new ground.\nBut, second, the new ground into which\nthe previously merely posited and external link\nis now sublated is the immanent reflection of this link:\nthe absolute reference of the two content determinations to each other.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-10'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-10'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-10'
SET topic.title = 'c. Complete ground — real ground returns to ground'
SET topic.description = 'In real ground, ground as content and ground as connection are only substrates. Real ground-connection is ground rather as sublated. Makes up side of grounded or positedness. As positedness, ground itself has returned to its ground. Is now something grounded: it has another ground. This ground determined: first, identical with ground by which it is grounded, both sides have same content. Second, new ground is immanent reflection of link: absolute reference of two content determinations to each other.'
SET topic.keyPoints = ['Ground as content and ground as connection are only substrates', 'Real ground-connection is ground rather as sublated', 'Ground itself has returned to its ground', 'New ground is immanent reflection of link', 'Absolute reference of two content determinations to each other'];
MATCH (segment:ChunkSegment {id: 'chunk:det-10'})
MATCH (topic:Topic {id: 'topic:det-10'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-10'})
SET c.title = 'c. Complete ground — real ground returns to ground'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 233
SET c.lineEnd = 254
SET c.description = 'In real ground, ground as content and ground as connection are only substrates. Real ground-connection is ground rather as sublated. Makes up side of grounded or positedness. As positedness, ground itself has returned to its ground. Is now something grounded: it has another ground. This ground determined: first, identical with ground by which it is grounded, both sides have same content. Second, new ground is immanent reflection of link: absolute reference of two content determinations to each other.'
SET c.keyPoints = ['Ground as content and ground as connection are only substrates', 'Real ground-connection is ground rather as sublated', 'Ground itself has returned to its ground', 'New ground is immanent reflection of link', 'Absolute reference of two content determinations to each other']
SET c.tags = ['sublation', 'reflection', 'citta']
SET c.orderInSource = 10
SET c.globalOrder = 82
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '1. In real ground, ground as content\nand ground as connection are only substrates.\nThe former is only posited as essential and as ground;\nthe connection is what the grounded immediately is\nas the indeterminate substrate of a diversified content,\na linking of this content which is not the content\'s own reflection\nbut is rather external and consequently a reflection which is only posited.\nThe real ground-connection is ground, therefore, rather as sublated;\nconsequently, it rather makes up the side of\nthe grounded or of the positedness.\nAs positedness, however, the ground itself\nhas now returned to its ground;\nit is now something grounded: it has another ground.\nThis ground will therefore be so determined that,\nfirst, it is identical with the ground by which it is grounded;\nboth sides have in this determination one and the same content;\nthe two content determinations and their linkage in\na something are equally to be found in the new ground.\nBut, second, the new ground into which\nthe previously merely posited and external link\nis now sublated is the immanent reflection of this link:\nthe absolute reference of the two content determinations to each other.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-10'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-10'})
MATCH (c:IntegratedChunk {id: 'det-10'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-10'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-10'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-10:kp:1'})
SET kp.chunkId = 'det-10'
SET kp.ordinal = 1
SET kp.text = 'Ground as content and ground as connection are only substrates';
MATCH (c:IntegratedChunk {id: 'det-10'})
MATCH (kp:KeyPoint {id: 'det-10:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-10:kp:2'})
SET kp.chunkId = 'det-10'
SET kp.ordinal = 2
SET kp.text = 'Real ground-connection is ground rather as sublated';
MATCH (c:IntegratedChunk {id: 'det-10'})
MATCH (kp:KeyPoint {id: 'det-10:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-10:kp:3'})
SET kp.chunkId = 'det-10'
SET kp.ordinal = 3
SET kp.text = 'Ground itself has returned to its ground';
MATCH (c:IntegratedChunk {id: 'det-10'})
MATCH (kp:KeyPoint {id: 'det-10:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-10:kp:4'})
SET kp.chunkId = 'det-10'
SET kp.ordinal = 4
SET kp.text = 'New ground is immanent reflection of link';
MATCH (c:IntegratedChunk {id: 'det-10'})
MATCH (kp:KeyPoint {id: 'det-10:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-10:kp:5'})
SET kp.chunkId = 'det-10'
SET kp.ordinal = 5
SET kp.text = 'Absolute reference of two content determinations to each other';
MATCH (c:IntegratedChunk {id: 'det-10'})
MATCH (kp:KeyPoint {id: 'det-10:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-11'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 256
SET segment.lineEnd = 263
SET segment.text = 'Because real ground has itself thus returned to its ground,\nthe identity of ground and grounded\nor the formality of ground reasserts itself in it.\nThe newly arisen ground-connection is\ntherefore the one which is complete,\nwhich contains the formal and real ground in itself\nat the same time and mediates the content determinations\nwhich in the real ground confronted each other immediately.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-11'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-11'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-11'
SET topic.title = 'Complete ground — formal and real'
SET topic.description = 'Because real ground has returned to its ground, identity of ground and grounded or formality of ground reasserts itself. Newly arisen ground-connection is one which is complete. Contains formal and real ground in itself at same time. Mediates content determinations which in real ground confronted each other immediately.'
SET topic.keyPoints = ['Identity of ground and grounded reasserts itself', 'Newly arisen ground-connection is complete', 'Contains formal and real ground in itself at same time', 'Mediates content determinations'];
MATCH (segment:ChunkSegment {id: 'chunk:det-11'})
MATCH (topic:Topic {id: 'topic:det-11'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-11'})
SET c.title = 'Complete ground — formal and real'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 256
SET c.lineEnd = 263
SET c.description = 'Because real ground has returned to its ground, identity of ground and grounded or formality of ground reasserts itself. Newly arisen ground-connection is one which is complete. Contains formal and real ground in itself at same time. Mediates content determinations which in real ground confronted each other immediately.'
SET c.keyPoints = ['Identity of ground and grounded reasserts itself', 'Newly arisen ground-connection is complete', 'Contains formal and real ground in itself at same time', 'Mediates content determinations']
SET c.tags = ['mediation', 'citta']
SET c.orderInSource = 11
SET c.globalOrder = 83
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Because real ground has itself thus returned to its ground,\nthe identity of ground and grounded\nor the formality of ground reasserts itself in it.\nThe newly arisen ground-connection is\ntherefore the one which is complete,\nwhich contains the formal and real ground in itself\nat the same time and mediates the content determinations\nwhich in the real ground confronted each other immediately.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-11'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-11'})
MATCH (c:IntegratedChunk {id: 'det-11'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-11'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-11'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-11:kp:1'})
SET kp.chunkId = 'det-11'
SET kp.ordinal = 1
SET kp.text = 'Identity of ground and grounded reasserts itself';
MATCH (c:IntegratedChunk {id: 'det-11'})
MATCH (kp:KeyPoint {id: 'det-11:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-11:kp:2'})
SET kp.chunkId = 'det-11'
SET kp.ordinal = 2
SET kp.text = 'Newly arisen ground-connection is complete';
MATCH (c:IntegratedChunk {id: 'det-11'})
MATCH (kp:KeyPoint {id: 'det-11:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-11:kp:3'})
SET kp.chunkId = 'det-11'
SET kp.ordinal = 3
SET kp.text = 'Contains formal and real ground in itself at same time';
MATCH (c:IntegratedChunk {id: 'det-11'})
MATCH (kp:KeyPoint {id: 'det-11:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-11:kp:4'})
SET kp.chunkId = 'det-11'
SET kp.ordinal = 4
SET kp.text = 'Mediates content determinations';
MATCH (c:IntegratedChunk {id: 'det-11'})
MATCH (kp:KeyPoint {id: 'det-11:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-12'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 265
SET segment.lineEnd = 306
SET segment.text = '2. Thus the ground-connection has more precisely\ndetermined itself as follows.\n\nFirst, something has a ground;\nit contains the content determination which is the ground\nand, in addition, a second determination as posited by the ground.\nBut, because of the indifference of content,\nthe one determination is not ground in itself,\nnor is the other in itself one that is grounded by the first;\nthis connection of ground and grounded is rather\nsublated in the immediacy of their content, is posited,\nand as such has its ground in another such connection.\n\nSince this second connection is\ndistinguished only according to form,\nit has the same content as the first;\nit still has the same two determinations of content\nbut is now their immediate linking together.\nThis linking, however, is of a general nature,\nand the content, therefore, is diversified into determinations\nthat are indifferent to each other.\nThe linking is not, therefore, their true absolute connection\nthat would make one determination the element of\nself-identity in the positedness,\nand the other determination\nthe positedness of this same self-identity;\non the contrary, the two are supported by a something\nand this something is what connects them,\nbut in a connection which is not reflected,\nis rather only immediate and, therefore,\nonly a relative ground as against\nthe linking in the other something.\nThe two somethings are therefore the two distinct\nconnections of content that have transpired.\nThey stand in the identical ground-connection of form;\nthey are one and the same whole content,\nnamely the two content determinations and their connection;\nthey are distinct only by the kind of this connection,\nwhich in the one is an immediate\nand in the other a posited connection;\nthrough this, they are distinguished\none from another as ground and grounded only according to form.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-12'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-12'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-12'
SET topic.title = 'Ground-connection determined — first and second'
SET topic.description = 'First, something has ground. Contains content determination which is ground and second determination as posited by ground. But because of indifference of content, one determination is not ground in itself. Connection sublated in immediacy of content, is posited, has its ground in another such connection. Second connection distinguished only according to form, has same content. Still has same two determinations but now their immediate linking together. Linking is not their true absolute connection. Two are supported by something, something connects them. But in connection which is not reflected, only immediate, only relative ground. Two somethings are two distinct connections. Stand in identical ground-connection of form. One and same whole content. Distinguished as ground and grounded only according to form.'
SET topic.keyPoints = ['Something has ground, contains ground and posited determination', 'Connection sublated in immediacy, has its ground in another connection', 'Second connection has same content, immediate linking together', 'Linking is not true absolute connection, only relative ground', 'Two somethings distinguished as ground and grounded only according to form'];
MATCH (segment:ChunkSegment {id: 'chunk:det-12'})
MATCH (topic:Topic {id: 'topic:det-12'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-12'})
SET c.title = 'Ground-connection determined — first and second'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 265
SET c.lineEnd = 306
SET c.description = 'First, something has ground. Contains content determination which is ground and second determination as posited by ground. But because of indifference of content, one determination is not ground in itself. Connection sublated in immediacy of content, is posited, has its ground in another such connection. Second connection distinguished only according to form, has same content. Still has same two determinations but now their immediate linking together. Linking is not their true absolute connection. Two are supported by something, something connects them. But in connection which is not reflected, only immediate, only relative ground. Two somethings are two distinct connections. Stand in identical ground-connection of form. One and same whole content. Distinguished as ground and grounded only according to form.'
SET c.keyPoints = ['Something has ground, contains ground and posited determination', 'Connection sublated in immediacy, has its ground in another connection', 'Second connection has same content, immediate linking together', 'Linking is not true absolute connection, only relative ground', 'Two somethings distinguished as ground and grounded only according to form']
SET c.tags = ['sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 12
SET c.globalOrder = 84
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '2. Thus the ground-connection has more precisely\ndetermined itself as follows.\n\nFirst, something has a ground;\nit contains the content determination which is the ground\nand, in addition, a second determination as posited by the ground.\nBut, because of the indifference of content,\nthe one determination is not ground in itself,\nnor is the other in itself one that is grounded by the first;\nthis connection of ground and grounded is rather\nsublated in the immediacy of their content, is posited,\nand as such has its ground in another such connection.\n\nSince this second connection is\ndistinguished only according to form,\nit has the same content as the first;\nit still has the same two determinations of content\nbut is now their immediate linking together.\nThis linking, however, is of a general nature,\nand the content, therefore, is diversified into determinations\nthat are indifferent to each other.\nThe linking is not, therefore, their true absolute connection\nthat would make one determination the element of\nself-identity in the positedness,\nand the other determination\nthe positedness of this same self-identity;\non the contrary, the two are supported by a something\nand this something is what connects them,\nbut in a connection which is not reflected,\nis rather only immediate and, therefore,\nonly a relative ground as against\nthe linking in the other something.\nThe two somethings are therefore the two distinct\nconnections of content that have transpired.\nThey stand in the identical ground-connection of form;\nthey are one and the same whole content,\nnamely the two content determinations and their connection;\nthey are distinct only by the kind of this connection,\nwhich in the one is an immediate\nand in the other a posited connection;\nthrough this, they are distinguished\none from another as ground and grounded only according to form.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-12'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-12'})
MATCH (c:IntegratedChunk {id: 'det-12'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-12'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-12'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-12:kp:1'})
SET kp.chunkId = 'det-12'
SET kp.ordinal = 1
SET kp.text = 'Something has ground, contains ground and posited determination';
MATCH (c:IntegratedChunk {id: 'det-12'})
MATCH (kp:KeyPoint {id: 'det-12:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-12:kp:2'})
SET kp.chunkId = 'det-12'
SET kp.ordinal = 2
SET kp.text = 'Connection sublated in immediacy, has its ground in another connection';
MATCH (c:IntegratedChunk {id: 'det-12'})
MATCH (kp:KeyPoint {id: 'det-12:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-12:kp:3'})
SET kp.chunkId = 'det-12'
SET kp.ordinal = 3
SET kp.text = 'Second connection has same content, immediate linking together';
MATCH (c:IntegratedChunk {id: 'det-12'})
MATCH (kp:KeyPoint {id: 'det-12:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-12:kp:4'})
SET kp.chunkId = 'det-12'
SET kp.ordinal = 4
SET kp.text = 'Linking is not true absolute connection, only relative ground';
MATCH (c:IntegratedChunk {id: 'det-12'})
MATCH (kp:KeyPoint {id: 'det-12:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-12:kp:5'})
SET kp.chunkId = 'det-12'
SET kp.ordinal = 5
SET kp.text = 'Two somethings distinguished as ground and grounded only according to form';
MATCH (c:IntegratedChunk {id: 'det-12'})
MATCH (kp:KeyPoint {id: 'det-12:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-13'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 307
SET segment.lineEnd = 359
SET segment.text = 'Second, this ground-connection is not only formal, but also real.\nFormal ground passes over into real ground, as has been shown;\nthe moments of the form reflect themselves into themselves;\nthey are a self-subsistent content,\nand the ground-connection contains\nalso one content with the character of ground\nand another with that of grounded.\nThe content constitutes at first the immediate\nidentity of both sides of the formal ground;\nso the two sides have one and the same content.\nBut the content also has the form in it,\nand so it is a twofold content\nthat behaves as ground and grounded.\nOne of the two content determinations of\nthe two somethings is therefore determined,\nnot merely as being common to them\naccording to external comparison,\nbut as their identical substrate\nand the foundation of their connection.\nAs against the other determination of the content,\nthis determination is essential\nand is the ground of the other which is posited,\nthat is, posited in the something,\nthe connection of which is the grounded.\nIn the first something, which is the ground-connection,\nthis second determination of the content is\nalso immediately and in itself linked with the first.\nBut the other something only contains\nthe one determination in itself as that\nin which it is immediately identical with the first something,\nbut the other as the one which is posited in it.\nThe former content determination is its\nground by virtue of its being originally linked\nin the first something with\nthe other content determination.\n\nThe ground-connection of the content determinations\nin the second something is thus mediated\nthrough the connection present in the first something.\nThe inference is this:\nsince determination B is implicitly linked\nwith determination A in a something,\nin a second something to which only\nthe one determination A immediately belongs,\nalso B is linked with it.\nIn the second something, not only is\nthis second determination mediated;\nalso mediated is that its immediate ground is mediated,\nnamely by virtue of its original connection\nwith B in the first something.\nThis connection is thus the ground of the ground A,\nand the whole ground-connection is present in\nthe second something as posited or grounded.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-13'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-13'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-13'
SET topic.title = 'Ground-connection also real — inference'
SET topic.description = 'Second, ground-connection is not only formal, but also real. Formal ground passes over into real ground. Moments of form reflect themselves into themselves, are self-subsistent content. Ground-connection contains one content with character of ground and another with that of grounded. Content is twofold content that behaves as ground and grounded. One content determination determined as identical substrate and foundation of connection. As against other determination, this determination is essential and is ground of other which is posited. In first something, second determination immediately linked with first. Other something only contains one determination as that in which immediately identical with first, but other as posited. Ground-connection in second something is mediated through connection in first something. Inference: since B implicitly linked with A in something, in second something to which only A belongs, also B is linked with it. This connection is ground of ground A. Whole ground-connection present in second something as posited or grounded.'
SET topic.keyPoints = ['Ground-connection is not only formal, but also real', 'Content is twofold content that behaves as ground and grounded', 'One determination is identical substrate and foundation', 'Ground-connection in second something is mediated through connection in first', 'Inference: since B linked with A, in second something also B is linked'];
MATCH (segment:ChunkSegment {id: 'chunk:det-13'})
MATCH (topic:Topic {id: 'topic:det-13'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-13'})
SET c.title = 'Ground-connection also real — inference'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 307
SET c.lineEnd = 359
SET c.description = 'Second, ground-connection is not only formal, but also real. Formal ground passes over into real ground. Moments of form reflect themselves into themselves, are self-subsistent content. Ground-connection contains one content with character of ground and another with that of grounded. Content is twofold content that behaves as ground and grounded. One content determination determined as identical substrate and foundation of connection. As against other determination, this determination is essential and is ground of other which is posited. In first something, second determination immediately linked with first. Other something only contains one determination as that in which immediately identical with first, but other as posited. Ground-connection in second something is mediated through connection in first something. Inference: since B implicitly linked with A in something, in second something to which only A belongs, also B is linked with it. This connection is ground of ground A. Whole ground-connection present in second something as posited or grounded.'
SET c.keyPoints = ['Ground-connection is not only formal, but also real', 'Content is twofold content that behaves as ground and grounded', 'One determination is identical substrate and foundation', 'Ground-connection in second something is mediated through connection in first', 'Inference: since B linked with A, in second something also B is linked']
SET c.tags = ['reflection', 'mediation', 'citta']
SET c.orderInSource = 13
SET c.globalOrder = 85
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Second, this ground-connection is not only formal, but also real.\nFormal ground passes over into real ground, as has been shown;\nthe moments of the form reflect themselves into themselves;\nthey are a self-subsistent content,\nand the ground-connection contains\nalso one content with the character of ground\nand another with that of grounded.\nThe content constitutes at first the immediate\nidentity of both sides of the formal ground;\nso the two sides have one and the same content.\nBut the content also has the form in it,\nand so it is a twofold content\nthat behaves as ground and grounded.\nOne of the two content determinations of\nthe two somethings is therefore determined,\nnot merely as being common to them\naccording to external comparison,\nbut as their identical substrate\nand the foundation of their connection.\nAs against the other determination of the content,\nthis determination is essential\nand is the ground of the other which is posited,\nthat is, posited in the something,\nthe connection of which is the grounded.\nIn the first something, which is the ground-connection,\nthis second determination of the content is\nalso immediately and in itself linked with the first.\nBut the other something only contains\nthe one determination in itself as that\nin which it is immediately identical with the first something,\nbut the other as the one which is posited in it.\nThe former content determination is its\nground by virtue of its being originally linked\nin the first something with\nthe other content determination.\n\nThe ground-connection of the content determinations\nin the second something is thus mediated\nthrough the connection present in the first something.\nThe inference is this:\nsince determination B is implicitly linked\nwith determination A in a something,\nin a second something to which only\nthe one determination A immediately belongs,\nalso B is linked with it.\nIn the second something, not only is\nthis second determination mediated;\nalso mediated is that its immediate ground is mediated,\nnamely by virtue of its original connection\nwith B in the first something.\nThis connection is thus the ground of the ground A,\nand the whole ground-connection is present in\nthe second something as posited or grounded.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-13'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-13'})
MATCH (c:IntegratedChunk {id: 'det-13'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-13'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-13'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-13:kp:1'})
SET kp.chunkId = 'det-13'
SET kp.ordinal = 1
SET kp.text = 'Ground-connection is not only formal, but also real';
MATCH (c:IntegratedChunk {id: 'det-13'})
MATCH (kp:KeyPoint {id: 'det-13:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-13:kp:2'})
SET kp.chunkId = 'det-13'
SET kp.ordinal = 2
SET kp.text = 'Content is twofold content that behaves as ground and grounded';
MATCH (c:IntegratedChunk {id: 'det-13'})
MATCH (kp:KeyPoint {id: 'det-13:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-13:kp:3'})
SET kp.chunkId = 'det-13'
SET kp.ordinal = 3
SET kp.text = 'One determination is identical substrate and foundation';
MATCH (c:IntegratedChunk {id: 'det-13'})
MATCH (kp:KeyPoint {id: 'det-13:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-13:kp:4'})
SET kp.chunkId = 'det-13'
SET kp.ordinal = 4
SET kp.text = 'Ground-connection in second something is mediated through connection in first';
MATCH (c:IntegratedChunk {id: 'det-13'})
MATCH (kp:KeyPoint {id: 'det-13:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-13:kp:5'})
SET kp.chunkId = 'det-13'
SET kp.ordinal = 5
SET kp.text = 'Inference: since B linked with A, in second something also B is linked';
MATCH (c:IntegratedChunk {id: 'det-13'})
MATCH (kp:KeyPoint {id: 'det-13:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:det-14'})
SET segment.sourceId = 'source-determinate-ground'
SET segment.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET segment.lineStart = 361
SET segment.lineEnd = 396
SET segment.text = '3. Real ground shows itself to be the self-external reflection of ground;\nits complete mediation is the restoration of its identity with itself.\nBut because this identity has in the process equally acquired\nthe externality of real ground,\nthe formal ground-connection in this unity\nof itself and real ground is just as much\nself-positing as self-sublating ground;\nthe ground-connection mediates itself with itself through its negation.\nThe ground is at first, as the original connection,\nthe connection of immediate content determinations.\nThe ground-connection, being essential form,\nhas for sides such that are sublated or are as moments.\nConsequently, as the form of immediate determinations,\nit connects itself with itself as self-identical\nwhile at the same time connecting with their negation;\naccordingly, it is ground not in and for itself\nbut as connected with the sublated ground-connection.\nSecond, the sublated connection or the immediate,\nwhich in the original and in the posited connection\nis the identical substrate, is likewise real ground\nnot in and for itself; that it is ground is\nrather posited by virtue of that original link.\n\nThus the ground-connection is in its totality\nessentially presupposing reflection;\nformal ground presupposes the immediate content determination,\nand this content presupposes form as real ground.\nGround is therefore form as an immediate linkage\nbut in such a manner that it repels itself from itself\nand rather presupposes immediacy,\nreferring itself therein as to another.\nThis immediate is the content determination, the simple ground;\nbut as such, that is, as ground, it is equally repelled from itself\nand refers itself to itself equally as to an other.\nThus the total ground-connection has taken on\nthe determination of conditioning mediation.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (segment:ChunkSegment {id: 'chunk:det-14'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:det-14'})
SET topic.sourceId = 'source-determinate-ground'
SET topic.topicRef = 'det-14'
SET topic.title = 'Complete mediation — conditioning'
SET topic.description = 'Real ground shows itself to be self-external reflection of ground. Its complete mediation is restoration of its identity with itself. But because identity has equally acquired externality of real ground, formal ground-connection is just as much self-positing as self-sublating ground. Ground-connection mediates itself with itself through its negation. Ground is connection of immediate content determinations. As form of immediate determinations, connects itself with itself as self-identical while connecting with their negation. Ground not in and for itself but as connected with sublated ground-connection. Sublated connection is real ground not in and for itself. Ground-connection is essentially presupposing reflection. Formal ground presupposes immediate content determination. Content presupposes form as real ground. Ground is form as immediate linkage but repels itself from itself, presupposes immediacy. Immediate is content determination, simple ground. But as ground, equally repelled from itself and refers itself to itself as to other. Total ground-connection has taken on determination of conditioning mediation.'
SET topic.keyPoints = ['Real ground is self-external reflection of ground', 'Complete mediation is restoration of identity with itself', 'Ground-connection mediates itself with itself through its negation', 'Ground-connection is essentially presupposing reflection', 'Total ground-connection has taken on determination of conditioning mediation'];
MATCH (segment:ChunkSegment {id: 'chunk:det-14'})
MATCH (topic:Topic {id: 'topic:det-14'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'det-14'})
SET c.title = 'Complete mediation — conditioning'
SET c.sourceId = 'source-determinate-ground'
SET c.sourceFile = 'relative/essence/reflection/ground/sources/determinate.txt'
SET c.lineStart = 361
SET c.lineEnd = 396
SET c.description = 'Real ground shows itself to be self-external reflection of ground. Its complete mediation is restoration of its identity with itself. But because identity has equally acquired externality of real ground, formal ground-connection is just as much self-positing as self-sublating ground. Ground-connection mediates itself with itself through its negation. Ground is connection of immediate content determinations. As form of immediate determinations, connects itself with itself as self-identical while connecting with their negation. Ground not in and for itself but as connected with sublated ground-connection. Sublated connection is real ground not in and for itself. Ground-connection is essentially presupposing reflection. Formal ground presupposes immediate content determination. Content presupposes form as real ground. Ground is form as immediate linkage but repels itself from itself, presupposes immediacy. Immediate is content determination, simple ground. But as ground, equally repelled from itself and refers itself to itself as to other. Total ground-connection has taken on determination of conditioning mediation.'
SET c.keyPoints = ['Real ground is self-external reflection of ground', 'Complete mediation is restoration of identity with itself', 'Ground-connection mediates itself with itself through its negation', 'Ground-connection is essentially presupposing reflection', 'Total ground-connection has taken on determination of conditioning mediation']
SET c.tags = ['negation', 'sublation', 'reflection', 'mediation', 'citta']
SET c.orderInSource = 14
SET c.globalOrder = 86
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '3. Real ground shows itself to be the self-external reflection of ground;\nits complete mediation is the restoration of its identity with itself.\nBut because this identity has in the process equally acquired\nthe externality of real ground,\nthe formal ground-connection in this unity\nof itself and real ground is just as much\nself-positing as self-sublating ground;\nthe ground-connection mediates itself with itself through its negation.\nThe ground is at first, as the original connection,\nthe connection of immediate content determinations.\nThe ground-connection, being essential form,\nhas for sides such that are sublated or are as moments.\nConsequently, as the form of immediate determinations,\nit connects itself with itself as self-identical\nwhile at the same time connecting with their negation;\naccordingly, it is ground not in and for itself\nbut as connected with the sublated ground-connection.\nSecond, the sublated connection or the immediate,\nwhich in the original and in the posited connection\nis the identical substrate, is likewise real ground\nnot in and for itself; that it is ground is\nrather posited by virtue of that original link.\n\nThus the ground-connection is in its totality\nessentially presupposing reflection;\nformal ground presupposes the immediate content determination,\nand this content presupposes form as real ground.\nGround is therefore form as an immediate linkage\nbut in such a manner that it repels itself from itself\nand rather presupposes immediacy,\nreferring itself therein as to another.\nThis immediate is the content determination, the simple ground;\nbut as such, that is, as ground, it is equally repelled from itself\nand refers itself to itself equally as to an other.\nThus the total ground-connection has taken on\nthe determination of conditioning mediation.';
MATCH (s:SourceText {id: 'source-determinate-ground'})
MATCH (c:IntegratedChunk {id: 'det-14'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:det-14'})
MATCH (c:IntegratedChunk {id: 'det-14'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'det-14'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'det-14'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'det-14:kp:1'})
SET kp.chunkId = 'det-14'
SET kp.ordinal = 1
SET kp.text = 'Real ground is self-external reflection of ground';
MATCH (c:IntegratedChunk {id: 'det-14'})
MATCH (kp:KeyPoint {id: 'det-14:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-14:kp:2'})
SET kp.chunkId = 'det-14'
SET kp.ordinal = 2
SET kp.text = 'Complete mediation is restoration of identity with itself';
MATCH (c:IntegratedChunk {id: 'det-14'})
MATCH (kp:KeyPoint {id: 'det-14:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-14:kp:3'})
SET kp.chunkId = 'det-14'
SET kp.ordinal = 3
SET kp.text = 'Ground-connection mediates itself with itself through its negation';
MATCH (c:IntegratedChunk {id: 'det-14'})
MATCH (kp:KeyPoint {id: 'det-14:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-14:kp:4'})
SET kp.chunkId = 'det-14'
SET kp.ordinal = 4
SET kp.text = 'Ground-connection is essentially presupposing reflection';
MATCH (c:IntegratedChunk {id: 'det-14'})
MATCH (kp:KeyPoint {id: 'det-14:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'det-14:kp:5'})
SET kp.chunkId = 'det-14'
SET kp.ordinal = 5
SET kp.text = 'Total ground-connection has taken on determination of conditioning mediation';
MATCH (c:IntegratedChunk {id: 'det-14'})
MATCH (kp:KeyPoint {id: 'det-14:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
