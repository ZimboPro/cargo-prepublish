use std::path::PathBuf;

use cargo_toml::Package;
use toml_edit::Item;

use crate::{PrepublishErrors, repository::get_repo_url, Args, keys::{DESC_KEY, CATEGORY_KEY, HOME_KEY, LICENSE_KEY, LICENSE_FILE_KEY, README_KEY, REPO_KEY}};


pub fn validate(package: &mut Package, cwd: &PathBuf) -> Result<(), PrepublishErrors> {
    // let mut str = String::new();
    // str += if package.description.is_some() {
    //     ""
    // } else {
    //     "Description doesn't exist\r\n"
    // };
    // str += if package.homepage.is_some() {
    //     ""
    // } else {
    //     "Homepage doesn't exist\r\n"
    // };
    // str += if package.license.is_some() || package.license_file.is_some() {
    //     ""
    // } else {
    //     "License or License File doesn't exist\r\n"
    // };
    // str += if package.readme.is_set() {
    //     ""
    // } else {
    //     "Readme doesn't exist\r\n"
    // };
    // str += if package.repository.is_some() {
    //     ""
    // } else {
    //     "Repository doesn't exist\r\n"
    // };
    // let args = Args {
    //     non_interactive: true,
    //     ..Args::default()
    // };
    // let (v, _) = get_repo_url(cwd, &args);
    // str += if v {
    //     ""
    // } else {
    //     "A remote repository doesn't exist"
    // };
    // if str.is_empty() {
    //     Ok(())
    // } else {
    //     Err(PrepublishErrors::InValid(str))
    // }
    let mut str = Vec::new();
    if package.description.is_none() {
        str.push("Description doesn't exist".to_owned());
    };
    if package.categories.is_empty() {
        str.push("No Categories are listed".to_owned());
    };
    if package.homepage.is_none() {
        str.push("Homepage doesn't exist".to_owned());
    };
    if package.license.is_none() && package.license_file.is_none() {
        str.push("License or License File doesn't exist".to_owned());
    };
    if !package.readme.is_set() {
        str.push("Readme doesn't exist".to_owned());
    };
    if package.repository.is_none() {
        str.push("Repository doesn't exist".to_owned());
    };
    let args = Args {
        non_interactive: true,
        ..Args::default()
    };
    let (v, _) = get_repo_url(cwd, &args);
    if !v {
        str.push("A remote repository doesn't exist".to_owned());
    };
    if str.is_empty() {
        println!("You package is almost ready for publishing.");
        println!("Suggest that you run these for final checks:");
        println!("cargo publish --dry-run");
        println!("");
        println!("To check that you are not sending any unnecessary files.");
        println!("cargo package --list");
        println!("");
        println!("More details can be found here https://doc.rust-lang.org/cargo/reference/publishing.html");
        Ok(())
    } else {
        Err(PrepublishErrors::InValid(str))
    }
}

pub fn validate_toml(package: &mut Item, cwd: &PathBuf) -> Result<(), PrepublishErrors> {
    let mut str = Vec::new();
    if package[DESC_KEY].is_none() {
        str.push("Description doesn't exist".to_owned());
    };
    if package[CATEGORY_KEY].is_none() || (package[CATEGORY_KEY].is_array() && package[CATEGORY_KEY].as_array().unwrap().is_empty()) {
        str.push("No Categories are listed".to_owned());
    };
    if package[HOME_KEY].is_none() {
        str.push("Homepage doesn't exist".to_owned());
    };
    if package[LICENSE_KEY].is_none() && package[LICENSE_FILE_KEY].is_none() {
        str.push("License or License File doesn't exist".to_owned());
    };
    if !package[README_KEY].is_none() {
        str.push("Readme doesn't exist".to_owned());
    };
    if package[REPO_KEY].is_none() {
        str.push("Repository doesn't exist".to_owned());
    };
    let args = Args {
        non_interactive: true,
        ..Args::default()
    };
    let (v, _) = get_repo_url(cwd, &args);
    if !v {
        str.push("A remote repository doesn't exist".to_owned());
    };
    if str.is_empty() {
        println!("You package is almost ready for publishing.");
        println!("Suggest that you run these for final checks:");
        println!("cargo publish --dry-run");
        println!("");
        println!("To check that you are not sending any unnecessary files.");
        println!("cargo package --list");
        println!("");
        println!("More details can be found here https://doc.rust-lang.org/cargo/reference/publishing.html");
        Ok(())
    } else {
        Err(PrepublishErrors::InValid(str))
    }
}