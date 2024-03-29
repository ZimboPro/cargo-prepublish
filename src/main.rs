#![doc = include_str!(r#"../README.md"#)]
mod author;
mod categories;
mod description;
mod documentation;
mod homepage;
mod keys;
mod keywords;
mod license;
mod readme;
mod repository;
mod validate;
mod supplementary;

use std::fs;

use categories::set_categories_toml;
use clap::Parser;
#[macro_use]
extern crate log;

use author::set_authors_toml;
use description::set_description_toml;
use documentation::{set_doc_rs_features, set_documentation_toml};
use homepage::set_homepage_toml;
use keys::{FEATURES_KEY, PACKAGE_KEY};
use keywords::set_keywords_toml;
use license::set_license_toml;
use readme::set_readme_toml;
use repository::set_repo_toml;
use supplementary::exclude_files;
use thiserror::Error;
use toml_edit::Document;
use validate::validate_toml;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  prepublish: Option<String>,
  /// Makes it non-interactive so user input won't be needed. Default is false
  #[arg(short, long)]
  pub non_interactive: bool,
  #[arg(short, long)]
  pub valid: bool,
}

#[derive(Debug, Error, Clone)]
pub enum PrepublishErrors {
  #[error("The Cargo.toml is invalid")]
  InValid(Vec<String>),
  #[error("This is not a git repo")]
  NotGit,
  #[error("Cargo.toml not in current directory")]
  CargoNotInDir,
}

fn main() -> Result<(), PrepublishErrors> {
  dotenvy::dotenv().ok();
  let args = Args::parse();

  env_logger::init();
  if let Ok(cwd) = std::env::current_dir() {
    let cargo_path = cwd.join("Cargo.toml");
    if !cargo_path.exists() {
      return Err(PrepublishErrors::CargoNotInDir);
    } else {
      let _ = fs::copy(&cargo_path, cwd.join("Cargo.toml.bak"));
      let content = fs::read_to_string(&cargo_path).unwrap();
      let mut doc = content.parse::<Document>().expect("Invalid TOML file");
      let package_data = &doc[PACKAGE_KEY];
      if package_data.is_none() {
        error!("The Cargo.toml file doesn't have package metadata");
      } else if args.valid {
        validate_toml(&mut doc[PACKAGE_KEY], &cwd)?;
      } else {
        set_authors_toml(&mut doc[PACKAGE_KEY], &args);
        set_categories_toml(&mut doc[PACKAGE_KEY], &args);
        set_keywords_toml(&mut doc[PACKAGE_KEY], &args);
        set_description_toml(&mut doc[PACKAGE_KEY], &args);
        set_documentation_toml(&mut doc[PACKAGE_KEY], &args);
        let has_features = doc.get(FEATURES_KEY).is_some()
          && doc[FEATURES_KEY].is_table()
          && !doc[FEATURES_KEY].as_table().unwrap().is_empty();
        set_homepage_toml(&mut doc[PACKAGE_KEY], &cwd, &args);
        set_license_toml(&mut doc[PACKAGE_KEY], &args);
        set_readme_toml(&mut doc[PACKAGE_KEY], &cwd, &args);
        set_repo_toml(&mut doc[PACKAGE_KEY], &cwd, &args);

        exclude_files(&mut doc[PACKAGE_KEY]);
        let contents = doc.to_string();
        let contents = set_doc_rs_features(contents, has_features);
        let _ = fs::write(cargo_path, contents);
      }
    }
  } else {
    error!("Error occurred getting the current directory");
  }
  Ok(())
}
