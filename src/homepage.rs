use std::path::PathBuf;

use cargo_toml::{Inheritable, Package};
use read_input::shortcut::input;

use crate::{repository::get_repo_url, Args};

pub fn set_homepage(package: &mut Package, cwd: &PathBuf, args: &Args) -> bool {
  let mut is_repo = true;
  if package.homepage.is_none() {
    if !args.non_interactive {
      println!("Please enter a home page for your package. Entering nothing will use a default the repo instead");
      let home_page = input::<String>().get();
      if home_page.is_empty() {
        let (valid, url) = get_repo_url(cwd, args);
        is_repo = valid;
        package.homepage = Some(Inheritable::Set(url));
      } else {
        package.homepage = Some(cargo_toml::Inheritable::Set(home_page));
      }
    } else {
      let (valid, url) = get_repo_url(cwd, args);
      is_repo = valid;
      package.homepage = Some(Inheritable::Set(url));
    }
  }
  is_repo
}
