//! Canonical Task Workbench tracks.
//!
//! The catalog is the semantic index for the task workbench: stable ids,
//! human titles, short focus notes, and example entrypoints.
//! Execution happens from `gds/examples`, while this module defines the
//! TaskFrame slice of operations currently in scope.

#[derive(Clone, Debug)]
pub struct TaskWorkbenchTrack {
    pub id: &'static str,
    pub title: &'static str,
    pub focus: &'static str,
    pub exemplar: &'static str,
    pub status: &'static str,
}

const TASK_TRACKS: [TaskWorkbenchTrack; 4] = [
    TaskWorkbenchTrack {
        id: "taskframe-shell-frame-seed",
        title: "TaskFrame Shell Frame Seed",
        focus: "minimal shell frame seeded into task runtime",
        exemplar: "examples/taskframe_shell_frame_seed.rs",
        status: "core",
    },
    TaskWorkbenchTrack {
        id: "taskframe-shell-model-feature-plan",
        title: "TaskFrame Shell Model Feature Plan",
        focus: "mediated shell model-feature-plan path into task runtime",
        exemplar: "examples/taskframe_shell_model_feature_plan.rs",
        status: "core",
    },
    TaskWorkbenchTrack {
        id: "taskframe-catalog-slice",
        title: "TaskFrame Catalog Slice",
        focus: "catalog/mod shape defining the taskframe workbench slice",
        exemplar: "examples/taskframe_catalog_slice.rs",
        status: "covered",
    },
    TaskWorkbenchTrack {
        id: "taskframe-runtime-lifecycle",
        title: "TaskFrame Runtime Lifecycle",
        focus: "begin/progress/end failure boundaries for top-level runtime",
        exemplar: "examples/taskframe_runtime_lifecycle.rs",
        status: "covered",
    },
];

pub fn task_workbench_tracks() -> &'static [TaskWorkbenchTrack] {
    &TASK_TRACKS
}

pub fn task_workbench_track(id: &str) -> Option<&'static TaskWorkbenchTrack> {
    TASK_TRACKS.iter().find(|track| track.id == id)
}

/// Render the task workbench slice as human-readable lines.
pub fn task_workbench_catalog_lines() -> Vec<String> {
    let mut lines = vec!["Task Workbench Slice".to_string()];
    lines.extend(task_workbench_tracks().iter().map(|track| {
        format!(
            "- {} | {} | {} | {}",
            track.id, track.title, track.status, track.exemplar
        )
    }));
    lines
}

/// Render the task workbench slice as a single text block.
pub fn task_workbench_catalog_text() -> String {
    task_workbench_catalog_lines().join("\n")
}

/// Convenience runner for ad-hoc catalog inspection.
///
/// Returns the rendered catalog text and also prints it to stdout.
pub fn run_task_workbench_catalog() -> String {
    let text = task_workbench_catalog_text();
    println!("{}", text);
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_core_tracks() {
        let tracks = task_workbench_tracks();
        assert!(tracks
            .iter()
            .any(|track| track.id == "taskframe-shell-frame-seed"));
        assert!(tracks
            .iter()
            .any(|track| track.id == "taskframe-shell-model-feature-plan"));
    }

    #[test]
    fn lookup_by_id_resolves_track() {
        let track = task_workbench_track("taskframe-runtime-lifecycle");
        assert!(track.is_some());

        let missing = task_workbench_track("missing");
        assert!(missing.is_none());
    }

    #[test]
    fn catalog_text_contains_track_ids() {
        let text = task_workbench_catalog_text();
        assert!(text.contains("Task Workbench Slice"));
        assert!(text.contains("taskframe-shell-frame-seed"));
    }
}
