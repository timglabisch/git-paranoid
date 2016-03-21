extern crate git2;
extern crate toml;
extern crate regex;
extern crate ansi_term;

mod rule;
mod violation;

use git2::Repository;
use git2::DiffFormat;
use git2::DiffOptions;
use git2::Oid;
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

    for b in repo.branches(None).unwrap() {

        let branch = b.unwrap();
        let branch_name = branch.0.name().unwrap().unwrap().to_string();

        let reference = branch.0.get();

        let oid = reference.target().unwrap();

        add_commit_to_commit_map_by_reference(oid.clone(), &mut commit_map, branch_name, &repo);
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

fn add_commit_to_commit_map_by_reference(
    oid : Oid,
    commit_map : &mut HashMap<Oid, Vec<String>>,
    branch_name : String,
    repo : &Repository
)
{

    if !commit_map.contains_key(&oid) {
        commit_map.insert(oid, vec![branch_name.clone()]);
    } else {
        commit_map.get_mut(&oid).unwrap().push(branch_name.clone());
    }

    let commit = repo.find_commit(oid.clone()).unwrap();

    for pid in commit.parent_ids() {
        add_commit_to_commit_map_by_reference(pid.clone(), commit_map, branch_name.clone(), &repo);
    }
}

fn analyze_commits(rules : &Vec<Rule>, commit_map : &HashMap<Oid, Vec<String>>, repo : &Repository) -> Vec<Violation>
{
    let mut violations = vec![];

    for (oid, branches) in commit_map {

        let commit = repo.find_commit(oid.clone()).unwrap();

        for parent in commit.parents() {
            let mut diffopts = DiffOptions::new();
            let diff = repo.diff_tree_to_tree(
                Some(&parent.tree().unwrap()),
                Some(&commit.tree().unwrap()),
                Some(&mut diffopts)
            ).unwrap();

            let _ = diff.print(DiffFormat::Patch, |delta, _hunk, line| {
                if line.origin() == '+' {
                    return true;
                }

                for rule in rules {
                    for branch in branches {
                        if rule.statify(
                            delta.new_file().path().unwrap().to_string_lossy().to_string(),
                            str::from_utf8(line.content()).unwrap().to_string(),
                            "author".to_string(),
                            branch.to_string()
                        ) {
                            violations.push(
                                Violation::new(
                                    rule.get_name(),
                                    oid.clone(),
                                    str::from_utf8(line.content()).unwrap().to_string()
                                )
                            )
                        }
                    }
                }

                true
            });
        }
    }

    violations
}
