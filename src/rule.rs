use toml::Parser;
use std::env;
use std::fs;
use std::io::Read;
use toml::Value;
use regex::Regex;

pub fn from_toml() -> Vec<Rule> {

    let mut file_content = String::new();
    fs::File::open(
        format!("{}/{}",
            env::current_dir().unwrap().display(),
            "paranoid.toml"
        )
    ).unwrap().read_to_string(&mut file_content).unwrap();

    let toml_rules = Parser::new(
        &file_content
    ).parse().expect("Wrong TOML");


    let mut buffer = vec![];

    for (key, subtable) in toml_rules.iter() {

        buffer.push(
            Rule::new(
                key.to_string(),
                subtable.lookup("path").expect("path is not given").as_str().expect("path must be a string").to_string(),
                extract_values_from_table(subtable.lookup("code").expect("code is not given")),
                extract_values_from_table(subtable.lookup("ignore_authors").expect("ignore_authors is not given")),
                extract_values_from_table(subtable.lookup("ignore_branches").expect("ignore_branches is not given")),
            )
        );
    }

    buffer
}

fn extract_values_from_table(arr : &Value) -> Vec<String> {
    match arr {
        &Value::Array(ref vs) => {
            let mut buffer = Vec::new();
            for v in vs {
                buffer.push(v.as_str().expect("cant convert to string").to_string());
            }

            buffer
        },
        _ => panic!("wrong type for array")
    }
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    path: Regex,
    code: Vec<Regex>,
    ignore_authors: Vec<Regex>,
    ignore_branches: Vec<Regex>
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
            path: Regex::new(&path).expect("path is not a valid regex"),
            code: Rule::regex_from_array(code),
            ignore_authors: Rule::regex_from_array(ignore_authors),
            ignore_branches: Rule::regex_from_array(ignore_branches)
        }
    }

    fn regex_from_array(arr : Vec<String>) -> Vec<Regex> {
        let mut buffer = vec![];

        for v in arr {
            buffer.push(Regex::new(&v).expect("wrong array regex"));
        }

        buffer
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn statify(
        &self,
        path : String,
        code : String,
        author : String,
        branch : String
    ) -> bool {
        for v in &self.ignore_authors {
            if v.is_match(&author) {
                return false;
            }
        }

        for v in &self.ignore_branches {
            if v.is_match(&branch) {
                return false;
            }
        }

        let mut found_code = false;

        for v in &self.code {
            if v.is_match(&code) {
                found_code = true;
            }
        }

        if self.path.is_match(&path) {
            return true && found_code;
        }

        false
    }
}
