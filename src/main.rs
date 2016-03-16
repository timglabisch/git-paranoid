extern crate git2;

use git2::Repository;

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
    }

    println!("Hello, world!");
}
