extern crate git2;

use git2::Repository;
use git2::DiffFormat;
use git2::DiffOptions;
use std::str;
use std::collections::HashMap;


fn main() {

    let mut found_commits = HashMap::new();

    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    // 1. collect commit id's to analyze and save it's branches
    // 2. analyze commits
    // toml

    for b in repo.branches(None).unwrap() {

        let branch = b.unwrap();
        let branch_name = branch.0.name().unwrap().unwrap().to_string();

        println!("{}", branch_name);


        let reference = branch.0.get();

        println!("{}", reference.name().unwrap());

        let oid = reference.target().unwrap();

        println!("{}", oid);

        let commit = repo.find_commit(oid).unwrap();

        if !found_commits.contains_key(&oid) {
            found_commits.insert(oid, vec![branch_name.clone()]);
        } else {
            found_commits.get_mut(&oid).unwrap().push(branch_name.clone());
        }

        println!("{}", commit.message().unwrap());

        let a = commit.tree().unwrap();
        let b = if commit.parents().len() == 1 {
            let parent = commit.parent(0).unwrap();
            parent.tree().unwrap()
        } else {
            panic!("b is wrong");
        };

        let mut diffopts = DiffOptions::new();
        let diff = repo.diff_tree_to_tree(Some(&a), Some(&b), Some(&mut diffopts)).unwrap();
        diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            match line.origin() {
                ' ' | '+' | '-' => print!("{}", line.origin()),
                _ => {}
            }
            print!("{}", str::from_utf8(line.content()).unwrap());
            true
        });
    }

    println!("Hello, world!");
}
