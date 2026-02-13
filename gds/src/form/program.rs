//! Minimal Form program types.
//!
//! This module exists to carry a small, stable “program shape” into the Form evaluator.
//! At runtime, the only required field today is `Morph.patterns`, which drives the
//! `FormOperator` chain.
//!
//! Dialectical convention (semantic mapping; not enforced by the kernel):
//! - `Shape`   → Essence (the program's essential envelope / what is “there” as form)
//! - `Context` → Determination of Essence / Reflection (how the form is determined)
//! - `Morph`   → Ground as **Active Ground** (the operator chain that actualizes the determination)

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

/// Form program payload passed into the Form evaluator.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FormShape {
    pub shape: Shape,
    pub context: Context,
    pub morph: Morph,
}

impl FormShape {
    pub fn new(shape: Shape, context: Context, morph: Morph) -> Self {
        Self {
            shape,
            context,
            morph,
        }
    }
}

/// Optional structural metadata about the program.
///
/// Semantic convention: this is the program's *Essence* envelope.
///
/// Currently unused by the evaluator, but retained as a small, non-opinionated envelope
/// for schema-first expansion.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Shape {
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub type_constraints: HashMap<String, String>,
    pub validation_rules: HashMap<String, String>,
}

impl Shape {
    pub fn new(
        required_fields: Vec<String>,
        optional_fields: Vec<String>,
        type_constraints: HashMap<String, String>,
        validation_rules: HashMap<String, String>,
    ) -> Self {
        Self {
            required_fields,
            optional_fields,
            type_constraints,
            validation_rules,
        }
    }
}

/// Execution context metadata.
///
/// Semantic convention: this carries *Determination of Essence / Reflection* inputs.
///
/// Currently unused by the evaluator; kept lightweight.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Context {
    pub dependencies: Vec<String>,
    pub execution_order: Vec<String>,
    pub runtime_strategy: String,
    pub conditions: Vec<String>,
}

impl Context {
    pub fn new(
        dependencies: Vec<String>,
        execution_order: Vec<String>,
        runtime_strategy: String,
        conditions: Vec<String>,
    ) -> Self {
        Self {
            dependencies,
            execution_order,
            runtime_strategy,
            conditions,
        }
    }
}

/// The only part of the program required by the current Form ISA.
///
/// Semantic convention: `Morph` is **Active Ground** — the effective ground that
/// realizes the program through the selected operator chain.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Morph {
    /// Names of Form operators to run, in order.
    pub patterns: Vec<String>,
}

impl Morph {
    pub fn new(patterns: Vec<String>) -> Self {
        Self { patterns }
    }
}

/// Domain specification envelope used for GDSL/SDSL representations in-kernel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Specification {
    pub name: String,
    pub version: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl Specification {
    pub fn new(name: String, version: Option<String>, attributes: HashMap<String, String>) -> Self {
        Self {
            name,
            version,
            attributes,
        }
    }
}

/// A named application-level Form module.
///
/// Each form contributes feature descriptors and one or more operator patterns
/// that can be compiled into `Morph.patterns` for execution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApplicationForm {
    pub name: String,
    pub domain: String,
    pub features: Vec<String>,
    pub patterns: Vec<String>,
    pub specifications: HashMap<String, String>,
}

impl ApplicationForm {
    pub fn new(
        name: String,
        domain: String,
        features: Vec<String>,
        patterns: Vec<String>,
        specifications: HashMap<String, String>,
    ) -> Self {
        Self {
            name,
            domain,
            features,
            patterns,
            specifications,
        }
    }
}

/// Program-level FormDB projection used by the Rust kernel.
///
/// `form` is the stable evaluator payload.
/// `gdsl` and `sdsl` carry canonical/specification metadata.
/// `application_forms` and `selected_forms` model the feature suite projected into execution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramSpec {
    pub form: FormShape,
    pub gdsl: Specification,
    pub sdsl: Vec<Specification>,
    pub application_forms: Vec<ApplicationForm>,
    pub selected_forms: Vec<String>,
}

impl ProgramSpec {
    pub fn new(
        form: FormShape,
        gdsl: Specification,
        sdsl: Vec<Specification>,
        application_forms: Vec<ApplicationForm>,
        selected_forms: Vec<String>,
    ) -> Self {
        Self {
            form,
            gdsl,
            sdsl,
            application_forms,
            selected_forms,
        }
    }

    /// Compiles a deterministic execution plan by resolving selected application forms
    /// and appending their patterns after the base `form.morph.patterns` chain.
    pub fn compile_execution_plan(&self) -> Result<ProgramExecutionPlan, ProgramSpecError> {
        let registry = ApplicationFormRegistry::new(self.application_forms.clone())?;
        registry.compile(&self.form.morph, &self.selected_forms)
    }
}

impl From<FormShape> for ProgramSpec {
    fn from(form: FormShape) -> Self {
        Self {
            form,
            gdsl: Specification::default(),
            sdsl: Vec::new(),
            application_forms: Vec::new(),
            selected_forms: Vec::new(),
        }
    }
}

impl From<ProgramSpec> for FormShape {
    fn from(program: ProgramSpec) -> Self {
        program.form
    }
}

/// Compiled runtime plan produced from a `ProgramSpec`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramExecutionPlan {
    pub selected_forms: Vec<String>,
    pub patterns: Vec<String>,
}

impl ProgramExecutionPlan {
    pub fn new(selected_forms: Vec<String>, patterns: Vec<String>) -> Self {
        Self {
            selected_forms,
            patterns,
        }
    }
}

/// Registry/executor for application forms.
#[derive(Debug, Clone, Default)]
pub struct ApplicationFormRegistry {
    forms: HashMap<String, ApplicationForm>,
    ordered_names: Vec<String>,
}

impl ApplicationFormRegistry {
    pub fn new(forms: Vec<ApplicationForm>) -> Result<Self, ProgramSpecError> {
        let mut registry = HashMap::with_capacity(forms.len());
        let mut ordered_names = Vec::with_capacity(forms.len());

        for form in forms {
            if registry.contains_key(&form.name) {
                return Err(ProgramSpecError::DuplicateApplicationForm(form.name));
            }
            ordered_names.push(form.name.clone());
            registry.insert(form.name.clone(), form);
        }

        Ok(Self {
            forms: registry,
            ordered_names,
        })
    }

    pub fn compile(
        &self,
        base_morph: &Morph,
        selected_forms: &[String],
    ) -> Result<ProgramExecutionPlan, ProgramSpecError> {
        let requested = if selected_forms.is_empty() {
            self.ordered_names.clone()
        } else {
            selected_forms.to_vec()
        };

        let mut seen = HashSet::with_capacity(requested.len());
        let mut selected = Vec::with_capacity(requested.len());
        let mut patterns = base_morph.patterns.clone();

        for form_name in requested {
            if !seen.insert(form_name.clone()) {
                return Err(ProgramSpecError::DuplicateFormSelection(form_name));
            }

            let form = self
                .forms
                .get(&form_name)
                .ok_or_else(|| ProgramSpecError::UnknownApplicationForm(form_name.clone()))?;

            selected.push(form_name);
            patterns.extend(form.patterns.iter().cloned());
        }

        Ok(ProgramExecutionPlan::new(selected, patterns))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramSpecError {
    DuplicateApplicationForm(String),
    UnknownApplicationForm(String),
    DuplicateFormSelection(String),
}

impl fmt::Display for ProgramSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateApplicationForm(name) => {
                write!(f, "duplicate application form registered: {name}")
            }
            Self::UnknownApplicationForm(name) => {
                write!(f, "unknown application form requested: {name}")
            }
            Self::DuplicateFormSelection(name) => {
                write!(f, "duplicate application form selected: {name}")
            }
        }
    }
}

impl Error for ProgramSpecError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_shape() -> FormShape {
        FormShape::new(
            Shape::default(),
            Context::new(vec![], vec![], "kernel".to_string(), vec![]),
            Morph::new(vec!["base.normalize".to_string()]),
        )
    }

    fn app_form(name: &str, pattern: &str) -> ApplicationForm {
        ApplicationForm::new(
            name.to_string(),
            "graph-ml".to_string(),
            vec![format!("feature.{name}")],
            vec![pattern.to_string()],
            HashMap::new(),
        )
    }

    #[test]
    fn compiles_selected_forms_in_order() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::default(),
            vec![],
            vec![
                app_form("centrality", "algo.pagerank"),
                app_form("community", "algo.leiden"),
            ],
            vec!["community".to_string(), "centrality".to_string()],
        );

        let plan = program
            .compile_execution_plan()
            .expect("plan should compile");

        assert_eq!(plan.selected_forms, vec!["community", "centrality"]);
        assert_eq!(
            plan.patterns,
            vec!["base.normalize", "algo.leiden", "algo.pagerank"]
        );
    }

    #[test]
    fn compiles_all_forms_when_selection_empty() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::default(),
            vec![],
            vec![app_form("a", "op.a"), app_form("b", "op.b")],
            vec![],
        );

        let plan = program
            .compile_execution_plan()
            .expect("plan should compile");
        assert_eq!(plan.selected_forms, vec!["a", "b"]);
        assert_eq!(plan.patterns, vec!["base.normalize", "op.a", "op.b"]);
    }

    #[test]
    fn errors_on_unknown_selected_form() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::default(),
            vec![],
            vec![app_form("a", "op.a")],
            vec!["missing".to_string()],
        );

        let error = program
            .compile_execution_plan()
            .expect_err("missing form should fail");
        assert_eq!(
            error,
            ProgramSpecError::UnknownApplicationForm("missing".to_string())
        );
    }
}
