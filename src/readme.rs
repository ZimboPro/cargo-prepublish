use std::{fs, path::PathBuf, str::FromStr};

use cargo_toml::Package;
use read_input::shortcut::input;

use crate::Args;

#[derive(Debug, PartialEq, Eq)]
enum OptionSkip {
  Yes,
  No,
  Skip,
}

#[derive(Debug, PartialEq, Eq)]
struct OptionSkipParseError;

impl FromStr for OptionSkip {
  type Err = OptionSkipParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "y" | "yes" => Ok(Self::Yes),
      "n" | "no" => Ok(Self::No),
      "" => Ok(Self::Skip),
      _ => Err(OptionSkipParseError),
    }
  }
}

pub fn set_readme(package: &mut Package, cwd: &PathBuf, args: &Args) {
  if !package.readme.is_set() {
    let t = fs::read_dir(cwd);
    let mut exists = false;
    if let Ok(files) = t {
      for file in files {
        let p = file.unwrap();
        if p
          .file_name()
          .to_ascii_lowercase()
          .to_str()
          .unwrap()
          .contains("readme.md")
        {
          package.readme.set(cargo_toml::OptionalFile::Path(p.path()));
          exists = true;
        }
      }
    }
    if !exists && !args.non_interactive {
      println!("A Readme doesn't exist. It is optional though but recommended.");
      println!("Would you like to generate a Readme? Y/N");
      let option = input::<OptionSkip>().get();
      if option == OptionSkip::Yes {
        let content = format!("# {}", package.name);
        let readme = cwd.join("README.md");
        let result = fs::write(&readme, content);
        if let Err(e) = result {
          error!("An error occurred generating the README: {}", e);
        }
        package.readme.set(cargo_toml::OptionalFile::Path(readme))
      }
    } else if !exists && args.non_interactive {
      let readme = cwd.join("README.md");
      package.readme.set(cargo_toml::OptionalFile::Path(readme))
    }
  }
}
