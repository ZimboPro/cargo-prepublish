use cargo_toml::Package;
use toml_edit::{Item, Value};

use crate::{keys::CATEGORY_KEY, Args};

pub fn set_categories(package: &mut Package, args: &Args) {
  if package.categories.is_empty() && !args.non_interactive {
    println!(
      r#"Categories are not set. Please enter at least one category manually. You can find the supported categories here https://crates.io/category_slugs"#
    );
  } else if package.categories.is_empty() {
    package.categories = Some(vec![]).into()
  }
}

pub fn set_categories_toml(package: &mut Item, args: &Args) {
  let have_categories = package.get(CATEGORY_KEY).is_some()
    && package[CATEGORY_KEY].is_array()
    && !package[CATEGORY_KEY].as_array().unwrap().is_empty();
  if !have_categories && !args.non_interactive {
    // let cat = "[] # You can find the supported categories here https://crates.io/category_slugs".parse::<toml_edit::Item>().unwrap();
    package[CATEGORY_KEY] = toml_edit::Item::Value(Value::Array(toml_edit::Array::new()));
    println!(
      r#"Categories are not set. Please enter at least one category manually. You can find the supported categories here https://crates.io/category_slugs"#
    );
  } else if !have_categories {
    // let cat = "[] # You can find the supported categories here https://crates.io/category_slugs".parse::<toml_edit::Item>().unwrap();
    package[CATEGORY_KEY] = toml_edit::Item::Value(Value::Array(toml_edit::Array::new()));
  }
}
