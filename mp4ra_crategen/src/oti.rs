use crate::database::Database;
use crate::doc_attrs;
use crate::record::Record;
use convert_case::{Case, Casing};
use quote::format_ident;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::fmt::Write;
use std::num::ParseIntError;

pub struct OtiGen {
    digit_start: regex::Regex,
    footnote_ref: regex::Regex,
}

impl OtiGen {
    pub fn new() -> OtiGen {
        OtiGen {
            digit_start: regex::Regex::new("^\\d").unwrap(),
            footnote_ref: regex::Regex::new(r#"\([^)]+\)"#).unwrap(),
        }
    }

    fn create_const_name(&self, oti: &Record) -> String {
        self.create_base_name(oti)
            .to_case(Case::UpperSnake)
            .to_string()
    }
    fn create_base_name(&self, oti: &Record) -> String {
        if oti.specification == "Deprecated" {
            return "Withdrawn".to_string();
        }
        // handle initial digit, as in "3GPP2 Compact Multimedia Format"
        let text = self
            .digit_start
            .replace(&oti.description, |caps: &regex::Captures| match &caps[0] {
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
            .to_string();

        let text = self
            .strip_footnote(&text)
            .replace(['/', '.', '|', '+'], "_");

        if oti.code == "02" {
            format!("{}BifsV2Config", text.trim())
        } else {
            text.trim().to_string()
        }
    }

    fn strip_footnote(&self, text: &str) -> String {
        self.footnote_ref.replace(text, "").trim().to_string()
    }

    pub(crate) fn gen_oti(&self, database: &Database, items: &mut Vec<syn::Item>) {
        let oti_list = database
            .load::<Record>("oti.csv")
            .expect("Failure generating boxes entries");
        self.gen_oti_consts(&oti_list, items);
        self.gen_debug(&oti_list, items);
    }
    fn gen_oti_consts(&self, oti_list: &[Record], items: &mut Vec<syn::Item>) {
        let mut consts: Vec<syn::ImplItem> = Vec::new();
        for oti in oti_list {
            let code = &oti.code;
            let const_name = self.create_const_name(oti);
            if const_name == "USER_PRIVATE" || const_name == "WITHDRAWN" {
                continue;
            }
            let const_ident = format_ident!("{}", const_name);
            let desc = self.strip_footnote(&oti.description);
            let mut doc = format!("{}\n\nType value: `0x{}`", desc, code);
            if !oti.specification.is_empty() {
                write!(&mut doc, "\n\nSpecification: _{}_", oti.specification).unwrap();
            }
            let attrs = doc_attrs(&doc);
            let init_expr: syn::Expr =
                syn::parse_str(&format!("ObjectTypeIdentifier(0x{})", code)).unwrap();
            consts.push(syn::parse_quote! {
                #(#attrs)*
                pub const #const_ident: ObjectTypeIdentifier = #init_expr;
            });
        }
        items.push(syn::parse_quote! {
            impl ObjectTypeIdentifier {
                #(#consts)*
            }
        });
    }

    fn gen_debug(&self, records: &[Record], items: &mut Vec<syn::Item>) {
        let mut match_arms: Vec<syn::Arm> = Vec::new();
        for oti in records {
            let code = CodeRange::try_from(oti.code.as_str())
                .unwrap_or_else(|_| panic!("Unexpected OTI {:?}", &oti.code));

            let const_name = self.create_const_name(oti);
            if const_name == "WITHDRAWN" {
                let arm: syn::Arm = syn::parse_str(&format!(
                    "ObjectTypeIdentifier(0x{}) => \"WITHDRAWN\",",
                    &oti.code
                ))
                .unwrap();
                match_arms.push(arm);
            } else {
                match code {
                    CodeRange::Single(_s) => {
                        let const_ident = format_ident!("{}", &const_name);
                        let name_str = &const_name;
                        match_arms.push(syn::parse_quote! {
                            ObjectTypeIdentifier::#const_ident => #name_str,
                        });
                    }
                    CodeRange::Range(s, e) => {
                        let arm: syn::Arm = syn::parse_str(&format!(
                            "ObjectTypeIdentifier({:#x}..={:#x}) => {:?},",
                            s, e, &const_name
                        ))
                        .unwrap();
                        match_arms.push(arm);
                    }
                }
            }
        }
        match_arms.push(syn::parse_quote! {
            ObjectTypeIdentifier(_) => "RESERVED",
        });

        items.push(syn::parse_quote! {
            impl fmt::Debug for ObjectTypeIdentifier {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let label = match *self {
                        #(#match_arms)*
                    };
                    write!(f, "{}({:#04x})", label, self.0)
                }
            }
        });
    }
}

enum CodeRange {
    Single(u8),
    Range(u8, u8),
}

impl TryFrom<&str> for CodeRange {
    type Error = ParseIntError;

    fn try_from(val: &str) -> Result<CodeRange, Self::Error> {
        let vals: Result<Vec<u8>, ParseIntError> = val
            .splitn(2, '-')
            .map(str::trim)
            .map(|s| u8::from_str_radix(s, 16))
            .collect();
        let vals = vals?;
        Ok(if vals.len() == 2 {
            CodeRange::Range(vals[0], vals[1])
        } else {
            CodeRange::Single(vals[0])
        })
    }
}
impl Display for CodeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeRange::Single(s) => write!(f, "{:#x}", s),
            CodeRange::Range(s, e) => write!(f, "{:#x}..={:#x}", s, e),
        }
    }
}
