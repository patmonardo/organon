import pl from 'tau-prolog';

const session = pl.create();

// Prolog program as string
const program = `
% ORGANON-FORMS Prolog Representation (extended for Hypothetical Chains)

% Form Definitions
form_def('f.identity.core', 'form-def', 'Form', [clause(require, ['identityKeys']), clause(incompatibleWith, ['anonymous'])], "Identity-bearing forms require stable keys.", _, _).
form_def('c.measurement.scope', 'context-def', 'Context', [clause(within, ['empirical-context']), clause(requiresEvidence, ['observation'])], "Claims are valid only under explicit evidential context.", _, _).
form_def('m.grounding.transition', 'morph-def', 'Morph', [clause(from, ['resolved']), clause(to, ['grounded']), clause(preserve, ['identityKeys'])], "Transition to grounded must preserve identity continuity.", _, _).

% Entity Definitions
entity('person', 'Person', ['id'], [], [], [], []).
entity('organization', 'Organization', ['orgId'], [], [], [], []).

% Property Definitions
property('name', 'c.measurement.scope', 'datatype', 'person', 'string', _, [1,1], true, _).
property('age', 'c.measurement.scope', 'datatype', 'person', 'integer', _, [0,1], true, _).

% Aspect Definitions
aspect('employs', 'employment', 'organization', 'person', 'employs', 'affirmed', []).

% Hypothetical Chain Rules
instantiates_form(EntityId, FormId) :-
    entity(EntityId, _, _, _, _, _, _),
    form_def(FormId, 'form-def', 'Form', Clauses, _, _, _),
    satisfies_clauses(EntityId, Clauses).

satisfies_clauses(_, []).
satisfies_clauses(EntityId, [clause(require, _)|Rest]) :-
    entity(EntityId, _, _, _, _, _, _),
    satisfies_clauses(EntityId, Rest).
satisfies_clauses(EntityId, [clause(prohibit, _)|Rest]) :-
    satisfies_clauses(EntityId, Rest).
satisfies_clauses(EntityId, [clause(incompatibleWith, _)|Rest]) :-
    satisfies_clauses(EntityId, Rest).

admissible_property(PropertyId, EntityId) :-
    property(PropertyId, ContextId, _, EntityId, _, _, _, _, _),
    form_def(ContextId, 'context-def', 'Context', Clauses, _, _, _),
    satisfies_clauses(EntityId, Clauses).

mediates_aspect(AspectId, MorphId) :-
    aspect(AspectId, _, SubjectId, ObjectId, _, _, _),
    form_def(MorphId, 'morph-def', 'Morph', Clauses, _, _, _),
    satisfies_morph_clauses(AspectId, Clauses).

satisfies_morph_clauses(AspectId, [clause(from, [State])|Rest]) :-
    State = 'resolved',
    satisfies_morph_clauses(AspectId, Rest).
satisfies_morph_clauses(AspectId, [clause(to, [State])|Rest]) :-
    State = 'grounded',
    satisfies_morph_clauses(AspectId, Rest).
satisfies_morph_clauses(AspectId, [clause(preserve, [Invariant])|Rest]) :-
    Invariant = 'identityKeys',
    satisfies_morph_clauses(AspectId, Rest).
satisfies_morph_clauses(_, []).

% Hegelian Dialectics
dialectical_sublate(Thesis, Antithesis, Synthesis) :-
    contradicts(Thesis, Antithesis),
    resolves_to(Thesis, Antithesis, Synthesis).

contradicts('anonymous', 'identityKeys').
resolves_to('anonymous', 'identityKeys', 'conditional_identity').

% Dependent Origination
originates_from(Effect, Conditions) :-
    dependent_on(Effect, Conditions),
    all_satisfied(Conditions).

dependent_on('grounded_aspect', ['resolved_entity', 'valid_property']).
all_satisfied([]).
all_satisfied([H|T]) :- satisfied(H), all_satisfied(T).

satisfied('resolved_entity') :- valid_entity(_).
satisfied('valid_property') :- valid_property(_).

% Claim Inference
infer_claim(EntityId, PropertyId, 'asserted') :-
    valid_entity(EntityId),
    admissible_property(PropertyId, EntityId).

infer_claim(EntityId, AspectId, 'inferred') :-
    valid_entity(EntityId),
    mediates_aspect(AspectId, _),
    originates_from('grounded_aspect', _).

valid_entity(EntityId) :-
    entity(EntityId, _, _, _, _, _, _),
    instantiates_form(EntityId, _).

valid_property(PropertyId) :-
    property(PropertyId, ContextId, _, _, _, _, _, _, _),
    form_def(ContextId, 'context-def', 'Context', _, _, _, _).
`;

// Load the program
session.consult(program, {
  success: () => {
    console.log('Extended program loaded successfully.');

    // Query 1: Infer claims for person
    session.query("findall(Claim, infer_claim('person', _, Claim), Claims).", {
      success: (goal) => {
        session.answer({
          success: (answer) => {
            console.log(
              'Inferred claims for person:',
              answer.lookup('Claims').toJavaScript(),
            );
          },
          fail: () => console.log('No more claims.'),
          limit: 1,
        });
      },
      error: (err) => console.error('Query error:', err),
    });

    // Query 2: Check dialectical sublation
    session.query(
      "dialectical_sublate('anonymous', 'identityKeys', Synthesis).",
      {
        success: (goal) => {
          session.answer({
            success: (answer) => {
              console.log(
                'Dialectical synthesis:',
                answer.lookup('Synthesis').toJavaScript(),
              );
            },
            fail: () => console.log('No synthesis found.'),
            limit: 1,
          });
        },
        error: (err) => console.error('Query error:', err),
      },
    );

    // Query 3: Dependent origination
    session.query("originates_from('grounded_aspect', Conditions).", {
      success: (goal) => {
        session.answer({
          success: (answer) => {
            console.log(
              'Origination conditions:',
              answer.lookup('Conditions').toJavaScript(),
            );
          },
          fail: () => console.log('No origination.'),
          limit: 1,
        });
      },
      error: (err) => console.error('Query error:', err),
    });
  },
  error: (err) => console.error('Consult error:', err),
});
