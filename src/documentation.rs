use read_input::shortcut::input;
use toml_edit::{Document, Formatted, Item, Value};

use crate::{keys::DOCS_KEY, Args};

pub fn set_doc_rs_features(cargo_content: String, has_features: bool) -> String {
  if has_features {
    let mut blocks: Vec<&str> = cargo_content.split("\n\n").collect();
    let content = if let Some(ind) = blocks
      .iter()
      .position(|b| b.contains("package.metadata.docs.rs"))
    {
      let f = blocks.get(ind).unwrap();
      if !f.contains("all-features") {
        let n = format!("{f}\nall-features = true");
        blocks.remove(ind);
        blocks.insert(ind, n.as_str().clone());
        return blocks.join("\n\n");
      }
      blocks.join("\n\n")
    } else {
      blocks.push("# You can read more about these settings here https://docs.rs/about/metadata\n[package.metadata.docs.rs]\nall-features = true");
      blocks.join("\n\n")
    };
    return content;
  }
  cargo_content
}

pub fn set_documentation_toml(package: &mut Item, args: &Args) {
  if package.get(DOCS_KEY).is_none() && !args.non_interactive {
    println!("If this will be the first time this crate is getting published, it is not needed to set the documentation property.");
    println!("docs.rs will be doing that automatically.");
    println!("Please enter an documentation url: Press enter to skip");
    let docs = input::<String>().get();
    if !docs.is_empty() {
      package[DOCS_KEY] = Item::Value(Value::String(Formatted::new(docs)));
    }
  }
}

pub fn set_doc_rs_features_toml(doc: &mut Document, has_features: bool) {
  if has_features {
    // let val = "true # Some cooment".parse::<toml_edit::Item>().unwrap();
    if doc.get("package.metadata.docs.rs").is_none() {
      println!("Doesn't contain");
      doc["package"]["metadata"]["docs"]["rs"]["all-features"] =
        Item::Value(Value::Boolean(Formatted::new(true)));
      if let Some(t) = doc["package"]["metadata"]["docs"]["rs"].as_inline_table_mut() {
        t.fmt()
      }
    } else if !doc["package.metadata.docs.rs"]["all-features"].is_value() {
      println!("contains");
      doc["package"]["metadata"]["docs"]["rs"]["all-features"] =
        Item::Value(Value::Boolean(Formatted::new(true)));
    }
  }
}
