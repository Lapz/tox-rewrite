use reporting::files;
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{self, Read},
    ops::Range,
    path::PathBuf,
    sync::Arc,
};
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FileId(salsa::InternId);

impl salsa::InternKey for FileId {
    fn from_intern_id(v: salsa::InternId) -> Self {
        FileId(v)
    }
    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

#[salsa::query_group(FileDatabaseStorage)]
pub trait FileDatabase {
    #[salsa::interned]
    fn intern_file(&self, path: PathBuf) -> FileId;

    fn source(&self, file: FileId) -> Arc<String>;
    fn name(&self, file: FileId) -> Arc<String>;
    fn line_index(&self, file: FileId, byte_index: usize) -> Option<usize>;
    fn line_range(&self, file: FileId, line_index: usize) -> Option<Range<usize>>;
}
pub trait FilesExt: salsa::Database {
    fn source(&self, file: FileId) -> &Arc<str>;
    fn path(&self, file: FileId) -> &OsStr;
}

fn source(db: &impl FileDatabase, file_id: FileId) -> Arc<String> {
    let contents = read_file(&db.lookup_intern_file(file_id))
        .expect("Couldn't read file. TODO handle deletion of file");
    Arc::new(contents)
}

fn name(db: &impl FileDatabase, file_id: FileId) -> Arc<String> {
    Arc::new(String::from(
        db.lookup_intern_file(file_id).to_str().unwrap(),
    ))
}

fn line_index(db: &impl FileDatabase, file_id: FileId, byte_index: usize) -> Option<usize> {
    unimplemented!()
}

fn line_range(db: &impl FileDatabase, file_id: FileId, line_index: usize) -> Option<Range<usize>> {
    unimplemented!()
}

fn read_file(name: &PathBuf) -> io::Result<String> {
    let mut file = File::open(name)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
