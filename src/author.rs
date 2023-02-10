use cargo_toml::Package;
use read_input::shortcut::input;

use crate::Args;

pub fn set_authors(package: &mut Package, args: &Args) {
  if package.authors.is_empty() && !args.non_interactive {
    println!(
      r#"No author has been set. Please enter an author eg "John Doe" or "John Doe <example@email.com>". Press enter if you want to skip."#
    );
    let mut author = input::<String>().get();
    let mut authors_input = Vec::new();
    while !author.is_empty() {
      authors_input.push(author.clone());
      author = input::<String>().get();
    }
    if !authors_input.is_empty() {
      package.authors.set(authors_input);
    }
  }
}
