use crate::hir::{Module, ModuleId};
use crate::HirDatabase;
use errors::{FileId, Reporter, WithError};
use std::path::PathBuf;

/// Resolves all modules
/// Our module structure is as follows
///  a module can be declared in the same file
/// i.e
/// -- main.tox --
/// mod foo;
///  |- main.tox
///  |-foo.tox
/// If the second form is
/// |-foo
/// | |-bar.tox

pub fn resolve_modules_query(
    db: &impl HirDatabase,
    file: FileId,
    mod_id: ModuleId,
) -> WithError<()> {
    let mut reporter = Reporter::new(file);
    let module = db.lower_module(file, mod_id);

    let name = db.lookup_intern_name(module.name);

    let span = module.span;

    let mut path_buf = PathBuf::from(db.path(module.file));
    path_buf.pop();

    let mut dir = path_buf.clone();

    dir.push(format!("{}", name));

    path_buf.push(format!("{}.tox", name));

    let (file_exists, dir_exists) = (path_buf.exists(), dir.exists());

    match (file_exists, dir_exists) {
        (false, false) => {
            reporter.error(
                format!("Unresolved module `{}`", name),
                "",
                (span.start().to_usize(), span.end().to_usize()),
            );

            Err(reporter.finish())
        }

        (true, false) => {
            if path_buf == PathBuf::from(db.path(module.file)) {
                reporter.error(
                    format!("Unresolved module `{}`", name),
                    format!("Sub-module folder for `{}` is missing", name),
                    (span.start().to_usize(), span.end().to_usize()),
                );

                return Err(reporter.finish());
            }

            // add a path from file -> module.file_id
            Ok(())
        }

        (false, true) => {
            let span = module.span;

            reporter.error(
                format!("Unresolved module `{}`", name),
                "Sub-module's exist but the module file doesn't ",
                (span.start().to_usize(), span.end().to_usize()),
            );

            Err(reporter.finish())
        }

        (true, true) => {
            dir.push(format!("{}.tox", name));

            // module exists and is the same as the one its being decleared in
            // check its children and report an err if its not found
            if path_buf == PathBuf::from(db.path(module.file)) && !dir.exists() {
                reporter.error(
                    format!("Unresolved module `{}`", name),
                    "",
                    (span.start().to_usize(), span.end().to_usize()),
                );

                Err(reporter.finish())
            } else {
                // add a path from file -> module.file_id
                Ok(())
            }
        }
    }
}