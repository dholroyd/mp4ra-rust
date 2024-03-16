use crate::database::Database;
use crate::record::Record;
use codegen::{Const, Scope};
use convert_case::{Case, Casing};
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
        // TODO: the naming strategy is to transform the description given in MP4RA data into
        //       a valid const-name.  This means we break if MP4RA reword a description (would be
        //       within their rights to do so).  For the sake of stability, we probably need to
        //       supply our own names, or to lobby MP4RA for stable values.  For now, we hope that
        //       they don't make changes; we should automatically asset this somehow!
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

    pub(crate) fn gen_oti(&self, database: &Database, scope: &mut Scope) {
        let oti_list = database
            .load::<Record>("oti.csv")
            .expect("Failure generating boxes entries");
        self.gen_oti_consts(&oti_list, scope);
        self.gen_debug(&oti_list, scope);
    }
    fn gen_oti_consts(&self, oti_list: &[Record], scope: &mut Scope) {
        let oti_impl = scope.new_impl("ObjectTypeIdentifier");
        for oti in oti_list {
            let code = &oti.code;
            let const_name = self.create_const_name(oti);
            if const_name == "USER_PRIVATE" || const_name == "WITHDRAWN" {
                continue;
            }
            let mut c = Const::new(
                &const_name,
                "ObjectTypeIdentifier",
                &format!("ObjectTypeIdentifier(0x{})", code),
            );
            c.vis("pub");
            let desc = self.strip_footnote(&oti.description);
            let mut doc = format!("{}\n\nType value: `0x{}`", desc, code);
            if !oti.specification.is_empty() {
                write!(&mut doc, "\n\nSpecification: _{}_", oti.specification).unwrap();
            }
            c.doc(&doc);
            oti_impl.push_const(c);
        }
    }

    fn gen_debug(&self, records: &[Record], scope: &mut Scope) {
        let mut debug_impl = codegen::Impl::new("ObjectTypeIdentifier");
        debug_impl.impl_trait("fmt::Debug");
        let fmt_fn = debug_impl.new_fn("fmt");
        fmt_fn
            .ret("fmt::Result")
            .arg_ref_self()
            .arg("f", "&mut fmt::Formatter<'_>");
        fmt_fn.line("let label = match *self {");
        for oti in records {
            let code = CodeRange::try_from(oti.code.as_str())
                .unwrap_or_else(|_| panic!("Unexpected OTI {:?}", &oti.code));

            let const_name = self.create_const_name(oti);
            if const_name == "WITHDRAWN" {
                fmt_fn.line(format!(
                    "    {}(0x{}) => \"WITHDRAWN\",",
                    "ObjectTypeIdentifier", &oti.code
                ));
            } else {
                match code {
                    CodeRange::Single(_s) => {
                        fmt_fn.line(format!(
                            "    {}::{} => {:?},",
                            "ObjectTypeIdentifier", &const_name, &const_name
                        ));
                    }
                    CodeRange::Range(s, e) => {
                        fmt_fn.line(format!(
                            "    {}({:#x}..={:#x}) => {:?},",
                            "ObjectTypeIdentifier", s, e, &const_name
                        ));
                    }
                }
            }
        }
        fmt_fn.line(format!(
            "    {}(_) => \"RESERVED\",",
            "ObjectTypeIdentifier"
        ));
        fmt_fn.line("};");
        fmt_fn.line("write!(f, \"{}({:#04x})\", label, self.0)");

        scope.push_impl(debug_impl);
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
