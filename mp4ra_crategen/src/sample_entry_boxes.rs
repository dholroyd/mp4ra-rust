use crate::database::Database;
use crate::doc_attrs;
use crate::record::Record;
use quote::format_ident;
use regex::Captures;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct SampleEntryBox {
    #[serde(with = "crate::record::code_format")]
    code: String,
    description: String,
    handler: String,
    specification: String,
}

pub struct SampleEntryBoxGen {
    bangstart: regex::Regex,
    numstart: regex::Regex,
}

impl SampleEntryBoxGen {
    pub fn new() -> SampleEntryBoxGen {
        SampleEntryBoxGen {
            bangstart: regex::Regex::new("^!").unwrap(),
            numstart: regex::Regex::new("^(\\d)").unwrap(),
        }
    }

    fn create_const_name(&self, text: &str) -> String {
        self.create_base_name(text).trim().to_uppercase()
    }

    fn create_base_name(&self, text: &str) -> String {
        // handle codes starting '!'
        let val = self.bangstart.replace(text, "compressed_");
        let val = self
            .numstart
            .replace(&val, |caps: &Captures| match &caps[1] {
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
                _ => unreachable!(),
            });
        val.to_string()
    }

    pub(crate) fn gen_sample_entry_boxes(
        &self,
        database: &Database,
        items: &mut Vec<syn::Item>,
        handler_variants_by_description: &HashMap<String, String>,
    ) -> Vec<Record> {
        let records = database
            .load::<SampleEntryBox>("sample-entries-boxes.csv")
            .expect("Failure generating sample entry boxes");
        self.gen_handler_function(&records, items, handler_variants_by_description);
        records
            .into_iter()
            .map(|r| Record {
                code: r.code,
                description: r.description,
                specification: r.specification,
            })
            .collect()
    }

    fn gen_handler_function(
        &self,
        records: &[SampleEntryBox],
        items: &mut Vec<syn::Item>,
        handler_variants_by_description: &HashMap<String, String>,
    ) {
        let mut match_arms: Vec<syn::Arm> = Vec::new();
        for se in records {
            let code = &se.code;
            let const_ident = format_ident!("{}", self.create_const_name(code));
            if let Some(handler_var) = handler_variants_by_description.get(&se.handler) {
                let handler_expr: syn::Expr =
                    syn::parse_str(&format!("Some({})", handler_var)).unwrap();
                match_arms.push(syn::parse_quote! {
                    BoxCode::#const_ident => #handler_expr,
                });
            } else {
                eprintln!("No handler for {:?}", &se.handler);
                match_arms.push(syn::parse_quote! {
                    BoxCode::#const_ident => None,
                });
            }
        }
        match_arms.push(syn::parse_quote! {
            _ => None,
        });

        let doc_text = doc_attrs("Return the handler type for a box code that appears inside a sample entry, if one is defined.");

        items.push(syn::parse_quote! {
            #(#doc_text)*
            pub fn sample_entry_box_handler(code: BoxCode) -> Option<HandlerCode> {
                match code {
                    #(#match_arms)*
                }
            }
        });
    }
}
