use crate::record::Record;
use codegen::{Const, Scope};

pub(crate) struct GenericGenerator {
    type_name: String,
    bangstart: regex::Regex,
    initdigit: regex::Regex,
}

impl GenericGenerator {
    pub(crate) fn new(type_name: &str) -> GenericGenerator {
        GenericGenerator {
            type_name: type_name.to_string(),
            bangstart: regex::Regex::new("^!").unwrap(),
            initdigit: regex::Regex::new("^\\d").unwrap(),
        }
    }

    pub(crate) fn gen(&self, box_list: &[Record], scope: &mut Scope) {
        self.gen_consts(box_list, scope);
    }

    fn gen_consts(&self, records: &[Record], scope: &mut Scope) {
        let my_impl = scope.new_impl(&self.type_name);
        for se in records {
            let code = &se.code;
            let var_name = self.create_const_name(code);
            let mut con = Const::new(
                &var_name,
                &self.type_name,
                &format!("{}::new(*b{:?})", &self.type_name, code),
            );
            con.vis("pub");
            con.doc(&format!(
                "{}\n\nFourCC: `{}`\n\nSpecification: _{}_",
                se.description, code, se.specification
            ));
            my_impl.push_const(con);
        }
    }

    fn create_const_name(&self, text: &str) -> String {
        self.create_base_name(text).to_uppercase()
    }
    fn create_base_name(&self, text: &str) -> String {
        // handle codes starting '!'
        let text = self.bangstart.replace(text, "compressed_").to_string();
        // handle initial digit,
        self.initdigit
            .replace(&text, |caps: &regex::Captures| match &caps[0] {
                "1" => "One_",
                "2" => "Two_",
                "3" => "Three_",
                "4" => "Four_",
                "5" => "Five_",
                "6" => "Six_",
                "7" => "Seven_",
                "8" => "Eight_",
                "9" => "Nine_",
                "0" => "Ten_",
                _ => panic!("unexpected {:?}", &caps[0]),
            })
            .to_string()
    }
}
