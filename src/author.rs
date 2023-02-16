use read_input::shortcut::input;
use toml_edit::{Item, Value};

use crate::{keys::AUTHORS_KEY, Args};

pub fn set_authors_toml(package: &mut Item, args: &Args) {
  if package.get(AUTHORS_KEY).is_none() && !args.non_interactive {
    println!(
      r#"No author has been set. Please enter an author eg "John Doe" or "John Doe <example@email.com>". Press enter if you want to skip."#
    );
    let mut author = input::<String>().get();
    let mut authors_input = toml_edit::Array::new();
    while !author.is_empty() {
      authors_input.push(author.clone());
      author = input::<String>().get();
    }
    if !authors_input.is_empty() {
      package[AUTHORS_KEY] = toml_edit::Item::Value(Value::Array(authors_input));
    }
  }
}
