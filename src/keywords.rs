use cargo_toml::Package;
use read_input::shortcut::input;
use toml_edit::{Item, Array, Value, Formatted};

use crate::{Args, keys::KEYWORDS_KEY};

pub fn set_keywords(package: &mut Package, args: &Args) {
    if package.keywords.is_empty() && !args.non_interactive {
      println!(
        r#"Keywords are not set. Please enter at least one keyword"#
      );
      let mut keywords = input::<String>().get();
      let mut keyword_inputs = Vec::new();
      while keywords.is_empty() || keywords.trim().is_empty() {
        println!("Please enter at least one");
        keywords = input::<String>().get();
      }

      println!("Please enter more keywords. (Press enter to skip)");
      while !keywords.is_empty() {
        if keywords.trim().is_empty() {
          println!("Please enter a valid word");
          continue;
        }
        if keywords.contains(" ") {
          keywords = keywords.replace(" ", "-");
        }
        keyword_inputs.push(keywords.clone().to_lowercase());
        keywords = input::<String>().get();
      }
      if !keyword_inputs.is_empty() {
        package.keywords.set(keyword_inputs);
      }
    } else if package.keywords.is_empty() {
      package.keywords = Some(vec![package.name.replace(" ", "-").to_lowercase()]).into()
    }
}

pub fn set_keywords_toml(package: &mut Item, args: &Args) {
    let has_keywords = (package[KEYWORDS_KEY].is_array() && !package[KEYWORDS_KEY].as_array().unwrap().is_empty());
    if has_keywords && !args.non_interactive {
      println!(
        r#"Keywords are not set. Please enter at least one keyword"#
      );
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
        if keyword.contains(" ") {
          keyword = keyword.replace(" ", "-");
        }
        keyword_inputs.push(keyword.clone().to_lowercase());
        keyword = input::<String>().get();
      }
      if !keyword_inputs.is_empty() {
        package[KEYWORDS_KEY] = toml_edit::Item::Value(Value::Array(keyword_inputs));
      }
    } else if has_keywords {
      package[KEYWORDS_KEY] =  toml_edit::Item::Value(Value::String(Formatted::new(package["name"].as_str().unwrap().replace(" ", "-").to_lowercase())))
    }
}
