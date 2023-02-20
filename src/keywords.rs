use read_input::shortcut::input;
use toml_edit::{Array, Formatted, Item, Value};

use crate::{keys::KEYWORDS_KEY, Args};

pub fn set_keywords_toml(package: &mut Item, args: &Args) {
  let has_keywords = package.get(KEYWORDS_KEY).is_some()
    && package[KEYWORDS_KEY].is_array()
    && !package[KEYWORDS_KEY].as_array().unwrap().is_empty();
  if !has_keywords && !args.non_interactive {
    println!(r#"Keywords are not set. Please enter at least one keyword"#);
    let mut keyword = input::<String>().get();
    let mut keyword_inputs = Array::new();
    while keyword.is_empty() || keyword.trim().is_empty() {
      println!("Please enter at least one");
      keyword = input::<String>().get();
    }

    println!("Please enter more keywords. (Press enter to skip)");
    while !keyword.is_empty() {
      if keyword.trim().is_empty() {
        println!("Please enter a valid word");
        continue;
      }
      if keyword.contains(' ') {
        keyword = keyword.replace(' ', "-");
      }
      keyword_inputs.push(keyword.clone().to_lowercase());
      keyword = input::<String>().get();
    }
    if !keyword_inputs.is_empty() {
      package[KEYWORDS_KEY] = toml_edit::Item::Value(Value::Array(keyword_inputs));
    }
  } else if !has_keywords {
    package[KEYWORDS_KEY] = toml_edit::Item::Value(Value::String(Formatted::new(
      package["name"]
        .as_str()
        .unwrap()
        .replace(' ', "-")
        .to_lowercase(),
    )))
  }
}
