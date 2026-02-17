MERGE (s:SourceText {id: 'source-shine'})
SET s.title = 'Shine'
SET s.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET s.totalLines = 216;
MATCH (protocol:CITProtocol {id: 'cit-citi-citta-reflection-debug-ir'})
MATCH (s:SourceText {id: 'source-shine'})
MERGE (protocol)-[:HAS_SOURCE]->(s);
MERGE (segment:ChunkSegment {id: 'chunk:shn-1'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 4
SET segment.lineEnd = 12
SET segment.text = '1. Being is shine.\n\nThe being of shine consists solely\nin the sublatedness of being,\nin being\'s nothingness;\nthis nothingness it has in essence,\napart from its nothingness,\napart from essence, it does not exist.\nIt is the negative posited as negative.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-1'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-1'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-1'
SET topic.title = 'Being is shine — negative posited as negative'
SET topic.description = 'Being of shine consists solely in sublatedness of being, in being\'s nothingness. This nothingness it has in essence. Apart from essence, it does not exist. It is the negative posited as negative.'
SET topic.keyPoints = ['Being of shine consists solely in sublatedness of being, in being\'s nothingness', 'This nothingness it has in essence', 'Apart from essence, it does not exist', 'It is the negative posited as negative'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-1'})
MATCH (topic:Topic {id: 'topic:shn-1'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-1'})
SET c.title = 'Being is shine — negative posited as negative'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 4
SET c.lineEnd = 12
SET c.description = 'Being of shine consists solely in sublatedness of being, in being\'s nothingness. This nothingness it has in essence. Apart from essence, it does not exist. It is the negative posited as negative.'
SET c.keyPoints = ['Being of shine consists solely in sublatedness of being, in being\'s nothingness', 'This nothingness it has in essence', 'Apart from essence, it does not exist', 'It is the negative posited as negative']
SET c.tags = ['negation', 'sublation', 'shine', 'citta']
SET c.orderInSource = 1
SET c.globalOrder = 32
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '1. Being is shine.\n\nThe being of shine consists solely\nin the sublatedness of being,\nin being\'s nothingness;\nthis nothingness it has in essence,\napart from its nothingness,\napart from essence, it does not exist.\nIt is the negative posited as negative.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-1'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-1'})
MATCH (c:IntegratedChunk {id: 'shn-1'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-1'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-1'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-1:kp:1'})
SET kp.chunkId = 'shn-1'
SET kp.ordinal = 1
SET kp.text = 'Being of shine consists solely in sublatedness of being, in being\'s nothingness';
MATCH (c:IntegratedChunk {id: 'shn-1'})
MATCH (kp:KeyPoint {id: 'shn-1:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-1:kp:2'})
SET kp.chunkId = 'shn-1'
SET kp.ordinal = 2
SET kp.text = 'This nothingness it has in essence';
MATCH (c:IntegratedChunk {id: 'shn-1'})
MATCH (kp:KeyPoint {id: 'shn-1:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-1:kp:3'})
SET kp.chunkId = 'shn-1'
SET kp.ordinal = 3
SET kp.text = 'Apart from essence, it does not exist';
MATCH (c:IntegratedChunk {id: 'shn-1'})
MATCH (kp:KeyPoint {id: 'shn-1:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-1:kp:4'})
SET kp.chunkId = 'shn-1'
SET kp.ordinal = 4
SET kp.text = 'It is the negative posited as negative';
MATCH (c:IntegratedChunk {id: 'shn-1'})
MATCH (kp:KeyPoint {id: 'shn-1:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-2'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 14
SET segment.lineEnd = 36
SET segment.text = 'Shine is all that remains of the sphere of being.\nBut it still seems to have an immediate side\nwhich is independent of essence\nand to be, in general, an other of essence.\nOther entails in general the two moments\nof existence and non-existence.\nSince the unessential no longer has a being,\nwhat is left to it of otherness is\nonly the pure moment of non-existence;\nshine is this immediate non-existence,\na non-existence in the determinateness of being,\nso that it has existence only with reference to another,\nin its non-existence;\nit is the non-self-subsistent\nwhich exists only in its negation.\nWhat is left over to it is thus only\nthe pure determinateness of immediacy;\nit is as reflected immediacy, that is,\none which is only by virtue of\nthe mediation of its negation\nand which, over against this mediation, is\nnothing except the empty determination\nof the immediacy of non-existence.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-2'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-2'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-2'
SET topic.title = 'Shine as remainder of being — immediate non-existence'
SET topic.description = 'Shine is all that remains of sphere of being. Seems to have immediate side independent of essence, other of essence. Other entails existence and non-existence. Since unessential no longer has being, only pure moment of non-existence remains. Shine is immediate non-existence, non-existence in determinateness of being. Exists only with reference to another, in its non-existence. Non-self-subsistent which exists only in its negation. Pure determinateness of immediacy, reflected immediacy. Only by virtue of mediation of its negation. Empty determination of immediacy of non-existence.'
SET topic.keyPoints = ['Shine is all that remains of sphere of being', 'Seems to have immediate side independent of essence', 'Only pure moment of non-existence remains', 'Shine is immediate non-existence, non-existence in determinateness of being', 'Non-self-subsistent which exists only in its negation', 'Reflected immediacy, only by virtue of mediation of its negation'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-2'})
MATCH (topic:Topic {id: 'topic:shn-2'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-2'})
SET c.title = 'Shine as remainder of being — immediate non-existence'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 14
SET c.lineEnd = 36
SET c.description = 'Shine is all that remains of sphere of being. Seems to have immediate side independent of essence, other of essence. Other entails existence and non-existence. Since unessential no longer has being, only pure moment of non-existence remains. Shine is immediate non-existence, non-existence in determinateness of being. Exists only with reference to another, in its non-existence. Non-self-subsistent which exists only in its negation. Pure determinateness of immediacy, reflected immediacy. Only by virtue of mediation of its negation. Empty determination of immediacy of non-existence.'
SET c.keyPoints = ['Shine is all that remains of sphere of being', 'Seems to have immediate side independent of essence', 'Only pure moment of non-existence remains', 'Shine is immediate non-existence, non-existence in determinateness of being', 'Non-self-subsistent which exists only in its negation', 'Reflected immediacy, only by virtue of mediation of its negation']
SET c.tags = ['negation', 'reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 2
SET c.globalOrder = 33
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Shine is all that remains of the sphere of being.\nBut it still seems to have an immediate side\nwhich is independent of essence\nand to be, in general, an other of essence.\nOther entails in general the two moments\nof existence and non-existence.\nSince the unessential no longer has a being,\nwhat is left to it of otherness is\nonly the pure moment of non-existence;\nshine is this immediate non-existence,\na non-existence in the determinateness of being,\nso that it has existence only with reference to another,\nin its non-existence;\nit is the non-self-subsistent\nwhich exists only in its negation.\nWhat is left over to it is thus only\nthe pure determinateness of immediacy;\nit is as reflected immediacy, that is,\none which is only by virtue of\nthe mediation of its negation\nand which, over against this mediation, is\nnothing except the empty determination\nof the immediacy of non-existence.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-2'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-2'})
MATCH (c:IntegratedChunk {id: 'shn-2'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-2'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-2'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-2:kp:1'})
SET kp.chunkId = 'shn-2'
SET kp.ordinal = 1
SET kp.text = 'Shine is all that remains of sphere of being';
MATCH (c:IntegratedChunk {id: 'shn-2'})
MATCH (kp:KeyPoint {id: 'shn-2:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-2:kp:2'})
SET kp.chunkId = 'shn-2'
SET kp.ordinal = 2
SET kp.text = 'Seems to have immediate side independent of essence';
MATCH (c:IntegratedChunk {id: 'shn-2'})
MATCH (kp:KeyPoint {id: 'shn-2:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-2:kp:3'})
SET kp.chunkId = 'shn-2'
SET kp.ordinal = 3
SET kp.text = 'Only pure moment of non-existence remains';
MATCH (c:IntegratedChunk {id: 'shn-2'})
MATCH (kp:KeyPoint {id: 'shn-2:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-2:kp:4'})
SET kp.chunkId = 'shn-2'
SET kp.ordinal = 4
SET kp.text = 'Shine is immediate non-existence, non-existence in determinateness of being';
MATCH (c:IntegratedChunk {id: 'shn-2'})
MATCH (kp:KeyPoint {id: 'shn-2:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-2:kp:5'})
SET kp.chunkId = 'shn-2'
SET kp.ordinal = 5
SET kp.text = 'Non-self-subsistent which exists only in its negation';
MATCH (c:IntegratedChunk {id: 'shn-2'})
MATCH (kp:KeyPoint {id: 'shn-2:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-2:kp:6'})
SET kp.chunkId = 'shn-2'
SET kp.ordinal = 6
SET kp.text = 'Reflected immediacy, only by virtue of mediation of its negation';
MATCH (c:IntegratedChunk {id: 'shn-2'})
MATCH (kp:KeyPoint {id: 'shn-2:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-3'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 38
SET segment.lineEnd = 93
SET segment.text = 'Shine, the “phenomenon” of skepticism,\nand also the “appearance” of idealism,\nis thus this immediacy which is not\na something nor a thing in general,\nnot an indifferent being that would exist apart\nfrom its determinateness and connection with the subject.\nSkepticism did not permit itself to say “It is,”\nand the more recent idealism did not permit itself\nto regard cognitions as a knowledge of the thing-in-itself.\nThe shine of the former was supposed absolutely\nnot to have the foundation of a being:\nthe thing-in-itself was not supposed\nto enter into these cognitions.\nBut at the same time skepticism allowed\na manifold of determinations for its shine,\nor rather the latter turned out to have\nthe full richness of the world for its content.\nLikewise for the appearance of idealism:\nit encompassed the full range of these manifold determinacies.\nSo, the shine of skepticism and the appearance of idealism\ndo immediately have a manifold of determination.\nThis content, therefore, might well have no being as foundation,\nno thing or thing-in-itself;\nfor itself, it remains as it is;\nit is simply transposed from being into shine,\nso that the latter has within itself those manifold\ndeterminacies that exist immediately,\neach an other to the other.\nThe shine is thus itself something immediately determined.\nIt can have this or that content;\nbut whatever content it has, it has not posited it\nbut possesses it immediately.\nIdealism, whether Leibnizian, Kantian, Fichtean, or in any\nother form, has not gone further than skepticism in this:\nit has not advanced beyond being as determinateness.\nSkepticism lets the content of its shine to be given to it;\nthe shine exists for it immediately,\nwhatever content it might have.\nThe Leibnizian monad develops its representations from itself\nbut is not their generating and controlling force;\nthey rise up in it as a froth, indifferent,\nimmediately present to each other and to the monad as well.\nLikewise Kant\'s appearance is a given content of perception\nthat presupposes affections, determinations of the subject\nwhich are immediate to each other and to the subject.\nAs for the infinite obstacle of Fichte\'s Idealism,\nit might well be that it has no thing-in-itself for foundation,\nso that it becomes a determinateness purely within the “I.”\nBut this determinateness that the “I” makes its own,\nsublating its externality,\nis to the “I” at the same time an immediate determinateness,\na limitation of the “I” which the latter may transcend\nbut which contains a side of indifference,\nand on account of this indifference,\nalthough internal to the “I,”\nit entails an immediate non-being of it.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-3'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-3'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-3'
SET topic.title = 'Shine in skepticism and idealism — manifold determinations'
SET topic.description = 'Shine is \'phenomenon\' of skepticism, \'appearance\' of idealism. Immediacy which is not something nor thing in general. Not indifferent being apart from determinateness and connection with subject. Skepticism: shine supposed absolutely not to have foundation of being. Idealism: thing-in-itself not supposed to enter into cognitions. But both allow manifold of determinations, full richness of world. Content might have no being as foundation, no thing-in-itself. Content simply transposed from being into shine. Shine is immediately determined, can have this or that content. But whatever content, has not posited it but possesses it immediately. Idealism (Leibnizian, Kantian, Fichtean) not gone further than skepticism. Not advanced beyond being as determinateness.'
SET topic.keyPoints = ['Shine is \'phenomenon\' of skepticism, \'appearance\' of idealism', 'Not something nor thing in general, not indifferent being', 'Both allow manifold of determinations, full richness of world', 'Content simply transposed from being into shine', 'Shine immediately determined but has not posited its content', 'Idealism not advanced beyond being as determinateness'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-3'})
MATCH (topic:Topic {id: 'topic:shn-3'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-3'})
SET c.title = 'Shine in skepticism and idealism — manifold determinations'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 38
SET c.lineEnd = 93
SET c.description = 'Shine is \'phenomenon\' of skepticism, \'appearance\' of idealism. Immediacy which is not something nor thing in general. Not indifferent being apart from determinateness and connection with subject. Skepticism: shine supposed absolutely not to have foundation of being. Idealism: thing-in-itself not supposed to enter into cognitions. But both allow manifold of determinations, full richness of world. Content might have no being as foundation, no thing-in-itself. Content simply transposed from being into shine. Shine is immediately determined, can have this or that content. But whatever content, has not posited it but possesses it immediately. Idealism (Leibnizian, Kantian, Fichtean) not gone further than skepticism. Not advanced beyond being as determinateness.'
SET c.keyPoints = ['Shine is \'phenomenon\' of skepticism, \'appearance\' of idealism', 'Not something nor thing in general, not indifferent being', 'Both allow manifold of determinations, full richness of world', 'Content simply transposed from being into shine', 'Shine immediately determined but has not posited its content', 'Idealism not advanced beyond being as determinateness']
SET c.tags = ['shine', 'mediation', 'appearance', 'citta']
SET c.orderInSource = 3
SET c.globalOrder = 34
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Shine, the “phenomenon” of skepticism,\nand also the “appearance” of idealism,\nis thus this immediacy which is not\na something nor a thing in general,\nnot an indifferent being that would exist apart\nfrom its determinateness and connection with the subject.\nSkepticism did not permit itself to say “It is,”\nand the more recent idealism did not permit itself\nto regard cognitions as a knowledge of the thing-in-itself.\nThe shine of the former was supposed absolutely\nnot to have the foundation of a being:\nthe thing-in-itself was not supposed\nto enter into these cognitions.\nBut at the same time skepticism allowed\na manifold of determinations for its shine,\nor rather the latter turned out to have\nthe full richness of the world for its content.\nLikewise for the appearance of idealism:\nit encompassed the full range of these manifold determinacies.\nSo, the shine of skepticism and the appearance of idealism\ndo immediately have a manifold of determination.\nThis content, therefore, might well have no being as foundation,\nno thing or thing-in-itself;\nfor itself, it remains as it is;\nit is simply transposed from being into shine,\nso that the latter has within itself those manifold\ndeterminacies that exist immediately,\neach an other to the other.\nThe shine is thus itself something immediately determined.\nIt can have this or that content;\nbut whatever content it has, it has not posited it\nbut possesses it immediately.\nIdealism, whether Leibnizian, Kantian, Fichtean, or in any\nother form, has not gone further than skepticism in this:\nit has not advanced beyond being as determinateness.\nSkepticism lets the content of its shine to be given to it;\nthe shine exists for it immediately,\nwhatever content it might have.\nThe Leibnizian monad develops its representations from itself\nbut is not their generating and controlling force;\nthey rise up in it as a froth, indifferent,\nimmediately present to each other and to the monad as well.\nLikewise Kant\'s appearance is a given content of perception\nthat presupposes affections, determinations of the subject\nwhich are immediate to each other and to the subject.\nAs for the infinite obstacle of Fichte\'s Idealism,\nit might well be that it has no thing-in-itself for foundation,\nso that it becomes a determinateness purely within the “I.”\nBut this determinateness that the “I” makes its own,\nsublating its externality,\nis to the “I” at the same time an immediate determinateness,\na limitation of the “I” which the latter may transcend\nbut which contains a side of indifference,\nand on account of this indifference,\nalthough internal to the “I,”\nit entails an immediate non-being of it.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-3'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-3'})
MATCH (c:IntegratedChunk {id: 'shn-3'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-3'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-3'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-3:kp:1'})
SET kp.chunkId = 'shn-3'
SET kp.ordinal = 1
SET kp.text = 'Shine is \'phenomenon\' of skepticism, \'appearance\' of idealism';
MATCH (c:IntegratedChunk {id: 'shn-3'})
MATCH (kp:KeyPoint {id: 'shn-3:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-3:kp:2'})
SET kp.chunkId = 'shn-3'
SET kp.ordinal = 2
SET kp.text = 'Not something nor thing in general, not indifferent being';
MATCH (c:IntegratedChunk {id: 'shn-3'})
MATCH (kp:KeyPoint {id: 'shn-3:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-3:kp:3'})
SET kp.chunkId = 'shn-3'
SET kp.ordinal = 3
SET kp.text = 'Both allow manifold of determinations, full richness of world';
MATCH (c:IntegratedChunk {id: 'shn-3'})
MATCH (kp:KeyPoint {id: 'shn-3:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-3:kp:4'})
SET kp.chunkId = 'shn-3'
SET kp.ordinal = 4
SET kp.text = 'Content simply transposed from being into shine';
MATCH (c:IntegratedChunk {id: 'shn-3'})
MATCH (kp:KeyPoint {id: 'shn-3:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-3:kp:5'})
SET kp.chunkId = 'shn-3'
SET kp.ordinal = 5
SET kp.text = 'Shine immediately determined but has not posited its content';
MATCH (c:IntegratedChunk {id: 'shn-3'})
MATCH (kp:KeyPoint {id: 'shn-3:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-3:kp:6'})
SET kp.chunkId = 'shn-3'
SET kp.ordinal = 6
SET kp.text = 'Idealism not advanced beyond being as determinateness';
MATCH (c:IntegratedChunk {id: 'shn-3'})
MATCH (kp:KeyPoint {id: 'shn-3:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-4'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 95
SET segment.lineEnd = 105
SET segment.text = '2. Shine thus contains an immediate presupposition,\nan independent side vis-à-vis essence.\nBut the task, inasmuch as this shine is distinct from essence,\nis not to demonstrate that it sublates itself\nand returns into essence,\nfor being has returned into essence in its totality;\nshine is the null as such.\nThe task is to demonstrate that the determinations which\ndistinguish it from essence are the determinations of essence itself;\nfurther, that this determinateness of essence,\nwhich shine is, is sublated in essence itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-4'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-4'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-4'
SET topic.title = 'Task: shine\'s determinations are essence\'s own'
SET topic.description = 'Shine contains immediate presupposition, independent side vis-à-vis essence. Task not to demonstrate shine sublates itself and returns into essence. Being has returned into essence in its totality; shine is null as such. Task: demonstrate determinations distinguishing shine from essence are determinations of essence itself. Further: this determinateness of essence (which shine is) is sublated in essence itself.'
SET topic.keyPoints = ['Shine contains immediate presupposition, independent side vis-à-vis essence', 'Task not to demonstrate shine sublates itself and returns into essence', 'Shine is null as such', 'Task: demonstrate shine\'s determinations are essence\'s own determinations', 'Determinateness of essence (which shine is) is sublated in essence itself'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-4'})
MATCH (topic:Topic {id: 'topic:shn-4'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-4'})
SET c.title = 'Task: shine\'s determinations are essence\'s own'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 95
SET c.lineEnd = 105
SET c.description = 'Shine contains immediate presupposition, independent side vis-à-vis essence. Task not to demonstrate shine sublates itself and returns into essence. Being has returned into essence in its totality; shine is null as such. Task: demonstrate determinations distinguishing shine from essence are determinations of essence itself. Further: this determinateness of essence (which shine is) is sublated in essence itself.'
SET c.keyPoints = ['Shine contains immediate presupposition, independent side vis-à-vis essence', 'Task not to demonstrate shine sublates itself and returns into essence', 'Shine is null as such', 'Task: demonstrate shine\'s determinations are essence\'s own determinations', 'Determinateness of essence (which shine is) is sublated in essence itself']
SET c.tags = ['sublation', 'shine', 'mediation', 'citta']
SET c.orderInSource = 4
SET c.globalOrder = 35
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = '2. Shine thus contains an immediate presupposition,\nan independent side vis-à-vis essence.\nBut the task, inasmuch as this shine is distinct from essence,\nis not to demonstrate that it sublates itself\nand returns into essence,\nfor being has returned into essence in its totality;\nshine is the null as such.\nThe task is to demonstrate that the determinations which\ndistinguish it from essence are the determinations of essence itself;\nfurther, that this determinateness of essence,\nwhich shine is, is sublated in essence itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-4'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-4'})
MATCH (c:IntegratedChunk {id: 'shn-4'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-4'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-4'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-4:kp:1'})
SET kp.chunkId = 'shn-4'
SET kp.ordinal = 1
SET kp.text = 'Shine contains immediate presupposition, independent side vis-à-vis essence';
MATCH (c:IntegratedChunk {id: 'shn-4'})
MATCH (kp:KeyPoint {id: 'shn-4:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-4:kp:2'})
SET kp.chunkId = 'shn-4'
SET kp.ordinal = 2
SET kp.text = 'Task not to demonstrate shine sublates itself and returns into essence';
MATCH (c:IntegratedChunk {id: 'shn-4'})
MATCH (kp:KeyPoint {id: 'shn-4:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-4:kp:3'})
SET kp.chunkId = 'shn-4'
SET kp.ordinal = 3
SET kp.text = 'Shine is null as such';
MATCH (c:IntegratedChunk {id: 'shn-4'})
MATCH (kp:KeyPoint {id: 'shn-4:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-4:kp:4'})
SET kp.chunkId = 'shn-4'
SET kp.ordinal = 4
SET kp.text = 'Task: demonstrate shine\'s determinations are essence\'s own determinations';
MATCH (c:IntegratedChunk {id: 'shn-4'})
MATCH (kp:KeyPoint {id: 'shn-4:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-4:kp:5'})
SET kp.chunkId = 'shn-4'
SET kp.ordinal = 5
SET kp.text = 'Determinateness of essence (which shine is) is sublated in essence itself';
MATCH (c:IntegratedChunk {id: 'shn-4'})
MATCH (kp:KeyPoint {id: 'shn-4:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-5'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 107
SET segment.lineEnd = 131
SET segment.text = 'What constitutes the shine is\nthe immediacy of non-being;\nthis non-being, however, is nothing else than\nthe negativity of essence within essence itself.\nIn essence, being is non-being.\nIts inherent nothingness is the\nnegative nature of essence itself.\nBut the immediacy or indifference\nwhich this non-being contains is\nessences\'s own absolute in-itself.\nThe negativity of essence is its self-equality\nor its simple immediacy and indifference.\nBeing has preserved itself in essence inasmuch\nas this latter, in its infinite negativity,\nhas this equality with itself;\nit is through this that essence is itself being.\nThe immediacy that the determinateness has\nin shine against essence is\nthus none other than essence\'s own immediacy,\nthough not the immediacy of an existent\nbut rather the absolutely mediated\nor reflective immediacy which is shine;\nbeing, not as being, but only as\nthe determinateness of being as against mediation;\nbeing as moment.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-5'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-5'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-5'
SET topic.title = 'Shine as essence\'s negativity — immediacy of non-being'
SET topic.description = 'What constitutes shine is immediacy of non-being. This non-being is nothing else than negativity of essence within essence itself. In essence, being is non-being. Inherent nothingness is negative nature of essence itself. Immediacy or indifference which non-being contains is essence\'s own absolute in-itself. Negativity of essence is its self-equality, simple immediacy and indifference. Being preserved in essence through infinite negativity having equality with itself. Through this essence is itself being. Immediacy determinateness has in shine against essence is essence\'s own immediacy. Not immediacy of existent but absolutely mediated or reflective immediacy which is shine. Being, not as being, but only as determinateness of being as against mediation. Being as moment.'
SET topic.keyPoints = ['What constitutes shine is immediacy of non-being', 'Non-being is negativity of essence within essence itself', 'In essence, being is non-being', 'Immediacy which non-being contains is essence\'s own absolute in-itself', 'Immediacy in shine is essence\'s own immediacy, absolutely mediated or reflective immediacy', 'Being as moment, determinateness of being as against mediation'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-5'})
MATCH (topic:Topic {id: 'topic:shn-5'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-5'})
SET c.title = 'Shine as essence\'s negativity — immediacy of non-being'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 107
SET c.lineEnd = 131
SET c.description = 'What constitutes shine is immediacy of non-being. This non-being is nothing else than negativity of essence within essence itself. In essence, being is non-being. Inherent nothingness is negative nature of essence itself. Immediacy or indifference which non-being contains is essence\'s own absolute in-itself. Negativity of essence is its self-equality, simple immediacy and indifference. Being preserved in essence through infinite negativity having equality with itself. Through this essence is itself being. Immediacy determinateness has in shine against essence is essence\'s own immediacy. Not immediacy of existent but absolutely mediated or reflective immediacy which is shine. Being, not as being, but only as determinateness of being as against mediation. Being as moment.'
SET c.keyPoints = ['What constitutes shine is immediacy of non-being', 'Non-being is negativity of essence within essence itself', 'In essence, being is non-being', 'Immediacy which non-being contains is essence\'s own absolute in-itself', 'Immediacy in shine is essence\'s own immediacy, absolutely mediated or reflective immediacy', 'Being as moment, determinateness of being as against mediation']
SET c.tags = ['negation', 'reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 5
SET c.globalOrder = 36
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'What constitutes the shine is\nthe immediacy of non-being;\nthis non-being, however, is nothing else than\nthe negativity of essence within essence itself.\nIn essence, being is non-being.\nIts inherent nothingness is the\nnegative nature of essence itself.\nBut the immediacy or indifference\nwhich this non-being contains is\nessences\'s own absolute in-itself.\nThe negativity of essence is its self-equality\nor its simple immediacy and indifference.\nBeing has preserved itself in essence inasmuch\nas this latter, in its infinite negativity,\nhas this equality with itself;\nit is through this that essence is itself being.\nThe immediacy that the determinateness has\nin shine against essence is\nthus none other than essence\'s own immediacy,\nthough not the immediacy of an existent\nbut rather the absolutely mediated\nor reflective immediacy which is shine;\nbeing, not as being, but only as\nthe determinateness of being as against mediation;\nbeing as moment.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-5'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-5'})
MATCH (c:IntegratedChunk {id: 'shn-5'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-5'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-5'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-5:kp:1'})
SET kp.chunkId = 'shn-5'
SET kp.ordinal = 1
SET kp.text = 'What constitutes shine is immediacy of non-being';
MATCH (c:IntegratedChunk {id: 'shn-5'})
MATCH (kp:KeyPoint {id: 'shn-5:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-5:kp:2'})
SET kp.chunkId = 'shn-5'
SET kp.ordinal = 2
SET kp.text = 'Non-being is negativity of essence within essence itself';
MATCH (c:IntegratedChunk {id: 'shn-5'})
MATCH (kp:KeyPoint {id: 'shn-5:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-5:kp:3'})
SET kp.chunkId = 'shn-5'
SET kp.ordinal = 3
SET kp.text = 'In essence, being is non-being';
MATCH (c:IntegratedChunk {id: 'shn-5'})
MATCH (kp:KeyPoint {id: 'shn-5:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-5:kp:4'})
SET kp.chunkId = 'shn-5'
SET kp.ordinal = 4
SET kp.text = 'Immediacy which non-being contains is essence\'s own absolute in-itself';
MATCH (c:IntegratedChunk {id: 'shn-5'})
MATCH (kp:KeyPoint {id: 'shn-5:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-5:kp:5'})
SET kp.chunkId = 'shn-5'
SET kp.ordinal = 5
SET kp.text = 'Immediacy in shine is essence\'s own immediacy, absolutely mediated or reflective immediacy';
MATCH (c:IntegratedChunk {id: 'shn-5'})
MATCH (kp:KeyPoint {id: 'shn-5:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-5:kp:6'})
SET kp.chunkId = 'shn-5'
SET kp.ordinal = 6
SET kp.text = 'Being as moment, determinateness of being as against mediation';
MATCH (c:IntegratedChunk {id: 'shn-5'})
MATCH (kp:KeyPoint {id: 'shn-5:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-6'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 133
SET segment.lineEnd = 141
SET segment.text = 'These two moments [nothingness but as subsisting],\nand being but as moment;\nor again, negativity existing in itself and reflected immediacy,\nthese two moments that are the moments of shine,\nare thus the moments of essence itself;\nit is not that there is a shine of being in essence,\nor a shine of essence in being:\nthe shine in the essence is not the shine of an other\nbut is rather shine as such, the shine of essence itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-6'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-6'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-6'
SET topic.title = 'Two moments of shine are moments of essence'
SET topic.description = 'Two moments: nothingness but as subsisting, and being but as moment. Or: negativity existing in itself and reflected immediacy. These two moments of shine are moments of essence itself. Not shine of being in essence, or shine of essence in being. Shine in essence is not shine of other but shine as such, shine of essence itself.'
SET topic.keyPoints = ['Two moments: nothingness but as subsisting, and being but as moment', 'Or: negativity existing in itself and reflected immediacy', 'These two moments of shine are moments of essence itself', 'Shine in essence is not shine of other but shine as such, shine of essence itself'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-6'})
MATCH (topic:Topic {id: 'topic:shn-6'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-6'})
SET c.title = 'Two moments of shine are moments of essence'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 133
SET c.lineEnd = 141
SET c.description = 'Two moments: nothingness but as subsisting, and being but as moment. Or: negativity existing in itself and reflected immediacy. These two moments of shine are moments of essence itself. Not shine of being in essence, or shine of essence in being. Shine in essence is not shine of other but shine as such, shine of essence itself.'
SET c.keyPoints = ['Two moments: nothingness but as subsisting, and being but as moment', 'Or: negativity existing in itself and reflected immediacy', 'These two moments of shine are moments of essence itself', 'Shine in essence is not shine of other but shine as such, shine of essence itself']
SET c.tags = ['reflection', 'shine', 'citta']
SET c.orderInSource = 6
SET c.globalOrder = 37
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'These two moments [nothingness but as subsisting],\nand being but as moment;\nor again, negativity existing in itself and reflected immediacy,\nthese two moments that are the moments of shine,\nare thus the moments of essence itself;\nit is not that there is a shine of being in essence,\nor a shine of essence in being:\nthe shine in the essence is not the shine of an other\nbut is rather shine as such, the shine of essence itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-6'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-6'})
MATCH (c:IntegratedChunk {id: 'shn-6'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-6'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-6'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-6:kp:1'})
SET kp.chunkId = 'shn-6'
SET kp.ordinal = 1
SET kp.text = 'Two moments: nothingness but as subsisting, and being but as moment';
MATCH (c:IntegratedChunk {id: 'shn-6'})
MATCH (kp:KeyPoint {id: 'shn-6:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-6:kp:2'})
SET kp.chunkId = 'shn-6'
SET kp.ordinal = 2
SET kp.text = 'Or: negativity existing in itself and reflected immediacy';
MATCH (c:IntegratedChunk {id: 'shn-6'})
MATCH (kp:KeyPoint {id: 'shn-6:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-6:kp:3'})
SET kp.chunkId = 'shn-6'
SET kp.ordinal = 3
SET kp.text = 'These two moments of shine are moments of essence itself';
MATCH (c:IntegratedChunk {id: 'shn-6'})
MATCH (kp:KeyPoint {id: 'shn-6:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-6:kp:4'})
SET kp.chunkId = 'shn-6'
SET kp.ordinal = 4
SET kp.text = 'Shine in essence is not shine of other but shine as such, shine of essence itself';
MATCH (c:IntegratedChunk {id: 'shn-6'})
MATCH (kp:KeyPoint {id: 'shn-6:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-7'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 143
SET segment.lineEnd = 159
SET segment.text = 'Shine is essence itself in the determinateness of being.\nEssence has a shine because it is determined within itself\nand is therefore distinguished from its absolute unity.\nBut this determinateness is as determinateness\njust as absolutely sublated in it.\nFor essence is what stands on its own:\nit exists as self-mediating through a negation which it itself is.\nIt is, therefore, the identical unit of absolute negativity and immediacy.\nThe negativity is negativity in itself;\nit is its reference to itself and thus immediacy in itself.\nBut it is negative reference to itself,\na self-repelling negating;\nthus the immediacy existing in itself is\nthe negative or the determinate over against the negativity.\nBut this determinateness is itself absolute negativity\nand this determining, which as determining immediately sublates itself,\nis a turning back into itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-7'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-7'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-7'
SET topic.title = 'Shine as essence itself in determinateness of being'
SET topic.description = 'Shine is essence itself in determinateness of being. Essence has shine because determined within itself, distinguished from absolute unity. This determinateness absolutely sublated in it. Essence stands on its own, self-mediating through negation which it itself is. Identical unit of absolute negativity and immediacy. Negativity is negativity in itself, reference to itself, immediacy in itself. But negative reference to itself, self-repelling negating. Immediacy existing in itself is negative or determinate over against negativity. But determinateness itself is absolute negativity. Determining immediately sublates itself, turning back into itself.'
SET topic.keyPoints = ['Shine is essence itself in determinateness of being', 'Essence has shine because determined within itself', 'Essence stands on its own, self-mediating through negation which it itself is', 'Identical unit of absolute negativity and immediacy', 'Determining immediately sublates itself, turning back into itself'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-7'})
MATCH (topic:Topic {id: 'topic:shn-7'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-7'})
SET c.title = 'Shine as essence itself in determinateness of being'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 143
SET c.lineEnd = 159
SET c.description = 'Shine is essence itself in determinateness of being. Essence has shine because determined within itself, distinguished from absolute unity. This determinateness absolutely sublated in it. Essence stands on its own, self-mediating through negation which it itself is. Identical unit of absolute negativity and immediacy. Negativity is negativity in itself, reference to itself, immediacy in itself. But negative reference to itself, self-repelling negating. Immediacy existing in itself is negative or determinate over against negativity. But determinateness itself is absolute negativity. Determining immediately sublates itself, turning back into itself.'
SET c.keyPoints = ['Shine is essence itself in determinateness of being', 'Essence has shine because determined within itself', 'Essence stands on its own, self-mediating through negation which it itself is', 'Identical unit of absolute negativity and immediacy', 'Determining immediately sublates itself, turning back into itself']
SET c.tags = ['negation', 'sublation', 'shine', 'mediation', 'citta']
SET c.orderInSource = 7
SET c.globalOrder = 38
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Shine is essence itself in the determinateness of being.\nEssence has a shine because it is determined within itself\nand is therefore distinguished from its absolute unity.\nBut this determinateness is as determinateness\njust as absolutely sublated in it.\nFor essence is what stands on its own:\nit exists as self-mediating through a negation which it itself is.\nIt is, therefore, the identical unit of absolute negativity and immediacy.\nThe negativity is negativity in itself;\nit is its reference to itself and thus immediacy in itself.\nBut it is negative reference to itself,\na self-repelling negating;\nthus the immediacy existing in itself is\nthe negative or the determinate over against the negativity.\nBut this determinateness is itself absolute negativity\nand this determining, which as determining immediately sublates itself,\nis a turning back into itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-7'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-7'})
MATCH (c:IntegratedChunk {id: 'shn-7'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-7'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-7'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-7:kp:1'})
SET kp.chunkId = 'shn-7'
SET kp.ordinal = 1
SET kp.text = 'Shine is essence itself in determinateness of being';
MATCH (c:IntegratedChunk {id: 'shn-7'})
MATCH (kp:KeyPoint {id: 'shn-7:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-7:kp:2'})
SET kp.chunkId = 'shn-7'
SET kp.ordinal = 2
SET kp.text = 'Essence has shine because determined within itself';
MATCH (c:IntegratedChunk {id: 'shn-7'})
MATCH (kp:KeyPoint {id: 'shn-7:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-7:kp:3'})
SET kp.chunkId = 'shn-7'
SET kp.ordinal = 3
SET kp.text = 'Essence stands on its own, self-mediating through negation which it itself is';
MATCH (c:IntegratedChunk {id: 'shn-7'})
MATCH (kp:KeyPoint {id: 'shn-7:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-7:kp:4'})
SET kp.chunkId = 'shn-7'
SET kp.ordinal = 4
SET kp.text = 'Identical unit of absolute negativity and immediacy';
MATCH (c:IntegratedChunk {id: 'shn-7'})
MATCH (kp:KeyPoint {id: 'shn-7:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-7:kp:5'})
SET kp.chunkId = 'shn-7'
SET kp.ordinal = 5
SET kp.text = 'Determining immediately sublates itself, turning back into itself';
MATCH (c:IntegratedChunk {id: 'shn-7'})
MATCH (kp:KeyPoint {id: 'shn-7:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-8'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 161
SET segment.lineEnd = 175
SET segment.text = 'Shine is the negative which has a being,\nbut in another, in its negation;\nit is a non-self-subsisting-being\nwhich is sublated within and null.\nAnd so it is the negative which returns into itself,\nthe non-subsistent as such, internally non-subsistent.\nThis reference of the negative or\nthe non-subsistent to itself is\nthe immediacy of this non-subsistent;\nit is an other than it;\nit is its determinateness over against it,\nor the negation over against the negative.\nBut this negation which stands over against the negative is\nnegativity as referring solely to itself,\nthe absolute sublation of the determinateness itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-8'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-8'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-8'
SET topic.title = 'Shine as negative returning into itself'
SET topic.description = 'Shine is negative which has being, but in another, in its negation. Non-self-subsisting-being sublated within and null. Negative which returns into itself, non-subsistent as such, internally non-subsistent. Reference of negative or non-subsistent to itself is immediacy of this non-subsistent. It is other than it, determinateness over against it, negation over against negative. Negation standing over against negative is negativity referring solely to itself. Absolute sublation of determinateness itself.'
SET topic.keyPoints = ['Shine is negative which has being, but in another, in its negation', 'Non-self-subsisting-being sublated within and null', 'Negative which returns into itself, internally non-subsistent', 'Reference of negative to itself is immediacy of this non-subsistent', 'Negation over against negative is negativity referring solely to itself', 'Absolute sublation of determinateness itself'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-8'})
MATCH (topic:Topic {id: 'topic:shn-8'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-8'})
SET c.title = 'Shine as negative returning into itself'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 161
SET c.lineEnd = 175
SET c.description = 'Shine is negative which has being, but in another, in its negation. Non-self-subsisting-being sublated within and null. Negative which returns into itself, non-subsistent as such, internally non-subsistent. Reference of negative or non-subsistent to itself is immediacy of this non-subsistent. It is other than it, determinateness over against it, negation over against negative. Negation standing over against negative is negativity referring solely to itself. Absolute sublation of determinateness itself.'
SET c.keyPoints = ['Shine is negative which has being, but in another, in its negation', 'Non-self-subsisting-being sublated within and null', 'Negative which returns into itself, internally non-subsistent', 'Reference of negative to itself is immediacy of this non-subsistent', 'Negation over against negative is negativity referring solely to itself', 'Absolute sublation of determinateness itself']
SET c.tags = ['negation', 'sublation', 'shine', 'citta']
SET c.orderInSource = 8
SET c.globalOrder = 39
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'Shine is the negative which has a being,\nbut in another, in its negation;\nit is a non-self-subsisting-being\nwhich is sublated within and null.\nAnd so it is the negative which returns into itself,\nthe non-subsistent as such, internally non-subsistent.\nThis reference of the negative or\nthe non-subsistent to itself is\nthe immediacy of this non-subsistent;\nit is an other than it;\nit is its determinateness over against it,\nor the negation over against the negative.\nBut this negation which stands over against the negative is\nnegativity as referring solely to itself,\nthe absolute sublation of the determinateness itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-8'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-8'})
MATCH (c:IntegratedChunk {id: 'shn-8'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-8'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-8'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-8:kp:1'})
SET kp.chunkId = 'shn-8'
SET kp.ordinal = 1
SET kp.text = 'Shine is negative which has being, but in another, in its negation';
MATCH (c:IntegratedChunk {id: 'shn-8'})
MATCH (kp:KeyPoint {id: 'shn-8:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-8:kp:2'})
SET kp.chunkId = 'shn-8'
SET kp.ordinal = 2
SET kp.text = 'Non-self-subsisting-being sublated within and null';
MATCH (c:IntegratedChunk {id: 'shn-8'})
MATCH (kp:KeyPoint {id: 'shn-8:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-8:kp:3'})
SET kp.chunkId = 'shn-8'
SET kp.ordinal = 3
SET kp.text = 'Negative which returns into itself, internally non-subsistent';
MATCH (c:IntegratedChunk {id: 'shn-8'})
MATCH (kp:KeyPoint {id: 'shn-8:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-8:kp:4'})
SET kp.chunkId = 'shn-8'
SET kp.ordinal = 4
SET kp.text = 'Reference of negative to itself is immediacy of this non-subsistent';
MATCH (c:IntegratedChunk {id: 'shn-8'})
MATCH (kp:KeyPoint {id: 'shn-8:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-8:kp:5'})
SET kp.chunkId = 'shn-8'
SET kp.ordinal = 5
SET kp.text = 'Negation over against negative is negativity referring solely to itself';
MATCH (c:IntegratedChunk {id: 'shn-8'})
MATCH (kp:KeyPoint {id: 'shn-8:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-8:kp:6'})
SET kp.chunkId = 'shn-8'
SET kp.ordinal = 6
SET kp.text = 'Absolute sublation of determinateness itself';
MATCH (c:IntegratedChunk {id: 'shn-8'})
MATCH (kp:KeyPoint {id: 'shn-8:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-9'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 177
SET segment.lineEnd = 190
SET segment.text = 'The determinateness that shine is in essence is,\ntherefore, infinite determinateness;\nit is only the negative which coincides with itself\nand hence a determinateness that, as determinateness,\nis self-subsistence and not determined.\nContrariwise, the self-subsistence, as self-referring immediacy,\nequally is just determinateness and moment,\nnegativity solely referring to itself.\nThis negativity which is identical with immediacy,\nand thus the immediacy which is identical with negativity, is essence.\nShine is, therefore, essence itself,\nbut essence in a determinateness, in such a way, however,\nthat the determinateness is only a moment,\nand the essence is the shining of itself within itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-9'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-9'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-9'
SET topic.title = 'Shine as infinite determinateness — essence'
SET topic.description = 'Determinateness that shine is in essence is infinite determinateness. Only negative which coincides with itself. Determinateness that, as determinateness, is self-subsistence and not determined. Contrariwise, self-subsistence as self-referring immediacy equally just determinateness and moment. Negativity solely referring to itself. Negativity identical with immediacy, immediacy identical with negativity: essence. Shine is essence itself, but essence in determinateness. Determinateness only a moment. Essence is shining of itself within itself.'
SET topic.keyPoints = ['Determinateness that shine is in essence is infinite determinateness', 'Only negative which coincides with itself', 'Determinateness that, as determinateness, is self-subsistence and not determined', 'Negativity identical with immediacy, immediacy identical with negativity: essence', 'Shine is essence itself, but essence in determinateness', 'Essence is shining of itself within itself'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-9'})
MATCH (topic:Topic {id: 'topic:shn-9'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-9'})
SET c.title = 'Shine as infinite determinateness — essence'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 177
SET c.lineEnd = 190
SET c.description = 'Determinateness that shine is in essence is infinite determinateness. Only negative which coincides with itself. Determinateness that, as determinateness, is self-subsistence and not determined. Contrariwise, self-subsistence as self-referring immediacy equally just determinateness and moment. Negativity solely referring to itself. Negativity identical with immediacy, immediacy identical with negativity: essence. Shine is essence itself, but essence in determinateness. Determinateness only a moment. Essence is shining of itself within itself.'
SET c.keyPoints = ['Determinateness that shine is in essence is infinite determinateness', 'Only negative which coincides with itself', 'Determinateness that, as determinateness, is self-subsistence and not determined', 'Negativity identical with immediacy, immediacy identical with negativity: essence', 'Shine is essence itself, but essence in determinateness', 'Essence is shining of itself within itself']
SET c.tags = ['negation', 'shine', 'citta']
SET c.orderInSource = 9
SET c.globalOrder = 40
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'The determinateness that shine is in essence is,\ntherefore, infinite determinateness;\nit is only the negative which coincides with itself\nand hence a determinateness that, as determinateness,\nis self-subsistence and not determined.\nContrariwise, the self-subsistence, as self-referring immediacy,\nequally is just determinateness and moment,\nnegativity solely referring to itself.\nThis negativity which is identical with immediacy,\nand thus the immediacy which is identical with negativity, is essence.\nShine is, therefore, essence itself,\nbut essence in a determinateness, in such a way, however,\nthat the determinateness is only a moment,\nand the essence is the shining of itself within itself.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-9'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-9'})
MATCH (c:IntegratedChunk {id: 'shn-9'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-9'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-9'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-9:kp:1'})
SET kp.chunkId = 'shn-9'
SET kp.ordinal = 1
SET kp.text = 'Determinateness that shine is in essence is infinite determinateness';
MATCH (c:IntegratedChunk {id: 'shn-9'})
MATCH (kp:KeyPoint {id: 'shn-9:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-9:kp:2'})
SET kp.chunkId = 'shn-9'
SET kp.ordinal = 2
SET kp.text = 'Only negative which coincides with itself';
MATCH (c:IntegratedChunk {id: 'shn-9'})
MATCH (kp:KeyPoint {id: 'shn-9:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-9:kp:3'})
SET kp.chunkId = 'shn-9'
SET kp.ordinal = 3
SET kp.text = 'Determinateness that, as determinateness, is self-subsistence and not determined';
MATCH (c:IntegratedChunk {id: 'shn-9'})
MATCH (kp:KeyPoint {id: 'shn-9:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-9:kp:4'})
SET kp.chunkId = 'shn-9'
SET kp.ordinal = 4
SET kp.text = 'Negativity identical with immediacy, immediacy identical with negativity: essence';
MATCH (c:IntegratedChunk {id: 'shn-9'})
MATCH (kp:KeyPoint {id: 'shn-9:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-9:kp:5'})
SET kp.chunkId = 'shn-9'
SET kp.ordinal = 5
SET kp.text = 'Shine is essence itself, but essence in determinateness';
MATCH (c:IntegratedChunk {id: 'shn-9'})
MATCH (kp:KeyPoint {id: 'shn-9:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-9:kp:6'})
SET kp.chunkId = 'shn-9'
SET kp.ordinal = 6
SET kp.text = 'Essence is shining of itself within itself';
MATCH (c:IntegratedChunk {id: 'shn-9'})
MATCH (kp:KeyPoint {id: 'shn-9:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (segment:ChunkSegment {id: 'chunk:shn-10'})
SET segment.sourceId = 'source-shine'
SET segment.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET segment.lineStart = 192
SET segment.lineEnd = 215
SET segment.text = 'In the sphere of being, non-being arises over against being,\neach equally an immediate, and the truth of both is becoming.\nIn the sphere of essence, we have the contrast\nfirst of essence and the non-essential,\nthen of essence and shine,\nthe non-essential and the shine\nbeing both the leftover of being.\nBut these two, and no less the\ndistinction of essence from them,\nconsist solely in this:\nthat essence is taken at first as an immediate,\nnot as it is in itself,\nnamely as an immediacy which is immediacy\nas pure mediacy or absolute negativity.\nThis first immediacy is thus only the determinateness of immediacy.\nThe sublating of this determinateness of essence consists, therefore,\nin nothing further than showing that the unessential is only shine,\nand that essence rather contains this shine within itself.\nFor essence is an infinite self-contained movement\nwhich determines its immediacy as negativity\nand its negativity as immediacy,\nand is thus the shining of itself within itself.\nIn this, in its self-movement,\nessence is reflection.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (segment:ChunkSegment {id: 'chunk:shn-10'})
MERGE (s)-[:HAS_CHUNK_SEGMENT]->(segment);
MERGE (topic:Topic {id: 'topic:shn-10'})
SET topic.sourceId = 'source-shine'
SET topic.topicRef = 'shn-10'
SET topic.title = 'Comparison with sphere of being — essence as reflection'
SET topic.description = 'In sphere of being, non-being arises over against being, each immediate, truth is becoming. In sphere of essence: contrast of essence and non-essential, then essence and shine. Non-essential and shine both leftover of being. Distinction consists in: essence taken at first as immediate, not as it is in itself. As immediacy which is immediacy as pure mediacy or absolute negativity. First immediacy is only determinateness of immediacy. Sublating determinateness: showing unessential is only shine, essence contains shine within itself. Essence is infinite self-contained movement. Determines immediacy as negativity and negativity as immediacy. Shining of itself within itself. In this self-movement, essence is reflection.'
SET topic.keyPoints = ['In sphere of being, non-being over against being, truth is becoming', 'In sphere of essence: contrast of essence and non-essential, then essence and shine', 'Essence taken at first as immediate, not as it is in itself', 'As immediacy which is immediacy as pure mediacy or absolute negativity', 'Essence is infinite self-contained movement', 'Determines immediacy as negativity and negativity as immediacy', 'In this self-movement, essence is reflection'];
MATCH (segment:ChunkSegment {id: 'chunk:shn-10'})
MATCH (topic:Topic {id: 'topic:shn-10'})
MERGE (segment)-[:YIELDS_TOPIC]->(topic);
MERGE (c:IntegratedChunk {id: 'shn-10'})
SET c.title = 'Comparison with sphere of being — essence as reflection'
SET c.sourceId = 'source-shine'
SET c.sourceFile = 'relative/essence/reflection/essence/sources/shine.txt'
SET c.lineStart = 192
SET c.lineEnd = 215
SET c.description = 'In sphere of being, non-being arises over against being, each immediate, truth is becoming. In sphere of essence: contrast of essence and non-essential, then essence and shine. Non-essential and shine both leftover of being. Distinction consists in: essence taken at first as immediate, not as it is in itself. As immediacy which is immediacy as pure mediacy or absolute negativity. First immediacy is only determinateness of immediacy. Sublating determinateness: showing unessential is only shine, essence contains shine within itself. Essence is infinite self-contained movement. Determines immediacy as negativity and negativity as immediacy. Shining of itself within itself. In this self-movement, essence is reflection.'
SET c.keyPoints = ['In sphere of being, non-being over against being, truth is becoming', 'In sphere of essence: contrast of essence and non-essential, then essence and shine', 'Essence taken at first as immediate, not as it is in itself', 'As immediacy which is immediacy as pure mediacy or absolute negativity', 'Essence is infinite self-contained movement', 'Determines immediacy as negativity and negativity as immediacy', 'In this self-movement, essence is reflection']
SET c.tags = ['sublation', 'reflection', 'shine', 'mediation', 'citta']
SET c.orderInSource = 10
SET c.globalOrder = 41
SET c.dialecticalRole = 'CITTA'
SET c.sourceText = 'In the sphere of being, non-being arises over against being,\neach equally an immediate, and the truth of both is becoming.\nIn the sphere of essence, we have the contrast\nfirst of essence and the non-essential,\nthen of essence and shine,\nthe non-essential and the shine\nbeing both the leftover of being.\nBut these two, and no less the\ndistinction of essence from them,\nconsist solely in this:\nthat essence is taken at first as an immediate,\nnot as it is in itself,\nnamely as an immediacy which is immediacy\nas pure mediacy or absolute negativity.\nThis first immediacy is thus only the determinateness of immediacy.\nThe sublating of this determinateness of essence consists, therefore,\nin nothing further than showing that the unessential is only shine,\nand that essence rather contains this shine within itself.\nFor essence is an infinite self-contained movement\nwhich determines its immediacy as negativity\nand its negativity as immediacy,\nand is thus the shining of itself within itself.\nIn this, in its self-movement,\nessence is reflection.';
MATCH (s:SourceText {id: 'source-shine'})
MATCH (c:IntegratedChunk {id: 'shn-10'})
MERGE (s)-[:HAS_CHUNK]->(c);
MATCH (topic:Topic {id: 'topic:shn-10'})
MATCH (c:IntegratedChunk {id: 'shn-10'})
MERGE (topic)-[:LIFTED_TO_IR]->(c);
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (cat:CITCategory {id: 'CITTA'})
MERGE (c)-[:CLASSIFIED_AS]->(cat);
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (root:CITCategory {id: 'CITTA'})
MERGE (c)-[:MANIFESTS]->(root);
MERGE (kp:KeyPoint {id: 'shn-10:kp:1'})
SET kp.chunkId = 'shn-10'
SET kp.ordinal = 1
SET kp.text = 'In sphere of being, non-being over against being, truth is becoming';
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (kp:KeyPoint {id: 'shn-10:kp:1'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-10:kp:2'})
SET kp.chunkId = 'shn-10'
SET kp.ordinal = 2
SET kp.text = 'In sphere of essence: contrast of essence and non-essential, then essence and shine';
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (kp:KeyPoint {id: 'shn-10:kp:2'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-10:kp:3'})
SET kp.chunkId = 'shn-10'
SET kp.ordinal = 3
SET kp.text = 'Essence taken at first as immediate, not as it is in itself';
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (kp:KeyPoint {id: 'shn-10:kp:3'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-10:kp:4'})
SET kp.chunkId = 'shn-10'
SET kp.ordinal = 4
SET kp.text = 'As immediacy which is immediacy as pure mediacy or absolute negativity';
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (kp:KeyPoint {id: 'shn-10:kp:4'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-10:kp:5'})
SET kp.chunkId = 'shn-10'
SET kp.ordinal = 5
SET kp.text = 'Essence is infinite self-contained movement';
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (kp:KeyPoint {id: 'shn-10:kp:5'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-10:kp:6'})
SET kp.chunkId = 'shn-10'
SET kp.ordinal = 6
SET kp.text = 'Determines immediacy as negativity and negativity as immediacy';
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (kp:KeyPoint {id: 'shn-10:kp:6'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
MERGE (kp:KeyPoint {id: 'shn-10:kp:7'})
SET kp.chunkId = 'shn-10'
SET kp.ordinal = 7
SET kp.text = 'In this self-movement, essence is reflection';
MATCH (c:IntegratedChunk {id: 'shn-10'})
MATCH (kp:KeyPoint {id: 'shn-10:kp:7'})
MERGE (c)-[:HAS_KEY_POINT]->(kp);
