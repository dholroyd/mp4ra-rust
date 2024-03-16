use crate::database::Database;
use crate::record::TypedRecord;
use codegen::{Const, Scope};
use std::collections::HashMap;

pub struct GenHandlers {
    initdigit: regex::Regex,
}
impl GenHandlers {
    pub fn new() -> GenHandlers {
        GenHandlers {
            initdigit: regex::Regex::new("^\\d").unwrap(),
        }
    }
    fn create_const_name(&self, text: &str) -> String {
        self.create_base_name(text).to_uppercase()
    }
    fn create_base_name(&self, text: &str) -> String {
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

    pub(crate) fn gen_handlers(
        &self,
        database: &Database,
        scope: &mut Scope,
    ) -> HashMap<String, String> {
        let handler_list = database
            .load::<TypedRecord>("handlers.csv")
            .expect("Failure generating boxes entries");
        let mut handler_variants_by_description = HashMap::new();
        self.gen_handlers_mapping(&handler_list, &mut handler_variants_by_description);
        self.gen_handlers_consts(&handler_list, scope);

        handler_variants_by_description
    }

    pub(crate) fn gen_handlers_mapping(
        &self,
        handler_list: &[TypedRecord],
        handler_variants_by_description: &mut HashMap<String, String>,
    ) {
        for handler in handler_list {
            let code = &handler.code;
            let const_name = self.create_const_name(&code);
            handler_variants_by_description.insert(
                handler.description.clone(),
                format!("HandlerCode::{}", const_name),
            );
        }
    }
    fn gen_handlers_consts(&self, box_list: &[TypedRecord], scope: &mut Scope) {
        let handler_impl = scope.new_impl("HandlerCode");
        for se in box_list {
            let code = &se.code;
            let var_name = self.create_const_name(&code);
            let mut con = Const::new(
                &var_name,
                "HandlerCode",
                &format!("HandlerCode::new(*b{:?})", code),
            );
            con.vis("pub");
            con.doc(&format!(
                "{}\n\nFourCC: `{}`\n\nSpecification: _{}_",
                se.description, code, se.specification
            ));
            handler_impl.push_const(con);
        }
    }
}
