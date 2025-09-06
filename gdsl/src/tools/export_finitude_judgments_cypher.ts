import { LOGICAL_OPERATIONS as FINITUDE_OPS } from '../logic/existence/finitude1'
import { extractJudgments } from '../engine/judgments'

function esc(s: string) { return (s ?? '').replace(/\\/g,'\\\\').replace(/"/g,'\\"') }

function main() {
  const Js = extractJudgments(FINITUDE_OPS)
  const lines: string[] = []
  lines.push(
    'CREATE CONSTRAINT IF NOT EXISTS FOR (j:Judgment) REQUIRE j.id IS UNIQUE;',
    'CREATE CONSTRAINT IF NOT EXISTS FOR (c:Concept) REQUIRE c.name IS UNIQUE;'
  )
  for (const j of Js) {
    lines.push(`MERGE (j:Judgment {id:"${esc(j.id)}"})`)
    if (j.type === 'Affirmative') lines.push(`SET j.type="Affirmative", j.source="${esc(j.source)}", j.predicate="${esc(j.predicate)}"`)
    if (j.type === 'Negative') lines.push(`SET j.type="Negative", j.source="${esc(j.source)}", j.predicate="${esc(j.predicate)}"`)
    if (j.type === 'Relational') lines.push(`SET j.type="Relational", j.source="${esc(j.source)}", j.relation="${esc(j.relation)}"`)
    // Subject/Object concepts
    const subj = 'subject' in j ? j.subject : ''
    const obj = 'object' in j ? j.object : ''
    lines.push(
      `MERGE (s:Concept {name:"${esc(subj)}"})`,
      `MERGE (o:Concept {name:"${esc(obj)}"})`,
      `MERGE (j)-[:SUBJECT]->(s)`,
      `MERGE (j)-[:OBJECT]->(o)`
    )
  }
  console.log(lines.join('\n'))
}

main()
