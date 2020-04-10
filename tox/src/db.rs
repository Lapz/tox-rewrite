use errors::{emit, ColorChoice, Config, Diagnostic, FileDatabase, FileId, StandardStream};
use parser::FilesExt;
use reporting::files;
use std::default::Default;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::{ops::Range, sync::Arc};
#[salsa::database(
    semant::HirDatabaseStorage,
    semant::InternDatabaseStorage,
    parser::ParseDatabaseStorage,
    errors::FileDatabaseStorage
)]
#[derive(Debug, Default)]
pub struct DatabaseImpl {
    runtime: salsa::Runtime<DatabaseImpl>,
    files: errors::Files<Arc<str>>,
    diagnostics: Vec<Diagnostic<FileId>>,
}

pub(crate) trait Diagnostics {
    fn emit(&self, diagnostics: &mut Vec<Diagnostic<FileId>>) -> io::Result<()>;
}

impl Diagnostics for DatabaseImpl {
    fn emit(&self, diagnostics: &mut Vec<Diagnostic<FileId>>) -> io::Result<()> {
        let writer = StandardStream::stderr(ColorChoice::Auto);
        let mut writer = writer.lock();
        let config = Config::default();

        while let Some(diagnostic) = diagnostics.pop() {
            emit(&mut writer, &config, &self.files, &diagnostic)?
        }

        Ok(())
    }
}

impl FilesExt for DatabaseImpl {
    fn source(&self, file: FileId) -> &Arc<str> {
        self.files.source(file)
    }

    fn path(&self, file: FileId) -> &OsStr {
        self.files.name(file)
    }

    fn load_file(&mut self, path: &PathBuf) -> FileId {
        let source = read_file(path).expect("Couldn't read a file");
        self.files.add(path, source.into())
    }
}

impl salsa::Database for DatabaseImpl {
    fn salsa_runtime(&self) -> &salsa::Runtime<DatabaseImpl> {
        &self.runtime
    }

    fn salsa_runtime_mut(&mut self) -> &mut salsa::Runtime<DatabaseImpl> {
        &mut self.runtime
    }
}

fn read_file(name: &PathBuf) -> io::Result<String> {
    let mut file = File::open(name)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

impl<'files> files::Files<'files> for DatabaseImpl {
    type FileId = errors::db::FileId;
    type Name = &'files String;
    type Source = &'files String;

    fn name(&self, file_id: errors::db::FileId) -> Option<&String> {
        Some(&errors::db::FileDatabase::name(self, file_id))
    }

    fn source(&self, file_id: errors::db::FileId) -> Option<&String> {
        Some(&errors::db::FileDatabase::source(self, file_id))
    }

    fn line_index(&self, file_id: errors::db::FileId, byte_index: usize) -> Option<usize> {
        errors::db::FileDatabase::line_index(self, file_id, byte_index)
    }

    fn line_range(&self, file_id: errors::db::FileId, line_index: usize) -> Option<Range<usize>> {
        errors::db::FileDatabase::line_range(self, file_id, line_index)
    }
}
