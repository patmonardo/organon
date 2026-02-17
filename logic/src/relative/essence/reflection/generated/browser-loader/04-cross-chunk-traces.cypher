MATCH (a:IntegratedChunk {id: 'ess-1'})
MATCH (b:IntegratedChunk {id: 'ess-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-1'})
MATCH (b:IntegratedChunk {id: 'ess-2'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-2 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-2'})
MATCH (b:IntegratedChunk {id: 'ess-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-2'})
MATCH (b:IntegratedChunk {id: 'ess-3'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-3 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-3'})
MATCH (b:IntegratedChunk {id: 'ess-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-3'})
MATCH (b:IntegratedChunk {id: 'ess-4'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-4 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-4'})
MATCH (b:IntegratedChunk {id: 'ess-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-4'})
MATCH (b:IntegratedChunk {id: 'ess-5'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-5 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-5'})
MATCH (b:IntegratedChunk {id: 'ess-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-5'})
MATCH (b:IntegratedChunk {id: 'ess-6'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-6 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-6'})
MATCH (b:IntegratedChunk {id: 'ess-7'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-6'})
MATCH (b:IntegratedChunk {id: 'ess-7'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from ess-7 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-7'})
MATCH (b:IntegratedChunk {id: 'ess-8'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-7'})
MATCH (b:IntegratedChunk {id: 'ess-8'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-8 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-8'})
MATCH (b:IntegratedChunk {id: 'ess-9'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-8'})
MATCH (b:IntegratedChunk {id: 'ess-9'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-9 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-9'})
MATCH (b:IntegratedChunk {id: 'ess-10'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-9'})
MATCH (b:IntegratedChunk {id: 'ess-10'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-10 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-10'})
MATCH (b:IntegratedChunk {id: 'ess-11'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-10'})
MATCH (b:IntegratedChunk {id: 'ess-11'})
MERGE (a)-[r:SPIRALS_TO]->(b)
SET r.reason = 'Dialectical transition inferred from ess-11 semantics.';
MATCH (a:IntegratedChunk {id: 'ess-11'})
MATCH (b:IntegratedChunk {id: 'ess-12'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ess-11'})
MATCH (b:IntegratedChunk {id: 'ess-12'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ess-12 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-1'})
MATCH (b:IntegratedChunk {id: 'ref-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-1'})
MATCH (b:IntegratedChunk {id: 'ref-2'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-2 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-2'})
MATCH (b:IntegratedChunk {id: 'ref-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-2'})
MATCH (b:IntegratedChunk {id: 'ref-3'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-3 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-3'})
MATCH (b:IntegratedChunk {id: 'ref-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-3'})
MATCH (b:IntegratedChunk {id: 'ref-4'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-4 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-4'})
MATCH (b:IntegratedChunk {id: 'ref-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-4'})
MATCH (b:IntegratedChunk {id: 'ref-5'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-5 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-5'})
MATCH (b:IntegratedChunk {id: 'ref-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-5'})
MATCH (b:IntegratedChunk {id: 'ref-6'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-6 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-6'})
MATCH (b:IntegratedChunk {id: 'ref-7'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-6'})
MATCH (b:IntegratedChunk {id: 'ref-7'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-7 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-7'})
MATCH (b:IntegratedChunk {id: 'ref-8'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-7'})
MATCH (b:IntegratedChunk {id: 'ref-8'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-8 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-8'})
MATCH (b:IntegratedChunk {id: 'ref-9'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-8'})
MATCH (b:IntegratedChunk {id: 'ref-9'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-9 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-9'})
MATCH (b:IntegratedChunk {id: 'ref-10'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-9'})
MATCH (b:IntegratedChunk {id: 'ref-10'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-10 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-10'})
MATCH (b:IntegratedChunk {id: 'ref-11'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-10'})
MATCH (b:IntegratedChunk {id: 'ref-11'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-11 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-11'})
MATCH (b:IntegratedChunk {id: 'ref-12'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-11'})
MATCH (b:IntegratedChunk {id: 'ref-12'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-12 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-12'})
MATCH (b:IntegratedChunk {id: 'ref-13'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-12'})
MATCH (b:IntegratedChunk {id: 'ref-13'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-13 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-13'})
MATCH (b:IntegratedChunk {id: 'ref-14'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-13'})
MATCH (b:IntegratedChunk {id: 'ref-14'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-14 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-14'})
MATCH (b:IntegratedChunk {id: 'ref-15'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-14'})
MATCH (b:IntegratedChunk {id: 'ref-15'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-15 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-15'})
MATCH (b:IntegratedChunk {id: 'ref-16'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-15'})
MATCH (b:IntegratedChunk {id: 'ref-16'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-16 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-16'})
MATCH (b:IntegratedChunk {id: 'ref-17'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-16'})
MATCH (b:IntegratedChunk {id: 'ref-17'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-17 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-17'})
MATCH (b:IntegratedChunk {id: 'ref-18'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-17'})
MATCH (b:IntegratedChunk {id: 'ref-18'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-18 semantics.';
MATCH (a:IntegratedChunk {id: 'ref-18'})
MATCH (b:IntegratedChunk {id: 'ref-19'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ref-18'})
MATCH (b:IntegratedChunk {id: 'ref-19'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ref-19 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-1'})
MATCH (b:IntegratedChunk {id: 'shn-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-1'})
MATCH (b:IntegratedChunk {id: 'shn-2'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from shn-2 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-2'})
MATCH (b:IntegratedChunk {id: 'shn-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-2'})
MATCH (b:IntegratedChunk {id: 'shn-3'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from shn-3 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-3'})
MATCH (b:IntegratedChunk {id: 'shn-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-3'})
MATCH (b:IntegratedChunk {id: 'shn-4'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from shn-4 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-4'})
MATCH (b:IntegratedChunk {id: 'shn-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-4'})
MATCH (b:IntegratedChunk {id: 'shn-5'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from shn-5 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-5'})
MATCH (b:IntegratedChunk {id: 'shn-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-5'})
MATCH (b:IntegratedChunk {id: 'shn-6'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from shn-6 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-6'})
MATCH (b:IntegratedChunk {id: 'shn-7'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-6'})
MATCH (b:IntegratedChunk {id: 'shn-7'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from shn-7 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-7'})
MATCH (b:IntegratedChunk {id: 'shn-8'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-7'})
MATCH (b:IntegratedChunk {id: 'shn-8'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from shn-8 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-8'})
MATCH (b:IntegratedChunk {id: 'shn-9'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-8'})
MATCH (b:IntegratedChunk {id: 'shn-9'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from shn-9 semantics.';
MATCH (a:IntegratedChunk {id: 'shn-9'})
MATCH (b:IntegratedChunk {id: 'shn-10'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'shn-9'})
MATCH (b:IntegratedChunk {id: 'shn-10'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from shn-10 semantics.';
MATCH (a:IntegratedChunk {id: 'idn-1'})
MATCH (b:IntegratedChunk {id: 'idn-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'idn-1'})
MATCH (b:IntegratedChunk {id: 'idn-2'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from idn-2 semantics.';
MATCH (a:IntegratedChunk {id: 'idn-2'})
MATCH (b:IntegratedChunk {id: 'idn-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'idn-2'})
MATCH (b:IntegratedChunk {id: 'idn-3'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from idn-3 semantics.';
MATCH (a:IntegratedChunk {id: 'idn-3'})
MATCH (b:IntegratedChunk {id: 'idn-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'idn-3'})
MATCH (b:IntegratedChunk {id: 'idn-4'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from idn-4 semantics.';
MATCH (a:IntegratedChunk {id: 'idn-4'})
MATCH (b:IntegratedChunk {id: 'idn-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'idn-4'})
MATCH (b:IntegratedChunk {id: 'idn-5'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from idn-5 semantics.';
MATCH (a:IntegratedChunk {id: 'idn-5'})
MATCH (b:IntegratedChunk {id: 'idn-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'idn-5'})
MATCH (b:IntegratedChunk {id: 'idn-6'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from idn-6 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-1'})
MATCH (b:IntegratedChunk {id: 'diff-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-1'})
MATCH (b:IntegratedChunk {id: 'diff-2'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from diff-2 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-2'})
MATCH (b:IntegratedChunk {id: 'diff-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-2'})
MATCH (b:IntegratedChunk {id: 'diff-3'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from diff-3 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-3'})
MATCH (b:IntegratedChunk {id: 'diff-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-3'})
MATCH (b:IntegratedChunk {id: 'diff-4'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from diff-4 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-4'})
MATCH (b:IntegratedChunk {id: 'diff-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-4'})
MATCH (b:IntegratedChunk {id: 'diff-5'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from diff-5 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-5'})
MATCH (b:IntegratedChunk {id: 'diff-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-5'})
MATCH (b:IntegratedChunk {id: 'diff-6'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from diff-6 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-6'})
MATCH (b:IntegratedChunk {id: 'diff-7'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-6'})
MATCH (b:IntegratedChunk {id: 'diff-7'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from diff-7 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-7'})
MATCH (b:IntegratedChunk {id: 'diff-8'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-7'})
MATCH (b:IntegratedChunk {id: 'diff-8'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from diff-8 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-8'})
MATCH (b:IntegratedChunk {id: 'diff-9'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-8'})
MATCH (b:IntegratedChunk {id: 'diff-9'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from diff-9 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-9'})
MATCH (b:IntegratedChunk {id: 'diff-10'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-9'})
MATCH (b:IntegratedChunk {id: 'diff-10'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from diff-10 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-10'})
MATCH (b:IntegratedChunk {id: 'diff-11'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-10'})
MATCH (b:IntegratedChunk {id: 'diff-11'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from diff-11 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-11'})
MATCH (b:IntegratedChunk {id: 'diff-12'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-11'})
MATCH (b:IntegratedChunk {id: 'diff-12'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from diff-12 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-12'})
MATCH (b:IntegratedChunk {id: 'diff-13'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-12'})
MATCH (b:IntegratedChunk {id: 'diff-13'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from diff-13 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-13'})
MATCH (b:IntegratedChunk {id: 'diff-14'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-13'})
MATCH (b:IntegratedChunk {id: 'diff-14'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from diff-14 semantics.';
MATCH (a:IntegratedChunk {id: 'diff-14'})
MATCH (b:IntegratedChunk {id: 'diff-15'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'diff-14'})
MATCH (b:IntegratedChunk {id: 'diff-15'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from diff-15 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-1'})
MATCH (b:IntegratedChunk {id: 'ctr-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-1'})
MATCH (b:IntegratedChunk {id: 'ctr-2'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-2 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-2'})
MATCH (b:IntegratedChunk {id: 'ctr-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-2'})
MATCH (b:IntegratedChunk {id: 'ctr-3'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-3 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-3'})
MATCH (b:IntegratedChunk {id: 'ctr-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-3'})
MATCH (b:IntegratedChunk {id: 'ctr-4'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-4 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-4'})
MATCH (b:IntegratedChunk {id: 'ctr-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-4'})
MATCH (b:IntegratedChunk {id: 'ctr-5'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-5 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-5'})
MATCH (b:IntegratedChunk {id: 'ctr-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-5'})
MATCH (b:IntegratedChunk {id: 'ctr-6'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-6 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-6'})
MATCH (b:IntegratedChunk {id: 'ctr-7'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-6'})
MATCH (b:IntegratedChunk {id: 'ctr-7'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-7 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-7'})
MATCH (b:IntegratedChunk {id: 'ctr-8'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-7'})
MATCH (b:IntegratedChunk {id: 'ctr-8'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-8 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-8'})
MATCH (b:IntegratedChunk {id: 'ctr-9'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-8'})
MATCH (b:IntegratedChunk {id: 'ctr-9'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-9 semantics.';
MATCH (a:IntegratedChunk {id: 'ctr-9'})
MATCH (b:IntegratedChunk {id: 'ctr-10'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'ctr-9'})
MATCH (b:IntegratedChunk {id: 'ctr-10'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from ctr-10 semantics.';
MATCH (a:IntegratedChunk {id: 'det-1'})
MATCH (b:IntegratedChunk {id: 'det-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-1'})
MATCH (b:IntegratedChunk {id: 'det-2'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-2 semantics.';
MATCH (a:IntegratedChunk {id: 'det-2'})
MATCH (b:IntegratedChunk {id: 'det-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-2'})
MATCH (b:IntegratedChunk {id: 'det-3'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-3 semantics.';
MATCH (a:IntegratedChunk {id: 'det-3'})
MATCH (b:IntegratedChunk {id: 'det-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-3'})
MATCH (b:IntegratedChunk {id: 'det-4'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from det-4 semantics.';
MATCH (a:IntegratedChunk {id: 'det-4'})
MATCH (b:IntegratedChunk {id: 'det-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-4'})
MATCH (b:IntegratedChunk {id: 'det-5'})
MERGE (a)-[r:SPIRALS_TO]->(b)
SET r.reason = 'Dialectical transition inferred from det-5 semantics.';
MATCH (a:IntegratedChunk {id: 'det-5'})
MATCH (b:IntegratedChunk {id: 'det-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-5'})
MATCH (b:IntegratedChunk {id: 'det-6'})
MERGE (a)-[r:SPIRALS_TO]->(b)
SET r.reason = 'Dialectical transition inferred from det-6 semantics.';
MATCH (a:IntegratedChunk {id: 'det-6'})
MATCH (b:IntegratedChunk {id: 'det-7'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-6'})
MATCH (b:IntegratedChunk {id: 'det-7'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-7 semantics.';
MATCH (a:IntegratedChunk {id: 'det-7'})
MATCH (b:IntegratedChunk {id: 'det-8'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-7'})
MATCH (b:IntegratedChunk {id: 'det-8'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-8 semantics.';
MATCH (a:IntegratedChunk {id: 'det-8'})
MATCH (b:IntegratedChunk {id: 'det-9'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-8'})
MATCH (b:IntegratedChunk {id: 'det-9'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-9 semantics.';
MATCH (a:IntegratedChunk {id: 'det-9'})
MATCH (b:IntegratedChunk {id: 'det-10'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-9'})
MATCH (b:IntegratedChunk {id: 'det-10'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-10 semantics.';
MATCH (a:IntegratedChunk {id: 'det-10'})
MATCH (b:IntegratedChunk {id: 'det-11'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-10'})
MATCH (b:IntegratedChunk {id: 'det-11'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-11 semantics.';
MATCH (a:IntegratedChunk {id: 'det-11'})
MATCH (b:IntegratedChunk {id: 'det-12'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-11'})
MATCH (b:IntegratedChunk {id: 'det-12'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-12 semantics.';
MATCH (a:IntegratedChunk {id: 'det-12'})
MATCH (b:IntegratedChunk {id: 'det-13'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-12'})
MATCH (b:IntegratedChunk {id: 'det-13'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-13 semantics.';
MATCH (a:IntegratedChunk {id: 'det-13'})
MATCH (b:IntegratedChunk {id: 'det-14'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'det-13'})
MATCH (b:IntegratedChunk {id: 'det-14'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from det-14 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-1'})
MATCH (b:IntegratedChunk {id: 'abs-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-1'})
MATCH (b:IntegratedChunk {id: 'abs-2'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-2 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-2'})
MATCH (b:IntegratedChunk {id: 'abs-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-2'})
MATCH (b:IntegratedChunk {id: 'abs-3'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-3 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-3'})
MATCH (b:IntegratedChunk {id: 'abs-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-3'})
MATCH (b:IntegratedChunk {id: 'abs-4'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-4 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-4'})
MATCH (b:IntegratedChunk {id: 'abs-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-4'})
MATCH (b:IntegratedChunk {id: 'abs-5'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-5 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-5'})
MATCH (b:IntegratedChunk {id: 'abs-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-5'})
MATCH (b:IntegratedChunk {id: 'abs-6'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-6 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-6'})
MATCH (b:IntegratedChunk {id: 'abs-7'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-6'})
MATCH (b:IntegratedChunk {id: 'abs-7'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-7 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-7'})
MATCH (b:IntegratedChunk {id: 'abs-8'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-7'})
MATCH (b:IntegratedChunk {id: 'abs-8'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-8 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-8'})
MATCH (b:IntegratedChunk {id: 'abs-9'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-8'})
MATCH (b:IntegratedChunk {id: 'abs-9'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-9 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-9'})
MATCH (b:IntegratedChunk {id: 'abs-10'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-9'})
MATCH (b:IntegratedChunk {id: 'abs-10'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-10 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-10'})
MATCH (b:IntegratedChunk {id: 'abs-11'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-10'})
MATCH (b:IntegratedChunk {id: 'abs-11'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-11 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-11'})
MATCH (b:IntegratedChunk {id: 'abs-12'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-11'})
MATCH (b:IntegratedChunk {id: 'abs-12'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-12 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-12'})
MATCH (b:IntegratedChunk {id: 'abs-13'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-12'})
MATCH (b:IntegratedChunk {id: 'abs-13'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-13 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-13'})
MATCH (b:IntegratedChunk {id: 'abs-14'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-13'})
MATCH (b:IntegratedChunk {id: 'abs-14'})
MERGE (a)-[r:REFLECTS]->(b)
SET r.reason = 'Dialectical transition inferred from abs-14 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-14'})
MATCH (b:IntegratedChunk {id: 'abs-15'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-14'})
MATCH (b:IntegratedChunk {id: 'abs-15'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from abs-15 semantics.';
MATCH (a:IntegratedChunk {id: 'abs-15'})
MATCH (b:IntegratedChunk {id: 'abs-16'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'abs-15'})
MATCH (b:IntegratedChunk {id: 'abs-16'})
MERGE (a)-[r:SPIRALS_TO]->(b)
SET r.reason = 'Dialectical transition inferred from abs-16 semantics.';
MATCH (a:IntegratedChunk {id: 'con-1'})
MATCH (b:IntegratedChunk {id: 'con-2'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-1'})
MATCH (b:IntegratedChunk {id: 'con-2'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-2 semantics.';
MATCH (a:IntegratedChunk {id: 'con-2'})
MATCH (b:IntegratedChunk {id: 'con-3'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-2'})
MATCH (b:IntegratedChunk {id: 'con-3'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-3 semantics.';
MATCH (a:IntegratedChunk {id: 'con-3'})
MATCH (b:IntegratedChunk {id: 'con-4'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-3'})
MATCH (b:IntegratedChunk {id: 'con-4'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-4 semantics.';
MATCH (a:IntegratedChunk {id: 'con-4'})
MATCH (b:IntegratedChunk {id: 'con-5'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-4'})
MATCH (b:IntegratedChunk {id: 'con-5'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-5 semantics.';
MATCH (a:IntegratedChunk {id: 'con-5'})
MATCH (b:IntegratedChunk {id: 'con-6'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-5'})
MATCH (b:IntegratedChunk {id: 'con-6'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-6 semantics.';
MATCH (a:IntegratedChunk {id: 'con-6'})
MATCH (b:IntegratedChunk {id: 'con-7'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-6'})
MATCH (b:IntegratedChunk {id: 'con-7'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-7 semantics.';
MATCH (a:IntegratedChunk {id: 'con-7'})
MATCH (b:IntegratedChunk {id: 'con-8'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-7'})
MATCH (b:IntegratedChunk {id: 'con-8'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-8 semantics.';
MATCH (a:IntegratedChunk {id: 'con-8'})
MATCH (b:IntegratedChunk {id: 'con-9'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-8'})
MATCH (b:IntegratedChunk {id: 'con-9'})
MERGE (a)-[r:MEDIATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-9 semantics.';
MATCH (a:IntegratedChunk {id: 'con-9'})
MATCH (b:IntegratedChunk {id: 'con-10'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-9'})
MATCH (b:IntegratedChunk {id: 'con-10'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-10 semantics.';
MATCH (a:IntegratedChunk {id: 'con-10'})
MATCH (b:IntegratedChunk {id: 'con-11'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-10'})
MATCH (b:IntegratedChunk {id: 'con-11'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-11 semantics.';
MATCH (a:IntegratedChunk {id: 'con-11'})
MATCH (b:IntegratedChunk {id: 'con-12'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-11'})
MATCH (b:IntegratedChunk {id: 'con-12'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-12 semantics.';
MATCH (a:IntegratedChunk {id: 'con-12'})
MATCH (b:IntegratedChunk {id: 'con-13'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-12'})
MATCH (b:IntegratedChunk {id: 'con-13'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-13 semantics.';
MATCH (a:IntegratedChunk {id: 'con-13'})
MATCH (b:IntegratedChunk {id: 'con-14'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-13'})
MATCH (b:IntegratedChunk {id: 'con-14'})
MERGE (a)-[r:NEGATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-14 semantics.';
MATCH (a:IntegratedChunk {id: 'con-14'})
MATCH (b:IntegratedChunk {id: 'con-15'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-14'})
MATCH (b:IntegratedChunk {id: 'con-15'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-15 semantics.';
MATCH (a:IntegratedChunk {id: 'con-15'})
MATCH (b:IntegratedChunk {id: 'con-16'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Sequential order in source text.';
MATCH (a:IntegratedChunk {id: 'con-15'})
MATCH (b:IntegratedChunk {id: 'con-16'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Dialectical transition inferred from con-16 semantics.';
MATCH (a:IntegratedChunk {id: 'idn-6'})
MATCH (b:IntegratedChunk {id: 'diff-2'})
MERGE (a)-[r:LAYER_NEGATION]->(b)
SET r.reason = 'Principle of Citta advances by negation from identity into difference.';
MATCH (a:IntegratedChunk {id: 'diff-15'})
MATCH (b:IntegratedChunk {id: 'ctr-1'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Law operation: identity and difference are sublated into contradiction.';
MATCH (a:IntegratedChunk {id: 'ctr-10'})
MATCH (b:IntegratedChunk {id: 'ref-2'})
MERGE (a)-[r:SUBLATES]->(b)
SET r.reason = 'Principle and law are sublated into Reflection as layered negation science.';
MATCH (a:IntegratedChunk {id: 'ref-15'})
MATCH (b:IntegratedChunk {id: 'det-1'})
MERGE (a)-[r:NEXT]->(b)
SET r.reason = 'Reflection sequence continues into Ground through determinate grounding.';
MATCH (a:IntegratedChunk {id: 'det-14'})
MATCH (b:IntegratedChunk {id: 'abs-1'})
MERGE (a)-[r:SPIRALS_TO]->(b)
SET r.reason = 'Complete mediation spirals into Absolute Ground.';
MATCH (a:IntegratedChunk {id: 'abs-16'})
MATCH (b:IntegratedChunk {id: 'con-1'})
MERGE (a)-[r:SPIRALS_TO]->(b)
SET r.reason = 'Determinate ground transitions into Condition as conditioned mediation.';
