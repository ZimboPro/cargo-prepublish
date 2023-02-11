use std::path::PathBuf;

use cargo_toml::{Inheritable, Package};
use git2::Repository;
use read_input::{shortcut::input, InputConstraints};
use regex::Regex;

use crate::Args;

pub fn set_repo(package: &mut Package, cwd: &PathBuf, args: &Args) -> bool {
  let mut is_repo = true;
  if package.repository.is_none() {
    println!("Please enter a git repository for your package. Entering nothing will use a default the current git repo instead");
    let repo = input::<String>().get();
    if repo.is_empty() {
      let (valid, url) = get_repo_url(cwd, args);
      is_repo = valid;
      package.repository = Some(Inheritable::Set(url));
    } else {
      package.repository = Some(cargo_toml::Inheritable::Set(repo));
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
  match Repository::open(cwd) {
    Ok(repo) => {
      let repos = repo.remotes();
      match repos {
        Ok(list) => {
          if list.is_empty() {
            warn!("This project is not in a git repository.");
            is_repo = false;
          } else {
            if list.len() == 1 {
              let name = list.get(0).unwrap();
              let remote = repo.find_remote(name);
              match remote {
                Ok(details) => {
                  let url = details.url().unwrap();
                  repo_url = extract_url(url);
                }
                Err(e) => error!("Error occurred getting remote details: {}", e),
              }
            } else {
              if !args.non_interactive {
                let mut cnt = 0;
                println!("Please select a repo");
                for i in &list {
                  println!("{}) {}", cnt, i.unwrap());
                  cnt = cnt + 1;
                }
                let opt = input::<usize>().max(cnt).get();
                repo_url = extract_url(list.get(opt).unwrap());
              } else {
                repo_url = extract_url(list.get(0).unwrap());
              }
            }
          }
        }
        Err(e) => {
          warn!("This project is not in a git repository. {}", e);
          is_repo = false;
        }
      }
    }
    Err(e) => {
      warn!("This project is not in a git repository. {}", e);
      is_repo = false;
    }
  }
  (is_repo, repo_url)
}
