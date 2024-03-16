//! Types with associated constants representing code points managed by the
//! [MPEG4 Registration Authority](https://mp4ra.org/).
//!
//! This crate has been generated from
//! [metadata published by the MP4RA](https://github.com/mp4ra/mp4ra.github.io/tree/dev/CSV)
//! (by code in the [mp4ra-rust project](https://github.com/dholroyd/mp4ra-rust)).

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, future_incompatible, missing_docs)]

pub use four_cc::FourCC;
use std::fmt;

include!("generated.rs");

/// _Object Type Identification_ values.
///
/// See also,
///  - the [Object Types section on mp4ra.org](http://mp4ra.org/#/object_types)
///
/// ## Conversions to and from `u8`
///
/// Conversion to or from `u8` will work for any value.
///
/// ```rust
/// # use mp4ra_rust::ObjectTypeIdentifier;
/// assert_eq!(ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_MAIN_PROFILE, 0x61.into());
/// assert_eq!(0x61u8, ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_MAIN_PROFILE.into());
/// ```
///
/// ## `Debug` implementation
///
/// The implementation of `Debug` tries to be helpful,
///
/// ```rust
/// # use mp4ra_rust::ObjectTypeIdentifier;
/// assert_eq!(
///     "VISUAL_ISO_IEC_13818_2_MAIN_PROFILE(0x61)",
///     format!("{:?}", ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_MAIN_PROFILE)
/// );
/// ```
///
/// ## Identifier values without provided constants
///
/// Constants are not provided for the range of values that the spec notes are _reserved_, but
/// you are still able to create ObjectTypeIdentification instances taking these values for the
/// sake of forward compatibility.
///
/// The spec also notes that some identifier values have been withdrawn and are deprecated.
/// Constants are not provided for these either.
///
/// Finally, some values are marked for _user private_ use, meaning that future versions of the
/// spec will avoid using these values.
///
/// While there are no specific constants for any of these, the implementation of `Debug` still
/// gives them special treatment,
///
/// ```rust
/// # use mp4ra_rust::ObjectTypeIdentifier;
/// assert_eq!("RESERVED(0x0b)", format!("{:?}", ObjectTypeIdentifier::from(0x0b)));
/// assert_eq!("WITHDRAWN(0xa5)", format!("{:?}", ObjectTypeIdentifier::from(0xa5)));
/// assert_eq!("USER_PRIVATE(0xc0)", format!("{:?}", ObjectTypeIdentifier::from(0xc0)))
/// ```
///
/// ## Defining constants
///
/// You may define your own constant values.  For example for application specific values that the
/// spec puts in the _user private_ range.  Note however that you can't change the output which
/// the `Debug` implementation produces for these values.
///
/// ```rust
/// # use mp4ra_rust::ObjectTypeIdentifier;
/// # let other_oti = ObjectTypeIdentifier::AFX_STREAM;
/// const MY_SPECIAL_OBJECT_TYPE: ObjectTypeIdentifier = ObjectTypeIdentifier(0xc1);
/// match other_oti {
///     MY_SPECIAL_OBJECT_TYPE => println!("special object"),
///     _ => println!("other type"),
/// }
/// // can't customise this "USER_PRIVATE" text,
/// assert_eq!("USER_PRIVATE(0xc1)", format!("{:?}", MY_SPECIAL_OBJECT_TYPE))
/// ```
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ObjectTypeIdentifier(pub u8);
impl From<ObjectTypeIdentifier> for u8 {
    fn from(val: ObjectTypeIdentifier) -> Self {
        val.0
    }
}
impl From<u8> for ObjectTypeIdentifier {
    fn from(val: u8) -> Self {
        ObjectTypeIdentifier(val)
    }
}

/// Codes identifying _handlers_, that declare track-types.
///
/// Commonly used values,
///  - [`HandlerCode::VIDE`](#associatedconstant.VIDE) - the code for video
///  - [`HandlerCode::SOUN`](#associatedconstant.SOUN) - the code for audio
///  - [`HandlerCode::TEXT`](#associatedconstant.TEXT) - the code for timed-text
///
/// See also,
///  - the [Handlers section on mp4ra.org](http://mp4ra.org/#/handlers)
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct HandlerCode(pub FourCC);
impl HandlerCode {
    /// Construct a `HandlerCode` from its four-cc value
    pub const fn new(code: [u8; 4]) -> HandlerCode {
        HandlerCode(FourCC(code))
    }
}
impl From<HandlerCode> for FourCC {
    fn from(val: HandlerCode) -> Self {
        val.0
    }
}
impl From<FourCC> for HandlerCode {
    fn from(val: FourCC) -> Self {
        HandlerCode(val)
    }
}

/// Codes identifying _sample entries_ registered with ISO.
///
/// The `handler()` method will give you the code for the handler specified for this kind of
/// _sample entry_, if a single handler is defined for it by MP4RA.
///
/// For example, to test if the _sample entry_ is audio or video,
///
/// ```rust
/// # use mp4ra_rust::SampleEntryCode;
/// use mp4ra_rust::HandlerCode;
///
/// let sample_entry1 = SampleEntryCode::AVC1;
/// assert_eq!(sample_entry1.handler(), Some(HandlerCode::VIDE));
///
/// let sample_entry2 = SampleEntryCode::MP4A;
/// assert_eq!(sample_entry2.handler(), Some(HandlerCode::SOUN));
/// ```
///
/// See also,
///  - the [Sample Entry Codes section on mp4ra.org](http://mp4ra.org/#/qtcodecs)
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct SampleEntryCode(pub FourCC);
impl SampleEntryCode {
    /// Construct a `SampleEntryCode` from its four-cc value
    pub const fn new(code: [u8; 4]) -> SampleEntryCode {
        SampleEntryCode(FourCC(code))
    }
}
impl From<SampleEntryCode> for FourCC {
    fn from(val: SampleEntryCode) -> Self {
        val.0
    }
}
impl From<FourCC> for SampleEntryCode {
    fn from(val: FourCC) -> Self {
        SampleEntryCode(val)
    }
}

/// Codes for ISO-family _box_ entries within an MP4 file.
///
/// See also,
///  - the [Boxes section on mp4ra.org](http://mp4ra.org/#/atoms)
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct BoxCode(pub FourCC);
impl BoxCode {
    /// Construct a `BoxCode` from its four-cc value
    pub const fn new(code: [u8; 4]) -> BoxCode {
        BoxCode(FourCC(code))
    }
}
impl From<BoxCode> for FourCC {
    fn from(val: BoxCode) -> Self {
        val.0
    }
}
impl From<FourCC> for BoxCode {
    fn from(val: FourCC) -> Self {
        BoxCode(val)
    }
}

/// Codes for MPEG4 _brands_, identifying with which specification some MP4 data is compatible .
///
/// See also,
///  - the [Brands section on mp4ra.org](http://mp4ra.org/#/brands)
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct BrandCode(pub FourCC);
impl BrandCode {
    /// Construct a `BrandCode` from its four-cc value
    pub const fn new(code: [u8; 4]) -> BrandCode {
        BrandCode(FourCC(code))
    }
}
impl From<BrandCode> for FourCC {
    fn from(val: BrandCode) -> Self {
        val.0
    }
}
impl From<FourCC> for BrandCode {
    fn from(val: FourCC) -> Self {
        BrandCode(val)
    }
}

/// Codes for _track reference link types_.
///
/// See also,
///  - the [Track References section on mp4ra.org](http://mp4ra.org/#/track_references)
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct TrackReferenceCode(pub FourCC);
impl TrackReferenceCode {
    /// Construct a `TrackReferenceCode` from its four-cc value
    pub const fn new(code: [u8; 4]) -> TrackReferenceCode {
        TrackReferenceCode(FourCC(code))
    }
}
impl From<TrackReferenceCode> for FourCC {
    fn from(val: TrackReferenceCode) -> Self {
        val.0
    }
}
impl From<FourCC> for TrackReferenceCode {
    fn from(val: FourCC) -> Self {
        TrackReferenceCode(val)
    }
}
