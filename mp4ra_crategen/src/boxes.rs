use crate::database::Database;
use codegen::{Scope, Const};
use crate::record::Record;

pub struct BoxGen {
    bangstart: regex::Regex,
}

impl BoxGen {
    pub fn new() -> BoxGen {
        BoxGen {
            bangstart: regex::Regex::new("^!").unwrap(),
        }
    }

    fn create_const_name(&self, text: &str) -> String {
        self.create_base_name(text)
            .to_uppercase()
    }
    fn create_base_name(&self, text: &str) -> String {
        // handle codes starting '!'
        self.bangstart.replace(&text, "compressed_")
            .to_string()
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
            let mut con = Const::new(&var_name, "BoxCode", &format!("BoxCode(FourCC(*b{:?}))", code));
            con.vis("pub");
            con.doc(&format!("{}\n\nFourCC: `{}`\n\nSpecification: _{}_", se.description, code, se.specification));
            box_impl.push_const(con);
        }
    }
}