use anyhow::Result;
use std::fs::{create_dir, rename};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
pub struct Promote {
    #[structopt(required = true, parse(from_os_str))]
    pub module: PathBuf,
}

impl Promote {
    pub fn run(self) -> Result<()> {
        if !self.module.is_file() {
            anyhow::bail!(Error::NoSuchFile(self.module))
        }

        match self.module.extension() {
            Some(ext) => {
                if ext != "rs" {
                    anyhow::bail!(Error::NotRustSource(self.module))
                }
            }
            None => anyhow::bail!(Error::NotRustSource(self.module)),
        }

        // input: src/<modname>.rs -> <modname>
        let new_mod_name = self
            .module
            .file_stem()
            .ok_or(Error::NotRustSource(self.module.clone()))?;

        // <modname> -> src/<modname>
        let new_mod_path = self.module.with_file_name(new_mod_name);

        println!("creating directory {:?}", new_mod_path);
        create_dir(new_mod_path.clone())?;

        // input: src/<modname>.rs -> src/<modname>/mod.rs
        let new_mod_file = new_mod_path.join("mod.rs");
        println!("moving {:?} to {:?}", self.module, new_mod_file);
        rename(self.module, new_mod_file)?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    /// The specified file does not exist
    #[error("no such file: {0}")]
    NoSuchFile(PathBuf),

    /// The specified path is not a file
    #[error("not a file: {0}")]
    NotFile(PathBuf),

    /// The given path is not a rust source file
    #[error("not a rust source file: {0}")]
    NotRustSource(PathBuf),
}
