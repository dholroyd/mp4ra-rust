use crate::database::Database;
use crate::doc_attrs;
use crate::record::Record;
use quote::format_ident;
use regex::Captures;
use std::collections::HashSet;

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

    pub(crate) fn gen_boxes(
        &self,
        database: &Database,
        items: &mut Vec<syn::Item>,
        extra_records: &[Record],
    ) {
        // Load box definitions from all three CSV data files. The files share
        // the same schema but have overlapping entries:
        //  - boxes.csv marks some codes as "Reserved" that have full definitions
        //    in boxes-qt.csv (e.g. clip, crgn, ctab, matt, pnot, etc.)
        //  - boxes-udta.csv and boxes-qt.csv share a few codes (albm, auth,
        //    clsf, cprt) with minor description differences
        // We insert extra records first, then load the CSV files in order so
        // that later sources overwrite earlier ones, ensuring the main CSV
        // files remain authoritative.  We key by const name (uppercased) to
        // also handle case-collisions like "CoLL" vs "coll".
        let mut boxes_by_const = std::collections::HashMap::<String, Record>::new();

        let sample_entry_box_consts: HashSet<String> = extra_records
            .iter()
            .map(|r| self.create_const_name(&r.code))
            .collect();

        for record in extra_records {
            boxes_by_const.insert(self.create_const_name(&record.code), record.clone());
        }

        for csv in ["boxes.csv", "boxes-qt.csv", "boxes-udta.csv"] {
            let records = database
                .load::<Record>(csv)
                .unwrap_or_else(|e| panic!("Failure loading {csv}: {e}"));
            for record in records {
                boxes_by_const.insert(self.create_const_name(&record.code), record);
            }
        }

        let mut box_list: Vec<Record> = boxes_by_const.into_values().collect();
        box_list.sort_by(|a, b| a.code.cmp(&b.code));

        self.gen_boxes_consts(&box_list, &sample_entry_box_consts, items);
    }

    fn gen_boxes_consts(
        &self,
        box_list: &[Record],
        sample_entry_box_consts: &HashSet<String>,
        items: &mut Vec<syn::Item>,
    ) {
        let mut consts: Vec<syn::ImplItem> = Vec::new();
        for se in box_list {
            let code = &se.code;
            let const_name = self.create_const_name(code);
            let const_ident = format_ident!("{}", const_name);
            let mut doc = format!(
                "{}\n\nFourCC: `{}`\n\nSpecification: _{}_",
                se.description, code, se.specification
            );
            if sample_entry_box_consts.contains(&const_name) {
                doc.push_str("\n\nThis box appears inside sample entries. Use [`sample_entry_box_handler`] to look up the handler type.");
            }
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
