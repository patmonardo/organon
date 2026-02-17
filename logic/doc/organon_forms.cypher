// Cypher Encoding of ORGANON-FORMS for FormDB FactStore
// This script creates the graph structure for storing and querying ORGANON-FORMS definitions.
// Focus: Forms as guides for FormDB storage in FactStore (graph-based durable grounding).

// 1. Create Form Definition Nodes (metaclass level)
CREATE (f_identity_core:FormDef {
  id: 'f.identity.core',
  kind: 'form-def',
  subject: 'Form',
  clauses: [
    {operator: 'require', args: ['identityKeys']},
    {operator: 'incompatibleWith', args: ['anonymous']}
  ],
  justification: "Identity-bearing forms require stable keys.",
  priority: null,
  provenance: null
});

CREATE (c_measurement_scope:ContextDef {
  id: 'c.measurement.scope',
  kind: 'context-def',
  subject: 'Context',
  clauses: [
    {operator: 'within', args: ['empirical-context']},
    {operator: 'requiresEvidence', args: ['observation']}
  ],
  justification: "Claims are valid only under explicit evidential context.",
  priority: null,
  provenance: null
});

CREATE (m_grounding_transition:MorphDef {
  id: 'm.grounding.transition',
  kind: 'morph-def',
  subject: 'Morph',
  clauses: [
    {operator: 'from', args: ['resolved']},
    {operator: 'to', args: ['grounded']},
    {operator: 'preserve', args: ['identityKeys']}
  ],
  justification: "Transition to grounded must preserve identity continuity.",
  priority: null,
  provenance: null
});

// 2. Create Entity Definition Nodes (class level)
CREATE (person:Entity {
  id: 'person',
  label: 'Person',
  identityKeys: ['id'],
  extends: [],
  disjointWith: [],
  equivalentTo: [],
  facets: {}
});

CREATE (organization:Entity {
  id: 'organization',
  label: 'Organization',
  identityKeys: ['orgId'],
  extends: [],
  disjointWith: [],
  equivalentTo: [],
  facets: {}
});

// 3. Create Property Definition Nodes
CREATE (name:Property {
  id: 'name',
  contextTypeId: 'c.measurement.scope',
  valueKind: 'datatype',
  subjectTypeId: 'person',
  datatype: 'string',
  objectTypeId: null,
  cardinality: {min: 1, max: 1, exact: null},
  functional: true,
  defaultValue: null
});

CREATE (age:Property {
  id: 'age',
  contextTypeId: 'c.measurement.scope',
  valueKind: 'datatype',
  subjectTypeId: 'person',
  datatype: 'integer',
  objectTypeId: null,
  cardinality: {min: 0, max: 1, exact: null},
  functional: true,
  defaultValue: null
});

// 4. Create Aspect Definition Nodes
CREATE (employs:Aspect {
  id: 'employs',
  relationType: 'employment',
  subjectTypeId: 'organization',
  objectTypeId: 'person',
  predicate: 'employs',
  polarity: 'affirmed',
  constraints: []
});

// 5. Create Relationships (Instantiation and Associations)

// Entity instantiates Form
CREATE (person)-[:INSTANTIATES]->(f_identity_core);
CREATE (organization)-[:INSTANTIATES]->(f_identity_core);

// Property scoped by Context
CREATE (name)-[:SCOPED_BY]->(c_measurement_scope);
CREATE (age)-[:SCOPED_BY]->(c_measurement_scope);

// Aspect mediates Morph
CREATE (employs)-[:MEDIATES]->(m_grounding_transition);

// Additional relationships for state semantics
// Assuming state nodes exist or are created dynamically
// For now, represent state transitions as relationships
CREATE (resolved:State {name: 'resolved'});
CREATE (grounded:State {name: 'grounded'});

CREATE (m_grounding_transition)-[:FROM]->(resolved);
CREATE (m_grounding_transition)-[:TO]->(grounded);
CREATE (m_grounding_transition)-[:PRESERVE {invariant: 'identityKeys'}]->(m_grounding_transition);

// Container for world-governing (if needed)
CREATE (default_container:Container {
  id: 'default',
  activeContexts: ['c.measurement.scope'],
  activeForms: ['f.identity.core'],
  activeMorphs: ['m.grounding.transition']
});

// Claims under container
CREATE (person)-[:HAS_PROPERTY {status: 'asserted', containerId: 'default'}]->(name);
CREATE (person)-[:HAS_PROPERTY {status: 'asserted', containerId: 'default'}]->(age);
CREATE (organization)-[:HAS_ASPECT {status: 'inferred', containerId: 'default'}]->(employs);

// Indexes for performance
CREATE INDEX ON :FormDef(id);
CREATE INDEX ON :ContextDef(id);
CREATE INDEX ON :MorphDef(id);
CREATE INDEX ON :Entity(id);
CREATE INDEX ON :Property(id);
CREATE INDEX ON :Aspect(id);
CREATE INDEX ON :Container(id);

// Example Queries (for validation)

// Find all entities instantiating a form
// MATCH (e:Entity)-[:INSTANTIATES]->(f:FormDef {id: 'f.identity.core'}) RETURN e.id;

// Find properties scoped by a context
// MATCH (p:Property)-[:SCOPED_BY]->(c:ContextDef {id: 'c.measurement.scope'}) RETURN p.id;

// Find aspects mediating a morph
// MATCH (a:Aspect)-[:MEDIATES]->(m:MorphDef {id: 'm.grounding.transition'}) RETURN a.id;

// Check state transitions
// MATCH (m:MorphDef)-[:FROM]->(s1:State), (m)-[:TO]->(s2:State) RETURN m.id, s1.name, s2.name;

// Query claims under container
// MATCH (e:Entity)-[r:HAS_PROPERTY]->(p:Property) WHERE r.containerId = 'default' RETURN e.id, p.id, r.status;
