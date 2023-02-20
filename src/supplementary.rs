use toml_edit::{Item, Value, Array};

use crate::keys::EXCLUDE_KEY;

pub fn exclude_files(package: &mut Item) {
    if package.get(EXCLUDE_KEY).is_none() {
        let mut excluded_files = Array::new();
        excluded_files.push("Cargo.toml.bak");
        package[EXCLUDE_KEY] = toml_edit::Item::Value(Value::Array(excluded_files));
    } else if package[EXCLUDE_KEY].is_array() {
        let files = package[EXCLUDE_KEY].as_array_mut().unwrap();
        let t = files.iter().find(|x| x.is_str() && x.as_str().unwrap() == "Cargo.toml.bak");
        if t.is_none() {
            files.push("Cargo.toml.bak");
            package[EXCLUDE_KEY] = toml_edit::Item::Value(Value::Array(files.clone()));
        }
    }
}