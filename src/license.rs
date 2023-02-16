use std::{str::FromStr};

use read_input::shortcut::{input, input_d};
use toml_edit::{Formatted, Item, Value};

use crate::{
  keys::{LICENSE_FILE_KEY, LICENSE_KEY},
  Args,
};

enum License {
  MIT,
  Apache2,
  MITApache,
  LGPL,
  GPL,
}

impl ToString for License {
  fn to_string(&self) -> String {
    match self {
      License::MIT => "MIT".to_owned(),
      License::Apache2 => "Apache-2.0".to_owned(),
      License::MITApache => "MIT OR Apache-2.0".to_owned(),
      License::LGPL => "LGPL-2.1-only AND MIT AND BSD-2-Clause".to_owned(),
      License::GPL => "GPL-2.0-or-later WITH Bison-exception-2.2".to_owned(),
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseLicenseError;

impl FromStr for License {
  type Err = ParseLicenseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "0" => Ok(Self::MIT),
      "1" => Ok(Self::Apache2),
      "2" => Ok(Self::MITApache),
      "3" => Ok(Self::LGPL),
      "4" => Ok(Self::GPL),
      _ => Err(ParseLicenseError),
    }
  }
}

pub fn set_license_toml(package: &mut Item, args: &Args) {
  if package.get(LICENSE_KEY).is_none() && package.get(LICENSE_FILE_KEY).is_none() {
    if !args.non_interactive {
      println!("A License doesn't exist. Would you like to use a file or a standard template?");
      println!("1) Template");
      println!("2) File");
      println!("Select an option (Default is 1):");
      let t = input_d::<u8>().default(1).get();
      if t == 1 {
        println!("Please select a template");
        println!("0) MIT");
        println!("1) Apache-2.0");
        println!("2) MIT OR Apache-2.0");
        println!("3) LGPL-2.1-only AND MIT AND BSD-2-Clause");
        println!("4) GPL-2.0-or-later WITH Bison-exception-2.2");
        let t = input::<License>().get();
        package[LICENSE_KEY] = toml_edit::Item::Value(Value::String(Formatted::new(t.to_string())));
      } else {
        package[LICENSE_FILE_KEY] =
          toml_edit::Item::Value(Value::String(Formatted::new("LICENSE.txt".to_string())));
        println!("The License File has been set to 'LICENSE.txt'. Update as necessary and/or make sure the file exists before publishing. Press enter to continue");
        input::<String>().get();
      }
    } else {
      package[LICENSE_KEY] = toml_edit::Item::Value(Value::String(Formatted::new(
        License::MITApache.to_string(),
      )));
    }
  }
}
