// mod ctx;
mod db;
mod hir;
// mod infer;
mod lower;
#[macro_use]
mod resolver;
// mod ty;

#[macro_use]
#[cfg(test)]
mod tests;
// mod util;

pub use db::{HirDatabase, HirDatabaseStorage, InternDatabaseStorage};

#[macro_export]
macro_rules! create_test {
    ($filename:ident) => {
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

            println!("{:?}", file_names);
            let db = crate::tests::MockDatabaseImpl::default();

            let handle = db.intern_file(file_names.remove(0));

            match db.resolve_source_file(handle) {
                Ok(_) => {}
                Err(errors) => println!("{:?}", errors),
            }

            assert!(db.resolve_source_file(handle).is_ok());
            Ok(())
        }
    };
}
