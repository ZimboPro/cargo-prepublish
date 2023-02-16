use std::path::PathBuf;

use toml_edit::Item;

use crate::{
  keys::{CATEGORY_KEY, DESC_KEY, HOME_KEY, LICENSE_FILE_KEY, LICENSE_KEY, README_KEY, REPO_KEY},
  repository::get_repo_url,
  Args, PrepublishErrors,
};

pub fn validate_toml(package: &mut Item, cwd: &PathBuf) -> Result<(), PrepublishErrors> {
  let mut str = Vec::new();
  if package.get(DESC_KEY).is_none() {
    str.push("Description doesn't exist".to_owned());
  };
  if package.get(CATEGORY_KEY).is_none()
    || (package[CATEGORY_KEY].is_array() && package[CATEGORY_KEY].as_array().unwrap().is_empty())
  {
    str.push("No Categories are listed".to_owned());
  };
  if package.get(HOME_KEY).is_none() {
    str.push("Homepage doesn't exist".to_owned());
  };
  if package.get(LICENSE_KEY).is_none() && package.get(LICENSE_FILE_KEY).is_none() {
    str.push("License or License File doesn't exist".to_owned());
  };
  if package.get(README_KEY).is_none() {
    str.push("Readme doesn't exist".to_owned());
  };
  if package.get(REPO_KEY).is_none() {
    str.push("Repository doesn't exist".to_owned());
  };
  let args = Args {
    non_interactive: true,
    ..Args::default()
  };
  let (v, _) = get_repo_url(cwd, &args);
  if !v {
    str.push("A remote repository doesn't exist".to_owned());
  };
  if str.is_empty() {
    println!("You package is almost ready for publishing.");
    println!("Suggest that you run these for final checks:");
    println!("cargo publish --dry-run");
    println!();
    println!("To check that you are not sending any unnecessary files.");
    println!("cargo package --list");
    println!();
    println!(
      "More details can be found here https://doc.rust-lang.org/cargo/reference/publishing.html"
    );
    Ok(())
  } else {
    Err(PrepublishErrors::InValid(str))
  }
}
