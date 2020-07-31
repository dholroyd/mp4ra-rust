# mp4ra-rust

Types with associated constants representing code points managed by the
[MPEG4 Registration Authority](https://mp4ra.org/).

This crate has been generated from metadata published by the MP4RA by code in the
[mp4ra-rust project](https://github.com/dholroyd/mp4ra-rust).

This crate is not complete, but the following MP4RA datasets are available so far,

 - [x] `boxes.csv`
 - [ ] `boxes-qt.csv`
 - [ ] `boxes-udta.csv`
 - [x] `brands.csv`
 - [ ] `checksum-types.csv`
 - [ ] `color-types.csv`
 - [ ] `data-references.csv`
 - [ ] `entity-groups.csv`
 - [x] `handlers.csv`
 - [ ] `item-properties.csv`
 - [ ] `item-references.csv`
 - [ ] `item-types.csv`
 - [ ] `multiview-attributes.csv`
 - [x] `oti.csv`
 - [ ] `sample-entries-boxes.csv`
 - [x] `sample-entries.csv`
 - [ ] `sample-entries-qt.csv`
 - [ ] `sample-groups.csv`
 - [ ] `schemes.csv`
 - [ ] `specifications.csv`
 - [ ] `stream-types.csv`
 - [ ] `track-groups.csv`
 - [x] `track-references.csv`
 - [ ] `track-references-qt.csv`
 - [ ] `track-selection.csv`

## struct vs. enum

Today this crate does not use `enum` types to represent each class of identifier.

Initial code did use `enum` types, but this was changed to prevent potential future problems when upgrading this
crate if the underlying specification has been extended to define values previously reserved.

### The `enum` problem
  
I require these types to be able to represent all values currently defined in the spec, plus out-of-spec values that
_might_ come to be defined in future (forward compatibility).  The initial solution was for each enum then included a
`Reserved(T)` or `Other(T)` variant alongside all the spec-defined ones.

So for example we used to have,

```rust
pub enum ObjectTypeIdentification {
    SystemsIsoIec144961,
    // ... lots of other variants ...
    /// Reserved for future use
    Reserved(pub u8),
}
```

The problem with the above model comes if I write code that matches against the `Reserved` variant (which is after all
the point of providing it),

```rust
match oti {
    ObjectTypeIdentification::SystemsIsoIec144961 => work_with_stream_data(),
    // .. maybe other matches ..
    ObjectTypeIdentification::Reserved(0x0b) => do_special_action(),

    _ => action_for_unhandled_object_type(oti),
}
```

...if a future version of the spec adds a definition for `0x0b` and the `enum` gains a specific variant for that
case, then on upgrading this crate an application with the above code will silently break because the `Reserved(0x0b)`
match-arm is no longer called, but the code will still compile just fine.

### Today's `struct` types

Each kind of identifier is now represented by a `struct` type (simple 'newtype' wrapper of a single underlying
id value).

The equivalent to the `enum` from the section above now looks like this,

```rust
struct ObjectTypeIdentification(u8);
impl ObjectTypeIdentification {
    // ...
    pub const SYSTEMS_ISO_IEC_14496_1: ObjectTypeIdentification = ...;
    // ...
}
```

Calling code equivalent to the problematic usage in the section above is now robust in the face of spec
updates,

```rust
match oti {
    ObjectTypeIdentification::SYSTEMS_ISO_IEC_14496_1 => work_with_stream_data(),
    // .. maybe other matches ..
    ObjectTypeIdentification(0x0b) => do_special_action(),

    _ => action_for_unhandled_object_type(oti),
}
```

...on release of a new version of this crate defining an associated constant with value
`ObjectTypeIdentification(0x0b)`, the above code still works exactly the same as when it was written.