use super::types::{ChunkTree, TaggedToken};
use super::util::ChunkScore;

pub trait ChunkParser {
    fn parse(&self, tokens: &[TaggedToken]) -> ChunkTree;

    fn evaluate(&self, gold: &[ChunkTree]) -> ChunkScore {
        self.accuracy(gold)
    }

    fn accuracy(&self, gold: &[ChunkTree]) -> ChunkScore {
        let mut chunkscore = ChunkScore::new(".*");
        for correct in gold {
            chunkscore.score(correct, &self.parse(&correct.leaves()));
        }
        chunkscore
    }
}
