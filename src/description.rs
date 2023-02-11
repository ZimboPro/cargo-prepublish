use cargo_toml::{Inheritable, Package};
use read_input::shortcut::input;

use crate::Args;

pub fn set_description(package: &mut Package, args: &Args) {
  if package.description.is_none() {
    if !args.non_interactive {
      println!(
        "Please enter a description for your package. Entering nothing will use a default instead"
      );
      let desc = input::<String>().get();
      if desc.is_empty() {
        package.description = Some(Inheritable::Set("The description goes here.".to_owned()));
      } else {
        package.description = Some(cargo_toml::Inheritable::Set(desc));
      }
    } else {
      package.description = Some(Inheritable::Set("The description goes here.".to_owned()));
    }
  }
}
