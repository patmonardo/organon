//! PGQL keywords and functions helper
//!
//! Mirrors the style of `polars-sql::keywords` to make learning both easier.

/// Get all PGQL keywords supported by the project.
pub fn all_keywords() -> Vec<&'static str> {
    // Source: a pragmatic PGQL-ish seed (add/remove as you see fit)
    let keywords: &[&str] = &[
        "MATCH", "SELECT", "FROM", "WHERE", "RETURN", "ORDER", "BY", "LIMIT", "GROUP", "HAVING",
        "DISTINCT", "AS", "JOIN", "UNWIND", "PATH", "VERTEX", "EDGE", "CREATE", "DELETE", "DROP",
        "INSERT", "UPDATE", "WITH", "USING", "OPTIONAL", "START", "END",
    ];
    keywords.to_vec()
}

/// Get a list of graph and aggregate functions that PGQL exposes (seed list).
pub fn all_functions() -> Vec<&'static str> {
    let functions: &[&str] = &[
        // graph-specific
        "SHORTEST_PATH",
        "SHORTEST_PATHS",
        "PATHS",
        "DEGREE",
        "IN_DEGREE",
        "OUT_DEGREE",
        "NEIGHBORS",
        "ADJACENT",
        "LABEL",
        "TYPE",
        "LENGTH",
        // aggregates / general
        "COUNT",
        "SUM",
        "AVG",
        "MIN",
        "MAX",
        "EXISTS",
        "STARTS_WITH",
        "ENDS_WITH",
    ];
    functions.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_keywords_present() {
        let kws = all_keywords();
        assert!(kws.iter().any(|k| *k == "MATCH"));
        assert!(kws.iter().any(|k| *k == "SELECT"));
        assert!(kws.iter().any(|k| *k == "RETURN"));
    }

    #[test]
    fn basic_functions_present() {
        let funcs = all_functions();
        assert!(funcs.iter().any(|f| *f == "SHORTEST_PATH"));
        assert!(funcs.iter().any(|f| *f == "DEGREE"));
        assert!(funcs.iter().any(|f| *f == "COUNT"));
    }
}
