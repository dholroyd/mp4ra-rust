use crate::database::Database;
use codegen::{Scope, Const};
use regex::Captures;
use crate::record::Record;

pub struct BoxGen {
    bangstart: regex::Regex,
    numstart: regex::Regex,
}

impl BoxGen {
    pub fn new() -> BoxGen {
        BoxGen {
            bangstart: regex::Regex::new("^!").unwrap(),
            numstart: regex::Regex::new("^(\\d)").unwrap(),
        }
    }

    fn create_const_name(&self, text: &str) -> String {
        self.create_base_name(text)
            .to_uppercase()
    }
    fn create_base_name(&self, text: &str) -> String {
        // handle codes starting '!'
        let val = self.bangstart.replace(&text, "compressed_");
        let val = self.numstart.replace(&val, |caps: &Captures| {
            match &caps[1] {
                "1" => "ONE_",
                "2" => "TWO_",
                "3" => "THREE_",
                "4" => "FOUR_",
                "5" => "FIVE_",
                "6" => "SIX_",
                "7" => "SEVEN_",
                "8" => "EIGHT_",
                "9" => "NINE_",
                "0" => "ZERO_",
                _ => unreachable!()
            }
        });
        val.to_string()
    }

    pub(crate) fn gen_boxes(&self, database: &Database, scope: &mut Scope) {
        let mut box_list = database.load::<Record>("boxes.csv").expect("Failure generating boxes entries");
        // the code "xml " appears twice, so remove dup (maybe we should keep both descriptions?)
        box_list.dedup_by_key(|bx| bx.code.clone());
        self.gen_boxes_consts(&box_list, scope);
    }

    fn gen_boxes_consts(&self, box_list: &[Record], scope: &mut Scope) {
        let box_impl = scope.new_impl("BoxCode");
        for se in box_list {
            let code = &se.code;
            let var_name = self.create_const_name(&code);
            let mut con = Const::new(&var_name, "BoxCode", &format!("BoxCode::new(*b{:?})", code));
            con.vis("pub");
            con.doc(&format!("{}\n\nFourCC: `{}`\n\nSpecification: _{}_", se.description, code, se.specification));
            box_impl.push_const(con);
        }
    }
}
