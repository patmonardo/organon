//! Canonical Projection Eval Workbench tracks.
//!
//! The catalog is the semantic index for projection/eval workbench slices:
//! stable ids, short focus notes, and exemplar entrypoints.

#[derive(Clone, Debug)]
pub struct ProjectionEvalWorkbenchTrack {
    pub id: &'static str,
    pub title: &'static str,
    pub focus: &'static str,
    pub exemplar: &'static str,
    pub status: &'static str,
}

const PROJECTION_EVAL_TRACKS: [ProjectionEvalWorkbenchTrack; 6] = [
    ProjectionEvalWorkbenchTrack {
        id: "pe-map-surfaces",
        title: "Map Surfaces",
        focus: "classify functions by pipeline kind, execution mode, and layer",
        exemplar: "examples/eval_map_surfaces.rs",
        status: "core",
    },
    ProjectionEvalWorkbenchTrack {
        id: "pe-train-template-law",
        title: "Train Template Law",
        focus: "internalize pipeline train template sequence and invariants",
        exemplar: "examples/eval_train_template_law.rs",
        status: "core",
    },
    ProjectionEvalWorkbenchTrack {
        id: "pe-predict-template-law",
        title: "Predict Template Law",
        focus: "internalize predict template sequence and cleanup guarantees",
        exemplar: "examples/eval_predict_template_law.rs",
        status: "core",
    },
    ProjectionEvalWorkbenchTrack {
        id: "pe-facade-boundary-discipline",
        title: "Facade Boundary Discipline",
        focus: "keep procedure orchestration thin and execution in projection eval",
        exemplar: "examples/eval_facade_boundary_discipline.rs",
        status: "covered",
    },
    ProjectionEvalWorkbenchTrack {
        id: "pe-catalog-symbolics",
        title: "Catalog Symbolics",
        focus: "pipeline identity, user scoping, and typed retrieval boundaries",
        exemplar: "examples/eval_catalog_symbolics.rs",
        status: "covered",
    },
    ProjectionEvalWorkbenchTrack {
        id: "pe-shell-taskframe-projection",
        title: "Shell TaskFrame Projection",
        focus: "project eval tracks into ProgramFeature and TaskFrame slices",
        exemplar: "examples/eval_shell_taskframe_projection.rs",
        status: "planned",
    },
];

pub fn projection_eval_workbench_tracks() -> &'static [ProjectionEvalWorkbenchTrack] {
    &PROJECTION_EVAL_TRACKS
}

pub fn projection_eval_workbench_track(id: &str) -> Option<&'static ProjectionEvalWorkbenchTrack> {
    PROJECTION_EVAL_TRACKS.iter().find(|track| track.id == id)
}

/// Render projection/eval workbench tracks as display lines.
pub fn projection_eval_workbench_catalog_lines() -> Vec<String> {
    let mut lines = vec!["Projection Eval Workbench Slice".to_string()];
    lines.extend(projection_eval_workbench_tracks().iter().map(|track| {
        format!(
            "- {} | {} | {} | {}",
            track.id, track.title, track.status, track.exemplar
        )
    }));
    lines
}

/// Render projection/eval workbench catalog as a text block.
pub fn projection_eval_workbench_catalog_text() -> String {
    projection_eval_workbench_catalog_lines().join("\n")
}

/// Convenience runner for quick projection/eval workbench inspection.
pub fn run_projection_eval_workbench_catalog() -> String {
    let text = projection_eval_workbench_catalog_text();
    println!("{}", text);
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_eval_catalog_includes_core_tracks() {
        let tracks = projection_eval_workbench_tracks();
        assert!(tracks.iter().any(|track| track.id == "pe-map-surfaces"));
        assert!(tracks
            .iter()
            .any(|track| track.id == "pe-train-template-law"));
    }

    #[test]
    fn projection_eval_catalog_lookup_resolves_track() {
        let track = projection_eval_workbench_track("pe-predict-template-law");
        assert!(track.is_some());

        let missing = projection_eval_workbench_track("missing");
        assert!(missing.is_none());
    }

    #[test]
    fn projection_eval_catalog_text_renders() {
        let text = projection_eval_workbench_catalog_text();
        assert!(text.contains("Projection Eval Workbench Slice"));
        assert!(text.contains("pe-catalog-symbolics"));
    }
}
