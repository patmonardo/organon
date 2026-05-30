//! Canonical Dataset Workbench tracks.
//!
//! The catalog is the semantic index for the workbench: stable ids, human
//! titles, short focus notes, and example entrypoints that stay readable while
//! the workbench provides the operator menu.

#[derive(Clone, Debug)]
pub struct DatasetWorkbenchTrack {
    pub id: &'static str,
    pub title: &'static str,
    pub focus: &'static str,
    pub exemplar: &'static str,
    pub status: &'static str,
}

const DATASET_TRACKS: [DatasetWorkbenchTrack; 9] = [
    DatasetWorkbenchTrack {
        id: "ds-create-sem-dataset",
        title: "Create Semantic Dataset",
        focus: "initialize dataset folder layout, semantic frame, and catalog frame",
        exemplar: "examples/dataset_create_sem_dataset.rs",
        status: "core",
    },
    DatasetWorkbenchTrack {
        id: "ds-download-unzip-archive",
        title: "Download and Unzip Archive",
        focus: "stage archive bytes, cache source, and extract into raw storage",
        exemplar: "examples/dataset_io_download_archive.rs",
        status: "core",
    },
    DatasetWorkbenchTrack {
        id: "ds-frame-dsl",
        title: "Frame DSL",
        focus: "dataset-facing frame shell and table authoring",
        exemplar: "examples/dataset_frame_dsl.rs",
        status: "covered",
    },
    DatasetWorkbenchTrack {
        id: "ds-source-corpus",
        title: "Source and Corpus",
        focus: "resource loading and corpus normalization",
        exemplar: "examples/dataset_source_corpus.rs",
        status: "covered",
    },
    DatasetWorkbenchTrack {
        id: "ds-tree-feature",
        title: "Tree and Feature Structures",
        focus: "tree structures and feature unification workflows",
        exemplar: "examples/dataset_tree_structures.rs",
        status: "covered",
    },
    DatasetWorkbenchTrack {
        id: "ds-model-plan",
        title: "Model Feature Plan",
        focus: "middle-fold modeling and plan semantics",
        exemplar: "examples/dataset_plan_moment.rs",
        status: "covered",
    },
    DatasetWorkbenchTrack {
        id: "ds-compile-ir",
        title: "Compile IR",
        focus: "lowering and compilation artifacts",
        exemplar: "examples/dataset_compile_ir.rs",
        status: "covered",
    },
    DatasetWorkbenchTrack {
        id: "ds-semantic-meta",
        title: "Semantic Meta Pipeline",
        focus: "semantic projection and metadata loops",
        exemplar: "examples/dataset_sem_meta_pipeline.rs",
        status: "covered",
    },
    DatasetWorkbenchTrack {
        id: "ds-streaming",
        title: "Streaming Surfaces",
        focus: "procedure and lazy streaming paths",
        exemplar: "examples/dataset_streaming_lazy.rs",
        status: "covered",
    },
];

pub fn dataset_workbench_tracks() -> &'static [DatasetWorkbenchTrack] {
    &DATASET_TRACKS
}

pub fn dataset_workbench_track(id: &str) -> Option<&'static DatasetWorkbenchTrack> {
    DATASET_TRACKS.iter().find(|track| track.id == id)
}
