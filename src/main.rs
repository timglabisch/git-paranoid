extern crate git2;

use git2::Repository;
use git2::DiffFormat;
use git2::DiffOptions;
use std::str;

fn main() {

    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    for b in repo.branches(None).unwrap() {

        let branch = b.unwrap();
        let branch_name = branch.0.name().unwrap().unwrap();

        println!("{}", branch_name);


        let reference = branch.0.get();

        println!("{}", reference.name().unwrap());

        let oid = reference.target().unwrap();

        println!("{}", oid);

        let commit = repo.find_commit(oid).unwrap();

        println!("{}", commit.message().unwrap());

        let a = commit.tree().unwrap();
        let b = if commit.parents().len() == 1 {
            let parent = commit.parent(0).unwrap();
            parent.tree().unwrap()
        } else {
            panic!("b is wrong");
        };

        let (mut diffopts, mut diffopts2) = (DiffOptions::new(), DiffOptions::new());
        let diff = repo.diff_tree_to_tree(Some(&a), Some(&b), Some(&mut diffopts2)).unwrap();
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
