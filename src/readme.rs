use std::{fs, path::PathBuf, str::FromStr};

use read_input::shortcut::input;
use toml_edit::{Formatted, Item, Value};

use crate::{keys::README_KEY, Args};

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

pub fn set_readme_toml(package: &mut Item, cwd: &PathBuf, args: &Args) {
  if package.get(README_KEY).is_none() {
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
          package[README_KEY] = toml_edit::Item::Value(Value::String(Formatted::new(
            p.file_name().to_str().unwrap().to_owned(),
          )));
          exists = true;
          break;
        }
      }
    }
    if !exists && !args.non_interactive {
      println!("A Readme doesn't exist. It is optional though but recommended.");
      println!("Would you like to generate a Readme? Y/N");
      let option = input::<OptionSkip>().get();
      if option == OptionSkip::Yes {
        let content = format!("# {}", package["name"].as_str().unwrap());
        let readme = cwd.join("README.md");
        let result = fs::write(readme, content);
        if let Err(e) = result {
          error!("An error occurred generating the README: {}", e);
        }
        package[README_KEY] =
          toml_edit::Item::Value(Value::String(Formatted::new("README.md".to_owned())));
      }
    } else if !exists && args.non_interactive {
      package[README_KEY] =
        toml_edit::Item::Value(Value::String(Formatted::new("README.md".to_owned())));
    }
  }
}
