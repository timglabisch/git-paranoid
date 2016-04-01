use git2::Oid;

pub struct Violation {
    rule_name : String,
    oid : Oid,
    branches : Vec<String>,
    code : String
}

impl Violation {
    pub fn new(
        rule_name : String,
        oid : Oid,
        branches : Vec<String>,
        code : String
    ) -> Violation {
        Violation {
            rule_name: rule_name,
            oid: oid,
            branches: branches,
            code: code
        }
    }

    pub fn get_oid(&self) -> Oid {
        self.oid.clone()
    }

    pub fn get_rule_name(&self) -> String {
        self.rule_name.clone()
    }

    pub fn get_branches(&self) -> Vec<String> {
        self.branches.clone()
    }

    pub fn get_code(&self) -> String {
        self.code.clone()
    }
}
