//! Canonical Shell Workbench tracks.
//!
//! The catalog is the semantic index for the shell workbench: stable ids,
//! human titles, short focus notes, and example entrypoints.

#[derive(Clone, Debug)]
pub struct ShellWorkbenchTrack {
    pub id: &'static str,
    pub title: &'static str,
    pub focus: &'static str,
    pub exemplar: &'static str,
    pub status: &'static str,
}

const SHELL_TRACKS: [ShellWorkbenchTrack; 5] = [
    ShellWorkbenchTrack {
        id: "shell-model-first",
        title: "Shell Model First",
        focus: "Model moment as naming entry into Model:Feature::Plan",
        exemplar: "examples/shell_model_first.rs",
        status: "covered",
    },
    ShellWorkbenchTrack {
        id: "shell-feature-first",
        title: "Shell Feature First",
        focus: "Feature companion moment and FeatStruct operator boundary",
        exemplar: "examples/shell_feature_first.rs",
        status: "covered",
    },
    ShellWorkbenchTrack {
        id: "shell-plan-first",
        title: "Shell Plan First",
        focus: "Plan ordering moment and runtime readiness surfaces",
        exemplar: "examples/shell_plan_first.rs",
        status: "covered",
    },
    ShellWorkbenchTrack {
        id: "shell-model-genesis",
        title: "Shell Model Genesis",
        focus: "three-box model genesis arc under ProgramFeature law",
        exemplar: "examples/shell_model_genesis.rs",
        status: "covered",
    },
    ShellWorkbenchTrack {
        id: "shell-trilogy",
        title: "Shell Trilogy",
        focus: "single-shell narrative for Model:Feature::Plan unity",
        exemplar: "examples/shell_trilogy.rs",
        status: "covered",
    },
];

pub fn shell_workbench_tracks() -> &'static [ShellWorkbenchTrack] {
    &SHELL_TRACKS
}

pub fn shell_workbench_track(id: &str) -> Option<&'static ShellWorkbenchTrack> {
    SHELL_TRACKS.iter().find(|track| track.id == id)
}

/// Render shell workbench tracks as display lines.
pub fn shell_workbench_catalog_lines() -> Vec<String> {
    let mut lines = vec!["Shell Workbench Slice".to_string()];
    lines.extend(shell_workbench_tracks().iter().map(|track| {
        format!(
            "- {} | {} | {} | {}",
            track.id, track.title, track.status, track.exemplar
        )
    }));
    lines
}

/// Render shell workbench catalog as a text block.
pub fn shell_workbench_catalog_text() -> String {
    shell_workbench_catalog_lines().join("\n")
}

/// Convenience runner for quick shell workbench inspection.
pub fn run_shell_workbench_catalog() -> String {
    let text = shell_workbench_catalog_text();
    println!("{}", text);
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_catalog_includes_core_tracks() {
        let tracks = shell_workbench_tracks();
        assert!(tracks.iter().any(|track| track.id == "shell-model-first"));
        assert!(tracks.iter().any(|track| track.id == "shell-plan-first"));
    }

    #[test]
    fn shell_catalog_lookup_resolves_track() {
        let track = shell_workbench_track("shell-trilogy");
        assert!(track.is_some());

        let missing = shell_workbench_track("missing");
        assert!(missing.is_none());
    }

    #[test]
    fn shell_catalog_text_renders() {
        let text = shell_workbench_catalog_text();
        assert!(text.contains("Shell Workbench Slice"));
        assert!(text.contains("shell-model-genesis"));
    }
}
