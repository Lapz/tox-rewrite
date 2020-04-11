use crate::AstNode;

#[cfg(test)]
use syntax::ast::SourceFile;

pub fn dump_debug<T: AstNode>(item: &T) -> String {
    format!("{:#?}", item.syntax())
}

#[cfg(test)]
pub fn parse<'a>(input: &'a str) -> SourceFile {
    use std::io::Write;
    use tempfile::NamedTempFile;

    let file = NamedTempFile::new().unwrap();
    write!(file, "{}", input).unwrap();
    let db = MockDatabaseImpl::default();
    let handle = db.intern_file(file.path());

    db.parse(handle).unwrap()
}
