use read_input::shortcut::input;
use toml_edit::{Formatted, Item, Value};

use crate::{keys::DESC_KEY, Args};

pub fn set_description_toml(package: &mut Item, args: &Args) {
  if package.get(DESC_KEY).is_none() {
    if !args.non_interactive {
      println!(
        "Please enter a description for your package. Entering nothing will use a default instead"
      );
      let desc = input::<String>().get();
      if desc.is_empty() {
        let desc_default = Formatted::new("The description goes here.".to_owned());
        package[DESC_KEY] = toml_edit::Item::Value(Value::String(desc_default));
      } else {
        let desc_value = Formatted::new(desc);
        package[DESC_KEY] = Item::Value(Value::String(desc_value));
      }
    } else {
      let desc_default = Formatted::new("The description goes here.".to_owned());
      package[DESC_KEY] = toml_edit::Item::Value(Value::String(desc_default));
    }
  }
}
