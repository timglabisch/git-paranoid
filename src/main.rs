extern crate git2;
extern crate toml;
extern crate regex;
extern crate ansi_term;
extern crate time;

mod rule;
mod violation;

use git2::Repository;
use git2::DiffFormat;
use git2::DiffOptions;
use git2::Oid;
use git2::Branch;
use git2::BranchType;
use std::str;
use std::collections::HashMap;
use rule::Rule;
use rule::from_toml;
use violation::Violation;
use ansi_term::Colour;


fn main() {

    let rules = from_toml();

    let mut commit_map = HashMap::new();

    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    // create a map that contains every startpoint
    for b in repo.branches(Some(BranchType::Local)).unwrap() {
        let branch = b.expect("unwrap branch");
        let branch_name = branch.0.name().expect("unwrap branch name").expect("unwrap branch name 2").to_string();

        let reference = branch.0.get();

        let oid = reference.target().expect("unwrap branch iod");

        let mut walker = repo.revwalk().expect("unwrap revwalk");
        walker.push(oid.clone()).unwrap();
        walker.set_sorting(git2::SORT_TIME);

        for r in walker {

            let r = r.expect("walker res");

            let commit = repo.find_commit(r.clone()).expect("unwrap commit to find date");

            if !((time::get_time().sec - commit.time().seconds()) <= 60*60*24*7) {
                break;
            }

            if !commit_map.contains_key(&r) {
                commit_map.insert(r, vec![branch_name.clone()]);
            } else {
                commit_map.get_mut(&r).unwrap().push(branch_name.clone());
            }
        }
    }

    let violations = analyze_commits(&rules, &commit_map, &repo);

    for violation in violations {
        println!(
            "[{}] violates in {} on branch {}",
            Colour::Red.bold().paint(violation.get_rule_name()),
            Colour::Red.bold().paint(
                format!("{}", violation.get_oid())
            ),
            commit_map.get(&violation.get_oid()).unwrap().join(",")
        );

        println!("{}", violation.get_line());
        println!("{}", "");
    }

}



fn analyze_commits(rules : &Vec<Rule>, commit_map : &HashMap<Oid, Vec<String>>, repo : &Repository) -> Vec<Violation>
{
    let mut violations = vec![];

    for (oid, branches) in commit_map {

        let commit = repo.find_commit(oid.clone()).expect("unwrap find commit");

        // skip merges
        if commit.parents().len() != 1 {
            continue;
        }

        let parent = commit.parent(0).expect("find parent 0");

        let mut diffopts = DiffOptions::new();
        let diff = repo.diff_tree_to_tree(
            Some(&parent.tree().expect("find parent tree")),
            Some(&commit.tree().expect("find commit tree")),
            Some(&mut diffopts)
        ).unwrap();

        let _ = diff.print(DiffFormat::Patch, |delta, _hunk, line| {
            if line.origin() == '+' {
                return true;
            }

            for rule in rules {
                for branch in branches {

                    let code = str::from_utf8(line.content());

                    // if it's not utf8, may its some kind of binary
                    if code.is_err() {
                        continue;
                    }

                    if rule.statify(
                        delta.new_file().path().expect("find path").to_string_lossy().to_string(),
                        code.expect("find code").to_string(),
                        "author".to_string(),
                        branch.to_string()
                    ) {
                        violations.push(
                            Violation::new(
                                rule.get_name(),
                                oid.clone(),
                                code.expect("find code 2").to_string()
                            )
                        )
                    }
                }
            }

            true
        });

    }

    violations
}
