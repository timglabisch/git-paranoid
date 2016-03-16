extern crate git2;

fn main() {

    let repo = match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("Hello, world!");
}
