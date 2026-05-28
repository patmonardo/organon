//! Dataset stdlib utilities (dataset resources, download, cache).

pub mod corpus;
pub mod corpus_util;
pub mod resources;

pub use corpus::{
    BracketedCorpusReader, CorpusFiles, CorpusReader, CorpusResource, PlaintextCorpusReader,
    WordListCorpusReader, XmlCorpusReader,
};
pub use corpus_util::{
    concat as corpus_concat, read_blankline_block, read_line_block, read_regexp_block,
    read_sexpr_block, read_whitespace_block, read_wordpunct_block, ConcatenatedCorpusView,
    StreamBackedCorpusView,
};
pub use resources::{
    catalog_resource_tables, data_home, data_home_with, fetch_resource, list_resources,
    normalize_resource_name, normalize_resource_url, resource_dir, split_resource_url,
    DatasetResource, DatasetResourceReport,
};
