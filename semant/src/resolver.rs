mod module;
mod source_file;

pub(crate) use module::resolve_modules_query;
pub(crate) use source_file::resolve_exports_query;
pub(crate) use source_file::resolve_source_file_query;
pub(crate) use source_file::FileTable;
