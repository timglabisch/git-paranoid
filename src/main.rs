extern crate git2;

use git2::Repository;
use git2::DiffFormat;
use git2::DiffOptions;
use git2::Oid;
use git2::Reference;
use std::str;
use std::collections::HashMap;


fn main() {

    let mut commitMap = HashMap::new();

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

        let oid = reference.target().unwrap();

        addCommitToCommitMapByReference(oid.clone(), &mut commitMap, branch_name, &repo);
    }

    analyzeCommits(&commitMap, &repo);

    println!("Hello, world!");
}

fn addCommitToCommitMapByReference(
    oid : Oid,
    commitMap : &mut HashMap<Oid, Vec<String>>,
    branch_name : String,
    repo : &Repository
)
{

    if !commitMap.contains_key(&oid) {
        commitMap.insert(oid, vec![branch_name.clone()]);
    } else {
        commitMap.get_mut(&oid).unwrap().push(branch_name.clone());
    }

    let commit = repo.find_commit(oid.clone()).unwrap();

    for pid in commit.parent_ids() {
        addCommitToCommitMapByReference(pid.clone(), commitMap, branch_name.clone(), &repo);
    }


}

fn analyzeCommits(commitMap : &HashMap<Oid, Vec<String>>, repo : &Repository)
{
    for (oid, branches) in commitMap {

        let commit = repo.find_commit(oid.clone()).unwrap();

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
}
