//! Canonical DataFrame Workbench tracks.
//!
//! The catalog is the semantic index for the workbench: stable ids, human
//! titles, short focus notes, and example entrypoints that stay readable while
//! the workbench provides the operator menu.

#[derive(Clone, Debug)]
pub struct DataFrameWorkbenchTrack {
    pub id: &'static str,
    pub title: &'static str,
    pub focus: &'static str,
    pub exemplar: &'static str,
    pub status: &'static str,
}

const DATAFRAME_TRACKS: [DataFrameWorkbenchTrack; 6] = [
    DataFrameWorkbenchTrack {
        id: "df-select-filter",
        title: "Select and Filter",
        focus: "selector grammar and predicate membership",
        exemplar: "examples/dataframe_select_filter.rs",
        status: "covered",
    },
    DataFrameWorkbenchTrack {
        id: "df-order-group",
        title: "Order and Group",
        focus: "sorting contracts and grouped aggregations",
        exemplar: "examples/dataframe_order_group.rs",
        status: "covered",
    },
    DataFrameWorkbenchTrack {
        id: "df-joins",
        title: "Join Operations",
        focus: "join semantics over aligned and divergent key spaces",
        exemplar: "examples/dataframe_join_operations.rs",
        status: "covered",
    },
    DataFrameWorkbenchTrack {
        id: "df-transformations",
        title: "Transformations",
        focus: "reshape, null/NaN repair, ranking, sampling, and distincts",
        exemplar: "examples/dataframe_transformations.rs",
        status: "covered",
    },
    DataFrameWorkbenchTrack {
        id: "df-ext-namespace",
        title: "Extension Namespace",
        focus: "registry semantics and ext namespace seeds",
        exemplar: "examples/dataframe_namespace_ext.rs",
        status: "covered",
    },
    DataFrameWorkbenchTrack {
        id: "df-pivot-modern",
        title: "Pivot Modernization",
        focus: "current pivot boundary and modernization target",
        exemplar: "examples/dataframe_pivot_unpivot_operations.rs",
        status: "boundary-tracked",
    },
];

pub fn dataframe_workbench_tracks() -> &'static [DataFrameWorkbenchTrack] {
    &DATAFRAME_TRACKS
}

pub fn dataframe_workbench_track(id: &str) -> Option<&'static DataFrameWorkbenchTrack> {
    DATAFRAME_TRACKS.iter().find(|track| track.id == id)
}
