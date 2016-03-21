extern crate git2;
extern crate toml;
extern crate regex;

mod rule;

use git2::Repository;
use git2::DiffFormat;
use git2::DiffOptions;
use git2::Oid;
use git2::Reference;
use std::str;
use std::collections::HashMap;
use rule::Rule;
use rule::from_toml;


fn main() {

    let rules = from_toml();

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

        let reference = branch.0.get();

        let oid = reference.target().unwrap();

        addCommitToCommitMapByReference(oid.clone(), &mut commitMap, branch_name, &repo);
    }

    analyzeCommits(&rules, &commitMap, &repo);


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

fn analyzeCommits(rules : &Vec<Rule>, commitMap : &HashMap<Oid, Vec<String>>, repo : &Repository)
{
    for (oid, branches) in commitMap {

        let commit = repo.find_commit(oid.clone()).unwrap();
        let mut rule_matched = false;

        // println!("{}", commit.message().unwrap());

        for parent in commit.parents() {
            let mut diffopts = DiffOptions::new();
            let diff = repo.diff_tree_to_tree(
                Some(&parent.tree().unwrap()),
                Some(&commit.tree().unwrap()),
                Some(&mut diffopts)
            ).unwrap();

            diff.print(DiffFormat::Patch, |delta, _hunk, line| {
                match line.origin() {
                    '+' => {
                        //print!("{}", delta.new_file().path().unwrap().to_string_lossy());
                        //print!("\t{}", line.origin());
                        //print!("\t{}", str::from_utf8(line.content()).unwrap());

                        for rule in rules {
                            for branch in branches {
                                if rule.statify(
                                    delta.new_file().path().unwrap().to_string_lossy().to_string(),
                                    str::from_utf8(line.content()).unwrap().to_string(),
                                    "author".to_string(),
                                    branch.to_string()
                                ) {
                                    rule_matched = true;
                                    break;
                                }
                            }
                        }
                    },
                    ' ' | '-' => {},
                    _ => {}
                }
                true
            });
        }

        if rule_matched {
            println!("take a deeper look at commit {}", commit.id());
        }
    }
}
