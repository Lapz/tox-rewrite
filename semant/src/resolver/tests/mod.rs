use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

use tempfile::{tempdir, Builder, TempDir};
#[derive(Debug, Deserialize)]
pub struct DirectoryStructure {
    contents: Vec<TestStructure>,
}

#[derive(Debug, Deserialize)]
pub struct TestStructure {
    #[serde(default)]
    name: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    kind: Type,
    #[serde(default)]
    contents: Option<DirectoryStructure>,
}

#[derive(Debug, Deserialize, PartialEq)]
enum Type {
    File,
    Dir,
}

impl std::default::Default for Type {
    fn default() -> Self {
        Type::File
    }
}

pub fn create_structure(dir: &Path, structure: &DirectoryStructure) -> io::Result<()> {
    for test in &structure.contents {
        create_test(&dir, test)?
    }

    Ok(())
}

pub fn create_test(dir: &Path, test: &TestStructure) -> io::Result<()> {
    if test.kind == Type::Dir {
        fs::create_dir(dir.join(&test.name))?;

        // let new_dir = Builder::new().prefix(&test.name).tempdir_in(dir.path())?;
        // let dir = TempDir::new_in()?;

        create_structure(
            &dir.join(&test.name).as_path(),
            test.contents.as_ref().unwrap(),
        )?;
    } else {
        let file_path = dir.join(&test.name);
        let mut file = File::create(file_path)?;
        write!(&mut file, "{}", test.text)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn load_file<P: AsRef<Path>>(p: P) -> DirectoryStructure {
        return ron::de::from_str::<DirectoryStructure>(
            &fs::read_to_string(p).expect("Couldn't read file"),
        )
        .expect("Invalid ron file");
    }
    #[test]
    fn it_works() -> io::Result<()> {
        let dir = tempdir()?;
        // println!("{:?}",);
        let structure = load_file(&format!(
            "{}/src/resolver/tests/with_dir.ron",
            env!("CARGO_MANIFEST_DIR")
        ));

        create_structure(&dir.path(), &structure)?;

        use walkdir::WalkDir;

        for entry in WalkDir::new(dir.path()) {
            println!("{}", entry?.path().display());
        }

        dir.close()?;

        Ok(())
    }
}
