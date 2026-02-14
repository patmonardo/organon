mod common;
mod grammar;
mod io;
mod nonprojective;
mod projective;

pub use grammar::DependencyGrammar;
pub use io::{parse_malt_tab, to_dot};
pub use nonprojective::NonprojectiveDependencyParser;
pub use projective::ProjectiveDependencyParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_malt_tab_builds_graph() {
        let input = "John N 2 nsubj\nloves V 0 ROOT\nMary N 2 obj";
        let graph = parse_malt_tab(input).expect("should parse malt tab");
        assert_eq!(graph.nodes().len(), 3);
        assert_eq!(graph.edges().len(), 2);
        assert_eq!(graph.root(), Some(1));
    }

    #[test]
    fn dot_render_contains_root() {
        let input = "John N 2 nsubj\nloves V 0 ROOT\nMary N 2 obj";
        let graph = parse_malt_tab(input).expect("should parse malt tab");
        let dot = to_dot(&graph);
        assert!(dot.contains("0 -> 2 [label=\"ROOT\"]"));
    }

    #[test]
    fn projective_dependency_parser_produces_parse() {
        let grammar = DependencyGrammar::from_string(
            "
            'scratch' -> 'cats' | 'walls'
            'walls' -> 'the'
            'cats' -> 'the'
            ",
        );
        let parser = ProjectiveDependencyParser::new(grammar);
        let parses = parser.parse(&["the", "cats", "scratch", "the", "walls"]);
        assert!(!parses.is_empty());
        assert!(parses.iter().any(|graph| graph.root().is_some()));
    }

    #[test]
    fn nonprojective_dependency_parser_produces_parse() {
        let grammar = DependencyGrammar::from_string(
            "
            'saw' -> 'man' | 'dog'
            'man' -> 'the'
            'dog' -> 'the'
            ",
        );
        let parser = NonprojectiveDependencyParser::new(grammar);
        let parses = parser.parse(&["the", "man", "saw", "the", "dog"]);
        assert!(!parses.is_empty());
    }
}
