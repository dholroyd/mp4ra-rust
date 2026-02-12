use crate::doc_attrs;
use crate::record::Record;
use quote::format_ident;

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

    pub(crate) fn gen(&self, box_list: &[Record], items: &mut Vec<syn::Item>) {
        self.gen_consts(box_list, items);
    }

    fn gen_consts(&self, records: &[Record], items: &mut Vec<syn::Item>) {
        let type_ident = format_ident!("{}", &self.type_name);
        let mut consts: Vec<syn::ImplItem> = Vec::new();
        for se in records {
            let code = &se.code;
            let const_ident = format_ident!("{}", self.create_const_name(code));
            let doc = format!(
                "{}\n\nFourCC: `{}`\n\nSpecification: _{}_",
                se.description, code, se.specification
            );
            let attrs = doc_attrs(&doc);
            let init_expr: syn::Expr =
                syn::parse_str(&format!("{}::new(*b{:?})", &self.type_name, code)).unwrap();
            consts.push(syn::parse_quote! {
                #(#attrs)*
                pub const #const_ident: #type_ident = #init_expr;
            });
        }
        items.push(syn::parse_quote! {
            impl #type_ident {
                #(#consts)*
            }
        });
    }

    fn create_const_name(&self, text: &str) -> String {
        self.create_base_name(text).trim().to_uppercase()
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
