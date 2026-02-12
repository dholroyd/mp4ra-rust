use crate::database::Database;
use crate::doc_attrs;
use crate::record::Record;
use quote::format_ident;
use regex::Captures;

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

    pub(crate) fn gen_boxes(&self, database: &Database, items: &mut Vec<syn::Item>) {
        let mut box_list = database
            .load::<Record>("boxes.csv")
            .expect("Failure generating boxes entries");
        // the code "xml " appears twice, so remove dup (maybe we should keep both descriptions?)
        box_list.dedup_by_key(|bx| bx.code.clone());
        self.gen_boxes_consts(&box_list, items);
    }

    fn gen_boxes_consts(&self, box_list: &[Record], items: &mut Vec<syn::Item>) {
        let mut consts: Vec<syn::ImplItem> = Vec::new();
        for se in box_list {
            let code = &se.code;
            let const_ident = format_ident!("{}", self.create_const_name(code));
            let doc = format!(
                "{}\n\nFourCC: `{}`\n\nSpecification: _{}_",
                se.description, code, se.specification
            );
            let attrs = doc_attrs(&doc);
            let init_expr: syn::Expr =
                syn::parse_str(&format!("BoxCode::new(*b{:?})", code)).unwrap();
            consts.push(syn::parse_quote! {
                #(#attrs)*
                pub const #const_ident: BoxCode = #init_expr;
            });
        }
        items.push(syn::parse_quote! {
            impl BoxCode {
                #(#consts)*
            }
        });
    }
}
