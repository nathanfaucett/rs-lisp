use cargo::{
  core::{compiler::CompileMode, Workspace},
  ops::{compile, CompileOptions},
  util::config::Config,
};
use std::{fs, path::Path};

#[inline]
pub fn build(manifest_path: &Path, dest_path: &Path) {
  let config = Config::default().unwrap();
  let workspace = Workspace::new(manifest_path, &config).unwrap();
  let compile_options = CompileOptions::new(&config, CompileMode::Build).unwrap();
  let complimation = compile(&workspace, &compile_options).unwrap();
  for cdylib in complimation.cdylibs {
    fs::copy(
      cdylib.path.clone(),
      dest_path.join(
        cdylib
          .path
          .file_name()
          .expect("Failed to get file name of binary"),
      ),
    )
    .expect("Failed to copy file");
  }
}
