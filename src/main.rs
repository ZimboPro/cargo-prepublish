mod author;
mod description;
mod documentation;
mod homepage;
mod license;
mod readme;
mod repository;
mod validate;

use std::fs;

use clap::Parser;
#[macro_use]
extern crate log;

use author::set_authors;
use description::set_description;
use documentation::{set_documentation, set_doc_rs_features};
use homepage::set_homepage;
use license::set_license;
use readme::set_readme;
use repository::set_repo;
use validate::validate;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum PrepublishErrors {
    #[error("The Cargo.toml is invalid")]
    InValid(Vec<String>),
    #[error("This is not a git repo")]
    NotGit,
    #[error("Cargo.toml not in current directory")]
    CargoNotInDir
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
      let cargo = cargo_toml::Manifest::from_path(&cargo_path);
      match cargo {
        Ok(mut c) => {
          if let Some(mut package) = c.package {
            if args.valid {
                return validate(&mut package, &cwd);
            } else {
                set_authors(&mut package, &args);
                set_license(&mut package, &args);
                set_description(&mut package, &args);
                let in_repo = set_homepage(&mut package, &cwd, &args);
                set_repo(&mut package, &cwd, &args);
                set_documentation(&mut package, &args);
                set_readme(&mut package, &cwd, &args);
                
                  c.package = Some(package);
                  let t = c.bin.iter().position(|x| x.path == Some("src/main.rs".to_string()));
                  if let Some(bin) = t {
                      c.bin.remove(bin);
                  }
                  let ser = toml::to_string_pretty(&c);
                  if let Ok(mut doc) = ser {
                    doc = set_doc_rs_features(doc, !c.features.is_empty());
                    let _ = fs::write(cargo_path, doc);
                }
                if !in_repo {
                  return Err(PrepublishErrors::NotGit);
                }
            }
            
          } else {
            warn!("The Cargo.toml file doesn't have package metadata")
          }
        }
        Err(e) => error!("Error: {}", e),
      }
    }
  } else {
    error!("Error occurred getting the current directory");
  }
  Ok(())
}
