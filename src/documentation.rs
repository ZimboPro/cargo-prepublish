use cargo_toml::{Inheritable, Package};
use read_input::shortcut::input;

use crate::Args;

pub fn set_documentation(package: &mut Package, args: &Args) {
  if package.documentation.is_none() && !args.non_interactive {
    println!("If this will be the first time this crate is getting published, it is not needed to set the documentation property.");
    println!("docs.rs will be doing that automatically.");
    println!("Please enter an documentation url: Press enter to skip");
    let docs = input::<String>().get();
    if !docs.is_empty() {
      package.documentation = Some(Inheritable::Set(docs));
    }
  }
}

pub fn set_doc_rs_features(cargo_content: String, has_features: bool) -> String {
    if has_features {
        let mut blocks: Vec<&str> = cargo_content.split("\n\n").collect();
        let content = if let Some(ind) = blocks.iter().position(|b| b.contains("package.metadata.docs.rs")) {
            let f = blocks.remove(ind);
            if !f.contains("all-features") {
                println!("no features");
                let n = format!("{}\nall-features = true", f);
                blocks.insert(ind, n.as_str().clone());
                return blocks.join("\n\n")
            }
            println!("features exist");
            blocks.join("\n\n")
        } else {
            println!("no docs rs");
            blocks.push("# You can read more about these settings here https://docs.rs/about/metadata\n[package.metadata.docs.rs]\nall-features = true");
            blocks.join("\n\n")
        };
        return content;
    }
    cargo_content
}