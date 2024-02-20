// --------
// WARNING
// This is generated code.
// If you need changes, alter the format-identifier_crategen project, not this file.
// --------

impl HandlerCode {
    /// 3GPP Scene Description
    /// 
    /// FourCC: `3gsd`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GSD: HandlerCode = HandlerCode(FourCC(*b"3gsd"));

    /// Auxiliary video
    /// 
    /// FourCC: `auxv`
    /// 
    /// Specification: _ISO_
    pub const AUXV: HandlerCode = HandlerCode(FourCC(*b"auxv"));

    /// Avid Metadata
    /// 
    /// FourCC: `avmd`
    /// 
    /// Specification: _Avid_
    pub const AVMD: HandlerCode = HandlerCode(FourCC(*b"avmd"));

    /// Closed Caption
    /// 
    /// FourCC: `clcp`
    /// 
    /// Specification: _Apple_
    pub const CLCP: HandlerCode = HandlerCode(FourCC(*b"clcp"));

    /// CPCM Auxiliary Metadata
    /// 
    /// FourCC: `cpad`
    /// 
    /// Specification: _DVB_
    pub const CPAD: HandlerCode = HandlerCode(FourCC(*b"cpad"));

    /// ClockReferenceStream
    /// 
    /// FourCC: `crsm`
    /// 
    /// Specification: _MPEG-4_
    pub const CRSM: HandlerCode = HandlerCode(FourCC(*b"crsm"));

    /// DVB Mandatory Metadata
    /// 
    /// FourCC: `dmbd`
    /// 
    /// Specification: _DVB_
    pub const DMBD: HandlerCode = HandlerCode(FourCC(*b"dmbd"));

    /// TV-Anytime Metadata, according to DVB specifications
    /// 
    /// FourCC: `dtva`
    /// 
    /// Specification: _DVB_
    pub const DTVA: HandlerCode = HandlerCode(FourCC(*b"dtva"));

    /// withdrawn, unused, do not use (was Dolby Vision Metadata)
    /// 
    /// FourCC: `dvmd`
    /// 
    /// Specification: _Deprecated_
    pub const DVMD: HandlerCode = HandlerCode(FourCC(*b"dvmd"));

    /// Font
    /// 
    /// FourCC: `fdsm`
    /// 
    /// Specification: _MPEG-4_
    pub const FDSM: HandlerCode = HandlerCode(FourCC(*b"fdsm"));

    /// General MPEG-4 systems streams (without specific handler)
    /// 
    /// FourCC: `gesm`
    /// 
    /// Specification: _see (1) below_
    pub const GESM: HandlerCode = HandlerCode(FourCC(*b"gesm"));

    /// Subtitle Graphics
    /// 
    /// FourCC: `GRAP`
    /// 
    /// Specification: _Sony_
    pub const GRAP: HandlerCode = HandlerCode(FourCC(*b"GRAP"));

    /// Hint
    /// 
    /// FourCC: `hint`
    /// 
    /// Specification: _ISO_
    pub const HINT: HandlerCode = HandlerCode(FourCC(*b"hint"));

    /// Hipix Rich Picture Format
    /// 
    /// FourCC: `hpix`
    /// 
    /// Specification: _Hipix_
    pub const HPIX: HandlerCode = HandlerCode(FourCC(*b"hpix"));

    /// ID3 version 2 meta-data handler (meta box)
    /// 
    /// FourCC: `ID32`
    /// 
    /// Specification: _id3v2_
    pub const ID32: HandlerCode = HandlerCode(FourCC(*b"ID32"));

    /// DVB IPDC ESG Metadata
    /// 
    /// FourCC: `ipdc`
    /// 
    /// Specification: _DVB_
    pub const IPDC: HandlerCode = HandlerCode(FourCC(*b"ipdc"));

    /// IPMP Stream
    /// 
    /// FourCC: `ipsm`
    /// 
    /// Specification: _MPEG-4_
    pub const IPSM: HandlerCode = HandlerCode(FourCC(*b"ipsm"));

    /// MPEG7Stream
    /// 
    /// FourCC: `m7sm`
    /// 
    /// Specification: _MPEG-4_
    pub const M7SM: HandlerCode = HandlerCode(FourCC(*b"m7sm"));

    /// Metadata
    /// 
    /// FourCC: `meta`
    /// 
    /// Specification: _ISO_
    pub const META: HandlerCode = HandlerCode(FourCC(*b"meta"));

    /// MPEG-J Stream
    /// 
    /// FourCC: `mjsm`
    /// 
    /// Specification: _MPEG-4_
    pub const MJSM: HandlerCode = HandlerCode(FourCC(*b"mjsm"));

    /// MPEG-21 Digital item
    /// 
    /// FourCC: `mp21`
    /// 
    /// Specification: _MPEG-21_
    pub const MP21: HandlerCode = HandlerCode(FourCC(*b"mp21"));

    /// MPEG-7 (binary meta-data)
    /// 
    /// FourCC: `mp7b`
    /// 
    /// Specification: _ISO_
    pub const MP7B: HandlerCode = HandlerCode(FourCC(*b"mp7b"));

    /// MPEG-7 (textual meta-data)
    /// 
    /// FourCC: `mp7t`
    /// 
    /// Specification: _ISO_
    pub const MP7T: HandlerCode = HandlerCode(FourCC(*b"mp7t"));

    /// MPD contained in a metabox
    /// 
    /// FourCC: `mpd `
    /// 
    /// Specification: _3GPP_
    pub const MPD : HandlerCode = HandlerCode(FourCC(*b"mpd "));

    /// MPD link contained in a metabox
    /// 
    /// FourCC: `mpdl`
    /// 
    /// Specification: _3GPP_
    pub const MPDL: HandlerCode = HandlerCode(FourCC(*b"mpdl"));

    /// QuickTime MPEG track handler
    /// 
    /// FourCC: `MPEG`
    /// 
    /// Specification: _Apple_
    pub const MPEG: HandlerCode = HandlerCode(FourCC(*b"MPEG"));

    /// QuickTime Music track handler
    /// 
    /// FourCC: `musi`
    /// 
    /// Specification: _Apple_
    pub const MUSI: HandlerCode = HandlerCode(FourCC(*b"musi"));

    /// Non-Real Time Metadata (XAVC Format)
    /// 
    /// FourCC: `nrtm`
    /// 
    /// Specification: _Sony_
    pub const NRTM: HandlerCode = HandlerCode(FourCC(*b"nrtm"));

    /// No handling required (meta-data)
    /// 
    /// FourCC: `null`
    /// 
    /// Specification: _ISO_
    pub const NULL: HandlerCode = HandlerCode(FourCC(*b"null"));

    /// ObjectContentInfoStream
    /// 
    /// FourCC: `ocsm`
    /// 
    /// Specification: _MPEG-4_
    pub const OCSM: HandlerCode = HandlerCode(FourCC(*b"ocsm"));

    /// ObjectDescriptorStream
    /// 
    /// FourCC: `odsm`
    /// 
    /// Specification: _MPEG-4_
    pub const ODSM: HandlerCode = HandlerCode(FourCC(*b"odsm"));

    /// Image Item and Image sequences
    /// 
    /// FourCC: `pict`
    /// 
    /// Specification: _HEIF_
    pub const PICT: HandlerCode = HandlerCode(FourCC(*b"pict"));

    /// QuickTime QuickDraw 3D ttrack handler
    /// 
    /// FourCC: `qd3d`
    /// 
    /// Specification: _Apple_
    pub const QD3D: HandlerCode = HandlerCode(FourCC(*b"qd3d"));

    /// QuickTime Subtitle track handler
    /// 
    /// FourCC: `sbtl`
    /// 
    /// Specification: _Apple_
    pub const SBTL: HandlerCode = HandlerCode(FourCC(*b"sbtl"));

    /// SceneDescriptionStream
    /// 
    /// FourCC: `sdsm`
    /// 
    /// Specification: _MPEG-4_
    pub const SDSM: HandlerCode = HandlerCode(FourCC(*b"sdsm"));

    /// Key Management Messages
    /// 
    /// FourCC: `skmm`
    /// 
    /// Specification: _DVB_
    pub const SKMM: HandlerCode = HandlerCode(FourCC(*b"skmm"));

    /// Samsung Video Metadata Handler
    /// 
    /// FourCC: `smhr`
    /// 
    /// Specification: _Samsung_
    pub const SMHR: HandlerCode = HandlerCode(FourCC(*b"smhr"));

    /// Audio
    /// 
    /// FourCC: `soun`
    /// 
    /// Specification: _ISO_
    pub const SOUN: HandlerCode = HandlerCode(FourCC(*b"soun"));

    /// QuickTime Sprite track handler
    /// 
    /// FourCC: `sprt`
    /// 
    /// Specification: _Apple_
    pub const SPRT: HandlerCode = HandlerCode(FourCC(*b"sprt"));

    /// QuickTime Streaming track handler
    /// 
    /// FourCC: `strm`
    /// 
    /// Specification: _Apple_
    pub const STRM: HandlerCode = HandlerCode(FourCC(*b"strm"));

    /// Subtitles
    /// 
    /// FourCC: `subt`
    /// 
    /// Specification: _ISO_
    pub const SUBT: HandlerCode = HandlerCode(FourCC(*b"subt"));

    /// Text
    /// 
    /// FourCC: `text`
    /// 
    /// Specification: _3GPP_
    pub const TEXT: HandlerCode = HandlerCode(FourCC(*b"text"));

    /// Timecode
    /// 
    /// FourCC: `tmcd`
    /// 
    /// Specification: _Apple_
    pub const TMCD: HandlerCode = HandlerCode(FourCC(*b"tmcd"));

    /// QuickTime Tween track handler
    /// 
    /// FourCC: `twen`
    /// 
    /// Specification: _Apple_
    pub const TWEN: HandlerCode = HandlerCode(FourCC(*b"twen"));

    /// URI identified metadata
    /// 
    /// FourCC: `uri `
    /// 
    /// Specification: _DVB_
    pub const URI : HandlerCode = HandlerCode(FourCC(*b"uri "));

    /// Video
    /// 
    /// FourCC: `vide`
    /// 
    /// Specification: _ISO_
    pub const VIDE: HandlerCode = HandlerCode(FourCC(*b"vide"));

    /// Volumetric visual media
    /// 
    /// FourCC: `volv`
    /// 
    /// Specification: _ISO_
    pub const VOLV: HandlerCode = HandlerCode(FourCC(*b"volv"));

    /// Haptics
    /// 
    /// FourCC: `hapt`
    /// 
    /// Specification: _ISO_
    pub const HAPT: HandlerCode = HandlerCode(FourCC(*b"hapt"));

}

impl SampleEntryCode {
    /// Return the identifier of a handler for this type of sample entry, if a single handler type is defined.  For those _sample entry_ types where MP4RA notes there are 'various' handlers, this method will return `None`
    pub fn handler(&self) -> Option<HandlerCode> {
        match *self {
          SampleEntryCode::TWO_DCC => Some(HandlerCode::META),
          SampleEntryCode::THREE_GLO => Some(HandlerCode::META),
          SampleEntryCode::THREE_GOR => Some(HandlerCode::META),
          SampleEntryCode::THREE_GVO => Some(HandlerCode::META),
          SampleEntryCode::SIX_VPT => Some(HandlerCode::VOLV),
          SampleEntryCode::A3D1 => Some(HandlerCode::VIDE),
          SampleEntryCode::A3D2 => Some(HandlerCode::VIDE),
          SampleEntryCode::A3D3 => Some(HandlerCode::VIDE),
          SampleEntryCode::A3D4 => Some(HandlerCode::VIDE),
          SampleEntryCode::A3DS => Some(HandlerCode::SOUN),
          SampleEntryCode::AC_3 => Some(HandlerCode::SOUN),
          SampleEntryCode::AC_4 => Some(HandlerCode::SOUN),
          SampleEntryCode::ALAC => Some(HandlerCode::SOUN),
          SampleEntryCode::ALAW => Some(HandlerCode::SOUN),
          SampleEntryCode::AV01 => Some(HandlerCode::VIDE),
          SampleEntryCode::AVC1 => Some(HandlerCode::VIDE),
          SampleEntryCode::AVC2 => Some(HandlerCode::VIDE),
          SampleEntryCode::AVC3 => Some(HandlerCode::VIDE),
          SampleEntryCode::AVC4 => Some(HandlerCode::VIDE),
          SampleEntryCode::AVCP => Some(HandlerCode::VIDE),
          SampleEntryCode::AVST => Some(HandlerCode::VIDE),
          SampleEntryCode::AVS3 => Some(HandlerCode::VIDE),
          SampleEntryCode::CAMM => Some(HandlerCode::VIDE),
          SampleEntryCode::CAVS => Some(HandlerCode::SOUN),
          SampleEntryCode::DAV1 => Some(HandlerCode::VIDE),
          SampleEntryCode::DRA1 => Some(HandlerCode::SOUN),
          SampleEntryCode::DRAC => Some(HandlerCode::VIDE),
          SampleEntryCode::DTS_PLUS => Some(HandlerCode::SOUN),
          SampleEntryCode::DTS_ => Some(HandlerCode::SOUN),
          SampleEntryCode::DTSC => Some(HandlerCode::SOUN),
          SampleEntryCode::DTSE => Some(HandlerCode::SOUN),
          SampleEntryCode::DTSH => Some(HandlerCode::SOUN),
          SampleEntryCode::DTSL => Some(HandlerCode::SOUN),
          SampleEntryCode::DTSX => Some(HandlerCode::SOUN),
          SampleEntryCode::DTSY => Some(HandlerCode::SOUN),
          SampleEntryCode::DVA1 => Some(HandlerCode::VIDE),
          SampleEntryCode::DVAV => Some(HandlerCode::VIDE),
          SampleEntryCode::DVH1 => Some(HandlerCode::VIDE),
          SampleEntryCode::DVHE => Some(HandlerCode::VIDE),
          SampleEntryCode::DYOL => Some(HandlerCode::META),
          SampleEntryCode::DYVM => Some(HandlerCode::VOLV),
          SampleEntryCode::DYVP => Some(HandlerCode::META),
          SampleEntryCode::EC_3 => Some(HandlerCode::SOUN),
          SampleEntryCode::EC_PLUS_3 => Some(HandlerCode::SOUN),
          SampleEntryCode::ENCA => Some(HandlerCode::SOUN),
          SampleEntryCode::ENCF => Some(HandlerCode::FDSM),
          SampleEntryCode::ENCM => Some(HandlerCode::META),
          SampleEntryCode::ENCS => None,
          SampleEntryCode::ENCT => Some(HandlerCode::TEXT),
          SampleEntryCode::ENCV => Some(HandlerCode::VIDE),
          SampleEntryCode::EVC1 => Some(HandlerCode::VIDE),
          SampleEntryCode::EVM1 => Some(HandlerCode::VIDE),
          SampleEntryCode::EVS1 => Some(HandlerCode::VIDE),
          SampleEntryCode::EVS2 => Some(HandlerCode::VIDE),
          SampleEntryCode::FDP  => Some(HandlerCode::HINT),
          SampleEntryCode::FFV1 => Some(HandlerCode::VIDE),
          SampleEntryCode::FLAC => Some(HandlerCode::SOUN),
          SampleEntryCode::G719 => Some(HandlerCode::SOUN),
          SampleEntryCode::G726 => Some(HandlerCode::SOUN),
          SampleEntryCode::HEV1 => Some(HandlerCode::VIDE),
          SampleEntryCode::HEV2 => Some(HandlerCode::VIDE),
          SampleEntryCode::HEV3 => Some(HandlerCode::VIDE),
          SampleEntryCode::HVC1 => Some(HandlerCode::VIDE),
          SampleEntryCode::HVC2 => Some(HandlerCode::VIDE),
          SampleEntryCode::HVC3 => Some(HandlerCode::VIDE),
          SampleEntryCode::HVT1 => Some(HandlerCode::VIDE),
          SampleEntryCode::HVT2 => Some(HandlerCode::VIDE),
          SampleEntryCode::HVT3 => Some(HandlerCode::VIDE),
          SampleEntryCode::ICPV => Some(HandlerCode::VIDE),
          SampleEntryCode::INVO => Some(HandlerCode::META),
          SampleEntryCode::INVP => Some(HandlerCode::META),
          SampleEntryCode::IXSE => Some(HandlerCode::META),
          SampleEntryCode::J2KI => Some(HandlerCode::VIDE),
          SampleEntryCode::JXSM => Some(HandlerCode::VIDE),
          SampleEntryCode::LHE1 => Some(HandlerCode::VIDE),
          SampleEntryCode::LHT1 => Some(HandlerCode::VIDE),
          SampleEntryCode::LHV1 => Some(HandlerCode::VIDE),
          SampleEntryCode::M2TS => Some(HandlerCode::HINT),
          SampleEntryCode::M4AE => Some(HandlerCode::SOUN),
          SampleEntryCode::MEBX => Some(HandlerCode::META),
          SampleEntryCode::METT => Some(HandlerCode::META),
          SampleEntryCode::METX => Some(HandlerCode::META),
          SampleEntryCode::MHA1 => Some(HandlerCode::SOUN),
          SampleEntryCode::MHA2 => Some(HandlerCode::SOUN),
          SampleEntryCode::MHM1 => Some(HandlerCode::SOUN),
          SampleEntryCode::MHM2 => Some(HandlerCode::SOUN),
          SampleEntryCode::MJP2 => Some(HandlerCode::VIDE),
          SampleEntryCode::MJPG => Some(HandlerCode::VIDE),
          SampleEntryCode::MLIX => Some(HandlerCode::META),
          SampleEntryCode::MLPA => Some(HandlerCode::SOUN),
          SampleEntryCode::MP4A => Some(HandlerCode::SOUN),
          SampleEntryCode::MP4S => None,
          SampleEntryCode::MP4V => Some(HandlerCode::VIDE),
          SampleEntryCode::MVC1 => Some(HandlerCode::VIDE),
          SampleEntryCode::MVC2 => Some(HandlerCode::VIDE),
          SampleEntryCode::MVC3 => Some(HandlerCode::VIDE),
          SampleEntryCode::MVC4 => Some(HandlerCode::VIDE),
          SampleEntryCode::MVD1 => Some(HandlerCode::VIDE),
          SampleEntryCode::MVD2 => Some(HandlerCode::VIDE),
          SampleEntryCode::MVD3 => Some(HandlerCode::VIDE),
          SampleEntryCode::MVD4 => Some(HandlerCode::VIDE),
          SampleEntryCode::OCPC => Some(HandlerCode::META),
          SampleEntryCode::OKSD => Some(HandlerCode::META),
          SampleEntryCode::OPUS => Some(HandlerCode::SOUN),
          SampleEntryCode::PM2T => Some(HandlerCode::HINT),
          SampleEntryCode::PRTP => Some(HandlerCode::HINT),
          SampleEntryCode::RAW  => Some(HandlerCode::SOUN),
          SampleEntryCode::RCVP => Some(HandlerCode::META),
          SampleEntryCode::RESV => Some(HandlerCode::VIDE),
          SampleEntryCode::RM2T => Some(HandlerCode::HINT),
          SampleEntryCode::RRTP => Some(HandlerCode::HINT),
          SampleEntryCode::RSRP => Some(HandlerCode::HINT),
          SampleEntryCode::RTCP => Some(HandlerCode::HINT),
          SampleEntryCode::RTMD => Some(HandlerCode::META),
          SampleEntryCode::RTP  => Some(HandlerCode::HINT),
          SampleEntryCode::RV60 => Some(HandlerCode::VIDE),
          SampleEntryCode::RVP2 => Some(HandlerCode::META),
          SampleEntryCode::S263 => Some(HandlerCode::VIDE),
          SampleEntryCode::SAMR => Some(HandlerCode::SOUN),
          SampleEntryCode::SAWB => Some(HandlerCode::SOUN),
          SampleEntryCode::SAWP => Some(HandlerCode::SOUN),
          SampleEntryCode::SBTT => Some(HandlerCode::SUBT),
          SampleEntryCode::SEVC => Some(HandlerCode::SOUN),
          SampleEntryCode::SEVS => Some(HandlerCode::SOUN),
          SampleEntryCode::SM2T => Some(HandlerCode::HINT),
          SampleEntryCode::SQCP => Some(HandlerCode::SOUN),
          SampleEntryCode::SRTP => Some(HandlerCode::HINT),
          SampleEntryCode::SSMV => Some(HandlerCode::SOUN),
          SampleEntryCode::STCP => Some(HandlerCode::HINT),
          SampleEntryCode::STGS => Some(HandlerCode::GRAP),
          SampleEntryCode::STMD => Some(HandlerCode::META),
          SampleEntryCode::STPP => Some(HandlerCode::SUBT),
          SampleEntryCode::STXT => Some(HandlerCode::TEXT),
          SampleEntryCode::SVC1 => Some(HandlerCode::VIDE),
          SampleEntryCode::SVC2 => Some(HandlerCode::VIDE),
          SampleEntryCode::SVCM => Some(HandlerCode::META),
          SampleEntryCode::TC64 => Some(HandlerCode::TMCD),
          SampleEntryCode::TMCD => Some(HandlerCode::TMCD),
          SampleEntryCode::TTSL => Some(HandlerCode::META),
          SampleEntryCode::TWOS => Some(HandlerCode::SOUN),
          SampleEntryCode::TX3G => Some(HandlerCode::TEXT),
          SampleEntryCode::ULAW => Some(HandlerCode::SOUN),
          SampleEntryCode::UNID => Some(HandlerCode::META),
          SampleEntryCode::URIM => Some(HandlerCode::META),
          SampleEntryCode::V3A1 => Some(HandlerCode::VOLV),
          SampleEntryCode::V3AG => Some(HandlerCode::VOLV),
          SampleEntryCode::V3C1 => Some(HandlerCode::VOLV),
          SampleEntryCode::V3CB => Some(HandlerCode::VOLV),
          SampleEntryCode::V3CG => Some(HandlerCode::VOLV),
          SampleEntryCode::V3E1 => Some(HandlerCode::VOLV),
          SampleEntryCode::V3EG => Some(HandlerCode::VOLV),
          SampleEntryCode::V3T1 => Some(HandlerCode::VOLV),
          SampleEntryCode::VC_1 => Some(HandlerCode::VIDE),
          SampleEntryCode::VP08 => Some(HandlerCode::VIDE),
          SampleEntryCode::VP09 => Some(HandlerCode::VIDE),
          SampleEntryCode::VRSP => Some(HandlerCode::META),
          SampleEntryCode::VVCN => Some(HandlerCode::VIDE),
          SampleEntryCode::VVC1 => Some(HandlerCode::VIDE),
          SampleEntryCode::VVI1 => Some(HandlerCode::VIDE),
          SampleEntryCode::VVS1 => Some(HandlerCode::VIDE),
          SampleEntryCode::WVTT => Some(HandlerCode::TEXT),
          SampleEntryCode::ENCU => Some(HandlerCode::SUBT),
          SampleEntryCode::ENCP => Some(HandlerCode::HAPT),
          SampleEntryCode::ENC3 => Some(HandlerCode::VOLV),
          SampleEntryCode::IAMF => Some(HandlerCode::SOUN),
          SampleEntryCode::IPCM => Some(HandlerCode::SOUN),
          SampleEntryCode::FPCM => Some(HandlerCode::SOUN),
          _ => None,
        }
    }
}

impl SampleEntryCode {
    /// 2D cartesian coordinates
    /// 
    /// FourCC: `2dcc`
    /// 
    /// Specification: _Metrics_
    pub const TWO_DCC: SampleEntryCode = SampleEntryCode(FourCC(*b"2dcc"));

    /// 3GPP Location
    /// 
    /// FourCC: `3glo`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GLO: SampleEntryCode = SampleEntryCode(FourCC(*b"3glo"));

    /// 3GPP Orientation
    /// 
    /// FourCC: `3gor`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GOR: SampleEntryCode = SampleEntryCode(FourCC(*b"3gor"));

    /// 3GPP Video Orientation
    /// 
    /// FourCC: `3gvo`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GVO: SampleEntryCode = SampleEntryCode(FourCC(*b"3gvo"));

    /// Dynamic viewport data
    /// 
    /// FourCC: `6vpt`
    /// 
    /// Specification: _V3C-SYS_
    pub const SIX_VPT: SampleEntryCode = SampleEntryCode(FourCC(*b"6vpt"));

    /// 3D-AVC track with 3D-AVC NAL units only
    /// 
    /// FourCC: `a3d1`
    /// 
    /// Specification: _NALu Video_
    pub const A3D1: SampleEntryCode = SampleEntryCode(FourCC(*b"a3d1"));

    /// 3D-AVC track with 3D-AVC NAL units only
    /// 
    /// FourCC: `a3d2`
    /// 
    /// Specification: _NALu Video_
    pub const A3D2: SampleEntryCode = SampleEntryCode(FourCC(*b"a3d2"));

    /// 3D-AVC track with 3D-AVC NAL units only
    /// 
    /// FourCC: `a3d3`
    /// 
    /// Specification: _NALu Video_
    pub const A3D3: SampleEntryCode = SampleEntryCode(FourCC(*b"a3d3"));

    /// 3D-AVC track with 3D-AVC NAL units only
    /// 
    /// FourCC: `a3d4`
    /// 
    /// Specification: _NALu Video_
    pub const A3D4: SampleEntryCode = SampleEntryCode(FourCC(*b"a3d4"));

    /// Auro-Cx 3D audio
    /// 
    /// FourCC: `a3ds`
    /// 
    /// Specification: _Auro_
    pub const A3DS: SampleEntryCode = SampleEntryCode(FourCC(*b"a3ds"));

    /// AC-3 audio
    /// 
    /// FourCC: `ac-3`
    /// 
    /// Specification: _ETSI AC-3_
    pub const AC_3: SampleEntryCode = SampleEntryCode(FourCC(*b"ac-3"));

    /// AC-4 audio
    /// 
    /// FourCC: `ac-4`
    /// 
    /// Specification: _ETSI AC-4_
    pub const AC_4: SampleEntryCode = SampleEntryCode(FourCC(*b"ac-4"));

    /// Apple lossless audio codec
    /// 
    /// FourCC: `alac`
    /// 
    /// Specification: _Apple_
    pub const ALAC: SampleEntryCode = SampleEntryCode(FourCC(*b"alac"));

    /// a-Law
    /// 
    /// FourCC: `alaw`
    /// 
    /// Specification: _QT_
    pub const ALAW: SampleEntryCode = SampleEntryCode(FourCC(*b"alaw"));

    /// AOM Video Codec
    /// 
    /// FourCC: `av01`
    /// 
    /// Specification: _AV1-ISOBMFF_
    pub const AV01: SampleEntryCode = SampleEntryCode(FourCC(*b"av01"));

    /// Advanced Video Coding
    /// 
    /// FourCC: `avc1`
    /// 
    /// Specification: _NALu Video_
    pub const AVC1: SampleEntryCode = SampleEntryCode(FourCC(*b"avc1"));

    /// Advanced Video Coding
    /// 
    /// FourCC: `avc2`
    /// 
    /// Specification: _NALu Video_
    pub const AVC2: SampleEntryCode = SampleEntryCode(FourCC(*b"avc2"));

    /// Advanced Video Coding
    /// 
    /// FourCC: `avc3`
    /// 
    /// Specification: _NALu Video_
    pub const AVC3: SampleEntryCode = SampleEntryCode(FourCC(*b"avc3"));

    /// Advanced Video Coding
    /// 
    /// FourCC: `avc4`
    /// 
    /// Specification: _NALu Video_
    pub const AVC4: SampleEntryCode = SampleEntryCode(FourCC(*b"avc4"));

    /// Advanced Video Coding Parameters
    /// 
    /// FourCC: `avcp`
    /// 
    /// Specification: _NALu Video_
    pub const AVCP: SampleEntryCode = SampleEntryCode(FourCC(*b"avcp"));

    /// 2nd Generation Audio Video Coding Standard of China (AVS Two)
    /// 
    /// FourCC: `avst`
    /// 
    /// Specification: _Avs2_
    pub const AVST: SampleEntryCode = SampleEntryCode(FourCC(*b"avst"));

    /// 3nd Generation Audio Video Coding Standard of China
    /// 
    /// FourCC: `avs3`
    /// 
    /// Specification: _Avs3_
    pub const AVS3: SampleEntryCode = SampleEntryCode(FourCC(*b"avs3"));

    /// Camera motion metadata track
    /// 
    /// FourCC: `camm`
    /// 
    /// Specification: _CamMotion_
    pub const CAMM: SampleEntryCode = SampleEntryCode(FourCC(*b"camm"));

    /// AVS2-P3 codec
    /// 
    /// FourCC: `cavs`
    /// 
    /// Specification: _GB-T-20090-9_
    pub const CAVS: SampleEntryCode = SampleEntryCode(FourCC(*b"cavs"));

    /// AV1-related Dolby Vision consistent with av01
    /// 
    /// FourCC: `dav1`
    /// 
    /// Specification: _Dolby Vision_
    pub const DAV1: SampleEntryCode = SampleEntryCode(FourCC(*b"dav1"));

    /// DRA Audio
    /// 
    /// FourCC: `dra1`
    /// 
    /// Specification: _DRA_
    pub const DRA1: SampleEntryCode = SampleEntryCode(FourCC(*b"dra1"));

    /// Dirac Video Coder
    /// 
    /// FourCC: `drac`
    /// 
    /// Specification: _Dirac_
    pub const DRAC: SampleEntryCode = SampleEntryCode(FourCC(*b"drac"));

    /// Enhancement layer for DTS layered audio
    /// 
    /// FourCC: `dts+`
    /// 
    /// Specification: _DTS_
    pub const DTS_PLUS: SampleEntryCode = SampleEntryCode(FourCC(*b"dts+"));

    /// Dependent base layer for DTS layered audio
    /// 
    /// FourCC: `dts-`
    /// 
    /// Specification: _DTS_
    pub const DTS_: SampleEntryCode = SampleEntryCode(FourCC(*b"dts-"));

    /// Core Substream
    /// 
    /// FourCC: `dtsc`
    /// 
    /// Specification: _DTS-HD_
    pub const DTSC: SampleEntryCode = SampleEntryCode(FourCC(*b"dtsc"));

    /// Extension Substream containing only LBR
    /// 
    /// FourCC: `dtse`
    /// 
    /// Specification: _DTS-HD_
    pub const DTSE: SampleEntryCode = SampleEntryCode(FourCC(*b"dtse"));

    /// Core Substream + Extension Substream
    /// 
    /// FourCC: `dtsh`
    /// 
    /// Specification: _DTS-HD_
    pub const DTSH: SampleEntryCode = SampleEntryCode(FourCC(*b"dtsh"));

    /// Extension Substream containing only XLL
    /// 
    /// FourCC: `dtsl`
    /// 
    /// Specification: _DTS-HD_
    pub const DTSL: SampleEntryCode = SampleEntryCode(FourCC(*b"dtsl"));

    /// DTS-UHD profile 2
    /// 
    /// FourCC: `dtsx`
    /// 
    /// Specification: _DTS-UHD_
    pub const DTSX: SampleEntryCode = SampleEntryCode(FourCC(*b"dtsx"));

    /// DTS-UHD profile 3 or higher
    /// 
    /// FourCC: `dtsy`
    /// 
    /// Specification: _DTS-UHD_
    pub const DTSY: SampleEntryCode = SampleEntryCode(FourCC(*b"dtsy"));

    /// AVC-based Dolby Vision derived from avc1
    /// 
    /// FourCC: `dva1`
    /// 
    /// Specification: _Dolby Vision_
    pub const DVA1: SampleEntryCode = SampleEntryCode(FourCC(*b"dva1"));

    /// AVC-based Dolby Vision derived from avc3
    /// 
    /// FourCC: `dvav`
    /// 
    /// Specification: _Dolby Vision_
    pub const DVAV: SampleEntryCode = SampleEntryCode(FourCC(*b"dvav"));

    /// HEVC-based Dolby Vision derived from hvc1
    /// 
    /// FourCC: `dvh1`
    /// 
    /// Specification: _Dolby Vision_
    pub const DVH1: SampleEntryCode = SampleEntryCode(FourCC(*b"dvh1"));

    /// HEVC-based Dolby Vision derived from hev1
    /// 
    /// FourCC: `dvhe`
    /// 
    /// Specification: _Dolby Vision_
    pub const DVHE: SampleEntryCode = SampleEntryCode(FourCC(*b"dvhe"));

    /// Dynamic overlay parameters
    /// 
    /// FourCC: `dyol`
    /// 
    /// Specification: _OMAF_
    pub const DYOL: SampleEntryCode = SampleEntryCode(FourCC(*b"dyol"));

    /// Dynamic spatial region data
    /// 
    /// FourCC: `dyvm`
    /// 
    /// Specification: _V3C-SYS_
    pub const DYVM: SampleEntryCode = SampleEntryCode(FourCC(*b"dyvm"));

    /// Dynamic viewpoint parameters
    /// 
    /// FourCC: `dyvp`
    /// 
    /// Specification: _OMAF_
    pub const DYVP: SampleEntryCode = SampleEntryCode(FourCC(*b"dyvp"));

    /// Enhanced AC-3 audio
    /// 
    /// FourCC: `ec-3`
    /// 
    /// Specification: _ETSI AC-3_
    pub const EC_3: SampleEntryCode = SampleEntryCode(FourCC(*b"ec-3"));

    /// withdrawn, unused, do not use (was enhanced AC-3 audio with JOC)
    /// 
    /// FourCC: `ec+3`
    /// 
    /// Specification: _Deprecated_
    pub const EC_PLUS_3: SampleEntryCode = SampleEntryCode(FourCC(*b"ec+3"));

    /// Encrypted/Protected audio
    /// 
    /// FourCC: `enca`
    /// 
    /// Specification: _ISO_
    pub const ENCA: SampleEntryCode = SampleEntryCode(FourCC(*b"enca"));

    /// Encrypted/Protected font
    /// 
    /// FourCC: `encf`
    /// 
    /// Specification: _ISO_
    pub const ENCF: SampleEntryCode = SampleEntryCode(FourCC(*b"encf"));

    /// Encrypted/Protected metadata
    /// 
    /// FourCC: `encm`
    /// 
    /// Specification: _ISO_
    pub const ENCM: SampleEntryCode = SampleEntryCode(FourCC(*b"encm"));

    /// Encrypted Systems stream
    /// 
    /// FourCC: `encs`
    /// 
    /// Specification: _ISO_
    pub const ENCS: SampleEntryCode = SampleEntryCode(FourCC(*b"encs"));

    /// Encrypted Text
    /// 
    /// FourCC: `enct`
    /// 
    /// Specification: _ISO_
    pub const ENCT: SampleEntryCode = SampleEntryCode(FourCC(*b"enct"));

    /// Encrypted/protected video
    /// 
    /// FourCC: `encv`
    /// 
    /// Specification: _ISO_
    pub const ENCV: SampleEntryCode = SampleEntryCode(FourCC(*b"encv"));

    /// Essential Video Coding
    /// 
    /// FourCC: `evc1`
    /// 
    /// Specification: _NALu Video_
    pub const EVC1: SampleEntryCode = SampleEntryCode(FourCC(*b"evc1"));

    /// Essential Video Coding slice base track
    /// 
    /// FourCC: `evm1`
    /// 
    /// Specification: _NALu Video_
    pub const EVM1: SampleEntryCode = SampleEntryCode(FourCC(*b"evm1"));

    /// Essential Video Coding slice component track without parameter sets
    /// 
    /// FourCC: `evs1`
    /// 
    /// Specification: _NALu Video_
    pub const EVS1: SampleEntryCode = SampleEntryCode(FourCC(*b"evs1"));

    /// Essential Video Coding slice component track that may contain parameter sets
    /// 
    /// FourCC: `evs2`
    /// 
    /// Specification: _NALu Video_
    pub const EVS2: SampleEntryCode = SampleEntryCode(FourCC(*b"evs2"));

    /// File delivery hints
    /// 
    /// FourCC: `fdp `
    /// 
    /// Specification: _ISO_
    pub const FDP : SampleEntryCode = SampleEntryCode(FourCC(*b"fdp "));

    /// An open lossless intra-frame video codec
    /// 
    /// FourCC: `FFV1`
    /// 
    /// Specification: _FF Video Codec_
    pub const FFV1: SampleEntryCode = SampleEntryCode(FourCC(*b"FFV1"));

    /// Free Lossless Audio Codec (FLAC)
    /// 
    /// FourCC: `fLaC`
    /// 
    /// Specification: _FLAC_
    pub const FLAC: SampleEntryCode = SampleEntryCode(FourCC(*b"fLaC"));

    /// ITU-T Recommendation G.719 (2008)
    /// 
    /// FourCC: `g719`
    /// 
    /// Specification: _ITU G.719_
    pub const G719: SampleEntryCode = SampleEntryCode(FourCC(*b"g719"));

    /// ITU-T Recommendation G.726 (1990)
    /// 
    /// FourCC: `g726`
    /// 
    /// Specification: _SDV_
    pub const G726: SampleEntryCode = SampleEntryCode(FourCC(*b"g726"));

    /// HEVC video with parameter sets in the Sample Entry or samples
    /// 
    /// FourCC: `hev1`
    /// 
    /// Specification: _NALu Video_
    pub const HEV1: SampleEntryCode = SampleEntryCode(FourCC(*b"hev1"));

    /// HEVC video with constrained extractors and/or aggregators and parameter sets in the Sample Entry or samples
    /// 
    /// FourCC: `hev2`
    /// 
    /// Specification: _NALu Video_
    pub const HEV2: SampleEntryCode = SampleEntryCode(FourCC(*b"hev2"));

    /// HEVC video with extractors and/or aggregators and parameter sets in the Sample Entry or samples
    /// 
    /// FourCC: `hev3`
    /// 
    /// Specification: _NALu Video_
    pub const HEV3: SampleEntryCode = SampleEntryCode(FourCC(*b"hev3"));

    /// HEVC video with parameter sets only in the Sample Entry
    /// 
    /// FourCC: `hvc1`
    /// 
    /// Specification: _NALu Video_
    pub const HVC1: SampleEntryCode = SampleEntryCode(FourCC(*b"hvc1"));

    /// HEVC video with constrained extractors and/or aggregators and parameter sets only in the Sample Entry
    /// 
    /// FourCC: `hvc2`
    /// 
    /// Specification: _NALu Video_
    pub const HVC2: SampleEntryCode = SampleEntryCode(FourCC(*b"hvc2"));

    /// HEVC video with extractors and/or aggregators and parameter sets only in the Sample Entry
    /// 
    /// FourCC: `hvc3`
    /// 
    /// Specification: _NALu Video_
    pub const HVC3: SampleEntryCode = SampleEntryCode(FourCC(*b"hvc3"));

    /// HEVC tile tracks
    /// 
    /// FourCC: `hvt1`
    /// 
    /// Specification: _NALu Video_
    pub const HVT1: SampleEntryCode = SampleEntryCode(FourCC(*b"hvt1"));

    /// HEVC slice segment data track
    /// 
    /// FourCC: `hvt2`
    /// 
    /// Specification: _NALu Video_
    pub const HVT2: SampleEntryCode = SampleEntryCode(FourCC(*b"hvt2"));

    /// HEVC Tile Track with Slice Segment Header Info
    /// 
    /// FourCC: `hvt3`
    /// 
    /// Specification: _NALu Video_
    pub const HVT3: SampleEntryCode = SampleEntryCode(FourCC(*b"hvt3"));

    /// Incomplete video
    /// 
    /// FourCC: `icpv`
    /// 
    /// Specification: _ISO_
    pub const ICPV: SampleEntryCode = SampleEntryCode(FourCC(*b"icpv"));

    /// Initial viewing orientation
    /// 
    /// FourCC: `invo`
    /// 
    /// Specification: _OMAF_
    pub const INVO: SampleEntryCode = SampleEntryCode(FourCC(*b"invo"));

    /// Initial viewpoint
    /// 
    /// FourCC: `invp`
    /// 
    /// Specification: _OMAF_
    pub const INVP: SampleEntryCode = SampleEntryCode(FourCC(*b"invp"));

    /// DVB Track Level Index Track
    /// 
    /// FourCC: `ixse`
    /// 
    /// Specification: _DVB_
    pub const IXSE: SampleEntryCode = SampleEntryCode(FourCC(*b"ixse"));

    /// Sequence of JPEG 2000 Contiguous Codestream boxes as defined in Rec. ITU-T T.800 | ISO/IEC 15444-1
    /// 
    /// FourCC: `j2ki`
    /// 
    /// Specification: _J2KHEIF_
    pub const J2KI: SampleEntryCode = SampleEntryCode(FourCC(*b"j2ki"));

    /// Video and image sequences coded to the JPEG-XS coding format
    /// 
    /// FourCC: `jxsm`
    /// 
    /// Specification: _JPXS_
    pub const JXSM: SampleEntryCode = SampleEntryCode(FourCC(*b"jxsm"));

    /// Layered HEVC
    /// 
    /// FourCC: `lhe1`
    /// 
    /// Specification: _NALu Video_
    pub const LHE1: SampleEntryCode = SampleEntryCode(FourCC(*b"lhe1"));

    /// Layered HEVC tile tracks
    /// 
    /// FourCC: `lht1`
    /// 
    /// Specification: _NALu Video_
    pub const LHT1: SampleEntryCode = SampleEntryCode(FourCC(*b"lht1"));

    /// Layered HEVC
    /// 
    /// FourCC: `lhv1`
    /// 
    /// Specification: _NALu Video_
    pub const LHV1: SampleEntryCode = SampleEntryCode(FourCC(*b"lhv1"));

    /// MPEG-2 transport stream for DMB
    /// 
    /// FourCC: `m2ts`
    /// 
    /// Specification: _DMB-MAF_
    pub const M2TS: SampleEntryCode = SampleEntryCode(FourCC(*b"m2ts"));

    /// MPEG-4 Audio Enhancement
    /// 
    /// FourCC: `m4ae`
    /// 
    /// Specification: _MP4v2_
    pub const M4AE: SampleEntryCode = SampleEntryCode(FourCC(*b"m4ae"));

    /// Timed metadata multiplex
    /// 
    /// FourCC: `mebx`
    /// 
    /// Specification: _ISO_
    pub const MEBX: SampleEntryCode = SampleEntryCode(FourCC(*b"mebx"));

    /// Text timed metadata that is not XML
    /// 
    /// FourCC: `mett`
    /// 
    /// Specification: _ISO_
    pub const METT: SampleEntryCode = SampleEntryCode(FourCC(*b"mett"));

    /// XML timed metadata
    /// 
    /// FourCC: `metx`
    /// 
    /// Specification: _ISO_
    pub const METX: SampleEntryCode = SampleEntryCode(FourCC(*b"metx"));

    /// MPEG-H Audio (single stream, unencapsulated)
    /// 
    /// FourCC: `mha1`
    /// 
    /// Specification: _MPEG-H_
    pub const MHA1: SampleEntryCode = SampleEntryCode(FourCC(*b"mha1"));

    /// MPEG-H Audio (multi-stream, unencapsulated)
    /// 
    /// FourCC: `mha2`
    /// 
    /// Specification: _MPEG-H_
    pub const MHA2: SampleEntryCode = SampleEntryCode(FourCC(*b"mha2"));

    /// MPEG-H Audio (single stream, MHAS encapsulated)
    /// 
    /// FourCC: `mhm1`
    /// 
    /// Specification: _MPEG-H_
    pub const MHM1: SampleEntryCode = SampleEntryCode(FourCC(*b"mhm1"));

    /// MPEG-H Audio (multi-stream, MHAS encapsulated)
    /// 
    /// FourCC: `mhm2`
    /// 
    /// Specification: _MPEG-H_
    pub const MHM2: SampleEntryCode = SampleEntryCode(FourCC(*b"mhm2"));

    /// Motion JPEG 2000
    /// 
    /// FourCC: `mjp2`
    /// 
    /// Specification: _MJ2_
    pub const MJP2: SampleEntryCode = SampleEntryCode(FourCC(*b"mjp2"));

    /// JPEG image sequences
    /// 
    /// FourCC: `mjpg`
    /// 
    /// Specification: _HEIF_
    pub const MJPG: SampleEntryCode = SampleEntryCode(FourCC(*b"mjpg"));

    /// DVB Movie level index track
    /// 
    /// FourCC: `mlix`
    /// 
    /// Specification: _DVB_
    pub const MLIX: SampleEntryCode = SampleEntryCode(FourCC(*b"mlix"));

    /// MLP Audio
    /// 
    /// FourCC: `mlpa`
    /// 
    /// Specification: _Dolby MLP_
    pub const MLPA: SampleEntryCode = SampleEntryCode(FourCC(*b"mlpa"));

    /// MPEG-4 Audio
    /// 
    /// FourCC: `mp4a`
    /// 
    /// Specification: _MP4v1_
    pub const MP4A: SampleEntryCode = SampleEntryCode(FourCC(*b"mp4a"));

    /// MPEG-4 Systems
    /// 
    /// FourCC: `mp4s`
    /// 
    /// Specification: _MP4v1_
    pub const MP4S: SampleEntryCode = SampleEntryCode(FourCC(*b"mp4s"));

    /// MPEG-4 Visual
    /// 
    /// FourCC: `mp4v`
    /// 
    /// Specification: _MP4v1_
    pub const MP4V: SampleEntryCode = SampleEntryCode(FourCC(*b"mp4v"));

    /// Multiview coding
    /// 
    /// FourCC: `mvc1`
    /// 
    /// Specification: _NALu Video_
    pub const MVC1: SampleEntryCode = SampleEntryCode(FourCC(*b"mvc1"));

    /// Multiview coding
    /// 
    /// FourCC: `mvc2`
    /// 
    /// Specification: _NALu Video_
    pub const MVC2: SampleEntryCode = SampleEntryCode(FourCC(*b"mvc2"));

    /// Multiview coding
    /// 
    /// FourCC: `mvc3`
    /// 
    /// Specification: _NALu Video_
    pub const MVC3: SampleEntryCode = SampleEntryCode(FourCC(*b"mvc3"));

    /// Multiview coding
    /// 
    /// FourCC: `mvc4`
    /// 
    /// Specification: _NALu Video_
    pub const MVC4: SampleEntryCode = SampleEntryCode(FourCC(*b"mvc4"));

    /// MVD stream
    /// 
    /// FourCC: `mvd1`
    /// 
    /// Specification: _NALu Video_
    pub const MVD1: SampleEntryCode = SampleEntryCode(FourCC(*b"mvd1"));

    /// MVD stream
    /// 
    /// FourCC: `mvd2`
    /// 
    /// Specification: _NALu Video_
    pub const MVD2: SampleEntryCode = SampleEntryCode(FourCC(*b"mvd2"));

    /// MVD stream
    /// 
    /// FourCC: `mvd3`
    /// 
    /// Specification: _NALu Video_
    pub const MVD3: SampleEntryCode = SampleEntryCode(FourCC(*b"mvd3"));

    /// MVD stream
    /// 
    /// FourCC: `mvd4`
    /// 
    /// Specification: _NALu Video_
    pub const MVD4: SampleEntryCode = SampleEntryCode(FourCC(*b"mvd4"));

    /// Object centre points correspondence between viewpoints
    /// 
    /// FourCC: `ocpc`
    /// 
    /// Specification: _OMAF_
    pub const OCPC: SampleEntryCode = SampleEntryCode(FourCC(*b"ocpc"));

    /// OMA Keys
    /// 
    /// FourCC: `oksd`
    /// 
    /// Specification: _OMA DRM XBS_
    pub const OKSD: SampleEntryCode = SampleEntryCode(FourCC(*b"oksd"));

    /// Opus audio coding
    /// 
    /// FourCC: `Opus`
    /// 
    /// Specification: _Opus_
    pub const OPUS: SampleEntryCode = SampleEntryCode(FourCC(*b"Opus"));

    /// Protected MPEG-2 Transport
    /// 
    /// FourCC: `pm2t`
    /// 
    /// Specification: _ISO_
    pub const PM2T: SampleEntryCode = SampleEntryCode(FourCC(*b"pm2t"));

    /// Protected RTP Reception
    /// 
    /// FourCC: `prtp`
    /// 
    /// Specification: _ISO_
    pub const PRTP: SampleEntryCode = SampleEntryCode(FourCC(*b"prtp"));

    /// Uncompressed audio
    /// 
    /// FourCC: `raw `
    /// 
    /// Specification: _MJ2_
    pub const RAW : SampleEntryCode = SampleEntryCode(FourCC(*b"raw "));

    /// Recommended viewport without indicating a viewpoint
    /// 
    /// FourCC: `rcvp`
    /// 
    /// Specification: _OMAF_
    pub const RCVP: SampleEntryCode = SampleEntryCode(FourCC(*b"rcvp"));

    /// Restricted Video
    /// 
    /// FourCC: `resv`
    /// 
    /// Specification: _NALu Video_
    pub const RESV: SampleEntryCode = SampleEntryCode(FourCC(*b"resv"));

    /// MPEG-2 Transport Reception
    /// 
    /// FourCC: `rm2t`
    /// 
    /// Specification: _ISO_
    pub const RM2T: SampleEntryCode = SampleEntryCode(FourCC(*b"rm2t"));

    /// RTP reception
    /// 
    /// FourCC: `rrtp`
    /// 
    /// Specification: _ISO_
    pub const RRTP: SampleEntryCode = SampleEntryCode(FourCC(*b"rrtp"));

    /// SRTP Reception
    /// 
    /// FourCC: `rsrp`
    /// 
    /// Specification: _ISO_
    pub const RSRP: SampleEntryCode = SampleEntryCode(FourCC(*b"rsrp"));

    /// RTCP reception hint track
    /// 
    /// FourCC: `rtcp`
    /// 
    /// Specification: _ISO_
    pub const RTCP: SampleEntryCode = SampleEntryCode(FourCC(*b"rtcp"));

    /// Real Time Metadata Sample Entry(XAVC Format)
    /// 
    /// FourCC: `rtmd`
    /// 
    /// Specification: _Sony_
    pub const RTMD: SampleEntryCode = SampleEntryCode(FourCC(*b"rtmd"));

    /// RTP Hints
    /// 
    /// FourCC: `rtp `
    /// 
    /// Specification: _ISO_
    pub const RTP : SampleEntryCode = SampleEntryCode(FourCC(*b"rtp "));

    /// RealVideo Codec 11
    /// 
    /// FourCC: `rv60`
    /// 
    /// Specification: _RealHD_
    pub const RV60: SampleEntryCode = SampleEntryCode(FourCC(*b"rv60"));

    /// Recommended viewport concerning one or more indicated viewpoints
    /// 
    /// FourCC: `rvp2`
    /// 
    /// Specification: _OMAF_
    pub const RVP2: SampleEntryCode = SampleEntryCode(FourCC(*b"rvp2"));

    /// ITU H.263 video (3GPP format)
    /// 
    /// FourCC: `s263`
    /// 
    /// Specification: _3GPP_
    pub const S263: SampleEntryCode = SampleEntryCode(FourCC(*b"s263"));

    /// Narrowband AMR voice
    /// 
    /// FourCC: `samr`
    /// 
    /// Specification: _3GPP_
    pub const SAMR: SampleEntryCode = SampleEntryCode(FourCC(*b"samr"));

    /// Wideband AMR voice
    /// 
    /// FourCC: `sawb`
    /// 
    /// Specification: _3GPP_
    pub const SAWB: SampleEntryCode = SampleEntryCode(FourCC(*b"sawb"));

    /// Extended AMR-WB (AMR-WB+)
    /// 
    /// FourCC: `sawp`
    /// 
    /// Specification: _3GPP_
    pub const SAWP: SampleEntryCode = SampleEntryCode(FourCC(*b"sawp"));

    /// Text subtitles
    /// 
    /// FourCC: `sbtt`
    /// 
    /// Specification: _ISO_
    pub const SBTT: SampleEntryCode = SampleEntryCode(FourCC(*b"sbtt"));

    /// EVRC Voice
    /// 
    /// FourCC: `sevc`
    /// 
    /// Specification: _3GPP2_
    pub const SEVC: SampleEntryCode = SampleEntryCode(FourCC(*b"sevc"));

    /// Enhanced Voice Services (EVS)
    /// 
    /// FourCC: `sevs`
    /// 
    /// Specification: _3GPP_
    pub const SEVS: SampleEntryCode = SampleEntryCode(FourCC(*b"sevs"));

    /// MPEG-2 Transport Server
    /// 
    /// FourCC: `sm2t`
    /// 
    /// Specification: _ISO_
    pub const SM2T: SampleEntryCode = SampleEntryCode(FourCC(*b"sm2t"));

    /// 13K Voice
    /// 
    /// FourCC: `sqcp`
    /// 
    /// Specification: _3GPP2_
    pub const SQCP: SampleEntryCode = SampleEntryCode(FourCC(*b"sqcp"));

    /// SRTP Hints
    /// 
    /// FourCC: `srtp`
    /// 
    /// Specification: _ISO_
    pub const SRTP: SampleEntryCode = SampleEntryCode(FourCC(*b"srtp"));

    /// SMV Voice
    /// 
    /// FourCC: `ssmv`
    /// 
    /// Specification: _3GPP2_
    pub const SSMV: SampleEntryCode = SampleEntryCode(FourCC(*b"ssmv"));

    /// SRTCP reception hint track
    /// 
    /// FourCC: `stcp`
    /// 
    /// Specification: _ISO_
    pub const STCP: SampleEntryCode = SampleEntryCode(FourCC(*b"stcp"));

    /// Subtitle Sample Entry (HMMP)
    /// 
    /// FourCC: `STGS`
    /// 
    /// Specification: _Sony_
    pub const STGS: SampleEntryCode = SampleEntryCode(FourCC(*b"STGS"));

    /// Metadata for ERP (equirectangular projection) regions
    /// 
    /// FourCC: `stmd`
    /// 
    /// Specification: _OMAF_
    pub const STMD: SampleEntryCode = SampleEntryCode(FourCC(*b"stmd"));

    /// Subtitles (Timed Text)
    /// 
    /// FourCC: `stpp`
    /// 
    /// Specification: _ISO_
    pub const STPP: SampleEntryCode = SampleEntryCode(FourCC(*b"stpp"));

    /// Simple timed text
    /// 
    /// FourCC: `stxt`
    /// 
    /// Specification: _ISO_
    pub const STXT: SampleEntryCode = SampleEntryCode(FourCC(*b"stxt"));

    /// Scalable Video Coding
    /// 
    /// FourCC: `svc1`
    /// 
    /// Specification: _NALu Video_
    pub const SVC1: SampleEntryCode = SampleEntryCode(FourCC(*b"svc1"));

    /// Scalable Video Coding
    /// 
    /// FourCC: `svc2`
    /// 
    /// Specification: _NALu Video_
    pub const SVC2: SampleEntryCode = SampleEntryCode(FourCC(*b"svc2"));

    /// SVC metadata
    /// 
    /// FourCC: `svcM`
    /// 
    /// Specification: _NALu Video_
    pub const SVCM: SampleEntryCode = SampleEntryCode(FourCC(*b"svcM"));

    /// 64 bit timecode samples
    /// 
    /// FourCC: `tc64`
    /// 
    /// Specification: _Apple_
    pub const TC64: SampleEntryCode = SampleEntryCode(FourCC(*b"tc64"));

    /// 32 bit timecode samples
    /// 
    /// FourCC: `tmcd`
    /// 
    /// Specification: _Apple_
    pub const TMCD: SampleEntryCode = SampleEntryCode(FourCC(*b"tmcd"));

    /// Sphere location for timed text
    /// 
    /// FourCC: `ttsl`
    /// 
    /// Specification: _OMAF_
    pub const TTSL: SampleEntryCode = SampleEntryCode(FourCC(*b"ttsl"));

    /// Uncompressed 16-bit audio
    /// 
    /// FourCC: `twos`
    /// 
    /// Specification: _MJ2_
    pub const TWOS: SampleEntryCode = SampleEntryCode(FourCC(*b"twos"));

    /// Timed Text stream
    /// 
    /// FourCC: `tx3g`
    /// 
    /// Specification: _3GPP_
    pub const TX3G: SampleEntryCode = SampleEntryCode(FourCC(*b"tx3g"));

    /// Samples have been compressed using uLaw 2:1.
    /// 
    /// FourCC: `ulaw`
    /// 
    /// Specification: _QT_
    pub const ULAW: SampleEntryCode = SampleEntryCode(FourCC(*b"ulaw"));

    /// Dynamic Range Control (DRC) data
    /// 
    /// FourCC: `unid`
    /// 
    /// Specification: _DRC_
    pub const UNID: SampleEntryCode = SampleEntryCode(FourCC(*b"unid"));

    /// Binary timed metadata identified by URI
    /// 
    /// FourCC: `urim`
    /// 
    /// Specification: _ISO_
    pub const URIM: SampleEntryCode = SampleEntryCode(FourCC(*b"urim"));

    /// V3C atlas track with atlas parameter sets only in sample entries
    /// 
    /// FourCC: `v3a1`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3A1: SampleEntryCode = SampleEntryCode(FourCC(*b"v3a1"));

    /// V3C atlas track with atlas parameter sets in sample entries or samples
    /// 
    /// FourCC: `v3ag`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3AG: SampleEntryCode = SampleEntryCode(FourCC(*b"v3ag"));

    /// V3C atlas track with a single atlas and atlas parameter sets only in sample entries
    /// 
    /// FourCC: `v3c1`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3C1: SampleEntryCode = SampleEntryCode(FourCC(*b"v3c1"));

    /// V3C atlas base track with common atlas data
    /// 
    /// FourCC: `v3cb`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3CB: SampleEntryCode = SampleEntryCode(FourCC(*b"v3cb"));

    /// V3C atlas track with a single atlas and atlas parameter sets in sample entries or samples
    /// 
    /// FourCC: `v3cg`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3CG: SampleEntryCode = SampleEntryCode(FourCC(*b"v3cg"));

    /// V3C bitstream track with atlas parameter sets only in sample entries
    /// 
    /// FourCC: `v3e1`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3E1: SampleEntryCode = SampleEntryCode(FourCC(*b"v3e1"));

    /// V3C bitstream track with atlas parameter sets in sample entries or samples
    /// 
    /// FourCC: `v3eg`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3EG: SampleEntryCode = SampleEntryCode(FourCC(*b"v3eg"));

    /// V3C atlas tile track with atlas tile data
    /// 
    /// FourCC: `v3t1`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3T1: SampleEntryCode = SampleEntryCode(FourCC(*b"v3t1"));

    /// SMPTE VC-1
    /// 
    /// FourCC: `vc-1`
    /// 
    /// Specification: _SMPTE_
    pub const VC_1: SampleEntryCode = SampleEntryCode(FourCC(*b"vc-1"));

    /// VP8 video
    /// 
    /// FourCC: `vp08`
    /// 
    /// Specification: _VPxx_
    pub const VP08: SampleEntryCode = SampleEntryCode(FourCC(*b"vp08"));

    /// VP9 video
    /// 
    /// FourCC: `vp09`
    /// 
    /// Specification: _VPxx_
    pub const VP09: SampleEntryCode = SampleEntryCode(FourCC(*b"vp09"));

    /// Viewing space
    /// 
    /// FourCC: `vrsp`
    /// 
    /// Specification: _OMAF_
    pub const VRSP: SampleEntryCode = SampleEntryCode(FourCC(*b"vrsp"));

    /// Versatile Video Coding with non-VCL (Video Coding Layer) NAL (Network Abstraction Layer) units only
    /// 
    /// FourCC: `vvcN`
    /// 
    /// Specification: _NALu Video_
    pub const VVCN: SampleEntryCode = SampleEntryCode(FourCC(*b"vvcN"));

    /// Versatile Video Coding with parameter sets only in sample entries
    /// 
    /// FourCC: `vvc1`
    /// 
    /// Specification: _NALu Video_
    pub const VVC1: SampleEntryCode = SampleEntryCode(FourCC(*b"vvc1"));

    /// Versatile Video Coding with parameter sets in sample entries or samples
    /// 
    /// FourCC: `vvi1`
    /// 
    /// Specification: _NALu Video_
    pub const VVI1: SampleEntryCode = SampleEntryCode(FourCC(*b"vvi1"));

    /// Versatile Video Coding (VVC) subpicture track that does not contain a conforming VVC bitstream
    /// 
    /// FourCC: `vvs1`
    /// 
    /// Specification: _NALu Video_
    pub const VVS1: SampleEntryCode = SampleEntryCode(FourCC(*b"vvs1"));

    /// WebVTT data
    /// 
    /// FourCC: `wvtt`
    /// 
    /// Specification: _ISO-Text_
    pub const WVTT: SampleEntryCode = SampleEntryCode(FourCC(*b"wvtt"));

    /// Encrypted/protected subtitles
    /// 
    /// FourCC: `encu`
    /// 
    /// Specification: _ISO_
    pub const ENCU: SampleEntryCode = SampleEntryCode(FourCC(*b"encu"));

    /// Encrypted/protected haptics
    /// 
    /// FourCC: `encp`
    /// 
    /// Specification: _ISO_
    pub const ENCP: SampleEntryCode = SampleEntryCode(FourCC(*b"encp"));

    /// Encrypted/protected volumetric visual
    /// 
    /// FourCC: `enc3`
    /// 
    /// Specification: _ISO_
    pub const ENC3: SampleEntryCode = SampleEntryCode(FourCC(*b"enc3"));

    /// Immersive Audio Model and Formats - Encapsulated IA Sequence
    /// 
    /// FourCC: `iamf`
    /// 
    /// Specification: _AOM-IAMF_
    pub const IAMF: SampleEntryCode = SampleEntryCode(FourCC(*b"iamf"));

    /// Integer based PCM format for audio
    /// 
    /// FourCC: `ipcm`
    /// 
    /// Specification: _ISO-UNCA_
    pub const IPCM: SampleEntryCode = SampleEntryCode(FourCC(*b"ipcm"));

    /// Floating-point based PCM format for audio
    /// 
    /// FourCC: `fpcm`
    /// 
    /// Specification: _ISO-UNCA_
    pub const FPCM: SampleEntryCode = SampleEntryCode(FourCC(*b"fpcm"));

}

impl BoxCode {
    /// 2D region quality ranking
    /// 
    /// FourCC: `2dqr`
    /// 
    /// Specification: _OMAF_
    pub const TWO_DQR: BoxCode = BoxCode(FourCC(*b"2dqr"));

    /// spatial relationship 2D source
    /// 
    /// FourCC: `2dss`
    /// 
    /// Specification: _OMAF_
    pub const TWO_DSS: BoxCode = BoxCode(FourCC(*b"2dss"));

    /// Asset information to identify, license and play
    /// 
    /// FourCC: `ainf`
    /// 
    /// Specification: _DECE_
    pub const AINF: BoxCode = BoxCode(FourCC(*b"ainf"));

    /// alternative startup sequence properties
    /// 
    /// FourCC: `assp`
    /// 
    /// Specification: _ISO_
    pub const ASSP: BoxCode = BoxCode(FourCC(*b"assp"));

    /// Auxiliary track type information
    /// 
    /// FourCC: `auxi`
    /// 
    /// Specification: _HEIF_
    pub const AUXI: BoxCode = BoxCode(FourCC(*b"auxi"));

    /// AVC NAL Unit Storage Box
    /// 
    /// FourCC: `avcn`
    /// 
    /// Specification: _DECE_
    pub const AVCN: BoxCode = BoxCode(FourCC(*b"avcn"));

    /// Box Index
    /// 
    /// FourCC: `bidx`
    /// 
    /// Specification: _ISO-Partial_
    pub const BIDX: BoxCode = BoxCode(FourCC(*b"bidx"));

    /// Base location and purchase location for license acquisition
    /// 
    /// FourCC: `bloc`
    /// 
    /// Specification: _DECE_
    pub const BLOC: BoxCode = BoxCode(FourCC(*b"bloc"));

    /// Buffer Model Description
    /// 
    /// FourCC: `bmdm`
    /// 
    /// Specification: _JPXS_
    pub const BMDM: BoxCode = BoxCode(FourCC(*b"bmdm"));

    /// Bits per component
    /// 
    /// FourCC: `bpcc`
    /// 
    /// Specification: _JPEG2000_
    pub const BPCC: BoxCode = BoxCode(FourCC(*b"bpcc"));

    /// Brotli-compressed box
    /// 
    /// FourCC: `brob`
    /// 
    /// Specification: _JPEG XL_
    pub const BROB: BoxCode = BoxCode(FourCC(*b"brob"));

    /// Buffering information
    /// 
    /// FourCC: `buff`
    /// 
    /// Specification: _NALu Video_
    pub const BUFF: BoxCode = BoxCode(FourCC(*b"buff"));

    /// binary XML container
    /// 
    /// FourCC: `bxml`
    /// 
    /// Specification: _ISO_
    pub const BXML: BoxCode = BoxCode(FourCC(*b"bxml"));

    /// OMA DRM Content ID
    /// 
    /// FourCC: `ccid`
    /// 
    /// Specification: _OMA DRM 2.1_
    pub const CCID: BoxCode = BoxCode(FourCC(*b"ccid"));

    /// Coding constraints box
    /// 
    /// FourCC: `ccst`
    /// 
    /// Specification: _HEIF_
    pub const CCST: BoxCode = BoxCode(FourCC(*b"ccst"));

    /// type and ordering of the components within the codestream
    /// 
    /// FourCC: `cdef`
    /// 
    /// Specification: _JPEG2000_
    pub const CDEF: BoxCode = BoxCode(FourCC(*b"cdef"));

    /// complete track information
    /// 
    /// FourCC: `cinf`
    /// 
    /// Specification: _ISO_
    pub const CINF: BoxCode = BoxCode(FourCC(*b"cinf"));

    /// Reserved
    /// 
    /// FourCC: `clip`
    /// 
    /// Specification: _ISO_
    pub const CLIP: BoxCode = BoxCode(FourCC(*b"clip"));

    /// mapping between a palette and codestream components
    /// 
    /// FourCC: `cmap`
    /// 
    /// Specification: _JPEG2000_
    pub const CMAP: BoxCode = BoxCode(FourCC(*b"cmap"));

    /// 64-bit chunk offset
    /// 
    /// FourCC: `co64`
    /// 
    /// Specification: _ISO_
    pub const CO64: BoxCode = BoxCode(FourCC(*b"co64"));

    /// composition information
    /// 
    /// FourCC: `coif`
    /// 
    /// Specification: _OMAF_
    pub const COIF: BoxCode = BoxCode(FourCC(*b"coif"));

    /// Content Information Box
    /// 
    /// FourCC: `coin`
    /// 
    /// Specification: _DECE_
    pub const COIN: BoxCode = BoxCode(FourCC(*b"coin"));

    /// specifies the colourspace of the image
    /// 
    /// FourCC: `colr`
    /// 
    /// Specification: _JPEG2000_
    pub const COLR: BoxCode = BoxCode(FourCC(*b"colr"));

    /// coverage information
    /// 
    /// FourCC: `covi`
    /// 
    /// Specification: _OMAF_
    pub const COVI: BoxCode = BoxCode(FourCC(*b"covi"));

    /// Reserved
    /// 
    /// FourCC: `crgn`
    /// 
    /// Specification: _ISO_
    pub const CRGN: BoxCode = BoxCode(FourCC(*b"crgn"));

    /// reserved for ClockReferenceStream header
    /// 
    /// FourCC: `crhd`
    /// 
    /// Specification: _MP4v1_
    pub const CRHD: BoxCode = BoxCode(FourCC(*b"crhd"));

    /// compact sample to group
    /// 
    /// FourCC: `csgp`
    /// 
    /// Specification: _ISO_
    pub const CSGP: BoxCode = BoxCode(FourCC(*b"csgp"));

    /// composition to decode timeline mapping
    /// 
    /// FourCC: `cslg`
    /// 
    /// Specification: _ISO_
    pub const CSLG: BoxCode = BoxCode(FourCC(*b"cslg"));

    /// corrected wall clock start time
    /// 
    /// FourCC: `cstb`
    /// 
    /// Specification: _ONVIF_
    pub const CSTB: BoxCode = BoxCode(FourCC(*b"cstb"));

    /// Reserved
    /// 
    /// FourCC: `ctab`
    /// 
    /// Specification: _ISO_
    pub const CTAB: BoxCode = BoxCode(FourCC(*b"ctab"));

    /// (composition) time to sample
    /// 
    /// FourCC: `ctts`
    /// 
    /// Specification: _ISO_
    pub const CTTS: BoxCode = BoxCode(FourCC(*b"ctts"));

    /// OMA DRM Cover URI
    /// 
    /// FourCC: `cvru`
    /// 
    /// Specification: _OMA DRM 2.1_
    pub const CVRU: BoxCode = BoxCode(FourCC(*b"cvru"));

    /// Data Integrity Hash
    /// 
    /// FourCC: `dihd`
    /// 
    /// Specification: _ISO-Partial_
    pub const DIHD: BoxCode = BoxCode(FourCC(*b"dihd"));

    /// data information box, container
    /// 
    /// FourCC: `dinf`
    /// 
    /// Specification: _ISO_
    pub const DINF: BoxCode = BoxCode(FourCC(*b"dinf"));

    /// Data Integrity
    /// 
    /// FourCC: `dint`
    /// 
    /// Specification: _ISO-Partial_
    pub const DINT: BoxCode = BoxCode(FourCC(*b"dint"));

    /// Mastering Display Metadata
    /// 
    /// FourCC: `dmon`
    /// 
    /// Specification: _JPXS_
    pub const DMON: BoxCode = BoxCode(FourCC(*b"dmon"));

    /// data reference box, declares source(s) of media data in track
    /// 
    /// FourCC: `dref`
    /// 
    /// Specification: _ISO_
    pub const DREF: BoxCode = BoxCode(FourCC(*b"dref"));

    /// DVB Sample Group Description Box
    /// 
    /// FourCC: `dsgd`
    /// 
    /// Specification: _DVB_
    pub const DSGD: BoxCode = BoxCode(FourCC(*b"dsgd"));

    /// DVB Sample to Group Box
    /// 
    /// FourCC: `dstg`
    /// 
    /// Specification: _DVB_
    pub const DSTG: BoxCode = BoxCode(FourCC(*b"dstg"));

    /// edit list container
    /// 
    /// FourCC: `edts`
    /// 
    /// Specification: _ISO_
    pub const EDTS: BoxCode = BoxCode(FourCC(*b"edts"));

    /// an edit list
    /// 
    /// FourCC: `elst`
    /// 
    /// Specification: _ISO_
    pub const ELST: BoxCode = BoxCode(FourCC(*b"elst"));

    /// event message
    /// 
    /// FourCC: `emsg`
    /// 
    /// Specification: _DASH_
    pub const EMSG: BoxCode = BoxCode(FourCC(*b"emsg"));

    /// Event information
    /// 
    /// FourCC: `evti`
    /// 
    /// Specification: _ATSC 3.0_
    pub const EVTI: BoxCode = BoxCode(FourCC(*b"evti"));

    /// extended type and type combination
    /// 
    /// FourCC: `etyp`
    /// 
    /// Specification: _ISO_
    pub const ETYP: BoxCode = BoxCode(FourCC(*b"etyp"));

    /// Exif Metadata
    /// 
    /// FourCC: `Exif`
    /// 
    /// Specification: _JPXS_
    pub const EXIF: BoxCode = BoxCode(FourCC(*b"Exif"));

    /// File delivery information (item info extension)
    /// 
    /// FourCC: `fdel`
    /// 
    /// Specification: _ISO_
    pub const FDEL: BoxCode = BoxCode(FourCC(*b"fdel"));

    /// FEC Informatiom
    /// 
    /// FourCC: `feci`
    /// 
    /// Specification: _ISO_
    pub const FECI: BoxCode = BoxCode(FourCC(*b"feci"));

    /// FEC Reservoir
    /// 
    /// FourCC: `fecr`
    /// 
    /// Specification: _ISO_
    pub const FECR: BoxCode = BoxCode(FourCC(*b"fecr"));

    /// Box File Index
    /// 
    /// FourCC: `fidx`
    /// 
    /// Specification: _ISO-Partial_
    pub const FIDX: BoxCode = BoxCode(FourCC(*b"fidx"));

    /// FD Item Information
    /// 
    /// FourCC: `fiin`
    /// 
    /// Specification: _ISO_
    pub const FIIN: BoxCode = BoxCode(FourCC(*b"fiin"));

    /// File Reservoir
    /// 
    /// FourCC: `fire`
    /// 
    /// Specification: _ISO_
    pub const FIRE: BoxCode = BoxCode(FourCC(*b"fire"));

    /// fisheye omnidirectional video
    /// 
    /// FourCC: `fovd`
    /// 
    /// Specification: _OMAF_
    pub const FOVD: BoxCode = BoxCode(FourCC(*b"fovd"));

    /// fisheye video essential info
    /// 
    /// FourCC: `fovi`
    /// 
    /// Specification: _OMAF_
    pub const FOVI: BoxCode = BoxCode(FourCC(*b"fovi"));

    /// File Partition
    /// 
    /// FourCC: `fpar`
    /// 
    /// Specification: _ISO_
    pub const FPAR: BoxCode = BoxCode(FourCC(*b"fpar"));

    /// free space
    /// 
    /// FourCC: `free`
    /// 
    /// Specification: _ISO_
    pub const FREE: BoxCode = BoxCode(FourCC(*b"free"));

    /// original format box
    /// 
    /// FourCC: `frma`
    /// 
    /// Specification: _ISO_
    pub const FRMA: BoxCode = BoxCode(FourCC(*b"frma"));

    /// Front Part
    /// 
    /// FourCC: `frpa`
    /// 
    /// Specification: _ISO-Partial_
    pub const FRPA: BoxCode = BoxCode(FourCC(*b"frpa"));

    /// file type and compatibility
    /// 
    /// FourCC: `ftyp`
    /// 
    /// Specification: _ISO_
    pub const FTYP: BoxCode = BoxCode(FourCC(*b"ftyp"));

    /// fisheye video supplemental info
    /// 
    /// FourCC: `fvsi`
    /// 
    /// Specification: _OMAF_
    pub const FVSI: BoxCode = BoxCode(FourCC(*b"fvsi"));

    /// Group ID to name
    /// 
    /// FourCC: `gitn`
    /// 
    /// Specification: _ISO_
    pub const GITN: BoxCode = BoxCode(FourCC(*b"gitn"));

    /// OMA DRM Group ID
    /// 
    /// FourCC: `grpi`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const GRPI: BoxCode = BoxCode(FourCC(*b"grpi"));

    /// Groups list box
    /// 
    /// FourCC: `grpl`
    /// 
    /// Specification: _ISO_
    pub const GRPL: BoxCode = BoxCode(FourCC(*b"grpl"));

    /// handler, declares the media (handler) type
    /// 
    /// FourCC: `hdlr`
    /// 
    /// Specification: _ISO_
    pub const HDLR: BoxCode = BoxCode(FourCC(*b"hdlr"));

    /// hint media header, overall information (hint track only)
    /// 
    /// FourCC: `hmhd`
    /// 
    /// Specification: _ISO_
    pub const HMHD: BoxCode = BoxCode(FourCC(*b"hmhd"));

    /// Hipix Rich Picture (user-data or meta-data)
    /// 
    /// FourCC: `hpix`
    /// 
    /// Specification: _Hipix_
    pub const HPIX: BoxCode = BoxCode(FourCC(*b"hpix"));

    /// OMA DRM Icon URI
    /// 
    /// FourCC: `icnu`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const ICNU: BoxCode = BoxCode(FourCC(*b"icnu"));

    /// ID3 version 2 container
    /// 
    /// FourCC: `ID32`
    /// 
    /// Specification: _id3v2_
    pub const ID32: BoxCode = BoxCode(FourCC(*b"ID32"));

    /// Item data
    /// 
    /// FourCC: `idat`
    /// 
    /// Specification: _ISO_
    pub const IDAT: BoxCode = BoxCode(FourCC(*b"idat"));

    /// Image Header
    /// 
    /// FourCC: `ihdr`
    /// 
    /// Specification: _JPEG2000_
    pub const IHDR: BoxCode = BoxCode(FourCC(*b"ihdr"));

    /// item information
    /// 
    /// FourCC: `iinf`
    /// 
    /// Specification: _ISO_
    pub const IINF: BoxCode = BoxCode(FourCC(*b"iinf"));

    /// item location
    /// 
    /// FourCC: `iloc`
    /// 
    /// Specification: _ISO_
    pub const ILOC: BoxCode = BoxCode(FourCC(*b"iloc"));

    /// Reserved
    /// 
    /// FourCC: `imap`
    /// 
    /// Specification: _ISO_
    pub const IMAP: BoxCode = BoxCode(FourCC(*b"imap"));

    /// Identified media data
    /// 
    /// FourCC: `imda`
    /// 
    /// Specification: _ISO_
    pub const IMDA: BoxCode = BoxCode(FourCC(*b"imda"));

    /// IPMP Information box
    /// 
    /// FourCC: `imif`
    /// 
    /// Specification: _ISO_
    pub const IMIF: BoxCode = BoxCode(FourCC(*b"imif"));

    /// Item information entry
    /// 
    /// FourCC: `infe`
    /// 
    /// Specification: _ISO_
    pub const INFE: BoxCode = BoxCode(FourCC(*b"infe"));

    /// OMA DRM Info URL
    /// 
    /// FourCC: `infu`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const INFU: BoxCode = BoxCode(FourCC(*b"infu"));

    /// Object Descriptor container box
    /// 
    /// FourCC: `iods`
    /// 
    /// Specification: _MP4v1_
    pub const IODS: BoxCode = BoxCode(FourCC(*b"iods"));

    /// ItemPropertyContainerBox
    /// 
    /// FourCC: `ipco`
    /// 
    /// Specification: _ISO_
    pub const IPCO: BoxCode = BoxCode(FourCC(*b"ipco"));

    /// reserved for IPMP Stream header
    /// 
    /// FourCC: `iphd`
    /// 
    /// Specification: _MP4v1_
    pub const IPHD: BoxCode = BoxCode(FourCC(*b"iphd"));

    /// ItemPropertyAssociation
    /// 
    /// FourCC: `ipma`
    /// 
    /// Specification: _ISO_
    pub const IPMA: BoxCode = BoxCode(FourCC(*b"ipma"));

    /// IPMP Control Box
    /// 
    /// FourCC: `ipmc`
    /// 
    /// Specification: _ISO_
    pub const IPMC: BoxCode = BoxCode(FourCC(*b"ipmc"));

    /// item protection
    /// 
    /// FourCC: `ipro`
    /// 
    /// Specification: _ISO_
    pub const IPRO: BoxCode = BoxCode(FourCC(*b"ipro"));

    /// Item Properties Box
    /// 
    /// FourCC: `iprp`
    /// 
    /// Specification: _ISO_
    pub const IPRP: BoxCode = BoxCode(FourCC(*b"iprp"));

    /// Item reference
    /// 
    /// FourCC: `iref`
    /// 
    /// Specification: _ISO_
    pub const IREF: BoxCode = BoxCode(FourCC(*b"iref"));

    /// JPEG 2000 header info
    /// 
    /// FourCC: `j2kH`
    /// 
    /// Specification: _J2KHEIF_
    pub const J2KH: BoxCode = BoxCode(FourCC(*b"j2kH"));

    /// JPEG 2000 prefix
    /// 
    /// FourCC: `j2kP`
    /// 
    /// Specification: _J2KHEIF_
    pub const J2KP: BoxCode = BoxCode(FourCC(*b"j2kP"));

    /// JPEG bitstream reconstruction data
    /// 
    /// FourCC: `jbrd`
    /// 
    /// Specification: _JPEG XL_
    pub const JBRD: BoxCode = BoxCode(FourCC(*b"jbrd"));

    /// JPEG 2000 Signature
    /// 
    /// FourCC: `jP  `
    /// 
    /// Specification: _JPEG2000_
    pub const JP  : BoxCode = BoxCode(FourCC(*b"jP  "));

    /// JPEG 2000 contiguous codestream
    /// 
    /// FourCC: `jp2c`
    /// 
    /// Specification: _JPEG2000_
    pub const JP2C: BoxCode = BoxCode(FourCC(*b"jp2c"));

    /// Header
    /// 
    /// FourCC: `jp2h`
    /// 
    /// Specification: _JPEG2000_
    pub const JP2H: BoxCode = BoxCode(FourCC(*b"jp2h"));

    /// intellectual property information
    /// 
    /// FourCC: `jp2i`
    /// 
    /// Specification: _JPEG2000_
    pub const JP2I: BoxCode = BoxCode(FourCC(*b"jp2i"));

    /// JPEG Universal Metadata Box Format (JUMBF)
    /// 
    /// FourCC: `jumb`
    /// 
    /// Specification: _JPEG Systems_
    pub const JUMB: BoxCode = BoxCode(FourCC(*b"jumb"));

    /// JPEG XL Signature
    /// 
    /// FourCC: `JXL `
    /// 
    /// Specification: _JPEG XL_
    pub const JXL : BoxCode = BoxCode(FourCC(*b"JXL "));

    /// JPEG XL Level
    /// 
    /// FourCC: `jxll`
    /// 
    /// Specification: _JPEG XL_
    pub const JXLL: BoxCode = BoxCode(FourCC(*b"jxll"));

    /// JPEG XL Codestream
    /// 
    /// FourCC: `jxlc`
    /// 
    /// Specification: _JPEG XL_
    pub const JXLC: BoxCode = BoxCode(FourCC(*b"jxlc"));

    /// JPEG XL Frame Index
    /// 
    /// FourCC: `jxli`
    /// 
    /// Specification: _JPEG XL_
    pub const JXLI: BoxCode = BoxCode(FourCC(*b"jxli"));

    /// JPEG XL Partial Codestream
    /// 
    /// FourCC: `jxlp`
    /// 
    /// Specification: _JPEG XL_
    pub const JXLP: BoxCode = BoxCode(FourCC(*b"jxlp"));

    /// JPEG XS Video Information
    /// 
    /// FourCC: `jpvi`
    /// 
    /// Specification: _JPXS_
    pub const JPVI: BoxCode = BoxCode(FourCC(*b"jpvi"));

    /// JPEG XS Video Support
    /// 
    /// FourCC: `jpvs`
    /// 
    /// Specification: _JPXS_
    pub const JPVS: BoxCode = BoxCode(FourCC(*b"jpvs"));

    /// JPEG XS Video Transport Parameter
    /// 
    /// FourCC: `jptp`
    /// 
    /// Specification: _JPXS_
    pub const JPTP: BoxCode = BoxCode(FourCC(*b"jptp"));

    /// JPEG XS Signature
    /// 
    /// FourCC: `JXS `
    /// 
    /// Specification: _JPXS_
    pub const JXS : BoxCode = BoxCode(FourCC(*b"JXS "));

    /// JPEG XS Profile and Level
    /// 
    /// FourCC: `jxpl`
    /// 
    /// Specification: _JPXS_
    pub const JXPL: BoxCode = BoxCode(FourCC(*b"jxpl"));

    /// Reserved
    /// 
    /// FourCC: `kmat`
    /// 
    /// Specification: _ISO_
    pub const KMAT: BoxCode = BoxCode(FourCC(*b"kmat"));

    /// Leval assignment
    /// 
    /// FourCC: `leva`
    /// 
    /// Specification: _ISO_
    pub const LEVA: BoxCode = BoxCode(FourCC(*b"leva"));

    /// Reserved
    /// 
    /// FourCC: `load`
    /// 
    /// Specification: _ISO_
    pub const LOAD: BoxCode = BoxCode(FourCC(*b"load"));

    /// Looping behavior
    /// 
    /// FourCC: `loop`
    /// 
    /// Specification: _WhatsApp_
    pub const LOOP: BoxCode = BoxCode(FourCC(*b"loop"));

    /// OMA DRM Lyrics URI
    /// 
    /// FourCC: `lrcu`
    /// 
    /// Specification: _OMA DRM 2.1_
    pub const LRCU: BoxCode = BoxCode(FourCC(*b"lrcu"));

    /// reserved for MPEG7Stream header
    /// 
    /// FourCC: `m7hd`
    /// 
    /// Specification: _MP4v1_
    pub const M7HD: BoxCode = BoxCode(FourCC(*b"m7hd"));

    /// Reserved
    /// 
    /// FourCC: `matt`
    /// 
    /// Specification: _ISO_
    pub const MATT: BoxCode = BoxCode(FourCC(*b"matt"));

    /// MD5IntegrityBox
    /// 
    /// FourCC: `md5i`
    /// 
    /// Specification: _HEIF_
    pub const MD5I: BoxCode = BoxCode(FourCC(*b"md5i"));

    /// media data container
    /// 
    /// FourCC: `mdat`
    /// 
    /// Specification: _ISO_
    pub const MDAT: BoxCode = BoxCode(FourCC(*b"mdat"));

    /// media header, overall information about the media
    /// 
    /// FourCC: `mdhd`
    /// 
    /// Specification: _ISO_
    pub const MDHD: BoxCode = BoxCode(FourCC(*b"mdhd"));

    /// container for the media information in a track
    /// 
    /// FourCC: `mdia`
    /// 
    /// Specification: _ISO_
    pub const MDIA: BoxCode = BoxCode(FourCC(*b"mdia"));

    /// Mutable DRM information
    /// 
    /// FourCC: `mdri`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const MDRI: BoxCode = BoxCode(FourCC(*b"mdri"));

    /// additional metadata container
    /// 
    /// FourCC: `meco`
    /// 
    /// Specification: _ISO_
    pub const MECO: BoxCode = BoxCode(FourCC(*b"meco"));

    /// movie extends header box
    /// 
    /// FourCC: `mehd`
    /// 
    /// Specification: _ISO_
    pub const MEHD: BoxCode = BoxCode(FourCC(*b"mehd"));

    /// media offset
    /// 
    /// FourCC: `meof`
    /// 
    /// Specification: _OMAF_
    pub const MEOF: BoxCode = BoxCode(FourCC(*b"meof"));

    /// metabox relation
    /// 
    /// FourCC: `mere`
    /// 
    /// Specification: _ISO_
    pub const MERE: BoxCode = BoxCode(FourCC(*b"mere"));

    /// mesh box
    /// 
    /// FourCC: `mesh`
    /// 
    /// Specification: _OMAF_
    pub const MESH: BoxCode = BoxCode(FourCC(*b"mesh"));

    /// Metadata container
    /// 
    /// FourCC: `meta`
    /// 
    /// Specification: _ISO_
    pub const META: BoxCode = BoxCode(FourCC(*b"meta"));

    /// movie fragment header
    /// 
    /// FourCC: `mfhd`
    /// 
    /// Specification: _ISO_
    pub const MFHD: BoxCode = BoxCode(FourCC(*b"mfhd"));

    /// Movie fragment random access
    /// 
    /// FourCC: `mfra`
    /// 
    /// Specification: _ISO_
    pub const MFRA: BoxCode = BoxCode(FourCC(*b"mfra"));

    /// Movie fragment random access offset
    /// 
    /// FourCC: `mfro`
    /// 
    /// Specification: _ISO_
    pub const MFRO: BoxCode = BoxCode(FourCC(*b"mfro"));

    /// media information container
    /// 
    /// FourCC: `minf`
    /// 
    /// Specification: _ISO_
    pub const MINF: BoxCode = BoxCode(FourCC(*b"minf"));

    /// reserved for MPEG-J Stream header
    /// 
    /// FourCC: `mjhd`
    /// 
    /// Specification: _MP4v1_
    pub const MJHD: BoxCode = BoxCode(FourCC(*b"mjhd"));

    /// multimap video
    /// 
    /// FourCC: `mmvi`
    /// 
    /// Specification: _V3C-SYS_
    pub const MMVI: BoxCode = BoxCode(FourCC(*b"mmvi"));

    /// movie fragment
    /// 
    /// FourCC: `moof`
    /// 
    /// Specification: _ISO_
    pub const MOOF: BoxCode = BoxCode(FourCC(*b"moof"));

    /// container for all the meta-data
    /// 
    /// FourCC: `moov`
    /// 
    /// Specification: _ISO_
    pub const MOOV: BoxCode = BoxCode(FourCC(*b"moov"));

    /// mesh omnidirectional video
    /// 
    /// FourCC: `movd`
    /// 
    /// Specification: _OMAF_
    pub const MOVD: BoxCode = BoxCode(FourCC(*b"movd"));

    /// MVC sub track view box
    /// 
    /// FourCC: `mstv`
    /// 
    /// Specification: _NALu Video_
    pub const MSTV: BoxCode = BoxCode(FourCC(*b"mstv"));

    /// Multiview group
    /// 
    /// FourCC: `mvcg`
    /// 
    /// Specification: _NALu Video_
    pub const MVCG: BoxCode = BoxCode(FourCC(*b"mvcg"));

    /// Multiview Information
    /// 
    /// FourCC: `mvci`
    /// 
    /// Specification: _NALu Video_
    pub const MVCI: BoxCode = BoxCode(FourCC(*b"mvci"));

    /// MVDDepthResolutionBox
    /// 
    /// FourCC: `3dpr`
    /// 
    /// Specification: _NALu Video_
    pub const THREE_DPR: BoxCode = BoxCode(FourCC(*b"3dpr"));

    /// movie extends box
    /// 
    /// FourCC: `mvex`
    /// 
    /// Specification: _ISO_
    pub const MVEX: BoxCode = BoxCode(FourCC(*b"mvex"));

    /// movie header, overall declarations
    /// 
    /// FourCC: `mvhd`
    /// 
    /// Specification: _ISO_
    pub const MVHD: BoxCode = BoxCode(FourCC(*b"mvhd"));

    /// Multiview Relation Attribute
    /// 
    /// FourCC: `mvra`
    /// 
    /// Specification: _NALu Video_
    pub const MVRA: BoxCode = BoxCode(FourCC(*b"mvra"));

    /// Null media header, overall information (some tracks only)
    /// 
    /// FourCC: `nmhd`
    /// 
    /// Specification: _ISO_
    pub const NMHD: BoxCode = BoxCode(FourCC(*b"nmhd"));

    /// reserved for ObjectContentInfoStream header
    /// 
    /// FourCC: `ochd`
    /// 
    /// Specification: _MP4v1_
    pub const OCHD: BoxCode = BoxCode(FourCC(*b"ochd"));

    /// OMA DRM Access Unit Format
    /// 
    /// FourCC: `odaf`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const ODAF: BoxCode = BoxCode(FourCC(*b"odaf"));

    /// OMA DRM Content Object
    /// 
    /// FourCC: `odda`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const ODDA: BoxCode = BoxCode(FourCC(*b"odda"));

    /// reserved for ObjectDescriptorStream header
    /// 
    /// FourCC: `odhd`
    /// 
    /// Specification: _MP4v1_
    pub const ODHD: BoxCode = BoxCode(FourCC(*b"odhd"));

    /// OMA DRM Discrete Media Headers
    /// 
    /// FourCC: `odhe`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const ODHE: BoxCode = BoxCode(FourCC(*b"odhe"));

    /// OMA DRM Rights Object
    /// 
    /// FourCC: `odrb`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const ODRB: BoxCode = BoxCode(FourCC(*b"odrb"));

    /// OMA DRM Container
    /// 
    /// FourCC: `odrm`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const ODRM: BoxCode = BoxCode(FourCC(*b"odrm"));

    /// OMA DRM Transaction Tracking
    /// 
    /// FourCC: `odtt`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const ODTT: BoxCode = BoxCode(FourCC(*b"odtt"));

    /// OMA DRM Common headers
    /// 
    /// FourCC: `ohdr`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const OHDR: BoxCode = BoxCode(FourCC(*b"ohdr"));

    /// OMAF timed text configuration
    /// 
    /// FourCC: `otcf`
    /// 
    /// Specification: _OMAF_
    pub const OTCF: BoxCode = BoxCode(FourCC(*b"otcf"));

    /// Original file type
    /// 
    /// FourCC: `otyp`
    /// 
    /// Specification: _ISO_
    pub const OTYP: BoxCode = BoxCode(FourCC(*b"otyp"));

    /// overlay configuration
    /// 
    /// FourCC: `ovly`
    /// 
    /// Specification: _OMAF_
    pub const OVLY: BoxCode = BoxCode(FourCC(*b"ovly"));

    /// sample padding bits
    /// 
    /// FourCC: `padb`
    /// 
    /// Specification: _ISO_
    pub const PADB: BoxCode = BoxCode(FourCC(*b"padb"));

    /// Partition Entry
    /// 
    /// FourCC: `paen`
    /// 
    /// Specification: _ISO_
    pub const PAEN: BoxCode = BoxCode(FourCC(*b"paen"));

    /// palette which maps a single component in index space to a multiple- component image
    /// 
    /// FourCC: `pclr`
    /// 
    /// Specification: _JPEG2000_
    pub const PCLR: BoxCode = BoxCode(FourCC(*b"pclr"));

    /// Partial Data
    /// 
    /// FourCC: `pdat`
    /// 
    /// Specification: _ISO-Partial_
    pub const PDAT: BoxCode = BoxCode(FourCC(*b"pdat"));

    /// Progressive download information
    /// 
    /// FourCC: `pdin`
    /// 
    /// Specification: _ISO_
    pub const PDIN: BoxCode = BoxCode(FourCC(*b"pdin"));

    /// Partial File Header
    /// 
    /// FourCC: `pfhd`
    /// 
    /// Specification: _ISO-Partial_
    pub const PFHD: BoxCode = BoxCode(FourCC(*b"pfhd"));

    /// Partial File
    /// 
    /// FourCC: `pfil`
    /// 
    /// Specification: _ISO-Partial_
    pub const PFIL: BoxCode = BoxCode(FourCC(*b"pfil"));

    /// primary item reference
    /// 
    /// FourCC: `pitm`
    /// 
    /// Specification: _ISO_
    pub const PITM: BoxCode = BoxCode(FourCC(*b"pitm"));

    /// Partial Segment Location
    /// 
    /// FourCC: `ploc`
    /// 
    /// Specification: _ISO-Partial_
    pub const PLOC: BoxCode = BoxCode(FourCC(*b"ploc"));

    /// Reserved
    /// 
    /// FourCC: `pnot`
    /// 
    /// Specification: _ISO_
    pub const PNOT: BoxCode = BoxCode(FourCC(*b"pnot"));

    /// projected omnidirectional video
    /// 
    /// FourCC: `povd`
    /// 
    /// Specification: _OMAF_
    pub const POVD: BoxCode = BoxCode(FourCC(*b"povd"));

    /// projection format
    /// 
    /// FourCC: `prfr`
    /// 
    /// Specification: _OMAF_
    pub const PRFR: BoxCode = BoxCode(FourCC(*b"prfr"));

    /// Producer reference time
    /// 
    /// FourCC: `prft`
    /// 
    /// Specification: _ISO_
    pub const PRFT: BoxCode = BoxCode(FourCC(*b"prft"));

    /// Partial Segment
    /// 
    /// FourCC: `pseg`
    /// 
    /// Specification: _ISO-Partial_
    pub const PSEG: BoxCode = BoxCode(FourCC(*b"pseg"));

    /// Partial Segment Header
    /// 
    /// FourCC: `pshd`
    /// 
    /// Specification: _ISO-Partial_
    pub const PSHD: BoxCode = BoxCode(FourCC(*b"pshd"));

    /// Protection system specific header
    /// 
    /// FourCC: `pssh`
    /// 
    /// Specification: _ISO Common Encryption_
    pub const PSSH: BoxCode = BoxCode(FourCC(*b"pssh"));

    /// Partial Top Level Entry
    /// 
    /// FourCC: `ptle`
    /// 
    /// Specification: _ISO-Partial_
    pub const PTLE: BoxCode = BoxCode(FourCC(*b"ptle"));

    /// grid resolution
    /// 
    /// FourCC: `res `
    /// 
    /// Specification: _JPEG2000_
    pub const RES : BoxCode = BoxCode(FourCC(*b"res "));

    /// grid resolution at which the image was captured
    /// 
    /// FourCC: `resc`
    /// 
    /// Specification: _JPEG2000_
    pub const RESC: BoxCode = BoxCode(FourCC(*b"resc"));

    /// default grid resolution at which the image should be displayed
    /// 
    /// FourCC: `resd`
    /// 
    /// Specification: _JPEG2000_
    pub const RESD: BoxCode = BoxCode(FourCC(*b"resd"));

    /// restricted scheme information box
    /// 
    /// FourCC: `rinf`
    /// 
    /// Specification: _ISO_
    pub const RINF: BoxCode = BoxCode(FourCC(*b"rinf"));

    /// rotation box
    /// 
    /// FourCC: `rotn`
    /// 
    /// Specification: _OMAF_
    pub const ROTN: BoxCode = BoxCode(FourCC(*b"rotn"));

    /// sphere region configuration
    /// 
    /// FourCC: `rosc`
    /// 
    /// Specification: _OMAF_
    pub const ROSC: BoxCode = BoxCode(FourCC(*b"rosc"));

    /// recommended viewport information
    /// 
    /// FourCC: `rvif`
    /// 
    /// Specification: _OMAF_
    pub const RVIF: BoxCode = BoxCode(FourCC(*b"rvif"));

    /// region-wise packing
    /// 
    /// FourCC: `rwpk`
    /// 
    /// Specification: _OMAF_
    pub const RWPK: BoxCode = BoxCode(FourCC(*b"rwpk"));

    /// Sample auxiliary information offsets
    /// 
    /// FourCC: `saio`
    /// 
    /// Specification: _ISO_
    pub const SAIO: BoxCode = BoxCode(FourCC(*b"saio"));

    /// Sample auxiliary information sizes
    /// 
    /// FourCC: `saiz`
    /// 
    /// Specification: _ISO_
    pub const SAIZ: BoxCode = BoxCode(FourCC(*b"saiz"));

    /// Sample to Group box
    /// 
    /// FourCC: `sbgp`
    /// 
    /// Specification: _ISO_
    pub const SBGP: BoxCode = BoxCode(FourCC(*b"sbgp"));

    /// scheme information box
    /// 
    /// FourCC: `schi`
    /// 
    /// Specification: _ISO_
    pub const SCHI: BoxCode = BoxCode(FourCC(*b"schi"));

    /// scheme type box
    /// 
    /// FourCC: `schm`
    /// 
    /// Specification: _ISO_
    pub const SCHM: BoxCode = BoxCode(FourCC(*b"schm"));

    /// Sample dependency
    /// 
    /// FourCC: `sdep`
    /// 
    /// Specification: _NALu Video_
    pub const SDEP: BoxCode = BoxCode(FourCC(*b"sdep"));

    /// reserved for SceneDescriptionStream header
    /// 
    /// FourCC: `sdhd`
    /// 
    /// Specification: _MP4v1_
    pub const SDHD: BoxCode = BoxCode(FourCC(*b"sdhd"));

    /// Independent and Disposable Samples Box
    /// 
    /// FourCC: `sdtp`
    /// 
    /// Specification: _ISO_
    pub const SDTP: BoxCode = BoxCode(FourCC(*b"sdtp"));

    /// SD Profile Box
    /// 
    /// FourCC: `sdvp`
    /// 
    /// Specification: _SDV_
    pub const SDVP: BoxCode = BoxCode(FourCC(*b"sdvp"));

    /// file delivery session group
    /// 
    /// FourCC: `segr`
    /// 
    /// Specification: _ISO_
    pub const SEGR: BoxCode = BoxCode(FourCC(*b"segr"));

    /// SEI information box
    /// 
    /// FourCC: `seii`
    /// 
    /// Specification: _NALu Video_
    pub const SEII: BoxCode = BoxCode(FourCC(*b"seii"));

    /// Sample specific encryption data
    /// 
    /// FourCC: `senc`
    /// 
    /// Specification: _ISO Common Encryption_
    pub const SENC: BoxCode = BoxCode(FourCC(*b"senc"));

    /// Sample group definition box
    /// 
    /// FourCC: `sgpd`
    /// 
    /// Specification: _ISO_
    pub const SGPD: BoxCode = BoxCode(FourCC(*b"sgpd"));

    /// Segment Index Box
    /// 
    /// FourCC: `sidx`
    /// 
    /// Specification: _ISO_
    pub const SIDX: BoxCode = BoxCode(FourCC(*b"sidx"));

    /// protection scheme information box
    /// 
    /// FourCC: `sinf`
    /// 
    /// Specification: _ISO_
    pub const SINF: BoxCode = BoxCode(FourCC(*b"sinf"));

    /// free space
    /// 
    /// FourCC: `skip`
    /// 
    /// Specification: _ISO_
    pub const SKIP: BoxCode = BoxCode(FourCC(*b"skip"));

    /// sound media header, overall information (sound track only)
    /// 
    /// FourCC: `smhd`
    /// 
    /// Specification: _ISO_
    pub const SMHD: BoxCode = BoxCode(FourCC(*b"smhd"));

    /// sub-picture region
    /// 
    /// FourCC: `sprg`
    /// 
    /// Specification: _OMAF_
    pub const SPRG: BoxCode = BoxCode(FourCC(*b"sprg"));

    /// System Renewability Message
    /// 
    /// FourCC: `srmb`
    /// 
    /// Specification: _DVB_
    pub const SRMB: BoxCode = BoxCode(FourCC(*b"srmb"));

    /// System Renewability Message container
    /// 
    /// FourCC: `srmc`
    /// 
    /// Specification: _DVB_
    pub const SRMC: BoxCode = BoxCode(FourCC(*b"srmc"));

    /// STRP Process
    /// 
    /// FourCC: `srpp`
    /// 
    /// Specification: _ISO_
    pub const SRPP: BoxCode = BoxCode(FourCC(*b"srpp"));

    /// sphere region quality ranking
    /// 
    /// FourCC: `srqr`
    /// 
    /// Specification: _OMAF_
    pub const SRQR: BoxCode = BoxCode(FourCC(*b"srqr"));

    /// Sub-sample index
    /// 
    /// FourCC: `ssix`
    /// 
    /// Specification: _ISO_
    pub const SSIX: BoxCode = BoxCode(FourCC(*b"ssix"));

    /// SVC sub track layer box
    /// 
    /// FourCC: `sstl`
    /// 
    /// Specification: _NALu Video_
    pub const SSTL: BoxCode = BoxCode(FourCC(*b"sstl"));

    /// sample table box, container for the time/space map
    /// 
    /// FourCC: `stbl`
    /// 
    /// Specification: _ISO_
    pub const STBL: BoxCode = BoxCode(FourCC(*b"stbl"));

    /// chunk offset, partial data-offset information
    /// 
    /// FourCC: `stco`
    /// 
    /// Specification: _ISO_
    pub const STCO: BoxCode = BoxCode(FourCC(*b"stco"));

    /// sample degradation priority
    /// 
    /// FourCC: `stdp`
    /// 
    /// Specification: _ISO_
    pub const STDP: BoxCode = BoxCode(FourCC(*b"stdp"));

    /// Subtitle Media Header Box
    /// 
    /// FourCC: `sthd`
    /// 
    /// Specification: _ISO_
    pub const STHD: BoxCode = BoxCode(FourCC(*b"sthd"));

    /// MVC sub track multiview group box
    /// 
    /// FourCC: `stmg`
    /// 
    /// Specification: _NALu Video_
    pub const STMG: BoxCode = BoxCode(FourCC(*b"stmg"));

    /// Sub-track definition
    /// 
    /// FourCC: `strd`
    /// 
    /// Specification: _ISO_
    pub const STRD: BoxCode = BoxCode(FourCC(*b"strd"));

    /// Sub-track information
    /// 
    /// FourCC: `stri`
    /// 
    /// Specification: _ISO_
    pub const STRI: BoxCode = BoxCode(FourCC(*b"stri"));

    /// sample-to-chunk, partial data-offset information
    /// 
    /// FourCC: `stsc`
    /// 
    /// Specification: _ISO_
    pub const STSC: BoxCode = BoxCode(FourCC(*b"stsc"));

    /// sample descriptions (codec types, initialization etc.)
    /// 
    /// FourCC: `stsd`
    /// 
    /// Specification: _ISO_
    pub const STSD: BoxCode = BoxCode(FourCC(*b"stsd"));

    /// Sub-track sample grouping
    /// 
    /// FourCC: `stsg`
    /// 
    /// Specification: _ISO_
    pub const STSG: BoxCode = BoxCode(FourCC(*b"stsg"));

    /// shadow sync sample table
    /// 
    /// FourCC: `stsh`
    /// 
    /// Specification: _ISO_
    pub const STSH: BoxCode = BoxCode(FourCC(*b"stsh"));

    /// sync sample table (random access points)
    /// 
    /// FourCC: `stss`
    /// 
    /// Specification: _ISO_
    pub const STSS: BoxCode = BoxCode(FourCC(*b"stss"));

    /// sample sizes (framing)
    /// 
    /// FourCC: `stsz`
    /// 
    /// Specification: _ISO_
    pub const STSZ: BoxCode = BoxCode(FourCC(*b"stsz"));

    /// Sub track tier box
    /// 
    /// FourCC: `stti`
    /// 
    /// Specification: _NALu Video_
    pub const STTI: BoxCode = BoxCode(FourCC(*b"stti"));

    /// (decoding) time-to-sample
    /// 
    /// FourCC: `stts`
    /// 
    /// Specification: _ISO_
    pub const STTS: BoxCode = BoxCode(FourCC(*b"stts"));

    /// Segment Type Box
    /// 
    /// FourCC: `styp`
    /// 
    /// Specification: _ISO_
    pub const STYP: BoxCode = BoxCode(FourCC(*b"styp"));

    /// compact sample sizes (framing)
    /// 
    /// FourCC: `stz2`
    /// 
    /// Specification: _ISO_
    pub const STZ2: BoxCode = BoxCode(FourCC(*b"stz2"));

    /// Sub-sample information
    /// 
    /// FourCC: `subs`
    /// 
    /// Specification: _ISO_
    pub const SUBS: BoxCode = BoxCode(FourCC(*b"subs"));

    /// signer identity information
    /// 
    /// FourCC: `suep`
    /// 
    /// Specification: _ONVIF_
    pub const SUEP: BoxCode = BoxCode(FourCC(*b"suep"));

    /// supplemental surveillance meta information
    /// 
    /// FourCC: `sumi`
    /// 
    /// Specification: _MPEG-VSAF_
    pub const SUMI: BoxCode = BoxCode(FourCC(*b"sumi"));

    /// Source URL
    /// 
    /// FourCC: `surl`
    /// 
    /// Specification: _ISO-Partial_
    pub const SURL: BoxCode = BoxCode(FourCC(*b"surl"));

    /// Multiview Group Relation
    /// 
    /// FourCC: `swtc`
    /// 
    /// Specification: _NALu Video_
    pub const SWTC: BoxCode = BoxCode(FourCC(*b"swtc"));

    /// Track Encryption
    /// 
    /// FourCC: `tenc`
    /// 
    /// Specification: _ISO Common Encryption_
    pub const TENC: BoxCode = BoxCode(FourCC(*b"tenc"));

    /// Track fragment adjustment box
    /// 
    /// FourCC: `tfad`
    /// 
    /// Specification: _3GPP_
    pub const TFAD: BoxCode = BoxCode(FourCC(*b"tfad"));

    /// Track fragment decode time
    /// 
    /// FourCC: `tfdt`
    /// 
    /// Specification: _ISO_
    pub const TFDT: BoxCode = BoxCode(FourCC(*b"tfdt"));

    /// Track fragment header
    /// 
    /// FourCC: `tfhd`
    /// 
    /// Specification: _ISO_
    pub const TFHD: BoxCode = BoxCode(FourCC(*b"tfhd"));

    /// Track fragment media adjustment box
    /// 
    /// FourCC: `tfma`
    /// 
    /// Specification: _3GPP_
    pub const TFMA: BoxCode = BoxCode(FourCC(*b"tfma"));

    /// Track fragment radom access
    /// 
    /// FourCC: `tfra`
    /// 
    /// Specification: _ISO_
    pub const TFRA: BoxCode = BoxCode(FourCC(*b"tfra"));

    /// Tier Bit rate
    /// 
    /// FourCC: `tibr`
    /// 
    /// Specification: _NALu Video_
    pub const TIBR: BoxCode = BoxCode(FourCC(*b"tibr"));

    /// Tier Information
    /// 
    /// FourCC: `tiri`
    /// 
    /// Specification: _NALu Video_
    pub const TIRI: BoxCode = BoxCode(FourCC(*b"tiri"));

    /// Track header, overall information about the track
    /// 
    /// FourCC: `tkhd`
    /// 
    /// Specification: _ISO_
    pub const TKHD: BoxCode = BoxCode(FourCC(*b"tkhd"));

    /// Track fragment
    /// 
    /// FourCC: `traf`
    /// 
    /// Specification: _ISO_
    pub const TRAF: BoxCode = BoxCode(FourCC(*b"traf"));

    /// container for an individual track or stream
    /// 
    /// FourCC: `trak`
    /// 
    /// Specification: _ISO_
    pub const TRAK: BoxCode = BoxCode(FourCC(*b"trak"));

    /// track reference container
    /// 
    /// FourCC: `tref`
    /// 
    /// Specification: _ISO_
    pub const TREF: BoxCode = BoxCode(FourCC(*b"tref"));

    /// track extension properties
    /// 
    /// FourCC: `trep`
    /// 
    /// Specification: _ISO_
    pub const TREP: BoxCode = BoxCode(FourCC(*b"trep"));

    /// track extends defaults
    /// 
    /// FourCC: `trex`
    /// 
    /// Specification: _ISO_
    pub const TREX: BoxCode = BoxCode(FourCC(*b"trex"));

    /// Track grouping information
    /// 
    /// FourCC: `trgr`
    /// 
    /// Specification: _ISO_
    pub const TRGR: BoxCode = BoxCode(FourCC(*b"trgr"));

    /// Facilitates random access and trick play modes
    /// 
    /// FourCC: `trik`
    /// 
    /// Specification: _DECE_
    pub const TRIK: BoxCode = BoxCode(FourCC(*b"trik"));

    /// track fragment run
    /// 
    /// FourCC: `trun`
    /// 
    /// Specification: _ISO_
    pub const TRUN: BoxCode = BoxCode(FourCC(*b"trun"));

    /// TileSubTrackGroupBox
    /// 
    /// FourCC: `tstb`
    /// 
    /// Specification: _NALu Video_
    pub const TSTB: BoxCode = BoxCode(FourCC(*b"tstb"));

    /// track type and compatibility
    /// 
    /// FourCC: `ttyp`
    /// 
    /// Specification: _ISO_
    pub const TTYP: BoxCode = BoxCode(FourCC(*b"ttyp"));

    /// type and-combination
    /// 
    /// FourCC: `tyco`
    /// 
    /// Specification: _ISO_
    pub const TYCO: BoxCode = BoxCode(FourCC(*b"tyco"));

    /// user-data
    /// 
    /// FourCC: `udta`
    /// 
    /// Specification: _ISO_
    pub const UDTA: BoxCode = BoxCode(FourCC(*b"udta"));

    /// a tool by which a vendor may provide access to additional information associated with a UUID
    /// 
    /// FourCC: `uinf`
    /// 
    /// Specification: _JPEG2000_
    pub const UINF: BoxCode = BoxCode(FourCC(*b"uinf"));

    /// Unique Identifier Technology Solution
    /// 
    /// FourCC: `UITS`
    /// 
    /// Specification: _Universal Music Group_
    pub const UITS: BoxCode = BoxCode(FourCC(*b"UITS"));

    /// a list of UUID’s
    /// 
    /// FourCC: `ulst`
    /// 
    /// Specification: _JPEG2000_
    pub const ULST: BoxCode = BoxCode(FourCC(*b"ulst"));

    /// a URL
    /// 
    /// FourCC: `url `
    /// 
    /// Specification: _JPEG2000_
    pub const URL : BoxCode = BoxCode(FourCC(*b"url "));

    /// user-extension box
    /// 
    /// FourCC: `uuid`
    /// 
    /// Specification: _ISO_
    pub const UUID: BoxCode = BoxCode(FourCC(*b"uuid"));

    /// V3C spatial region collection
    /// 
    /// FourCC: `v3sc`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3SC: BoxCode = BoxCode(FourCC(*b"v3sc"));

    /// video media header, overall information (video track only)
    /// 
    /// FourCC: `vmhd`
    /// 
    /// Specification: _ISO_
    pub const VMHD: BoxCode = BoxCode(FourCC(*b"vmhd"));

    /// Volumetric media bounding box
    /// 
    /// FourCC: `vpbb`
    /// 
    /// Specification: _V3C-SYS_
    pub const VPBB: BoxCode = BoxCode(FourCC(*b"vpbb"));

    /// viewing space
    /// 
    /// FourCC: `vssn`
    /// 
    /// Specification: _OMAF_
    pub const VSSN: BoxCode = BoxCode(FourCC(*b"vssn"));

    /// V3C unit header
    /// 
    /// FourCC: `vunt`
    /// 
    /// Specification: _V3C-SYS_
    pub const VUNT: BoxCode = BoxCode(FourCC(*b"vunt"));

    /// volumetric visual media header
    /// 
    /// FourCC: `vvhd`
    /// 
    /// Specification: _ISO_
    pub const VVHD: BoxCode = BoxCode(FourCC(*b"vvhd"));

    /// Multiview Scene Information
    /// 
    /// FourCC: `vwdi`
    /// 
    /// Specification: _NALu Video_
    pub const VWDI: BoxCode = BoxCode(FourCC(*b"vwdi"));

    /// segment position in the watermark pattern
    /// 
    /// FourCC: `wmpi`
    /// 
    /// Specification: _DASH-IF watermarking_
    pub const WMPI: BoxCode = BoxCode(FourCC(*b"wmpi"));

    /// XML container
    /// 
    /// FourCC: `xml `
    /// 
    /// Specification: _ISO_
    pub const XML : BoxCode = BoxCode(FourCC(*b"xml "));

    /// Compressed movie fragment
    /// 
    /// FourCC: `!mof`
    /// 
    /// Specification: _ISO_
    pub const COMPRESSED_MOF: BoxCode = BoxCode(FourCC(*b"!mof"));

    /// Compressed movie
    /// 
    /// FourCC: `!mov`
    /// 
    /// Specification: _ISO_
    pub const COMPRESSED_MOV: BoxCode = BoxCode(FourCC(*b"!mov"));

    /// Compressed segment index
    /// 
    /// FourCC: `!six`
    /// 
    /// Specification: _ISO_
    pub const COMPRESSED_SIX: BoxCode = BoxCode(FourCC(*b"!six"));

    /// Compressed subsegment index
    /// 
    /// FourCC: `!ssx`
    /// 
    /// Specification: _ISO_
    pub const COMPRESSED_SSX: BoxCode = BoxCode(FourCC(*b"!ssx"));

    /// SVC region of interest box
    /// 
    /// FourCC: `iroi`
    /// 
    /// Specification: _NALu Video_
    pub const IROI: BoxCode = BoxCode(FourCC(*b"iroi"));

    /// Tier dependency box
    /// 
    /// FourCC: `ldep`
    /// 
    /// Specification: _NALu Video_
    pub const LDEP: BoxCode = BoxCode(FourCC(*b"ldep"));

    /// SVC rect region box
    /// 
    /// FourCC: `rrgn`
    /// 
    /// Specification: _NALu Video_
    pub const RRGN: BoxCode = BoxCode(FourCC(*b"rrgn"));

    /// SVC dependency range
    /// 
    /// FourCC: `svdr`
    /// 
    /// Specification: _NALu Video_
    pub const SVDR: BoxCode = BoxCode(FourCC(*b"svdr"));

    /// Initial parameter sets box for tiers
    /// 
    /// FourCC: `svip`
    /// 
    /// Specification: _NALu Video_
    pub const SVIP: BoxCode = BoxCode(FourCC(*b"svip"));

    /// Priority range
    /// 
    /// FourCC: `svpr`
    /// 
    /// Specification: _NALu Video_
    pub const SVPR: BoxCode = BoxCode(FourCC(*b"svpr"));

    /// SVC lightweight transcoding
    /// 
    /// FourCC: `tran`
    /// 
    /// Specification: _NALu Video_
    pub const TRAN: BoxCode = BoxCode(FourCC(*b"tran"));

    /// View priority Box
    /// 
    /// FourCC: `vipr`
    /// 
    /// Specification: _NALu Video_
    pub const VIPR: BoxCode = BoxCode(FourCC(*b"vipr"));

    /// SDP information
    /// 
    /// FourCC: `sdp `
    /// 
    /// Specification: _ISO_
    pub const SDP : BoxCode = BoxCode(FourCC(*b"sdp "));

    /// Stereo Video Box
    /// 
    /// FourCC: `stvi`
    /// 
    /// Specification: _ISO_
    pub const STVI: BoxCode = BoxCode(FourCC(*b"stvi"));

}

impl ObjectTypeIdentifier {
    /// Forbidden
    /// 
    /// Type value: `0x00`
    pub const FORBIDDEN: ObjectTypeIdentifier = ObjectTypeIdentifier(0x00);

    /// Systems ISO/IEC 14496-1
    /// 
    /// Type value: `0x01`
    /// 
    /// Specification: _MPEG-4_
    pub const SYSTEMS_ISO_IEC_14496_1: ObjectTypeIdentifier = ObjectTypeIdentifier(0x01);

    /// Systems ISO/IEC 14496-1
    /// 
    /// Type value: `0x02`
    /// 
    /// Specification: _MPEG-4_
    pub const SYSTEMS_ISO_IEC_14496_1_BIFS_V_2_CONFIG: ObjectTypeIdentifier = ObjectTypeIdentifier(0x02);

    /// Interaction Stream
    /// 
    /// Type value: `0x03`
    /// 
    /// Specification: _MPEG-4_
    pub const INTERACTION_STREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(0x03);

    /// Extended BIFS
    /// 
    /// Type value: `0x04`
    /// 
    /// Specification: _MPEG-4_
    pub const EXTENDED_BIFS: ObjectTypeIdentifier = ObjectTypeIdentifier(0x04);

    /// AFX Stream
    /// 
    /// Type value: `0x05`
    /// 
    /// Specification: _MPEG-4_
    pub const AFX_STREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(0x05);

    /// Font Data Stream
    /// 
    /// Type value: `0x06`
    /// 
    /// Specification: _MPEG-4_
    pub const FONT_DATA_STREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(0x06);

    /// Synthetised Texture
    /// 
    /// Type value: `0x07`
    /// 
    /// Specification: _MPEG-4_
    pub const SYNTHETISED_TEXTURE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x07);

    /// Text Stream
    /// 
    /// Type value: `0x08`
    /// 
    /// Specification: _MPEG-4_
    pub const TEXT_STREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(0x08);

    /// LASeR Stream
    /// 
    /// Type value: `0x09`
    /// 
    /// Specification: _MPEG-4_
    pub const LA_SE_R_STREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(0x09);

    /// Simple Aggregation Format  Stream
    /// 
    /// Type value: `0x0A`
    /// 
    /// Specification: _MPEG-4_
    pub const SIMPLE_AGGREGATION_FORMAT_STREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(0x0A);

    /// Visual ISO/IEC 14496-2
    /// 
    /// Type value: `0x20`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_14496_2: ObjectTypeIdentifier = ObjectTypeIdentifier(0x20);

    /// Visual ITU-T Recommendation H.264 | ISO/IEC 14496-10
    /// 
    /// Type value: `0x21`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10: ObjectTypeIdentifier = ObjectTypeIdentifier(0x21);

    /// Parameter Sets for ITU-T Recommendation H.264 | ISO/IEC 14496-10
    /// 
    /// Type value: `0x22`
    /// 
    /// Specification: _MPEG-4_
    pub const PARAMETER_SETS_FOR_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10: ObjectTypeIdentifier = ObjectTypeIdentifier(0x22);

    /// Visual ISO/IEC 23008-2 | ITU-T Recommendation H.265
    /// 
    /// Type value: `0x23`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_23008_2_ITU_T_RECOMMENDATION_H_265: ObjectTypeIdentifier = ObjectTypeIdentifier(0x23);

    /// Audio ISO/IEC 14496-3
    /// 
    /// Type value: `0x40`
    /// 
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_14496_3: ObjectTypeIdentifier = ObjectTypeIdentifier(0x40);

    /// Visual ISO/IEC 13818-2 Simple Profile
    /// 
    /// Type value: `0x60`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_SIMPLE_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x60);

    /// Visual ISO/IEC 13818-2 Main Profile
    /// 
    /// Type value: `0x61`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_MAIN_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x61);

    /// Visual ISO/IEC 13818-2 SNR Profile
    /// 
    /// Type value: `0x62`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_SNR_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x62);

    /// Visual ISO/IEC 13818-2 Spatial Profile
    /// 
    /// Type value: `0x63`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_SPATIAL_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x63);

    /// Visual ISO/IEC 13818-2 High Profile
    /// 
    /// Type value: `0x64`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_HIGH_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x64);

    /// Visual ISO/IEC 13818-2 422 Profile
    /// 
    /// Type value: `0x65`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_422_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x65);

    /// Audio ISO/IEC 13818-7 Main Profile
    /// 
    /// Type value: `0x66`
    /// 
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_13818_7_MAIN_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x66);

    /// Audio ISO/IEC 13818-7 LowComplexity Profile
    /// 
    /// Type value: `0x67`
    /// 
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_13818_7_LOW_COMPLEXITY_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x67);

    /// Audio ISO/IEC 13818-7 Scaleable Sampling Rate Profile
    /// 
    /// Type value: `0x68`
    /// 
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_13818_7_SCALEABLE_SAMPLING_RATE_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(0x68);

    /// Audio ISO/IEC 13818-3
    /// 
    /// Type value: `0x69`
    /// 
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_13818_3: ObjectTypeIdentifier = ObjectTypeIdentifier(0x69);

    /// Visual ISO/IEC 11172-2
    /// 
    /// Type value: `0x6A`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_11172_2: ObjectTypeIdentifier = ObjectTypeIdentifier(0x6A);

    /// Audio ISO/IEC 11172-3
    /// 
    /// Type value: `0x6B`
    /// 
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_11172_3: ObjectTypeIdentifier = ObjectTypeIdentifier(0x6B);

    /// Visual ISO/IEC 10918-1
    /// 
    /// Type value: `0x6C`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_10918_1: ObjectTypeIdentifier = ObjectTypeIdentifier(0x6C);

    /// Portable Network Graphics
    /// 
    /// Type value: `0x6D`
    /// 
    /// Specification: _PNG_
    pub const PORTABLE_NETWORK_GRAPHICS: ObjectTypeIdentifier = ObjectTypeIdentifier(0x6D);

    /// Visual ISO/IEC 15444-1
    /// 
    /// Type value: `0x6E`
    /// 
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_15444_1: ObjectTypeIdentifier = ObjectTypeIdentifier(0x6E);

    /// EVRC Voice
    /// 
    /// Type value: `0xA0`
    /// 
    /// Specification: _3GPP2_
    pub const EVRC_VOICE: ObjectTypeIdentifier = ObjectTypeIdentifier(0xA0);

    /// SMV Voice
    /// 
    /// Type value: `0xA1`
    /// 
    /// Specification: _3GPP2_
    pub const SMV_VOICE: ObjectTypeIdentifier = ObjectTypeIdentifier(0xA1);

    /// 3GPP2 Compact Multimedia Format
    /// 
    /// Type value: `0xA2`
    /// 
    /// Specification: _3GPP2_
    pub const THREE_GPP_2_COMPACT_MULTIMEDIA_FORMAT: ObjectTypeIdentifier = ObjectTypeIdentifier(0xA2);

    /// SMPTE VC-1 Video
    /// 
    /// Type value: `0xA3`
    /// 
    /// Specification: _SMPTE_
    pub const SMPTE_VC_1_VIDEO: ObjectTypeIdentifier = ObjectTypeIdentifier(0xA3);

    /// Dirac Video Coder
    /// 
    /// Type value: `0xA4`
    /// 
    /// Specification: _Dirac_
    pub const DIRAC_VIDEO_CODER: ObjectTypeIdentifier = ObjectTypeIdentifier(0xA4);

    /// DRA Audio
    /// 
    /// Type value: `0xA7`
    /// 
    /// Specification: _DRA_
    pub const DRA_AUDIO: ObjectTypeIdentifier = ObjectTypeIdentifier(0xA7);

    /// ITU G.719 Audio
    /// 
    /// Type value: `0xA8`
    /// 
    /// Specification: _ITU G.719_
    pub const ITU_G_719_AUDIO: ObjectTypeIdentifier = ObjectTypeIdentifier(0xA8);

    /// Core Substream
    /// 
    /// Type value: `0xA9`
    /// 
    /// Specification: _DTS-HD_
    pub const CORE_SUBSTREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(0xA9);

    /// Core Substream + Extension Substream
    /// 
    /// Type value: `0xAA`
    /// 
    /// Specification: _DTS-HD_
    pub const CORE_SUBSTREAM_EXTENSION_SUBSTREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(0xAA);

    /// Extension Substream containing only XLL
    /// 
    /// Type value: `0xAB`
    /// 
    /// Specification: _DTS-HD_
    pub const EXTENSION_SUBSTREAM_CONTAINING_ONLY_XLL: ObjectTypeIdentifier = ObjectTypeIdentifier(0xAB);

    /// Extension Substream containing only LBR
    /// 
    /// Type value: `0xAC`
    /// 
    /// Specification: _DTS-HD_
    pub const EXTENSION_SUBSTREAM_CONTAINING_ONLY_LBR: ObjectTypeIdentifier = ObjectTypeIdentifier(0xAC);

    /// Opus audio
    /// 
    /// Type value: `0xAD`
    /// 
    /// Specification: _Opus_
    pub const OPUS_AUDIO: ObjectTypeIdentifier = ObjectTypeIdentifier(0xAD);

    /// Auro-Cx 3D audio
    /// 
    /// Type value: `0xAF`
    /// 
    /// Specification: _Auro_
    pub const AURO_CX_3_D_AUDIO: ObjectTypeIdentifier = ObjectTypeIdentifier(0xAF);

    /// RealVideo Codec 11
    /// 
    /// Type value: `0xB0`
    /// 
    /// Specification: _RealHD_
    pub const REAL_VIDEO_CODEC_11: ObjectTypeIdentifier = ObjectTypeIdentifier(0xB0);

    /// VP9 Video
    /// 
    /// Type value: `0xB1`
    /// 
    /// Specification: _VPxx_
    pub const VP_9_VIDEO: ObjectTypeIdentifier = ObjectTypeIdentifier(0xB1);

    /// DTS-UHD profile 2
    /// 
    /// Type value: `0xB2`
    /// 
    /// Specification: _DTS-UHD_
    pub const DTS_UHD_PROFILE_2: ObjectTypeIdentifier = ObjectTypeIdentifier(0xB2);

    /// DTS-UHD profile 3 or higher
    /// 
    /// Type value: `0xB3`
    /// 
    /// Specification: _DTS-UHD_
    pub const DTS_UHD_PROFILE_3_OR_HIGHER: ObjectTypeIdentifier = ObjectTypeIdentifier(0xB3);

    /// 13K Voice
    /// 
    /// Type value: `0xE1`
    /// 
    /// Specification: _3GPP2_
    pub const ONE_3_K_VOICE: ObjectTypeIdentifier = ObjectTypeIdentifier(0xE1);

    /// no object type specified
    /// 
    /// Type value: `0xFF`
    /// 
    /// Specification: _MPEG-4_
    pub const NO_OBJECT_TYPE_SPECIFIED: ObjectTypeIdentifier = ObjectTypeIdentifier(0xFF);

}

impl fmt::Debug for ObjectTypeIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match *self {
            ObjectTypeIdentifier::FORBIDDEN => "FORBIDDEN",
            ObjectTypeIdentifier::SYSTEMS_ISO_IEC_14496_1 => "SYSTEMS_ISO_IEC_14496_1",
            ObjectTypeIdentifier::SYSTEMS_ISO_IEC_14496_1_BIFS_V_2_CONFIG => "SYSTEMS_ISO_IEC_14496_1_BIFS_V_2_CONFIG",
            ObjectTypeIdentifier::INTERACTION_STREAM => "INTERACTION_STREAM",
            ObjectTypeIdentifier::EXTENDED_BIFS => "EXTENDED_BIFS",
            ObjectTypeIdentifier::AFX_STREAM => "AFX_STREAM",
            ObjectTypeIdentifier::FONT_DATA_STREAM => "FONT_DATA_STREAM",
            ObjectTypeIdentifier::SYNTHETISED_TEXTURE => "SYNTHETISED_TEXTURE",
            ObjectTypeIdentifier::TEXT_STREAM => "TEXT_STREAM",
            ObjectTypeIdentifier::LA_SE_R_STREAM => "LA_SE_R_STREAM",
            ObjectTypeIdentifier::SIMPLE_AGGREGATION_FORMAT_STREAM => "SIMPLE_AGGREGATION_FORMAT_STREAM",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_14496_2 => "VISUAL_ISO_IEC_14496_2",
            ObjectTypeIdentifier::VISUAL_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10 => "VISUAL_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10",
            ObjectTypeIdentifier::PARAMETER_SETS_FOR_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10 => "PARAMETER_SETS_FOR_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_23008_2_ITU_T_RECOMMENDATION_H_265 => "VISUAL_ISO_IEC_23008_2_ITU_T_RECOMMENDATION_H_265",
            ObjectTypeIdentifier::AUDIO_ISO_IEC_14496_3 => "AUDIO_ISO_IEC_14496_3",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_SIMPLE_PROFILE => "VISUAL_ISO_IEC_13818_2_SIMPLE_PROFILE",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_MAIN_PROFILE => "VISUAL_ISO_IEC_13818_2_MAIN_PROFILE",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_SNR_PROFILE => "VISUAL_ISO_IEC_13818_2_SNR_PROFILE",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_SPATIAL_PROFILE => "VISUAL_ISO_IEC_13818_2_SPATIAL_PROFILE",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_HIGH_PROFILE => "VISUAL_ISO_IEC_13818_2_HIGH_PROFILE",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_422_PROFILE => "VISUAL_ISO_IEC_13818_2_422_PROFILE",
            ObjectTypeIdentifier::AUDIO_ISO_IEC_13818_7_MAIN_PROFILE => "AUDIO_ISO_IEC_13818_7_MAIN_PROFILE",
            ObjectTypeIdentifier::AUDIO_ISO_IEC_13818_7_LOW_COMPLEXITY_PROFILE => "AUDIO_ISO_IEC_13818_7_LOW_COMPLEXITY_PROFILE",
            ObjectTypeIdentifier::AUDIO_ISO_IEC_13818_7_SCALEABLE_SAMPLING_RATE_PROFILE => "AUDIO_ISO_IEC_13818_7_SCALEABLE_SAMPLING_RATE_PROFILE",
            ObjectTypeIdentifier::AUDIO_ISO_IEC_13818_3 => "AUDIO_ISO_IEC_13818_3",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_11172_2 => "VISUAL_ISO_IEC_11172_2",
            ObjectTypeIdentifier::AUDIO_ISO_IEC_11172_3 => "AUDIO_ISO_IEC_11172_3",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_10918_1 => "VISUAL_ISO_IEC_10918_1",
            ObjectTypeIdentifier::PORTABLE_NETWORK_GRAPHICS => "PORTABLE_NETWORK_GRAPHICS",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_15444_1 => "VISUAL_ISO_IEC_15444_1",
            ObjectTypeIdentifier::EVRC_VOICE => "EVRC_VOICE",
            ObjectTypeIdentifier::SMV_VOICE => "SMV_VOICE",
            ObjectTypeIdentifier::THREE_GPP_2_COMPACT_MULTIMEDIA_FORMAT => "THREE_GPP_2_COMPACT_MULTIMEDIA_FORMAT",
            ObjectTypeIdentifier::SMPTE_VC_1_VIDEO => "SMPTE_VC_1_VIDEO",
            ObjectTypeIdentifier::DIRAC_VIDEO_CODER => "DIRAC_VIDEO_CODER",
            ObjectTypeIdentifier(0xA5) => "WITHDRAWN",
            ObjectTypeIdentifier(0xA6) => "WITHDRAWN",
            ObjectTypeIdentifier::DRA_AUDIO => "DRA_AUDIO",
            ObjectTypeIdentifier::ITU_G_719_AUDIO => "ITU_G_719_AUDIO",
            ObjectTypeIdentifier::CORE_SUBSTREAM => "CORE_SUBSTREAM",
            ObjectTypeIdentifier::CORE_SUBSTREAM_EXTENSION_SUBSTREAM => "CORE_SUBSTREAM_EXTENSION_SUBSTREAM",
            ObjectTypeIdentifier::EXTENSION_SUBSTREAM_CONTAINING_ONLY_XLL => "EXTENSION_SUBSTREAM_CONTAINING_ONLY_XLL",
            ObjectTypeIdentifier::EXTENSION_SUBSTREAM_CONTAINING_ONLY_LBR => "EXTENSION_SUBSTREAM_CONTAINING_ONLY_LBR",
            ObjectTypeIdentifier::OPUS_AUDIO => "OPUS_AUDIO",
            ObjectTypeIdentifier(0xAE) => "WITHDRAWN",
            ObjectTypeIdentifier::AURO_CX_3_D_AUDIO => "AURO_CX_3_D_AUDIO",
            ObjectTypeIdentifier::REAL_VIDEO_CODEC_11 => "REAL_VIDEO_CODEC_11",
            ObjectTypeIdentifier::VP_9_VIDEO => "VP_9_VIDEO",
            ObjectTypeIdentifier::DTS_UHD_PROFILE_2 => "DTS_UHD_PROFILE_2",
            ObjectTypeIdentifier::DTS_UHD_PROFILE_3_OR_HIGHER => "DTS_UHD_PROFILE_3_OR_HIGHER",
            ObjectTypeIdentifier(0xc0..=0xe0) => "USER_PRIVATE",
            ObjectTypeIdentifier::ONE_3_K_VOICE => "ONE_3_K_VOICE",
            ObjectTypeIdentifier(0xe2..=0xfe) => "USER_PRIVATE",
            ObjectTypeIdentifier::NO_OBJECT_TYPE_SPECIFIED => "NO_OBJECT_TYPE_SPECIFIED",
            ObjectTypeIdentifier(_) => "RESERVED",
        };
        write!(f, "{}({:#04x})", label, self.0)
    }
}

impl BrandCode {
    /// Single intra-coded picture
    /// 
    /// FourCC: `1pic`
    /// 
    /// Specification: _HEIF_
    pub const ONE_PIC: BrandCode = BrandCode(FourCC(*b"1pic"));

    /// 3GPP2
    /// 
    /// FourCC: `3g2a`
    /// 
    /// Specification: _3GPP2_
    pub const THREE_G2A: BrandCode = BrandCode(FourCC(*b"3g2a"));

    /// 3GPP Release 6 extended-presentation Profile
    /// 
    /// FourCC: `3ge6`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GE6: BrandCode = BrandCode(FourCC(*b"3ge6"));

    /// 3GPP Release 7 extended-presentation Profile
    /// 
    /// FourCC: `3ge7`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GE7: BrandCode = BrandCode(FourCC(*b"3ge7"));

    /// 3GPP Release 9 Extended Presentation Profile
    /// 
    /// FourCC: `3ge9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GE9: BrandCode = BrandCode(FourCC(*b"3ge9"));

    /// 3GPP Release 9 File-delivery Server Profile
    /// 
    /// FourCC: `3gf9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GF9: BrandCode = BrandCode(FourCC(*b"3gf9"));

    /// 3GPP Release 6 General Profile
    /// 
    /// FourCC: `3gg6`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GG6: BrandCode = BrandCode(FourCC(*b"3gg6"));

    /// 3GPP Release 9 General Profile
    /// 
    /// FourCC: `3gg9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GG9: BrandCode = BrandCode(FourCC(*b"3gg9"));

    /// 3GPP Release 9 Adaptive Streaming Profile
    /// 
    /// FourCC: `3gh9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GH9: BrandCode = BrandCode(FourCC(*b"3gh9"));

    /// 3GPP Release 9 Media Segment Profile
    /// 
    /// FourCC: `3gm9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GM9: BrandCode = BrandCode(FourCC(*b"3gm9"));

    /// 3GPP Media Segment conforming to the Media Segment Format for 3GP DASH
    /// 
    /// FourCC: `3gmA`
    /// 
    /// Specification: _3GPP-DASH_
    pub const THREE_GMA: BrandCode = BrandCode(FourCC(*b"3gmA"));

    /// 3GPP Release 4
    /// 
    /// FourCC: `3gp4`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GP4: BrandCode = BrandCode(FourCC(*b"3gp4"));

    /// 3GPP Release 5
    /// 
    /// FourCC: `3gp5`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GP5: BrandCode = BrandCode(FourCC(*b"3gp5"));

    /// 3GPP Release 6 basic Profile
    /// 
    /// FourCC: `3gp6`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GP6: BrandCode = BrandCode(FourCC(*b"3gp6"));

    /// 3GPP Release 7
    /// 
    /// FourCC: `3gp7`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GP7: BrandCode = BrandCode(FourCC(*b"3gp7"));

    /// 3GPP Release 8
    /// 
    /// FourCC: `3gp8`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GP8: BrandCode = BrandCode(FourCC(*b"3gp8"));

    /// 3GPP Release 9 Basic Profile
    /// 
    /// FourCC: `3gp9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GP9: BrandCode = BrandCode(FourCC(*b"3gp9"));

    /// 3GPP Release 6 progressive-download Profile
    /// 
    /// FourCC: `3gr6`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GR6: BrandCode = BrandCode(FourCC(*b"3gr6"));

    /// 3GPP Release 9 Progressive DownloadProfile
    /// 
    /// FourCC: `3gr9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GR9: BrandCode = BrandCode(FourCC(*b"3gr9"));

    /// 3GPP Release 6 streaming-server Profile
    /// 
    /// FourCC: `3gs6`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GS6: BrandCode = BrandCode(FourCC(*b"3gs6"));

    /// 3GPP Release 9 Streaming ServerProfile
    /// 
    /// FourCC: `3gs9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GS9: BrandCode = BrandCode(FourCC(*b"3gs9"));

    /// 3GPP Release 8 Media Stream Recording Profile
    /// 
    /// FourCC: `3gt8`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GT8: BrandCode = BrandCode(FourCC(*b"3gt8"));

    /// 3GPP Release 9 Media Stream Recording Profile
    /// 
    /// FourCC: `3gt9`
    /// 
    /// Specification: _3GPP_
    pub const THREE_GT9: BrandCode = BrandCode(FourCC(*b"3gt9"));

    /// 3GPP TV over 3GPP services
    /// 
    /// FourCC: `3gtv`
    /// 
    /// Specification: _3GPP-TVP_
    pub const THREE_GTV: BrandCode = BrandCode(FourCC(*b"3gtv"));

    /// 3GPP VR Presentation
    /// 
    /// FourCC: `3gvr`
    /// 
    /// Specification: _3GPP-VR_
    pub const THREE_GVR: BrandCode = BrandCode(FourCC(*b"3gvr"));

    /// 3GPP VR Advanced Video Media Profile
    /// 
    /// FourCC: `3vra`
    /// 
    /// Specification: _3GPP-VR_
    pub const THREE_VRA: BrandCode = BrandCode(FourCC(*b"3vra"));

    /// 3GPP VR Basic Video Media Profile
    /// 
    /// FourCC: `3vrb`
    /// 
    /// Specification: _3GPP-VR_
    pub const THREE_VRB: BrandCode = BrandCode(FourCC(*b"3vrb"));

    /// 3GPP VR Main Video Media Profile
    /// 
    /// FourCC: `3vrm`
    /// 
    /// Specification: _3GPP-VR_
    pub const THREE_VRM: BrandCode = BrandCode(FourCC(*b"3vrm"));

    /// Advanced tiling OMAF video profile
    /// 
    /// FourCC: `adti`
    /// 
    /// Specification: _OMAF_
    pub const ADTI: BrandCode = BrandCode(FourCC(*b"adti"));

    /// ID3 metadata carried as timed metadata in CMAF
    /// 
    /// FourCC: `aid3`
    /// 
    /// Specification: _CMAF-ID3_
    pub const AID3: BrandCode = BrandCode(FourCC(*b"aid3"));

    /// ARRI Digital Camera
    /// 
    /// FourCC: `ARRI`
    /// 
    /// Specification: _ARRI_
    pub const ARRI: BrandCode = BrandCode(FourCC(*b"ARRI"));

    /// AOM Video Codec
    /// 
    /// FourCC: `av01`
    /// 
    /// Specification: _AV1-ISOBMFF_
    pub const AV01: BrandCode = BrandCode(FourCC(*b"av01"));

    /// Advanced Video Coding extensions
    /// 
    /// FourCC: `avc1`
    /// 
    /// Specification: _ISO_
    pub const AVC1: BrandCode = BrandCode(FourCC(*b"avc1"));

    /// AVC image and image collection brands
    /// 
    /// FourCC: `avci`
    /// 
    /// Specification: _HEIF_
    pub const AVCI: BrandCode = BrandCode(FourCC(*b"avci"));

    /// AVC image sequence brands
    /// 
    /// FourCC: `avcs`
    /// 
    /// Specification: _HEIF_
    pub const AVCS: BrandCode = BrandCode(FourCC(*b"avcs"));

    /// AVC-based viewport-dependent OMAF video profile
    /// 
    /// FourCC: `avde`
    /// 
    /// Specification: _OMAF_
    pub const AVDE: BrandCode = BrandCode(FourCC(*b"avde"));

    /// AV1 image format brand
    /// 
    /// FourCC: `avif`
    /// 
    /// Specification: _AVIF_
    pub const AVIF: BrandCode = BrandCode(FourCC(*b"avif"));

    /// AV1 intra-only brand
    /// 
    /// FourCC: `avio`
    /// 
    /// Specification: _AVIF_
    pub const AVIO: BrandCode = BrandCode(FourCC(*b"avio"));

    /// AV1 image sequence brand
    /// 
    /// FourCC: `avis`
    /// 
    /// Specification: _AVIF_
    pub const AVIS: BrandCode = BrandCode(FourCC(*b"avis"));

    /// Blinkbox Master File: H.264 video and 16-bit little-endian LPCM audio
    /// 
    /// FourCC: `bbxm`
    /// 
    /// Specification: _Blinkbox_
    pub const BBXM: BrandCode = BrandCode(FourCC(*b"bbxm"));

    /// Canon Digital Camera
    /// 
    /// FourCC: `CAEP`
    /// 
    /// Specification: _Canon_
    pub const CAEP: BrandCode = BrandCode(FourCC(*b"CAEP"));

    /// Convergent Designs
    /// 
    /// FourCC: `CDes`
    /// 
    /// Specification: _Convergent_
    pub const CDES: BrandCode = BrandCode(FourCC(*b"CDes"));

    /// CMAF Media Profile - AC-4 Main
    /// 
    /// FourCC: `ca4m`
    /// 
    /// Specification: _ETSI AC-4_
    pub const CA4M: BrandCode = BrandCode(FourCC(*b"ca4m"));

    /// CMAF Media Profile - AC-4 Single Stream
    /// 
    /// FourCC: `ca4s`
    /// 
    /// Specification: _ETSI AC-4_
    pub const CA4S: BrandCode = BrandCode(FourCC(*b"ca4s"));

    /// CMAF Media Profile - AAC Adaptive Audio
    /// 
    /// FourCC: `caaa`
    /// 
    /// Specification: _CMAF_
    pub const CAAA: BrandCode = BrandCode(FourCC(*b"caaa"));

    /// CMAF Media Profile - AAC Core
    /// 
    /// FourCC: `caac`
    /// 
    /// Specification: _CMAF_
    pub const CAAC: BrandCode = BrandCode(FourCC(*b"caac"));

    /// CMAF Media Profile for OMAF 3D audio baseline profile
    /// 
    /// FourCC: `cabl`
    /// 
    /// Specification: _OMAF_
    pub const CABL: BrandCode = BrandCode(FourCC(*b"cabl"));

    /// CMAF Media Profile - AAC multichannel adaptive audio
    /// 
    /// FourCC: `cama`
    /// 
    /// Specification: _CMAF_
    pub const CAMA: BrandCode = BrandCode(FourCC(*b"cama"));

    /// CMAF Media Profile - AAC multichannel audio
    /// 
    /// FourCC: `camc`
    /// 
    /// Specification: _CMAF_
    pub const CAMC: BrandCode = BrandCode(FourCC(*b"camc"));

    /// Casio Digital Camera
    /// 
    /// FourCC: `caqv`
    /// 
    /// Specification: _Casio_
    pub const CAQV: BrandCode = BrandCode(FourCC(*b"caqv"));

    /// CMAF Media Profile - MPEG-D USAC audio
    /// 
    /// FourCC: `casu`
    /// 
    /// Specification: _CMAF_
    pub const CASU: BrandCode = BrandCode(FourCC(*b"casu"));

    /// CMAF Supplemental Data - CEA-608/708
    /// 
    /// FourCC: `ccea`
    /// 
    /// Specification: _CMAF_
    pub const CCEA: BrandCode = BrandCode(FourCC(*b"ccea"));

    /// Common container file format
    /// 
    /// FourCC: `ccff`
    /// 
    /// Specification: _DECE_
    pub const CCFF: BrandCode = BrandCode(FourCC(*b"ccff"));

    /// CMAF Media Profile - HEVC HDR10 (chd1) with SCTE Dynamic Metadata app #1 (ST2094-10)
    /// 
    /// FourCC: `cdm1`
    /// 
    /// Specification: _SCTE-215-1-1b_
    pub const CDM1: BrandCode = BrandCode(FourCC(*b"cdm1"));

    /// CMAF Media Profile compatibility to HDR10+ (ST2094-40)
    /// 
    /// FourCC: `cdm4`
    /// 
    /// Specification: _SCTE-215-1-1b_
    pub const CDM4: BrandCode = BrandCode(FourCC(*b"cdm4"));

    /// CMAF Media Profile - Enhanced AC-3
    /// 
    /// FourCC: `ceac`
    /// 
    /// Specification: _ETSI AC-3_
    pub const CEAC: BrandCode = BrandCode(FourCC(*b"ceac"));

    /// CMAF Media Profile - AVC HD
    /// 
    /// FourCC: `cfhd`
    /// 
    /// Specification: _CMAF_
    pub const CFHD: BrandCode = BrandCode(FourCC(*b"cfhd"));

    /// CMAF Media Profile - AVC SD
    /// 
    /// FourCC: `cfsd`
    /// 
    /// Specification: _CMAF_
    pub const CFSD: BrandCode = BrandCode(FourCC(*b"cfsd"));

    /// CMAF Media Profile - HEVC HDR10
    /// 
    /// FourCC: `chd1`
    /// 
    /// Specification: _CMAF_
    pub const CHD1: BrandCode = BrandCode(FourCC(*b"chd1"));

    /// CMAF Media Profile - AVC HDHF
    /// 
    /// FourCC: `chdf`
    /// 
    /// Specification: _CMAF_
    pub const CHDF: BrandCode = BrandCode(FourCC(*b"chdf"));

    /// CMAF Media Profile for the HEVC-based viewport-dependent OMAF video profile
    /// 
    /// FourCC: `chev`
    /// 
    /// Specification: _OMAF_
    pub const CHEV: BrandCode = BrandCode(FourCC(*b"chev"));

    /// CMAF High frame rate Media Profile - HEVC HDR10H
    /// 
    /// FourCC: `chd2`
    /// 
    /// Specification: _CMAF_
    pub const CHD2: BrandCode = BrandCode(FourCC(*b"chd2"));

    /// CMAF Media Profile - HEVC HHD10
    /// 
    /// FourCC: `chh1`
    /// 
    /// Specification: _CMAF_
    pub const CHH1: BrandCode = BrandCode(FourCC(*b"chh1"));

    /// CMAF Media Profile - HEVC HHD8
    /// 
    /// FourCC: `chhd`
    /// 
    /// Specification: _CMAF_
    pub const CHHD: BrandCode = BrandCode(FourCC(*b"chhd"));

    /// CMAF Interlaced Media Profile - INT10
    /// 
    /// FourCC: `cint`
    /// 
    /// Specification: _CMAF_
    pub const CINT: BrandCode = BrandCode(FourCC(*b"cint"));

    /// CMAF Media Profile - HEVC HLG10
    /// 
    /// FourCC: `clg1`
    /// 
    /// Specification: _CMAF_
    pub const CLG1: BrandCode = BrandCode(FourCC(*b"clg1"));

    /// CMAF High frame rate Media Profile - HEVC HLG10H
    /// 
    /// FourCC: `clg2`
    /// 
    /// Specification: _CMAF_
    pub const CLG2: BrandCode = BrandCode(FourCC(*b"clg2"));

    /// CMAF Structural Brand
    /// 
    /// FourCC: `cmf2`
    /// 
    /// Specification: _CMAF_
    pub const CMF2: BrandCode = BrandCode(FourCC(*b"cmf2"));

    /// CMAF Structural Brand
    /// 
    /// FourCC: `cmfc`
    /// 
    /// Specification: _CMAF_
    pub const CMFC: BrandCode = BrandCode(FourCC(*b"cmfc"));

    /// CMAF Fragment Format
    /// 
    /// FourCC: `cmff`
    /// 
    /// Specification: _CMAF_
    pub const CMFF: BrandCode = BrandCode(FourCC(*b"cmff"));

    /// CMAF Chunk Format
    /// 
    /// FourCC: `cmfl`
    /// 
    /// Specification: _CMAF_
    pub const CMFL: BrandCode = BrandCode(FourCC(*b"cmfl"));

    /// CMAF Segment Format
    /// 
    /// FourCC: `cmfs`
    /// 
    /// Specification: _CMAF_
    pub const CMFS: BrandCode = BrandCode(FourCC(*b"cmfs"));

    /// CMAF Media Profile - MPEG-H 3D audio LC (mhm2)
    /// 
    /// FourCC: `cmhm`
    /// 
    /// Specification: _CMAF_
    pub const CMHM: BrandCode = BrandCode(FourCC(*b"cmhm"));

    /// CMAF Media Profile - MPEG-H 3D audio LC (mhm1)
    /// 
    /// FourCC: `cmhs`
    /// 
    /// Specification: _CMAF_
    pub const CMHS: BrandCode = BrandCode(FourCC(*b"cmhs"));

    /// Compressed boxes
    /// 
    /// FourCC: `comp`
    /// 
    /// Specification: _ISO_
    pub const COMP: BrandCode = BrandCode(FourCC(*b"comp"));

    /// CMAF Media Profile - Scalable HEVC media profile
    /// 
    /// FourCC: `csh1`
    /// 
    /// Specification: _CMAF_
    pub const CSH1: BrandCode = BrandCode(FourCC(*b"csh1"));

    /// CMAF Media Profile - HEVC UHD10
    /// 
    /// FourCC: `cud1`
    /// 
    /// Specification: _CMAF_
    pub const CUD1: BrandCode = BrandCode(FourCC(*b"cud1"));

    /// CMAF High frame rate Media Profile - HEVC UHD10H
    /// 
    /// FourCC: `cud2`
    /// 
    /// Specification: _CMAF_
    pub const CUD2: BrandCode = BrandCode(FourCC(*b"cud2"));

    /// CMAF Media Profile - HEVC UHD8
    /// 
    /// FourCC: `cud8`
    /// 
    /// Specification: _CMAF_
    pub const CUD8: BrandCode = BrandCode(FourCC(*b"cud8"));

    /// CMAF High frame rate Media Profile - HEVC UHD8H
    /// 
    /// FourCC: `cud9`
    /// 
    /// Specification: _CMAF_
    pub const CUD9: BrandCode = BrandCode(FourCC(*b"cud9"));

    /// CMAF Media Profile for the unconstrained HEVC-based viewport-independent OMAF video profile
    /// 
    /// FourCC: `cuvd`
    /// 
    /// Specification: _OMAF_
    pub const CUVD: BrandCode = BrandCode(FourCC(*b"cuvd"));

    /// CMAF Media Profile for the HEVC-based viewport-independent OMAF video profile
    /// 
    /// FourCC: `cvid`
    /// 
    /// Specification: _OMAF_
    pub const CVID: BrandCode = BrandCode(FourCC(*b"cvid"));

    /// CMAF media profile for the VVC-based viewport-independent OMAF video profile
    /// 
    /// FourCC: `cvvc`
    /// 
    /// Specification: _OMAF_
    pub const CVVC: BrandCode = BrandCode(FourCC(*b"cvvc"));

    /// CMAF Media Profile - WebVTT
    /// 
    /// FourCC: `cwvt`
    /// 
    /// Specification: _CMAF_
    pub const CWVT: BrandCode = BrandCode(FourCC(*b"cwvt"));

    /// DMB AF audio with MPEG Layer II audio, MOT slide show, DLS, JPG/PNG/MNG images
    /// 
    /// FourCC: `da0a`
    /// 
    /// Specification: _DMB-MAF_
    pub const DA0A: BrandCode = BrandCode(FourCC(*b"da0a"));

    /// DMB AF, extending da0a , with 3GPP timed text, DID, TVA, REL, IPMP
    /// 
    /// FourCC: `da0b`
    /// 
    /// Specification: _DMB-MAF_
    pub const DA0B: BrandCode = BrandCode(FourCC(*b"da0b"));

    /// DMB AF audio with ER-BSAC audio, JPG/PNG/MNG images
    /// 
    /// FourCC: `da1a`
    /// 
    /// Specification: _DMB-MAF_
    pub const DA1A: BrandCode = BrandCode(FourCC(*b"da1a"));

    /// DMB AF, extending da1a, with 3GPP timed text, DID, TVA, REL, IPMP
    /// 
    /// FourCC: `da1b`
    /// 
    /// Specification: _DMB-MAF_
    pub const DA1B: BrandCode = BrandCode(FourCC(*b"da1b"));

    /// DMB AF audio with HE-AAC v2 audio, MOT slide show, DLS, JPG/PNG/MNG images
    /// 
    /// FourCC: `da2a`
    /// 
    /// Specification: _DMB-MAF_
    pub const DA2A: BrandCode = BrandCode(FourCC(*b"da2a"));

    /// DMB AF extending da2a, with 3GPP timed text, DID, TVA, REL, IPMP
    /// 
    /// FourCC: `da2b`
    /// 
    /// Specification: _DMB-MAF_
    pub const DA2B: BrandCode = BrandCode(FourCC(*b"da2b"));

    /// DMB AF audio with HE-AAC, JPG/PNG/MNG images
    /// 
    /// FourCC: `da3a`
    /// 
    /// Specification: _DMB-MAF_
    pub const DA3A: BrandCode = BrandCode(FourCC(*b"da3a"));

    /// DMB AF extending da3a with BIFS, 3GPP timed text, DID, TVA, REL, IPMP
    /// 
    /// FourCC: `da3b`
    /// 
    /// Specification: _DMB-MAF_
    pub const DA3B: BrandCode = BrandCode(FourCC(*b"da3b"));

    /// ISO base media file format file specifically designed for DASH including movie fragments and Segment Index.
    /// 
    /// FourCC: `dash`
    /// 
    /// Specification: _DASH_
    pub const DASH: BrandCode = BrandCode(FourCC(*b"dash"));

    /// Dolby Vision cross-compatible with HDR10
    /// 
    /// FourCC: `db1p`
    /// 
    /// Specification: _Dolby_
    pub const DB1P: BrandCode = BrandCode(FourCC(*b"db1p"));

    /// Dolby Vision cross-compatible with SDR
    /// 
    /// FourCC: `db2g`
    /// 
    /// Specification: _Dolby_
    pub const DB2G: BrandCode = BrandCode(FourCC(*b"db2g"));

    /// Dolby Vision cross-compatible with HLG (VUI =18)
    /// 
    /// FourCC: `db4h`
    /// 
    /// Specification: _Dolby_
    pub const DB4H: BrandCode = BrandCode(FourCC(*b"db4h"));

    /// Dolby Vision cross-compatible with HLG (VUI=14)
    /// 
    /// FourCC: `db4g`
    /// 
    /// Specification: _Dolby_
    pub const DB4G: BrandCode = BrandCode(FourCC(*b"db4g"));

    /// MP4 files with Dolby content (e.g. Dolby AC-4, Dolby Digital Plus, Dolby TrueHD (Dolby MLP))
    /// 
    /// FourCC: `dby1`
    /// 
    /// Specification: _Dolby_
    pub const DBY1: BrandCode = BrandCode(FourCC(*b"dby1"));

    /// DMB AF supporting all the components defined in the specification
    /// 
    /// FourCC: `dmb1`
    /// 
    /// Specification: _DMB-MAF_
    pub const DMB1: BrandCode = BrandCode(FourCC(*b"dmb1"));

    /// Media Segment conforming to the DASH Self-Initializing Media Segment format type for ISO base media file format
    /// 
    /// FourCC: `dsms`
    /// 
    /// Specification: _DASH_
    pub const DSMS: BrandCode = BrandCode(FourCC(*b"dsms"));

    ///  CMAF media profile for audio codecs dtsc dtsh or dtse
    /// 
    /// FourCC: `dts1`
    /// 
    /// Specification: _DTS-HD_
    pub const DTS1: BrandCode = BrandCode(FourCC(*b"dts1"));

    ///  CMAF media profile for audio codec dtsx
    /// 
    /// FourCC: `dts2`
    /// 
    /// Specification: _DTS-UHD_
    pub const DTS2: BrandCode = BrandCode(FourCC(*b"dts2"));

    ///  CMAF media profile for audio codec dtsy
    /// 
    /// FourCC: `dts3`
    /// 
    /// Specification: _DTS-UHD_
    pub const DTS3: BrandCode = BrandCode(FourCC(*b"dts3"));

    /// DMB AF video with AVC video, ER-BSAC audio, BIFS, JPG/PNG/MNG images, TS
    /// 
    /// FourCC: `dv1a`
    /// 
    /// Specification: _DMB-MAF_
    pub const DV1A: BrandCode = BrandCode(FourCC(*b"dv1a"));

    /// DMB AF, extending dv1a, with 3GPP timed text, DID, TVA, REL, IPMP
    /// 
    /// FourCC: `dv1b`
    /// 
    /// Specification: _DMB-MAF_
    pub const DV1B: BrandCode = BrandCode(FourCC(*b"dv1b"));

    /// DMB AF video with AVC video, HE-AACv2 audio, BIFS, JPG/PNG/MNG images, TS
    /// 
    /// FourCC: `dv2a`
    /// 
    /// Specification: _DMB-MAF_
    pub const DV2A: BrandCode = BrandCode(FourCC(*b"dv2a"));

    /// DMB AF extending dv2a, with 3GPP timed text, DID, TVA, REL, IPMP
    /// 
    /// FourCC: `dv2b`
    /// 
    /// Specification: _DMB-MAF_
    pub const DV2B: BrandCode = BrandCode(FourCC(*b"dv2b"));

    /// DMB AF video with AVC video, HE-AAC audio, BIFS, JPG/PNG/MNG images, TS
    /// 
    /// FourCC: `dv3a`
    /// 
    /// Specification: _DMB-MAF_
    pub const DV3A: BrandCode = BrandCode(FourCC(*b"dv3a"));

    /// DMB AF extending dv3a with 3GPP timed text, DID, TVA, REL, IPMP
    /// 
    /// FourCC: `dv3b`
    /// 
    /// Specification: _DMB-MAF_
    pub const DV3B: BrandCode = BrandCode(FourCC(*b"dv3b"));

    /// DVB RTP
    /// 
    /// FourCC: `dvr1`
    /// 
    /// Specification: _DVB_
    pub const DVR1: BrandCode = BrandCode(FourCC(*b"dvr1"));

    /// DVB Transport Stream
    /// 
    /// FourCC: `dvt1`
    /// 
    /// Specification: _DVB_
    pub const DVT1: BrandCode = BrandCode(FourCC(*b"dvt1"));

    /// DxO ONE camera
    /// 
    /// FourCC: `dxo `
    /// 
    /// Specification: _DxO_
    pub const DXO : BrandCode = BrandCode(FourCC(*b"dxo "));

    /// Event message box present
    /// 
    /// FourCC: `emsg`
    /// 
    /// Specification: _DASH_
    pub const EMSG: BrandCode = BrandCode(FourCC(*b"emsg"));

    /// EVC Baseline coded image
    /// 
    /// FourCC: `evbi`
    /// 
    /// Specification: _HEIF_
    pub const EVBI: BrandCode = BrandCode(FourCC(*b"evbi"));

    /// EVC Baseline coded image sequence
    /// 
    /// FourCC: `evbs`
    /// 
    /// Specification: _HEIF_
    pub const EVBS: BrandCode = BrandCode(FourCC(*b"evbs"));

    /// EVC Main coded image
    /// 
    /// FourCC: `evmi`
    /// 
    /// Specification: _HEIF_
    pub const EVMI: BrandCode = BrandCode(FourCC(*b"evmi"));

    /// EVC Main coded image sequenc
    /// 
    /// FourCC: `evms`
    /// 
    /// Specification: _HEIF_
    pub const EVMS: BrandCode = BrandCode(FourCC(*b"evms"));

    /// HEVC image and image collection brands
    /// 
    /// FourCC: `heic`
    /// 
    /// Specification: _HEIF_
    pub const HEIC: BrandCode = BrandCode(FourCC(*b"heic"));

    /// L-HEVC image and image collection brands
    /// 
    /// FourCC: `heim`
    /// 
    /// Specification: _HEIF_
    pub const HEIM: BrandCode = BrandCode(FourCC(*b"heim"));

    /// L-HEVC image and image collection brands
    /// 
    /// FourCC: `heis`
    /// 
    /// Specification: _HEIF_
    pub const HEIS: BrandCode = BrandCode(FourCC(*b"heis"));

    /// HEVC image and image collection brands
    /// 
    /// FourCC: `heix`
    /// 
    /// Specification: _HEIF_
    pub const HEIX: BrandCode = BrandCode(FourCC(*b"heix"));

    /// OMAF HEVC image profile
    /// 
    /// FourCC: `heoi`
    /// 
    /// Specification: _OMAF_
    pub const HEOI: BrandCode = BrandCode(FourCC(*b"heoi"));

    /// HEVC image sequence brands
    /// 
    /// FourCC: `hevc`
    /// 
    /// Specification: _HEIF_
    pub const HEVC: BrandCode = BrandCode(FourCC(*b"hevc"));

    /// HEVC-based viewport-dependent OMAF video profile
    /// 
    /// FourCC: `hevd`
    /// 
    /// Specification: _OMAF_
    pub const HEVD: BrandCode = BrandCode(FourCC(*b"hevd"));

    /// HEVC-based viewport-independent OMAF video profile
    /// 
    /// FourCC: `hevi`
    /// 
    /// Specification: _OMAF_
    pub const HEVI: BrandCode = BrandCode(FourCC(*b"hevi"));

    /// L-HEVC image sequence brands
    /// 
    /// FourCC: `hevm`
    /// 
    /// Specification: _HEIF_
    pub const HEVM: BrandCode = BrandCode(FourCC(*b"hevm"));

    /// L-HEVC image sequence brands
    /// 
    /// FourCC: `hevs`
    /// 
    /// Specification: _HEIF_
    pub const HEVS: BrandCode = BrandCode(FourCC(*b"hevs"));

    /// HEVC image sequence brands
    /// 
    /// FourCC: `hevx`
    /// 
    /// Specification: _HEIF_
    pub const HEVX: BrandCode = BrandCode(FourCC(*b"hevx"));

    /// L-HEVC explicit reconstruction
    /// 
    /// FourCC: `hvce`
    /// 
    /// Specification: _NALu Video_
    pub const HVCE: BrandCode = BrandCode(FourCC(*b"hvce"));

    /// L-HEVC implicit reconstruction
    /// 
    /// FourCC: `hvci`
    /// 
    /// Specification: _NALu Video_
    pub const HVCI: BrandCode = BrandCode(FourCC(*b"hvci"));

    /// L-HEVC extended explicit reconstruction brand
    /// 
    /// FourCC: `hvcx`
    /// 
    /// Specification: _NALu Video_
    pub const HVCX: BrandCode = BrandCode(FourCC(*b"hvcx"));

    /// HEVC Tile Track
    /// 
    /// FourCC: `hvti`
    /// 
    /// Specification: _NALu Video_
    pub const HVTI: BrandCode = BrandCode(FourCC(*b"hvti"));

    /// The IFE-SD Media Profile
    /// 
    /// FourCC: `ifsd`
    /// 
    /// Specification: _IFE_
    pub const IFSD: BrandCode = BrandCode(FourCC(*b"ifsd"));

    /// The IFE-HSD Media Profile
    /// 
    /// FourCC: `ifhs`
    /// 
    /// Specification: _IFE_
    pub const IFHS: BrandCode = BrandCode(FourCC(*b"ifhs"));

    /// The IFE-HD Media Profile
    /// 
    /// FourCC: `ifhd`
    /// 
    /// Specification: _IFE_
    pub const IFHD: BrandCode = BrandCode(FourCC(*b"ifhd"));

    /// The IFE-HHD10 Media Profile
    /// 
    /// FourCC: `ifhx`
    /// 
    /// Specification: _IFE_
    pub const IFHX: BrandCode = BrandCode(FourCC(*b"ifhx"));

    /// The IFE-HDHDR Media Profile
    /// 
    /// FourCC: `ifhh`
    /// 
    /// Specification: _IFE_
    pub const IFHH: BrandCode = BrandCode(FourCC(*b"ifhh"));

    /// The IFE-UHD10 Media Profile
    /// 
    /// FourCC: `ifhu`
    /// 
    /// Specification: _IFE_
    pub const IFHU: BrandCode = BrandCode(FourCC(*b"ifhu"));

    /// The IFE-HDR10 Media Profile
    /// 
    /// FourCC: `ifhr`
    /// 
    /// Specification: _IFE_
    pub const IFHR: BrandCode = BrandCode(FourCC(*b"ifhr"));

    /// The IFE-AAC Core Media Profile
    /// 
    /// FourCC: `ifaa`
    /// 
    /// Specification: _IFE_
    pub const IFAA: BrandCode = BrandCode(FourCC(*b"ifaa"));

    /// The IFE-AV1-SD Media Profile
    /// 
    /// FourCC: `ifas`
    /// 
    /// Specification: _IFE_
    pub const IFAS: BrandCode = BrandCode(FourCC(*b"ifas"));

    /// The IFE-AV1-HD Media Profile
    /// 
    /// FourCC: `ifah`
    /// 
    /// Specification: _IFE_
    pub const IFAH: BrandCode = BrandCode(FourCC(*b"ifah"));

    /// The IFE-AV1-HDHDR Media Profile
    /// 
    /// FourCC: `ifai`
    /// 
    /// Specification: _IFE_
    pub const IFAI: BrandCode = BrandCode(FourCC(*b"ifai"));

    /// The IFE-AV1-UHD10 Media Profile
    /// 
    /// FourCC: `ifau`
    /// 
    /// Specification: _IFE_
    pub const IFAU: BrandCode = BrandCode(FourCC(*b"ifau"));

    /// The IFE-AV1-HDR10 Media Profile
    /// 
    /// FourCC: `ifav`
    /// 
    /// Specification: _IFE_
    pub const IFAV: BrandCode = BrandCode(FourCC(*b"ifav"));

    /// Apple iFrame Specification, Version 8.1 Jan 2013
    /// 
    /// FourCC: `ifrm`
    /// 
    /// Specification: _Apple_
    pub const IFRM: BrandCode = BrandCode(FourCC(*b"ifrm"));

    /// CMAF Media Profile - IMSC1 Image
    /// 
    /// FourCC: `im1i`
    /// 
    /// Specification: _CMAF_
    pub const IM1I: BrandCode = BrandCode(FourCC(*b"im1i"));

    /// CMAF Media Profile - IMSC1 Text
    /// 
    /// FourCC: `im1t`
    /// 
    /// Specification: _CMAF_
    pub const IM1T: BrandCode = BrandCode(FourCC(*b"im1t"));

    /// CMAF Media Profile - IMSC1.1 Image
    /// 
    /// FourCC: `im2i`
    /// 
    /// Specification: _CMAF_
    pub const IM2I: BrandCode = BrandCode(FourCC(*b"im2i"));

    /// CMAF Media Profile - IMSC1.1 Text
    /// 
    /// FourCC: `im2t`
    /// 
    /// Specification: _CMAF_
    pub const IM2T: BrandCode = BrandCode(FourCC(*b"im2t"));

    /// Files encrypted according to ISMACryp 2.0
    /// 
    /// FourCC: `isc2`
    /// 
    /// Specification: _ISMACryp2_
    pub const ISC2: BrandCode = BrandCode(FourCC(*b"isc2"));

    /// All files based on the 2004 edition of the ISO file format
    /// 
    /// FourCC: `iso2`
    /// 
    /// Specification: _ISO_
    pub const ISO2: BrandCode = BrandCode(FourCC(*b"iso2"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `iso3`
    /// 
    /// Specification: _ISO_
    pub const ISO3: BrandCode = BrandCode(FourCC(*b"iso3"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `iso4`
    /// 
    /// Specification: _ISO_
    pub const ISO4: BrandCode = BrandCode(FourCC(*b"iso4"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `iso5`
    /// 
    /// Specification: _ISO_
    pub const ISO5: BrandCode = BrandCode(FourCC(*b"iso5"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `iso6`
    /// 
    /// Specification: _ISO_
    pub const ISO6: BrandCode = BrandCode(FourCC(*b"iso6"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `iso7`
    /// 
    /// Specification: _ISO_
    pub const ISO7: BrandCode = BrandCode(FourCC(*b"iso7"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `iso8`
    /// 
    /// Specification: _ISO_
    pub const ISO8: BrandCode = BrandCode(FourCC(*b"iso8"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `iso9`
    /// 
    /// Specification: _ISO_
    pub const ISO9: BrandCode = BrandCode(FourCC(*b"iso9"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `isoa`
    /// 
    /// Specification: _ISO_
    pub const ISOA: BrandCode = BrandCode(FourCC(*b"isoa"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `isob`
    /// 
    /// Specification: _ISO_
    pub const ISOB: BrandCode = BrandCode(FourCC(*b"isob"));

    /// Version of the ISO file format
    /// 
    /// FourCC: `isoc`
    /// 
    /// Specification: _ISO_
    pub const ISOC: BrandCode = BrandCode(FourCC(*b"isoc"));

    /// All files based on the ISO Base Media File Format
    /// 
    /// FourCC: `isom`
    /// 
    /// Specification: _ISO_
    pub const ISOM: BrandCode = BrandCode(FourCC(*b"isom"));

    /// JPEG 2000 image and image collections in ISO/IEC 23008-12 files
    /// 
    /// FourCC: `j2ki`
    /// 
    /// Specification: _J2KHEIF_
    pub const J2KI: BrandCode = BrandCode(FourCC(*b"j2ki"));

    /// Motion JPEG 2000 in ISO/IEC 23008-12 files
    /// 
    /// FourCC: `j2ks`
    /// 
    /// Specification: _J2KHEIF_
    pub const J2KS: BrandCode = BrandCode(FourCC(*b"j2ks"));

    /// JPEG 2000 image sequence in ISO/IEC 23008-12 files
    /// 
    /// FourCC: `j2is`
    /// 
    /// Specification: _J2KHEIF_
    pub const J2IS: BrandCode = BrandCode(FourCC(*b"j2is"));

    /// JPEG2000 Profile 0
    /// 
    /// FourCC: `J2P0`
    /// 
    /// Specification: _JPEG2000_
    pub const J2P0: BrandCode = BrandCode(FourCC(*b"J2P0"));

    /// JPEG2000 Profile 1
    /// 
    /// FourCC: `J2P1`
    /// 
    /// Specification: _JPEG2000_
    pub const J2P1: BrandCode = BrandCode(FourCC(*b"J2P1"));

    /// JPEG2000 Part 1
    /// 
    /// FourCC: `jp2 `
    /// 
    /// Specification: _JPEG2000_
    pub const JP2 : BrandCode = BrandCode(FourCC(*b"jp2 "));

    /// JPEG-specific still image brand
    /// 
    /// FourCC: `jpeg`
    /// 
    /// Specification: _HEIF_
    pub const JPEG: BrandCode = BrandCode(FourCC(*b"jpeg"));

    /// JPEG image sequence brands
    /// 
    /// FourCC: `jpgs`
    /// 
    /// Specification: _HEIF_
    pub const JPGS: BrandCode = BrandCode(FourCC(*b"jpgs"));

    /// JPEG 2000 Part 6 Compound Images
    /// 
    /// FourCC: `jpm `
    /// 
    /// Specification: _JPM_
    pub const JPM : BrandCode = BrandCode(FourCC(*b"jpm "));

    /// OMAF legacy image profile
    /// 
    /// FourCC: `jpoi`
    /// 
    /// Specification: _OMAF_
    pub const JPOI: BrandCode = BrandCode(FourCC(*b"jpoi"));

    /// The JPSearch data interchange format, for the exchange of image collections and respective metadata
    /// 
    /// FourCC: `jpsi`
    /// 
    /// Specification: _JPSearch_
    pub const JPSI: BrandCode = BrandCode(FourCC(*b"jpsi"));

    /// JPEG2000 Part 2
    /// 
    /// FourCC: `jpx `
    /// 
    /// Specification: _JPX_
    pub const JPX : BrandCode = BrandCode(FourCC(*b"jpx "));

    /// JPEG XR
    /// 
    /// FourCC: `jpxb`
    /// 
    /// Specification: _JPXR_
    pub const JPXB: BrandCode = BrandCode(FourCC(*b"jpxb"));

    /// JPEG XL
    /// 
    /// FourCC: `jxl `
    /// 
    /// Specification: _JPEG XL_
    pub const JXL : BrandCode = BrandCode(FourCC(*b"jxl "));

    /// Still Image File Format for JPEG XS
    /// 
    /// FourCC: `jxs `
    /// 
    /// Specification: _JPXS_
    pub const JXS : BrandCode = BrandCode(FourCC(*b"jxs "));

    /// Codestream for JPEG XS
    /// 
    /// FourCC: `jxsc`
    /// 
    /// Specification: _JPXS_
    pub const JXSC: BrandCode = BrandCode(FourCC(*b"jxsc"));

    /// JPEG XS image and image collections for HEIF
    /// 
    /// FourCC: `jxsi`
    /// 
    /// Specification: _JPXS_
    pub const JXSI: BrandCode = BrandCode(FourCC(*b"jxsi"));

    /// JPEG XS image sequences for HEIF
    /// 
    /// FourCC: `jxss`
    /// 
    /// Specification: _JPXS_
    pub const JXSS: BrandCode = BrandCode(FourCC(*b"jxss"));

    /// Leica digital camera
    /// 
    /// FourCC: `LCAG`
    /// 
    /// Specification: _Leica_
    pub const LCAG: BrandCode = BrandCode(FourCC(*b"LCAG"));

    /// L-HEVC Tile Track Explicit brand
    /// 
    /// FourCC: `lhte`
    /// 
    /// Specification: _NALu Video_
    pub const LHTE: BrandCode = BrandCode(FourCC(*b"lhte"));

    /// L-HEVC Tile Track Implicit brand
    /// 
    /// FourCC: `lhti`
    /// 
    /// Specification: _NALu Video_
    pub const LHTI: BrandCode = BrandCode(FourCC(*b"lhti"));

    /// last Media Segment indicator for ISO base media file format.
    /// 
    /// FourCC: `lmsg`
    /// 
    /// Specification: _DASH_
    pub const LMSG: BrandCode = BrandCode(FourCC(*b"lmsg"));

    /// iTunes MPEG-4 audio protected or not, can contain audio + video + 3g text track + chapter track
    /// 
    /// FourCC: `M4A `
    /// 
    /// Specification: _iTunes_
    pub const M4A : BrandCode = BrandCode(FourCC(*b"M4A "));

    /// iTunes AudioBook protected or not, can contain audio + video + 3g text track + chapter track
    /// 
    /// FourCC: `M4B `
    /// 
    /// Specification: _iTunes_
    pub const M4B : BrandCode = BrandCode(FourCC(*b"M4B "));

    /// MPEG-4 protected audio
    /// 
    /// FourCC: `M4P `
    /// 
    /// Specification: _iTunes_
    pub const M4P : BrandCode = BrandCode(FourCC(*b"M4P "));

    /// MPEG-4 protected audio+video
    /// 
    /// FourCC: `M4V `
    /// 
    /// Specification: _iTunes_
    pub const M4V : BrandCode = BrandCode(FourCC(*b"M4V "));

    /// AVIF Baseline Profile
    /// 
    /// FourCC: `MA1B`
    /// 
    /// Specification: _AVIF_
    pub const MA1B: BrandCode = BrandCode(FourCC(*b"MA1B"));

    /// AVIF Advanced Profile
    /// 
    /// FourCC: `MA1A`
    /// 
    /// Specification: _AVIF_
    pub const MA1A: BrandCode = BrandCode(FourCC(*b"MA1A"));

    /// Media File for Samsung video Metadata
    /// 
    /// FourCC: `MFSM`
    /// 
    /// Specification: _Samsung_
    pub const MFSM: BrandCode = BrandCode(FourCC(*b"MFSM"));

    /// Home and Mobile Multimedia Platform (HMMP)
    /// 
    /// FourCC: `MGSV`
    /// 
    /// Specification: _Sony_
    pub const MGSV: BrandCode = BrandCode(FourCC(*b"MGSV"));

    /// Multi-Image Application format brand for MIAF AVC Basic Profile
    /// 
    /// FourCC: `MiAB`
    /// 
    /// Specification: _MIAF_
    pub const MIAB: BrandCode = BrandCode(FourCC(*b"MiAB"));

    /// Multi-Image Application format brand for fragmented alpha video
    /// 
    /// FourCC: `MiAC`
    /// 
    /// Specification: _MIAF_
    pub const MIAC: BrandCode = BrandCode(FourCC(*b"MiAC"));

    /// Multi-Image Application format brand for general MIAF requirements
    /// 
    /// FourCC: `miaf`
    /// 
    /// Specification: _MIAF_
    pub const MIAF: BrandCode = BrandCode(FourCC(*b"miaf"));

    /// Mutli-Image Application format brand for animation
    /// 
    /// FourCC: `MiAn`
    /// 
    /// Specification: _MIAF_
    pub const MIAN: BrandCode = BrandCode(FourCC(*b"MiAn"));

    /// Multi-Image Application format brand for burst capture
    /// 
    /// FourCC: `MiBu`
    /// 
    /// Specification: _MIAF_
    pub const MIBU: BrandCode = BrandCode(FourCC(*b"MiBu"));

    /// Multi-Image Application format brand for CMAF compatibility
    /// 
    /// FourCC: `MiCm`
    /// 
    /// Specification: _MIAF_
    pub const MICM: BrandCode = BrandCode(FourCC(*b"MiCm"));

    /// Image file format structural brand
    /// 
    /// FourCC: `mif1`
    /// 
    /// Specification: _HEIF_
    pub const MIF1: BrandCode = BrandCode(FourCC(*b"mif1"));

    /// Image file format structural brand CICP alpha and depth
    /// 
    /// FourCC: `mif2`
    /// 
    /// Specification: _HEIF_
    pub const MIF2: BrandCode = BrandCode(FourCC(*b"mif2"));

    /// Multi-Image Application format brand for MIAF HEVC Advanced Profile
    /// 
    /// FourCC: `MiHA`
    /// 
    /// Specification: _MIAF_
    pub const MIHA: BrandCode = BrandCode(FourCC(*b"MiHA"));

    /// Multi-Image Application format brand for MIAF HEVC Basic Profile
    /// 
    /// FourCC: `MiHB`
    /// 
    /// Specification: _MIAF_
    pub const MIHB: BrandCode = BrandCode(FourCC(*b"MiHB"));

    /// Multi-Image Application format brand for MIAF HEVC Extended Profile
    /// 
    /// FourCC: `MiHE`
    /// 
    /// Specification: _MIAF_
    pub const MIHE: BrandCode = BrandCode(FourCC(*b"MiHE"));

    /// Multi-Image Application format brand for progressive decoding and rendering
    /// 
    /// FourCC: `MiPr`
    /// 
    /// Specification: _MIAF_
    pub const MIPR: BrandCode = BrandCode(FourCC(*b"MiPr"));

    /// Motion JPEG 2000 simple profile
    /// 
    /// FourCC: `mj2s`
    /// 
    /// Specification: _MJ2_
    pub const MJ2S: BrandCode = BrandCode(FourCC(*b"mj2s"));

    /// Motion JPEG 2000, general profile
    /// 
    /// FourCC: `mjp2`
    /// 
    /// Specification: _MJ2_
    pub const MJP2: BrandCode = BrandCode(FourCC(*b"mjp2"));

    /// MPEG-21
    /// 
    /// FourCC: `mp21`
    /// 
    /// Specification: _MPEG-21_
    pub const MP21: BrandCode = BrandCode(FourCC(*b"mp21"));

    /// MP4 version 1
    /// 
    /// FourCC: `mp41`
    /// 
    /// Specification: _MP4v2_
    pub const MP41: BrandCode = BrandCode(FourCC(*b"mp41"));

    /// MP4 version 2
    /// 
    /// FourCC: `mp42`
    /// 
    /// Specification: _MP4v2_
    pub const MP42: BrandCode = BrandCode(FourCC(*b"mp42"));

    /// MPEG-7 file-level metadata
    /// 
    /// FourCC: `mp71`
    /// 
    /// Specification: _ISO_
    pub const MP71: BrandCode = BrandCode(FourCC(*b"mp71"));

    /// Photo Player Multimedia Application Format
    /// 
    /// FourCC: `MPPI`
    /// 
    /// Specification: _ISO-MAF_
    pub const MPPI: BrandCode = BrandCode(FourCC(*b"MPPI"));

    /// Compliance with the MMT Processing Unit format
    /// 
    /// FourCC: `mpuf`
    /// 
    /// Specification: _MMT_
    pub const MPUF: BrandCode = BrandCode(FourCC(*b"mpuf"));

    /// Image file format structural brand
    /// 
    /// FourCC: `msf1`
    /// 
    /// Specification: _HEIF_
    pub const MSF1: BrandCode = BrandCode(FourCC(*b"msf1"));

    /// Media Segment conforming to the general format type for ISO base media file format.
    /// 
    /// FourCC: `msdh`
    /// 
    /// Specification: _DASH_
    pub const MSDH: BrandCode = BrandCode(FourCC(*b"msdh"));

    /// Media Segment conforming to the Indexed Media Segment format type for ISO base media file format.
    /// 
    /// FourCC: `msix`
    /// 
    /// Specification: _DASH_
    pub const MSIX: BrandCode = BrandCode(FourCC(*b"msix"));

    /// Portable multimedia CE products using MP4 file format with AVC video codec and AAC audio codec
    /// 
    /// FourCC: `MSNV`
    /// 
    /// Specification: _IEC 62592_
    pub const MSNV: BrandCode = BrandCode(FourCC(*b"MSNV"));

    /// Nikon Digital Camera
    /// 
    /// FourCC: `niko`
    /// 
    /// Specification: _Nikon_
    pub const NIKO: BrandCode = BrandCode(FourCC(*b"niko"));

    /// Non-linear storyline toolset brand
    /// 
    /// FourCC: `nlsl`
    /// 
    /// Specification: _OMAF_
    pub const NLSL: BrandCode = BrandCode(FourCC(*b"nlsl"));

    /// No Leading Picture Sync Brand
    /// 
    /// FourCC: `nras`
    /// 
    /// Specification: _NALu Video_
    pub const NRAS: BrandCode = BrandCode(FourCC(*b"nras"));

    /// OMAF 2D audio legacy profile
    /// 
    /// FourCC: `oa2d`
    /// 
    /// Specification: _OMAF_
    pub const OA2D: BrandCode = BrandCode(FourCC(*b"oa2d"));

    /// OMAF 3D audio baseline profile
    /// 
    /// FourCC: `oabl`
    /// 
    /// Specification: _OMAF_
    pub const OABL: BrandCode = BrandCode(FourCC(*b"oabl"));

    /// OMA DCF (DRM Content Format)
    /// 
    /// FourCC: `odcf`
    /// 
    /// Specification: _OMA DRM 2.0_
    pub const ODCF: BrandCode = BrandCode(FourCC(*b"odcf"));

    /// OMAF viewport-independent baseline presentation profile
    /// 
    /// FourCC: `ompp`
    /// 
    /// Specification: _OMAF_
    pub const OMPP: BrandCode = BrandCode(FourCC(*b"ompp"));

    /// OMA PDCF (DRM Content Format)
    /// 
    /// FourCC: `opf2`
    /// 
    /// Specification: _OMA DRM 2.1_
    pub const OPF2: BrandCode = BrandCode(FourCC(*b"opf2"));

    /// OMA Adapted PDCF
    /// 
    /// FourCC: `opx2`
    /// 
    /// Specification: _OMA DRM XBS_
    pub const OPX2: BrandCode = BrandCode(FourCC(*b"opx2"));

    /// OMAF viewport-dependent baseline presentation profile
    /// 
    /// FourCC: `ovdp`
    /// 
    /// Specification: _OMAF_
    pub const OVDP: BrandCode = BrandCode(FourCC(*b"ovdp"));

    /// Overlay toolset brand
    /// 
    /// FourCC: `ovly`
    /// 
    /// Specification: _OMAF_
    pub const OVLY: BrandCode = BrandCode(FourCC(*b"ovly"));

    /// Generic Partial File
    /// 
    /// FourCC: `paff`
    /// 
    /// Specification: _ISO-Partial_
    pub const PAFF: BrandCode = BrandCode(FourCC(*b"paff"));

    /// Panasonic Digital Camera
    /// 
    /// FourCC: `pana`
    /// 
    /// Specification: _Panasonic_
    pub const PANA: BrandCode = BrandCode(FourCC(*b"pana"));

    /// Protected Interoperable File Format
    /// 
    /// FourCC: `piff`
    /// 
    /// Specification: _PIFF_
    pub const PIFF: BrandCode = BrandCode(FourCC(*b"piff"));

    /// Mixed Partial File
    /// 
    /// FourCC: `pmff`
    /// 
    /// Specification: _ISO-Partial_
    pub const PMFF: BrandCode = BrandCode(FourCC(*b"pmff"));

    /// Panasonic Video Intercom
    /// 
    /// FourCC: `pnvi`
    /// 
    /// Specification: _Panasonic Video Intercom_
    pub const PNVI: BrandCode = BrandCode(FourCC(*b"pnvi"));

    /// Image file format brand for predictively coded image items
    /// 
    /// FourCC: `pred`
    /// 
    /// Specification: _HEIF_
    pub const PRED: BrandCode = BrandCode(FourCC(*b"pred"));

    /// QuickTime
    /// 
    /// FourCC: `qt  `
    /// 
    /// Specification: _QT_
    pub const QT  : BrandCode = BrandCode(FourCC(*b"qt  "));

    /// combination brand to indicate relative addressing
    /// 
    /// FourCC: `relo`
    /// 
    /// Specification: _ISO_
    pub const RELO: BrandCode = BrandCode(FourCC(*b"relo"));

    /// Representation Index Segment used to index MPEG-2 TS based Media Segments.
    /// 
    /// FourCC: `risx`
    /// 
    /// Specification: _DASH_
    pub const RISX: BrandCode = BrandCode(FourCC(*b"risx"));

    /// Ross Video
    /// 
    /// FourCC: `ROSS`
    /// 
    /// Specification: _Ross_
    pub const ROSS: BrandCode = BrandCode(FourCC(*b"ROSS"));

    /// SD Video
    /// 
    /// FourCC: `sdv `
    /// 
    /// Specification: _SDV_
    pub const SDV : BrandCode = BrandCode(FourCC(*b"sdv "));

    /// Home and Mobile Multimedia Platform (HMMP)
    /// 
    /// FourCC: `SEAU`
    /// 
    /// Specification: _Sony_
    pub const SEAU: BrandCode = BrandCode(FourCC(*b"SEAU"));

    /// Home and Mobile Multimedia Platform (HMMP)
    /// 
    /// FourCC: `SEBK`
    /// 
    /// Specification: _Sony_
    pub const SEBK: BrandCode = BrandCode(FourCC(*b"SEBK"));

    /// Video contents Sony Entertainment Network provides by using MP4 file format
    /// 
    /// FourCC: `senv`
    /// 
    /// Specification: _Sony_
    pub const SENV: BrandCode = BrandCode(FourCC(*b"senv"));

    /// Media Segment conforming to the Sub-Indexed Media Segment format type for ISO base media file format.
    /// 
    /// FourCC: `sims`
    /// 
    /// Specification: _DASH_
    pub const SIMS: BrandCode = BrandCode(FourCC(*b"sims"));

    /// Single Index Segment used to index MPEG-2 TS based Media Segments.
    /// 
    /// FourCC: `sisx`
    /// 
    /// Specification: _DASH_
    pub const SISX: BrandCode = BrandCode(FourCC(*b"sisx"));

    /// HEVC-based simple tiling OMAF video profile
    /// 
    /// FourCC: `siti`
    /// 
    /// Specification: _OMAF_
    pub const SITI: BrandCode = BrandCode(FourCC(*b"siti"));

    /// VVC-based simple tiling OMAF video profile
    /// 
    /// FourCC: `sitv`
    /// 
    /// Specification: _OMAF_
    pub const SITV: BrandCode = BrandCode(FourCC(*b"sitv"));

    /// Dynamic metadata for Single Layer SDR-compatible HDR video streams
    /// 
    /// FourCC: `slh1`
    /// 
    /// Specification: _SL-HDR_
    pub const SLH1: BrandCode = BrandCode(FourCC(*b"slh1"));

    /// Dynamic metadata for Single Layer PQ-based HDR video streams
    /// 
    /// FourCC: `slh2`
    /// 
    /// Specification: _SL-HDR_
    pub const SLH2: BrandCode = BrandCode(FourCC(*b"slh2"));

    /// Dynamic metadata for Single Layer HLG-based HDR video streams
    /// 
    /// FourCC: `slh3`
    /// 
    /// Specification: _SL-HDR_
    pub const SLH3: BrandCode = BrandCode(FourCC(*b"slh3"));

    /// Subsegment Index Segment used to index MPEG-2 TS based Media Segments.
    /// 
    /// FourCC: `ssss`
    /// 
    /// Specification: _DASH_
    pub const SSSS: BrandCode = BrandCode(FourCC(*b"ssss"));

    /// OMAF IMSC1 timed text profile
    /// 
    /// FourCC: `ttml`
    /// 
    /// Specification: _OMAF_
    pub const TTML: BrandCode = BrandCode(FourCC(*b"ttml"));

    /// OMAF WebVTT timed text profile
    /// 
    /// FourCC: `ttwv`
    /// 
    /// Specification: _OMAF_
    pub const TTWV: BrandCode = BrandCode(FourCC(*b"ttwv"));

    /// Unconstrained HEVC-based viewport-independent OMAF video profile
    /// 
    /// FourCC: `uhvi`
    /// 
    /// Specification: _OMAF_
    pub const UHVI: BrandCode = BrandCode(FourCC(*b"uhvi"));

    /// Unified identifiers
    /// 
    /// FourCC: `unif`
    /// 
    /// Specification: _ISO_
    pub const UNIF: BrandCode = BrandCode(FourCC(*b"unif"));

    /// UltraViolet file brand – conforming to the DECE Common File Format spec, Annex E.
    /// 
    /// FourCC: `uvvu`
    /// 
    /// Specification: _DECE_
    pub const UVVU: BrandCode = BrandCode(FourCC(*b"uvvu"));

    /// Multi-track encapsulation mode for V3C data with partial access support
    /// 
    /// FourCC: `v3mp`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3MP: BrandCode = BrandCode(FourCC(*b"v3mp"));

    /// Multi-track encapsulation mode for V3C data
    /// 
    /// FourCC: `v3mt`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3MT: BrandCode = BrandCode(FourCC(*b"v3mt"));

    /// Non-timed encpasulation mode for V3C data
    /// 
    /// FourCC: `v3nt`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3NT: BrandCode = BrandCode(FourCC(*b"v3nt"));

    /// Single-track encapsulation mode for V3C data
    /// 
    /// FourCC: `v3st`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3ST: BrandCode = BrandCode(FourCC(*b"v3st"));

    /// VVC-based viewport-independent OMAF video profile
    /// 
    /// FourCC: `vvci`
    /// 
    /// Specification: _OMAF_
    pub const VVCI: BrandCode = BrandCode(FourCC(*b"vvci"));

    /// VVC coded image item
    /// 
    /// FourCC: `vvic`
    /// 
    /// Specification: _HEIF_
    pub const VVIC: BrandCode = BrandCode(FourCC(*b"vvic"));

    /// VVC coded image sequence
    /// 
    /// FourCC: `vvis`
    /// 
    /// Specification: _HEIF_
    pub const VVIS: BrandCode = BrandCode(FourCC(*b"vvis"));

    /// OMAF VVC image profile
    /// 
    /// FourCC: `vvoi`
    /// 
    /// Specification: _OMAF_
    pub const VVOI: BrandCode = BrandCode(FourCC(*b"vvoi"));

    /// Viewpoint toolset brand
    /// 
    /// FourCC: `vwpt`
    /// 
    /// Specification: _OMAF_
    pub const VWPT: BrandCode = BrandCode(FourCC(*b"vwpt"));

    /// XAVC File Format
    /// 
    /// FourCC: `XAVC`
    /// 
    /// Specification: _Sony_
    pub const XAVC: BrandCode = BrandCode(FourCC(*b"XAVC"));

    /// Google specification for use by YouTube apps
    /// 
    /// FourCC: `yt4 `
    /// 
    /// Specification: _Youtube_
    pub const YT4 : BrandCode = BrandCode(FourCC(*b"yt4 "));

    /// Immersive Audio Model and Formats - Encapsulated IA Sequence
    /// 
    /// FourCC: `iamf`
    /// 
    /// Specification: _AOM-IAMF_
    pub const IAMF: BrandCode = BrandCode(FourCC(*b"iamf"));

}

impl TrackReferenceCode {
    /// Additional audio track
    /// 
    /// FourCC: `adda`
    /// 
    /// Specification: _ISO_
    pub const ADDA: TrackReferenceCode = TrackReferenceCode(FourCC(*b"adda"));

    /// DRC metadata track
    /// 
    /// FourCC: `adrc`
    /// 
    /// Specification: _ISO_
    pub const ADRC: TrackReferenceCode = TrackReferenceCode(FourCC(*b"adrc"));

    /// Auxiliary track reference
    /// 
    /// FourCC: `auxl`
    /// 
    /// Specification: _ISO_
    pub const AUXL: TrackReferenceCode = TrackReferenceCode(FourCC(*b"auxl"));

    /// AVC parameter set stream link
    /// 
    /// FourCC: `avcp`
    /// 
    /// Specification: _NALu Video_
    pub const AVCP: TrackReferenceCode = TrackReferenceCode(FourCC(*b"avcp"));

    /// this track describes the referenced track.
    /// 
    /// FourCC: `cdsc`
    /// 
    /// Specification: _ISO_
    pub const CDSC: TrackReferenceCode = TrackReferenceCode(FourCC(*b"cdsc"));

    /// this track describes the referenced tracks and track groups collectively
    /// 
    /// FourCC: `cdtg`
    /// 
    /// Specification: _OMAF_
    pub const CDTG: TrackReferenceCode = TrackReferenceCode(FourCC(*b"cdtg"));

    /// track containing the depth view
    /// 
    /// FourCC: `deps`
    /// 
    /// Specification: _NALu Video_
    pub const DEPS: TrackReferenceCode = TrackReferenceCode(FourCC(*b"deps"));

    /// this track has an MPEG-4 dependency on the referenced track
    /// 
    /// FourCC: `dpnd`
    /// 
    /// Specification: _MPEG-4_
    pub const DPND: TrackReferenceCode = TrackReferenceCode(FourCC(*b"dpnd"));

    /// EVC slice base track
    /// 
    /// FourCC: `evcr`
    /// 
    /// Specification: _NALu Video_
    pub const EVCR: TrackReferenceCode = TrackReferenceCode(FourCC(*b"evcr"));

    /// this track uses fonts carried/defined in the referenced track
    /// 
    /// FourCC: `font`
    /// 
    /// Specification: _ISO_
    pub const FONT: TrackReferenceCode = TrackReferenceCode(FourCC(*b"font"));

    /// Hint dependency
    /// 
    /// FourCC: `hind`
    /// 
    /// Specification: _ISO_
    pub const HIND: TrackReferenceCode = TrackReferenceCode(FourCC(*b"hind"));

    /// links hint track to original media track
    /// 
    /// FourCC: `hint`
    /// 
    /// Specification: _ISO_
    pub const HINT: TrackReferenceCode = TrackReferenceCode(FourCC(*b"hint"));

    /// this track contains IPI declarations for the referenced track
    /// 
    /// FourCC: `ipir`
    /// 
    /// Specification: _MPEG-4_
    pub const IPIR: TrackReferenceCode = TrackReferenceCode(FourCC(*b"ipir"));

    /// Audio layer track dependency
    /// 
    /// FourCC: `lyra`
    /// 
    /// Specification: _DTS_
    pub const LYRA: TrackReferenceCode = TrackReferenceCode(FourCC(*b"lyra"));

    /// used in indicating combinations that result into mixed network abstraction layer unit types in a coded picture of VVC
    /// 
    /// FourCC: `mixn`
    /// 
    /// Specification: _NALu Video_
    pub const MIXN: TrackReferenceCode = TrackReferenceCode(FourCC(*b"mixn"));

    /// this track is an OD track which uses the referenced track as an included elementary stream track
    /// 
    /// FourCC: `mpod`
    /// 
    /// Specification: _MPEG-4_
    pub const MPOD: TrackReferenceCode = TrackReferenceCode(FourCC(*b"mpod"));

    /// track that contains an 'oref' sample group
    /// 
    /// FourCC: `oref`
    /// 
    /// Specification: _NALu Video_
    pub const OREF: TrackReferenceCode = TrackReferenceCode(FourCC(*b"oref"));

    /// resolved by extracting an indicated subset of the referenced VVC track to reconstruct a VVC bitstream
    /// 
    /// FourCC: `recr`
    /// 
    /// Specification: _NALu Video_
    pub const RECR: TrackReferenceCode = TrackReferenceCode(FourCC(*b"recr"));

    /// HEVC Tile Track
    /// 
    /// FourCC: `sabt`
    /// 
    /// Specification: _NALu Video_
    pub const SABT: TrackReferenceCode = TrackReferenceCode(FourCC(*b"sabt"));

    /// Scalable base
    /// 
    /// FourCC: `sbas`
    /// 
    /// Specification: _NALu Video_
    pub const SBAS: TrackReferenceCode = TrackReferenceCode(FourCC(*b"sbas"));

    /// Scalable extraction
    /// 
    /// FourCC: `scal`
    /// 
    /// Specification: _NALu Video_
    pub const SCAL: TrackReferenceCode = TrackReferenceCode(FourCC(*b"scal"));

    /// reference to a shadow sync sample track
    /// 
    /// FourCC: `shsc`
    /// 
    /// Specification: _OMAF_
    pub const SHSC: TrackReferenceCode = TrackReferenceCode(FourCC(*b"shsc"));

    /// the referenced VVC subpicture tracks or 'alte' track groups of VVC subpicture tracks are used to reconstruct a VVC bitstream
    /// 
    /// FourCC: `subp`
    /// 
    /// Specification: _NALu Video_
    pub const SUBP: TrackReferenceCode = TrackReferenceCode(FourCC(*b"subp"));

    /// subtitle or timed text or overlay graphical information
    /// 
    /// FourCC: `subt`
    /// 
    /// Specification: _ISO_
    pub const SUBT: TrackReferenceCode = TrackReferenceCode(FourCC(*b"subt"));

    /// AVC Switch from
    /// 
    /// FourCC: `swfr`
    /// 
    /// Specification: _NALu Video_
    pub const SWFR: TrackReferenceCode = TrackReferenceCode(FourCC(*b"swfr"));

    /// AVC Switch to
    /// 
    /// FourCC: `swto`
    /// 
    /// Specification: _NALu Video_
    pub const SWTO: TrackReferenceCode = TrackReferenceCode(FourCC(*b"swto"));

    /// this track uses the referenced track as its synchronization source.
    /// 
    /// FourCC: `sync`
    /// 
    /// Specification: _MPEG-4_
    pub const SYNC: TrackReferenceCode = TrackReferenceCode(FourCC(*b"sync"));

    /// HEVC Tile track base
    /// 
    /// FourCC: `tbas`
    /// 
    /// Specification: _NALu Video_
    pub const TBAS: TrackReferenceCode = TrackReferenceCode(FourCC(*b"tbas"));

    /// Thumbnail track reference
    /// 
    /// FourCC: `thmb`
    /// 
    /// Specification: _ISO_
    pub const THMB: TrackReferenceCode = TrackReferenceCode(FourCC(*b"thmb"));

    /// Time code. Usually references a time code track.
    /// 
    /// FourCC: `tmcd`
    /// 
    /// Specification: _Apple_
    pub const TMCD: TrackReferenceCode = TrackReferenceCode(FourCC(*b"tmcd"));

    /// V3C atlas track
    /// 
    /// FourCC: `v3cs`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3CS: TrackReferenceCode = TrackReferenceCode(FourCC(*b"v3cs"));

    /// V3C atlas tile track
    /// 
    /// FourCC: `v3ct`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3CT: TrackReferenceCode = TrackReferenceCode(FourCC(*b"v3ct"));

    /// V3C attribute video track
    /// 
    /// FourCC: `v3va`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3VA: TrackReferenceCode = TrackReferenceCode(FourCC(*b"v3va"));

    /// V3C geometry video track
    /// 
    /// FourCC: `v3vg`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3VG: TrackReferenceCode = TrackReferenceCode(FourCC(*b"v3vg"));

    /// V3C occupancy video track
    /// 
    /// FourCC: `v3vo`
    /// 
    /// Specification: _V3C-SYS_
    pub const V3VO: TrackReferenceCode = TrackReferenceCode(FourCC(*b"v3vo"));

    /// Auxiliary video depth
    /// 
    /// FourCC: `vdep`
    /// 
    /// Specification: _ISO_
    pub const VDEP: TrackReferenceCode = TrackReferenceCode(FourCC(*b"vdep"));

    /// Auxiliary video parallax
    /// 
    /// FourCC: `vplx`
    /// 
    /// Specification: _ISO_
    pub const VPLX: TrackReferenceCode = TrackReferenceCode(FourCC(*b"vplx"));

    /// reference to a track that contains a 'vopi' sample group for VVC video
    /// 
    /// FourCC: `vref`
    /// 
    /// Specification: _NALu Video_
    pub const VREF: TrackReferenceCode = TrackReferenceCode(FourCC(*b"vref"));

    /// reference to a VVC operating point entity group
    /// 
    /// FourCC: `vreg`
    /// 
    /// Specification: _NALu Video_
    pub const VREG: TrackReferenceCode = TrackReferenceCode(FourCC(*b"vreg"));

    /// the referenced track is a non video coding layer track of VVC
    /// 
    /// FourCC: `vvcN`
    /// 
    /// Specification: _NALu Video_
    pub const VVCN: TrackReferenceCode = TrackReferenceCode(FourCC(*b"vvcN"));

}