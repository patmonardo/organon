//! Procedure registry for node property step execution.
//!
//! Provides a lightweight mapping from normalized procedure names
//! (e.g. `gds.pagerank.mutate`) to executable handlers. This replaces
//! Java's mutate-mode callable/stub machinery with a Rust algorithm-spec dispatch table.

use crate::projection::eval::pipeline::node_property_step::{
    DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE, DEGREE_CENTRALITY_MUTATE, FASTRP_MUTATE, HARMONIC_MUTATE,
    HITS_MUTATE, PAGERANK_MUTATE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcedureKind {
    DebugWriteConstantDouble,
    PageRankMutate,
    FastRPMutate,
    DegreeCentralityMutate,
    HitsMutate,
    HarmonicMutate,
}

pub struct ProcedureRegistry;

impl ProcedureRegistry {
    pub fn resolve(name: &str) -> Option<ProcedureKind> {
        match Self::canonical_mutate_name(name).as_str() {
            "gds.debug.writeconstantdouble.mutate" => Some(ProcedureKind::DebugWriteConstantDouble),
            "gds.pagerank.mutate" => Some(ProcedureKind::PageRankMutate),
            "gds.fastrp.mutate" => Some(ProcedureKind::FastRPMutate),
            "gds.degree.mutate" | "gds.degree_centrality.mutate" => {
                Some(ProcedureKind::DegreeCentralityMutate)
            }
            "gds.hits.mutate" => Some(ProcedureKind::HitsMutate),
            "gds.closeness.harmonic.mutate" | "gds.harmonic.mutate" => {
                Some(ProcedureKind::HarmonicMutate)
            }
            _ => None,
        }
    }

    pub fn supported_names() -> &'static [&'static str] {
        &[
            DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE,
            PAGERANK_MUTATE,
            FASTRP_MUTATE,
            DEGREE_CENTRALITY_MUTATE,
            HITS_MUTATE,
            HARMONIC_MUTATE,
        ]
    }

    /// Canonicalize a procedure name into mutate-mode GDS form.
    ///
    /// This accepts user-facing aliases such as `pageRank`, `GDS.FASTRP`,
    /// `gds.beta.closeness.harmonic`, and full mutate names. The result is the
    /// name stored in pipeline IR and later resolved by this registry.
    pub fn canonical_mutate_name(raw: &str) -> String {
        let mut name = raw.trim().to_ascii_lowercase();

        if !name.starts_with("gds.") {
            name = format!("gds.{name}");
        }

        name = name.replacen("gds.beta.", "gds.", 1);
        name = name.replacen("gds.alpha.", "gds.", 1);

        if !name.ends_with(".mutate") {
            name = format!("{name}.mutate");
        }

        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_supported() {
        assert_eq!(
            ProcedureRegistry::resolve(DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE),
            Some(ProcedureKind::DebugWriteConstantDouble)
        );
        assert_eq!(
            ProcedureRegistry::resolve(PAGERANK_MUTATE),
            Some(ProcedureKind::PageRankMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve(FASTRP_MUTATE),
            Some(ProcedureKind::FastRPMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve(DEGREE_CENTRALITY_MUTATE),
            Some(ProcedureKind::DegreeCentralityMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve(HITS_MUTATE),
            Some(ProcedureKind::HitsMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve(HARMONIC_MUTATE),
            Some(ProcedureKind::HarmonicMutate)
        );
    }

    #[test]
    fn test_resolve_unknown() {
        assert_eq!(ProcedureRegistry::resolve("gds.unknown.mutate"), None);
    }

    #[test]
    fn test_resolve_canonical_forms() {
        assert_eq!(
            ProcedureRegistry::resolve("pageRank"),
            Some(ProcedureKind::PageRankMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve("GDS.FASTRP"),
            Some(ProcedureKind::FastRPMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve("gds.degree"),
            Some(ProcedureKind::DegreeCentralityMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve("gds.beta.closeness.harmonic"),
            Some(ProcedureKind::HarmonicMutate)
        );
        assert_eq!(
            ProcedureRegistry::resolve("gds.alpha.debug.writeConstantDouble.mutate"),
            Some(ProcedureKind::DebugWriteConstantDouble)
        );
    }

    #[test]
    fn test_canonical_mutate_name_strips_tier_prefixes() {
        assert_eq!(
            ProcedureRegistry::canonical_mutate_name("gds.beta.pageRank"),
            "gds.pagerank.mutate"
        );
        assert_eq!(
            ProcedureRegistry::canonical_mutate_name("alpha.fastRP.mutate"),
            "gds.fastrp.mutate"
        );
    }
}
