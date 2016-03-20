use toml::Parser;
use std::env;
use std::fs;
use std::io::Read;

pub fn from_toml() -> Vec<Rule> {

    let mut file_content = String::new();
    fs::File::open(
        format!("{}/{}",
            env::current_dir().unwrap().display(),
            "paranoid.toml"
        )
    ).unwrap().read_to_string(&mut file_content).unwrap();

    println!("File Content: {}", file_content);

    let value = Parser::new(
        &file_content
    ).parse().expect("Wrong TOML");

    vec![
        Rule::new(
            "foo".to_string(),
            "foo".to_string(),
            vec!("foo".to_string()),
            vec!("foo".to_string()),
            vec!("foo".to_string()),
        )
    ]
}

pub struct Rule {
    name: String,
    path: String,
    code: Vec<String>,
    ignore_authors: Vec<String>,
    ignore_branches: Vec<String>
}

impl Rule {
    pub fn new(
        name : String,
        path : String,
        code : Vec<String>,
        ignore_authors : Vec<String>,
        ignore_branches: Vec<String>
    ) -> Rule {
        Rule {
            name: name,
            path: path,
            code: code,
            ignore_authors: ignore_authors,
            ignore_branches: ignore_branches
        }
    }
}
