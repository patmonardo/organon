//! Dataset operational utilities (I/O helpers).

pub mod download;
pub mod extract;

pub use download::{
    copy_local, download_if_missing, download_to_dir, download_url, stream_to_writer,
    DownloadReport,
};
pub use extract::{extract_archive, ExtractReport};
