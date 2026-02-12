use crate::record::Record;
use std::io::Write;

mod boxes;
mod database;
mod generic;
mod handlers;
mod oti;
mod record;
mod sample_entry;

pub(crate) fn doc_attrs(doc: &str) -> Vec<syn::Attribute> {
    doc.lines()
        .map(|line| {
            let line = format!(" {}", line);
            syn::parse_quote!(#[doc = #line])
        })
        .collect()
}

fn main() {
    let mut items: Vec<syn::Item> = Vec::new();

    let database = database::Database::new("../mp4ra.github.io/data");
    let handler_variants_by_description =
        handlers::GenHandlers::new().gen_handlers(&database, &mut items);
    sample_entry::SampleEntryGen::new().gen_sample_entries(
        &database,
        &mut items,
        &handler_variants_by_description,
    );
    boxes::BoxGen::new().gen_boxes(&database, &mut items);
    oti::OtiGen::new().gen_oti(&database, &mut items);

    generic::GenericGenerator::new("BrandCode").gen(
        &database
            .load::<Record>("brands.csv")
            .expect("Failure generating brands entries"),
        &mut items,
    );

    generic::GenericGenerator::new("TrackReferenceCode").gen(
        &database
            .load::<Record>("track-references.csv")
            .expect("Failure generating track-reference entries"),
        &mut items,
    );

    let file = syn::File {
        shebang: None,
        attrs: vec![],
        items,
    };
    let formatted = prettyplease::unparse(&file);

    let mut out = std::fs::File::create("../mp4ra-rust/src/generated.rs").unwrap();
    out.write_all(
        b"// --------\n\
          // WARNING\n\
          // This is generated code.\n\
          // If you need changes, alter the mp4ra_crategen project, not this file.\n\
          // --------\n",
    )
    .expect("failed to write output");
    out.write_all(formatted.as_bytes())
        .expect("failed to write output");
}
