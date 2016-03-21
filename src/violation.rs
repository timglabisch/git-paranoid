use git2::Oid;

pub struct Violation {
    rule_name : String,
    oid : Oid,
    line : String,
}

impl Violation {
    pub fn new(
        rule_name : String,
        oid : Oid,
        line : String
    ) -> Violation {
        Violation {
            rule_name: rule_name,
            oid: oid,
            line: line
        }
    }

    pub fn get_oid(&self) -> Oid {
        self.oid.clone()
    }

    pub fn get_rule_name(&self) -> String {
        self.rule_name.clone()
    }

    pub fn get_line(&self) -> String {
        self.line.clone()
    }
}
