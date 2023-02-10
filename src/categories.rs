use cargo_toml::Package;
use read_input::shortcut::input;

use crate::Args;

pub fn set_categories(package: &mut Package, args: &Args) {
    if package.categories.is_empty() && !args.non_interactive {
      println!(
        r#"Categories are not set. Please enter at least one category"#
      );
      let mut category = input::<String>().get();
      let mut cat_inputs = Vec::new();
      while cat_inputs.is_empty() {
        if category.contains(" ") {
          category = category.replace(" ", "-");
        }
        if !category.is_empty() {
          cat_inputs.push(category.clone().to_lowercase());
        } else {
          println!("Please enter at least one");
          category = input::<String>().get();
        }
      }
      println!("Please enter more categories. (Press enter to skip)");
      while !category.is_empty() {
        if category.contains(" ") {
          category = category.replace(" ", "-");
        }
        cat_inputs.push(category.clone().to_lowercase());
        category = input::<String>().get();
      }
      if !cat_inputs.is_empty() {
        package.categories.set(cat_inputs);
      }
    } else if package.categories.is_empty() {
      package.categories = Some(vec![package.name.replace(" ", "-").to_lowercase()]).into()
    }
}