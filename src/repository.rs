use std::path::PathBuf;

use read_input::{shortcut::input, InputConstraints};
use regex::Regex;
use rustygit::Repository;
use toml_edit::{Formatted, Item, Value};

use crate::{keys::REPO_KEY, Args};

pub fn set_repo_toml(package: &mut Item, cwd: &PathBuf, args: &Args) -> bool {
  let mut is_repo = true;
  if package.get(REPO_KEY).is_none() {
    println!("Please enter a git repository for your package. Entering nothing will use a default the current git repo instead");
    let repo = input::<String>().get();
    if repo.is_empty() {
      let (valid, url) = get_repo_url(cwd, args);
      if valid {
        package[REPO_KEY] = toml_edit::Item::Value(Value::String(Formatted::new(url)));
      } else {
        error!("This is not a git repository");
      }
      is_repo = valid;
    } else {
      package[REPO_KEY] = toml_edit::Item::Value(Value::String(Formatted::new(repo)));
    }
  }
  is_repo
}

pub fn extract_url(repo: &str) -> String {
  if !repo.starts_with("http") {
    let regex = Regex::new(r#"(?P<prefix>(git|ssh|http(s)?)|(git@(?P<host>[\w\.]+)))(:(//)?)(?P<path>[\w\.@:/\-~]+)(\.git)(/)?"#).unwrap();
    let url = regex.captures(repo).unwrap();
    format!("http://{}/{}", &url["host"], &url["path"])
  } else {
    repo.to_string()
  }
}

pub fn get_repo_url(cwd: &PathBuf, args: &Args) -> (bool, String) {
  let mut is_repo = true;
  let mut repo_url = String::new();
  let repo = Repository::new(cwd);
  match repo.list_remotes() {
    Ok(list) => {
      if list.is_empty() {
        warn!("This project is not in a git repository.");
        is_repo = false;
      } else if list.len() == 1 && !list[0].trim().is_empty() {
        let name = list.get(0).unwrap();
        let remote = repo.show_remote_uri(name);
        match remote {
          Ok(details) => {
            repo_url = extract_url(&details);
          }
          Err(e) => error!("Error occurred getting remote details: {}", e),
        }
      } else if !args.non_interactive {
        let mut cnt = 0;
        println!("Please select a repo");
        for i in &list {
          println!("{cnt}) {i}");
          cnt += 1;
        }
        let opt = input::<usize>().max(cnt).get();
        repo_url = extract_url(list.get(opt).unwrap());
      } else {
        warn!("This project is not in a git repository or there are multiple remotes set.");
        is_repo = false;
      }
    }
    Err(e) => {
      warn!("This project is not in a git repository. {}", e);
      is_repo = false;
    }
  }
  (is_repo, repo_url)
}
