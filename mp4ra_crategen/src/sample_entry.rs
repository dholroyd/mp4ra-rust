use serde::Deserialize;
use regex;
use codegen::{Scope, Const};
use crate::database::Database;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct SampleEntry {
    #[serde(with = "crate::record::code_format")]
    code: String,
    description: String,
    handler: String,
    specification: String,
    #[serde(rename = "ObjectType")]
    object_type: Option<String>,
}

pub struct SampleEntryGen {
    plusend: regex::Regex,
    plusin: regex::Regex,
    initdigit: regex::Regex,
}

impl SampleEntryGen {
    pub fn new() -> SampleEntryGen {
        SampleEntryGen {
            plusend: regex::Regex::new("\\+$").unwrap(),
            plusin: regex::Regex::new("\\+").unwrap(),
            initdigit: regex::Regex::new("^\\d").unwrap(),
        }
    }

    fn create_const_name(&self, text: &str) -> String {
        self.create_base_name(text)
            .to_uppercase()
    }

    fn create_base_name(&self, text: &str) -> String {
        let text = text.replace("-", "_");

        // handle codes ending in '+'
        let text = self.plusend.replace(&text, "_plus");

        // handle codes containing internal '+'
        let text = self.plusin.replace(&text, "_plus_");

        // handle initial digit,
        self.initdigit.replace(&text, |caps: &regex::Captures| {
            match &caps[0] {
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
            }
        })
            .to_string()
    }

    pub(crate) fn gen_sample_entries(&self, database: &Database, scope: &mut Scope, handler_variants_by_description: &HashMap<String, String>) {
        let records = database.load::<SampleEntry>("sample-entries.csv").expect("Failure generating sample entries");
        self.gen_sample_entries_handler_impls(&records, scope, handler_variants_by_description);
        self.gen_sample_entries_consts(&records, scope);
    }

    fn gen_sample_entries_handler_impls(&self, records: &[SampleEntry], scope: &mut Scope, handler_variants_by_description: &HashMap<String, String>) {
        let mut sample_entry_impl = codegen::Impl::new("SampleEntryCode");
        let handler_name_fn = sample_entry_impl.new_fn("handler");
        handler_name_fn.vis("pub")
            .ret("Option<HandlerCode>")
            .arg_ref_self()
            .doc("Return the identifier of a handler for this type of sample entry, if a single handler type is defined.  For those _sample entry_ types where MP4RA notes there are 'various' handlers, this method will return `None`");
        handler_name_fn.line("match *self {");
        for se in records {
            let code = &se.code;
            let const_name = self.create_const_name(&code);
            if let Some(handler_var) = handler_variants_by_description.get(&se.handler) {
                handler_name_fn.line(format!("  SampleEntryCode::{} => Some({}),", &const_name, handler_var));
            } else {
                eprintln!("No handler for {:?}", &se.handler);
                handler_name_fn.line(format!("  SampleEntryCode::{} => None,", &const_name));
            }
        }
        handler_name_fn.line("  _ => None,");
        handler_name_fn.line("}");

        scope.push_impl(sample_entry_impl);
    }

    fn gen_sample_entries_consts(&self, records: &[SampleEntry], scope: &mut Scope) {
        let se_impl = scope.new_impl("SampleEntryCode");
        for se in records {
            let code = &se.code;
            let var_const = self.create_const_name(&code);
            let mut con = Const::new(&var_const, "SampleEntryCode", &format!("SampleEntryCode(FourCC(*b{:?}))", code));
            con.vis("pub");
            con.doc(&format!("{}\n\nFourCC: `{}`\n\nSpecification: _{}_", se.description, code, se.specification));
            se_impl.push_const(con);
        }
    }
}