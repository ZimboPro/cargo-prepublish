use std::path::PathBuf;

use read_input::shortcut::input;
use toml_edit::{Formatted, Item, Value};

use crate::{keys::HOME_KEY, repository::get_repo_url, Args};

pub fn set_homepage_toml(package: &mut Item, cwd: &PathBuf, args: &Args) -> bool {
  let mut is_repo = true;
  if package.get(HOME_KEY).is_none() {
    if !args.non_interactive {
      println!("Please enter a home page for your package. Entering nothing will use a default the repo instead");
      let home_page = input::<String>().get();
      if home_page.is_empty() {
        let (valid, url) = get_repo_url(cwd, args);
        is_repo = valid;
        if valid {
          package[HOME_KEY] = Item::Value(Value::String(Formatted::new(url)));
        }
      } else {
        package[HOME_KEY] = Item::Value(Value::String(Formatted::new(home_page)));
      }
    } else {
      let (valid, url) = get_repo_url(cwd, args);
      is_repo = valid;
      if valid {
        package[HOME_KEY] = Item::Value(Value::String(Formatted::new(url)));
      } else {
        error!("This is not a git repository with a remote URL set.");
      }
    }
  }
  is_repo
}
