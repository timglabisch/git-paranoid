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
