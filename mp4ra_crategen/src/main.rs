use crate::record::Record;
use codegen::Scope;
use std::io::Write;

mod boxes;
mod database;
mod generic;
mod handlers;
mod oti;
mod record;
mod sample_entry;

fn main() {
    let mut scope = Scope::new();
    scope.raw(
        "// --------
// WARNING
// This is generated code.
// If you need changes, alter the format-identifier_crategen project, not this file.
// --------",
    );
    let database = database::Database::new("../mp4ra.github.io/data");
    let handler_variants_by_description =
        handlers::GenHandlers::new().gen_handlers(&database, &mut scope);
    sample_entry::SampleEntryGen::new().gen_sample_entries(
        &database,
        &mut scope,
        &handler_variants_by_description,
    );
    boxes::BoxGen::new().gen_boxes(&database, &mut scope);
    oti::OtiGen::new().gen_oti(&database, &mut scope);

    generic::GenericGenerator::new("BrandCode").gen(
        &database
            .load::<Record>("brands.csv")
            .expect("Failure generating brands entries"),
        &mut scope,
    );

    generic::GenericGenerator::new("TrackReferenceCode").gen(
        &database
            .load::<Record>("track-references.csv")
            .expect("Failure generating track-reference entries"),
        &mut scope,
    );

    let mut out = std::fs::File::create("../mp4ra-rust/src/generated.rs").unwrap();
    out.write_all(scope.to_string().as_bytes())
        .expect("failed to write output");
}
