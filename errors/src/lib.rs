pub mod db;
mod files;
pub mod pos;
mod reporter;
pub use crate::reporter::Reporter;
pub use codespan_reporting::{
    diagnostic::Diagnostic,
    files::Files,
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};

pub use db::{FileDatabase, FileDatabaseStorage, FileId};
pub type WithError<T> = Result<T, Vec<Diagnostic<FileId>>>;
pub use db::MockDatabaseImpl;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
