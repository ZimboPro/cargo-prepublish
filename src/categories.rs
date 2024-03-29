use toml_edit::Item;

use crate::{keys::CATEGORY_KEY, Args};

pub fn set_categories_toml(package: &mut Item, args: &Args) {
  let have_categories = package.get(CATEGORY_KEY).is_some()
    && package[CATEGORY_KEY].is_array()
    && !package[CATEGORY_KEY].as_array().unwrap().is_empty();
  if !have_categories && !args.non_interactive {
    let cat = format!("{CATEGORY_KEY} = [] # You can find the supported categories here https://crates.io/category_slugs").parse::<toml_edit::Document>().unwrap();
    let cat = cat.get(CATEGORY_KEY).unwrap();
    package[CATEGORY_KEY] = cat.clone();
    println!(
      r#"Categories are not set. Please enter at least one category manually. You can find the supported categories here https://crates.io/category_slugs"#
    );
  } else if !have_categories {
    let cat = format!("{CATEGORY_KEY} = [] # You can find the supported categories here https://crates.io/category_slugs").parse::<toml_edit::Document>().unwrap();
    let cat = cat.get(CATEGORY_KEY).unwrap();
    package[CATEGORY_KEY] = cat.clone();
  }
}
