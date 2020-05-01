// mod ctx;
mod db;
mod hir;
mod infer;
mod lower;
mod util;
#[macro_use]
mod resolver;
// mod ty;

#[macro_use]
#[cfg(test)]
mod tests;
// mod util;

pub use db::{HirDatabase, HirDatabaseStorage, InternDatabaseStorage, TypeCtx};
pub use infer::Ctx;
pub use syntax::TextRange;

#[macro_export]
macro_rules! create_test {
    ($filename:ident ,is_err) => {
        $crate::__create_test!($filename, is_err);
    };
    ($filename:ident ) => {
        $crate::__create_test!($filename, is_ok);
    };
}

#[macro_export]
macro_rules! __create_test {
    ($filename:ident,$kind:ident) => {
        #[test]
        fn $filename() -> std::io::Result<()> {
            use crate::HirDatabase;
            use errors::db::FileDatabase;

            let dir = tempfile::tempdir()?;

            let structure = crate::tests::load_file(&format!(
                "{}/src/tests/{}.ron",
                env!("CARGO_MANIFEST_DIR"),
                stringify!($filename)
            ));

            let mut file_names = Vec::new();

            crate::tests::create_structure(&dir.path(), &structure, &mut file_names)?;

            let db = crate::tests::MockDatabaseImpl::default();

            let handle = db.intern_file(file_names.remove(0));

            match db.resolve_source_file(handle) {
                Ok(_) => {}
                Err(errors) => println!("{:?}", errors),
            }

            assert!(db.resolve_source_file(handle).$kind());
            Ok(())
        }
    };
}
