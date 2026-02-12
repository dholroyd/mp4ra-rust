// --------
// WARNING
// This is generated code.
// If you need changes, alter the mp4ra_crategen project, not this file.
// --------
impl HandlerCode {
    /// 3GPP Scene Description
    ///
    /// FourCC: `3gsd`
    ///
    /// Specification: _3GPP_
    pub const THREE_GSD: HandlerCode = HandlerCode::new(*b"3gsd");
    /// Auxiliary video
    ///
    /// FourCC: `auxv`
    ///
    /// Specification: _ISO_
    pub const AUXV: HandlerCode = HandlerCode::new(*b"auxv");
    /// Avid Metadata
    ///
    /// FourCC: `avmd`
    ///
    /// Specification: _Avid_
    pub const AVMD: HandlerCode = HandlerCode::new(*b"avmd");
    /// Closed Caption
    ///
    /// FourCC: `clcp`
    ///
    /// Specification: _Apple_
    pub const CLCP: HandlerCode = HandlerCode::new(*b"clcp");
    /// CPCM Auxiliary Metadata
    ///
    /// FourCC: `cpad`
    ///
    /// Specification: _DVB_
    pub const CPAD: HandlerCode = HandlerCode::new(*b"cpad");
    /// ClockReferenceStream
    ///
    /// FourCC: `crsm`
    ///
    /// Specification: _MP4v2_
    pub const CRSM: HandlerCode = HandlerCode::new(*b"crsm");
    /// DVB Mandatory Metadata
    ///
    /// FourCC: `dmbd`
    ///
    /// Specification: _DVB_
    pub const DMBD: HandlerCode = HandlerCode::new(*b"dmbd");
    /// TV-Anytime Metadata, according to DVB specifications
    ///
    /// FourCC: `dtva`
    ///
    /// Specification: _DVB_
    pub const DTVA: HandlerCode = HandlerCode::new(*b"dtva");
    /// withdrawn, unused, do not use (was Dolby Vision Metadata)
    ///
    /// FourCC: `dvmd`
    ///
    /// Specification: _Deprecated_
    pub const DVMD: HandlerCode = HandlerCode::new(*b"dvmd");
    /// Font
    ///
    /// FourCC: `fdsm`
    ///
    /// Specification: _ISO_
    pub const FDSM: HandlerCode = HandlerCode::new(*b"fdsm");
    /// General MPEG-4 systems streams (without specific handler)
    ///
    /// FourCC: `gesm`
    ///
    /// Specification: _see (1) below_
    pub const GESM: HandlerCode = HandlerCode::new(*b"gesm");
    /// Subtitle Graphics
    ///
    /// FourCC: `GRAP`
    ///
    /// Specification: _Sony_
    pub const GRAP: HandlerCode = HandlerCode::new(*b"GRAP");
    /// Hint
    ///
    /// FourCC: `hint`
    ///
    /// Specification: _ISO_
    pub const HINT: HandlerCode = HandlerCode::new(*b"hint");
    /// Hipix Rich Picture Format
    ///
    /// FourCC: `hpix`
    ///
    /// Specification: _Hipix_
    pub const HPIX: HandlerCode = HandlerCode::new(*b"hpix");
    /// ID3 version 2 meta-data handler (meta box)
    ///
    /// FourCC: `ID32`
    ///
    /// Specification: _id3v2_
    pub const ID32: HandlerCode = HandlerCode::new(*b"ID32");
    /// DVB IPDC ESG Metadata
    ///
    /// FourCC: `ipdc`
    ///
    /// Specification: _DVB_
    pub const IPDC: HandlerCode = HandlerCode::new(*b"ipdc");
    /// IPMP Stream
    ///
    /// FourCC: `ipsm`
    ///
    /// Specification: _MP4v2_
    pub const IPSM: HandlerCode = HandlerCode::new(*b"ipsm");
    /// MPEG7Stream
    ///
    /// FourCC: `m7sm`
    ///
    /// Specification: _MP4v2_
    pub const M7SM: HandlerCode = HandlerCode::new(*b"m7sm");
    /// Metadata
    ///
    /// FourCC: `meta`
    ///
    /// Specification: _ISO_
    pub const META: HandlerCode = HandlerCode::new(*b"meta");
    /// MPEG-J Stream
    ///
    /// FourCC: `mjsm`
    ///
    /// Specification: _MP4v2_
    pub const MJSM: HandlerCode = HandlerCode::new(*b"mjsm");
    /// MPEG-21 Digital item
    ///
    /// FourCC: `mp21`
    ///
    /// Specification: _MPEG-21_
    pub const MP21: HandlerCode = HandlerCode::new(*b"mp21");
    /// MPEG-7 (binary meta-data)
    ///
    /// FourCC: `mp7b`
    ///
    /// Specification: _ISO_
    pub const MP7B: HandlerCode = HandlerCode::new(*b"mp7b");
    /// MPEG-7 (textual meta-data)
    ///
    /// FourCC: `mp7t`
    ///
    /// Specification: _ISO_
    pub const MP7T: HandlerCode = HandlerCode::new(*b"mp7t");
    /// MPD contained in a metabox
    ///
    /// FourCC: `mpd `
    ///
    /// Specification: _3GPP_
    pub const MPD: HandlerCode = HandlerCode::new(*b"mpd ");
    /// MPD link contained in a metabox
    ///
    /// FourCC: `mpdl`
    ///
    /// Specification: _3GPP_
    pub const MPDL: HandlerCode = HandlerCode::new(*b"mpdl");
    /// QuickTime MPEG track handler
    ///
    /// FourCC: `MPEG`
    ///
    /// Specification: _Apple_
    pub const MPEG: HandlerCode = HandlerCode::new(*b"MPEG");
    /// QuickTime Music track handler
    ///
    /// FourCC: `musi`
    ///
    /// Specification: _Apple_
    pub const MUSI: HandlerCode = HandlerCode::new(*b"musi");
    /// Non-Real Time Metadata (XAVC Format)
    ///
    /// FourCC: `nrtm`
    ///
    /// Specification: _Sony_
    pub const NRTM: HandlerCode = HandlerCode::new(*b"nrtm");
    /// No handling required (meta-data)
    ///
    /// FourCC: `null`
    ///
    /// Specification: _ISO_
    pub const NULL: HandlerCode = HandlerCode::new(*b"null");
    /// ObjectContentInfoStream
    ///
    /// FourCC: `ocsm`
    ///
    /// Specification: _MP4v2_
    pub const OCSM: HandlerCode = HandlerCode::new(*b"ocsm");
    /// ObjectDescriptorStream
    ///
    /// FourCC: `odsm`
    ///
    /// Specification: _MP4v2_
    pub const ODSM: HandlerCode = HandlerCode::new(*b"odsm");
    /// Image Item and Image sequences
    ///
    /// FourCC: `pict`
    ///
    /// Specification: _HEIF_
    pub const PICT: HandlerCode = HandlerCode::new(*b"pict");
    /// QuickTime QuickDraw 3D ttrack handler
    ///
    /// FourCC: `qd3d`
    ///
    /// Specification: _Apple_
    pub const QD3D: HandlerCode = HandlerCode::new(*b"qd3d");
    /// QuickTime Subtitle track handler
    ///
    /// FourCC: `sbtl`
    ///
    /// Specification: _Apple_
    pub const SBTL: HandlerCode = HandlerCode::new(*b"sbtl");
    /// SceneDescriptionStream
    ///
    /// FourCC: `sdsm`
    ///
    /// Specification: _MP4v2_
    pub const SDSM: HandlerCode = HandlerCode::new(*b"sdsm");
    /// Key Management Messages
    ///
    /// FourCC: `skmm`
    ///
    /// Specification: _DVB_
    pub const SKMM: HandlerCode = HandlerCode::new(*b"skmm");
    /// Samsung Video Metadata Handler
    ///
    /// FourCC: `smhr`
    ///
    /// Specification: _Samsung_
    pub const SMHR: HandlerCode = HandlerCode::new(*b"smhr");
    /// Audio
    ///
    /// FourCC: `soun`
    ///
    /// Specification: _ISO_
    pub const SOUN: HandlerCode = HandlerCode::new(*b"soun");
    /// QuickTime Sprite track handler
    ///
    /// FourCC: `sprt`
    ///
    /// Specification: _Apple_
    pub const SPRT: HandlerCode = HandlerCode::new(*b"sprt");
    /// QuickTime Streaming track handler
    ///
    /// FourCC: `strm`
    ///
    /// Specification: _Apple_
    pub const STRM: HandlerCode = HandlerCode::new(*b"strm");
    /// Subtitles
    ///
    /// FourCC: `subt`
    ///
    /// Specification: _ISO_
    pub const SUBT: HandlerCode = HandlerCode::new(*b"subt");
    /// Text
    ///
    /// FourCC: `text`
    ///
    /// Specification: _3GPP_
    pub const TEXT: HandlerCode = HandlerCode::new(*b"text");
    /// Timecode
    ///
    /// FourCC: `tmcd`
    ///
    /// Specification: _Apple_
    pub const TMCD: HandlerCode = HandlerCode::new(*b"tmcd");
    /// QuickTime Tween track handler
    ///
    /// FourCC: `twen`
    ///
    /// Specification: _Apple_
    pub const TWEN: HandlerCode = HandlerCode::new(*b"twen");
    /// URI identified metadata
    ///
    /// FourCC: `uri `
    ///
    /// Specification: _DVB_
    pub const URI: HandlerCode = HandlerCode::new(*b"uri ");
    /// Video
    ///
    /// FourCC: `vide`
    ///
    /// Specification: _ISO_
    pub const VIDE: HandlerCode = HandlerCode::new(*b"vide");
    /// Volumetric visual media
    ///
    /// FourCC: `volv`
    ///
    /// Specification: _ISO_
    pub const VOLV: HandlerCode = HandlerCode::new(*b"volv");
    /// Haptics
    ///
    /// FourCC: `hapt`
    ///
    /// Specification: _ISO_
    pub const HAPT: HandlerCode = HandlerCode::new(*b"hapt");
    /// Neural network handler (items defined in the MetaBox represent neural networks)
    ///
    /// FourCC: `nnet`
    ///
    /// Specification: _NALu Video_
    pub const NNET: HandlerCode = HandlerCode::new(*b"nnet");
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
            SampleEntryCode::AV3A => Some(HandlerCode::SOUN),
            SampleEntryCode::DAV1 => Some(HandlerCode::VIDE),
            SampleEntryCode::DEPI => Some(HandlerCode::META),
            SampleEntryCode::DFCE => Some(HandlerCode::META),
            SampleEntryCode::DIPI => Some(HandlerCode::META),
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
            SampleEntryCode::EVTE => Some(HandlerCode::META),
            SampleEntryCode::FDP => Some(HandlerCode::HINT),
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
            SampleEntryCode::ICP3 => Some(HandlerCode::VOLV),
            SampleEntryCode::ICPA => Some(HandlerCode::SOUN),
            SampleEntryCode::ICPH => Some(HandlerCode::HINT),
            SampleEntryCode::ICPM => Some(HandlerCode::META),
            SampleEntryCode::ICPP => Some(HandlerCode::HAPT),
            SampleEntryCode::ICPS => None,
            SampleEntryCode::ICPT => Some(HandlerCode::TEXT),
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
            SampleEntryCode::RAW => Some(HandlerCode::SOUN),
            SampleEntryCode::RCVP => Some(HandlerCode::META),
            SampleEntryCode::RESV => Some(HandlerCode::VIDE),
            SampleEntryCode::RESA => Some(HandlerCode::SOUN),
            SampleEntryCode::RM2T => Some(HandlerCode::HINT),
            SampleEntryCode::RRTP => Some(HandlerCode::HINT),
            SampleEntryCode::RSRP => Some(HandlerCode::HINT),
            SampleEntryCode::RTCP => Some(HandlerCode::HINT),
            SampleEntryCode::RTMD => Some(HandlerCode::META),
            SampleEntryCode::RTP => Some(HandlerCode::HINT),
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
            SampleEntryCode::VQME => Some(HandlerCode::META),
            SampleEntryCode::VRSP => Some(HandlerCode::META),
            SampleEntryCode::VVCN => Some(HandlerCode::VIDE),
            SampleEntryCode::VVC1 => Some(HandlerCode::VIDE),
            SampleEntryCode::VVI1 => Some(HandlerCode::VIDE),
            SampleEntryCode::VVS1 => Some(HandlerCode::VIDE),
            SampleEntryCode::WVTT => Some(HandlerCode::TEXT),
            SampleEntryCode::ENCU => Some(HandlerCode::SUBT),
            SampleEntryCode::ENCP => Some(HandlerCode::HAPT),
            SampleEntryCode::ENC3 => Some(HandlerCode::VOLV),
            SampleEntryCode::UNCV => Some(HandlerCode::VIDE),
            SampleEntryCode::IAMF => Some(HandlerCode::SOUN),
            SampleEntryCode::IPCM => Some(HandlerCode::SOUN),
            SampleEntryCode::FPCM => Some(HandlerCode::SOUN),
            SampleEntryCode::CNDM => Some(HandlerCode::VIDE),
            SampleEntryCode::LVC1 => Some(HandlerCode::VIDE),
            SampleEntryCode::APAC => Some(HandlerCode::SOUN),
            SampleEntryCode::APV1 => Some(HandlerCode::VIDE),
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
    pub const TWO_DCC: SampleEntryCode = SampleEntryCode::new(*b"2dcc");
    /// 3GPP Location
    ///
    /// FourCC: `3glo`
    ///
    /// Specification: _3GPP_
    pub const THREE_GLO: SampleEntryCode = SampleEntryCode::new(*b"3glo");
    /// 3GPP Orientation
    ///
    /// FourCC: `3gor`
    ///
    /// Specification: _3GPP_
    pub const THREE_GOR: SampleEntryCode = SampleEntryCode::new(*b"3gor");
    /// 3GPP Video Orientation
    ///
    /// FourCC: `3gvo`
    ///
    /// Specification: _3GPP_
    pub const THREE_GVO: SampleEntryCode = SampleEntryCode::new(*b"3gvo");
    /// Dynamic viewport data
    ///
    /// FourCC: `6vpt`
    ///
    /// Specification: _V3C-SYS_
    pub const SIX_VPT: SampleEntryCode = SampleEntryCode::new(*b"6vpt");
    /// 3D-AVC track with 3D-AVC NAL units only
    ///
    /// FourCC: `a3d1`
    ///
    /// Specification: _NALu Video_
    pub const A3D1: SampleEntryCode = SampleEntryCode::new(*b"a3d1");
    /// 3D-AVC track with 3D-AVC NAL units only
    ///
    /// FourCC: `a3d2`
    ///
    /// Specification: _NALu Video_
    pub const A3D2: SampleEntryCode = SampleEntryCode::new(*b"a3d2");
    /// 3D-AVC track with 3D-AVC NAL units only
    ///
    /// FourCC: `a3d3`
    ///
    /// Specification: _NALu Video_
    pub const A3D3: SampleEntryCode = SampleEntryCode::new(*b"a3d3");
    /// 3D-AVC track with 3D-AVC NAL units only
    ///
    /// FourCC: `a3d4`
    ///
    /// Specification: _NALu Video_
    pub const A3D4: SampleEntryCode = SampleEntryCode::new(*b"a3d4");
    /// Auro-Cx 3D audio
    ///
    /// FourCC: `a3ds`
    ///
    /// Specification: _Auro_
    pub const A3DS: SampleEntryCode = SampleEntryCode::new(*b"a3ds");
    /// AC-3 audio
    ///
    /// FourCC: `ac-3`
    ///
    /// Specification: _ETSI AC-3_
    pub const AC_3: SampleEntryCode = SampleEntryCode::new(*b"ac-3");
    /// AC-4 audio
    ///
    /// FourCC: `ac-4`
    ///
    /// Specification: _ETSI AC-4_
    pub const AC_4: SampleEntryCode = SampleEntryCode::new(*b"ac-4");
    /// Apple lossless audio codec
    ///
    /// FourCC: `alac`
    ///
    /// Specification: _Apple_
    pub const ALAC: SampleEntryCode = SampleEntryCode::new(*b"alac");
    /// a-Law
    ///
    /// FourCC: `alaw`
    ///
    /// Specification: _QT_
    pub const ALAW: SampleEntryCode = SampleEntryCode::new(*b"alaw");
    /// AOM Video Codec
    ///
    /// FourCC: `av01`
    ///
    /// Specification: _AV1-ISOBMFF_
    pub const AV01: SampleEntryCode = SampleEntryCode::new(*b"av01");
    /// Advanced Video Coding
    ///
    /// FourCC: `avc1`
    ///
    /// Specification: _NALu Video_
    pub const AVC1: SampleEntryCode = SampleEntryCode::new(*b"avc1");
    /// Advanced Video Coding
    ///
    /// FourCC: `avc2`
    ///
    /// Specification: _NALu Video_
    pub const AVC2: SampleEntryCode = SampleEntryCode::new(*b"avc2");
    /// Advanced Video Coding
    ///
    /// FourCC: `avc3`
    ///
    /// Specification: _NALu Video_
    pub const AVC3: SampleEntryCode = SampleEntryCode::new(*b"avc3");
    /// Advanced Video Coding
    ///
    /// FourCC: `avc4`
    ///
    /// Specification: _NALu Video_
    pub const AVC4: SampleEntryCode = SampleEntryCode::new(*b"avc4");
    /// Advanced Video Coding Parameters
    ///
    /// FourCC: `avcp`
    ///
    /// Specification: _NALu Video_
    pub const AVCP: SampleEntryCode = SampleEntryCode::new(*b"avcp");
    /// 2nd Generation Audio Video Coding Standard of China (AVS Two)
    ///
    /// FourCC: `avst`
    ///
    /// Specification: _Avs2_
    pub const AVST: SampleEntryCode = SampleEntryCode::new(*b"avst");
    /// 3nd Generation Audio Video Coding Standard of China
    ///
    /// FourCC: `avs3`
    ///
    /// Specification: _Avs3_
    pub const AVS3: SampleEntryCode = SampleEntryCode::new(*b"avs3");
    /// Camera motion metadata track
    ///
    /// FourCC: `camm`
    ///
    /// Specification: _CamMotion_
    pub const CAMM: SampleEntryCode = SampleEntryCode::new(*b"camm");
    /// AVS2-P3 Codec
    ///
    /// FourCC: `cavs`
    ///
    /// Specification: _GB-T-20090-9_
    pub const CAVS: SampleEntryCode = SampleEntryCode::new(*b"cavs");
    /// AVS3-P3 Codec
    ///
    /// FourCC: `av3a`
    ///
    /// Specification: _T-AI-109.7_
    pub const AV3A: SampleEntryCode = SampleEntryCode::new(*b"av3a");
    /// AV1-related Dolby Vision consistent with av01
    ///
    /// FourCC: `dav1`
    ///
    /// Specification: _Dolby Vision_
    pub const DAV1: SampleEntryCode = SampleEntryCode::new(*b"dav1");
    /// Decoder power indication
    ///
    /// FourCC: `depi`
    ///
    /// Specification: _Metrics_
    pub const DEPI: SampleEntryCode = SampleEntryCode::new(*b"depi");
    /// Display fine control
    ///
    /// FourCC: `dfce`
    ///
    /// Specification: _Metrics_
    pub const DFCE: SampleEntryCode = SampleEntryCode::new(*b"dfce");
    /// Display power indication
    ///
    /// FourCC: `dipi`
    ///
    /// Specification: _Metrics_
    pub const DIPI: SampleEntryCode = SampleEntryCode::new(*b"dipi");
    /// DRA Audio
    ///
    /// FourCC: `dra1`
    ///
    /// Specification: _DRA_
    pub const DRA1: SampleEntryCode = SampleEntryCode::new(*b"dra1");
    /// Dirac Video Coder
    ///
    /// FourCC: `drac`
    ///
    /// Specification: _Dirac_
    pub const DRAC: SampleEntryCode = SampleEntryCode::new(*b"drac");
    /// Enhancement layer for DTS layered audio
    ///
    /// FourCC: `dts+`
    ///
    /// Specification: _DTS_
    pub const DTS_PLUS: SampleEntryCode = SampleEntryCode::new(*b"dts+");
    /// Dependent base layer for DTS layered audio
    ///
    /// FourCC: `dts-`
    ///
    /// Specification: _DTS_
    pub const DTS_: SampleEntryCode = SampleEntryCode::new(*b"dts-");
    /// Core Substream
    ///
    /// FourCC: `dtsc`
    ///
    /// Specification: _DTS-HD_
    pub const DTSC: SampleEntryCode = SampleEntryCode::new(*b"dtsc");
    /// Extension Substream containing only LBR
    ///
    /// FourCC: `dtse`
    ///
    /// Specification: _DTS-HD_
    pub const DTSE: SampleEntryCode = SampleEntryCode::new(*b"dtse");
    /// Core Substream + Extension Substream
    ///
    /// FourCC: `dtsh`
    ///
    /// Specification: _DTS-HD_
    pub const DTSH: SampleEntryCode = SampleEntryCode::new(*b"dtsh");
    /// Extension Substream containing only XLL
    ///
    /// FourCC: `dtsl`
    ///
    /// Specification: _DTS-HD_
    pub const DTSL: SampleEntryCode = SampleEntryCode::new(*b"dtsl");
    /// DTS-UHD profile 2
    ///
    /// FourCC: `dtsx`
    ///
    /// Specification: _DTS-UHD_
    pub const DTSX: SampleEntryCode = SampleEntryCode::new(*b"dtsx");
    /// DTS-UHD profile 3 or higher
    ///
    /// FourCC: `dtsy`
    ///
    /// Specification: _DTS-UHD_
    pub const DTSY: SampleEntryCode = SampleEntryCode::new(*b"dtsy");
    /// AVC-based Dolby Vision derived from avc1
    ///
    /// FourCC: `dva1`
    ///
    /// Specification: _Dolby Vision_
    pub const DVA1: SampleEntryCode = SampleEntryCode::new(*b"dva1");
    /// AVC-based Dolby Vision derived from avc3
    ///
    /// FourCC: `dvav`
    ///
    /// Specification: _Dolby Vision_
    pub const DVAV: SampleEntryCode = SampleEntryCode::new(*b"dvav");
    /// HEVC-based Dolby Vision derived from hvc1
    ///
    /// FourCC: `dvh1`
    ///
    /// Specification: _Dolby Vision_
    pub const DVH1: SampleEntryCode = SampleEntryCode::new(*b"dvh1");
    /// HEVC-based Dolby Vision derived from hev1
    ///
    /// FourCC: `dvhe`
    ///
    /// Specification: _Dolby Vision_
    pub const DVHE: SampleEntryCode = SampleEntryCode::new(*b"dvhe");
    /// Dynamic overlay parameters
    ///
    /// FourCC: `dyol`
    ///
    /// Specification: _OMAF_
    pub const DYOL: SampleEntryCode = SampleEntryCode::new(*b"dyol");
    /// Dynamic spatial region data
    ///
    /// FourCC: `dyvm`
    ///
    /// Specification: _V3C-SYS_
    pub const DYVM: SampleEntryCode = SampleEntryCode::new(*b"dyvm");
    /// Dynamic viewpoint parameters
    ///
    /// FourCC: `dyvp`
    ///
    /// Specification: _OMAF_
    pub const DYVP: SampleEntryCode = SampleEntryCode::new(*b"dyvp");
    /// Enhanced AC-3 audio
    ///
    /// FourCC: `ec-3`
    ///
    /// Specification: _ETSI AC-3_
    pub const EC_3: SampleEntryCode = SampleEntryCode::new(*b"ec-3");
    /// withdrawn, unused, do not use (was enhanced AC-3 audio with JOC)
    ///
    /// FourCC: `ec+3`
    ///
    /// Specification: _Deprecated_
    pub const EC_PLUS_3: SampleEntryCode = SampleEntryCode::new(*b"ec+3");
    /// Encrypted/Protected audio
    ///
    /// FourCC: `enca`
    ///
    /// Specification: _ISO_
    pub const ENCA: SampleEntryCode = SampleEntryCode::new(*b"enca");
    /// Encrypted/Protected font
    ///
    /// FourCC: `encf`
    ///
    /// Specification: _ISO_
    pub const ENCF: SampleEntryCode = SampleEntryCode::new(*b"encf");
    /// Encrypted/Protected metadata
    ///
    /// FourCC: `encm`
    ///
    /// Specification: _ISO_
    pub const ENCM: SampleEntryCode = SampleEntryCode::new(*b"encm");
    /// Encrypted Systems stream
    ///
    /// FourCC: `encs`
    ///
    /// Specification: _ISO_
    pub const ENCS: SampleEntryCode = SampleEntryCode::new(*b"encs");
    /// Encrypted Text
    ///
    /// FourCC: `enct`
    ///
    /// Specification: _ISO_
    pub const ENCT: SampleEntryCode = SampleEntryCode::new(*b"enct");
    /// Encrypted/protected video
    ///
    /// FourCC: `encv`
    ///
    /// Specification: _ISO_
    pub const ENCV: SampleEntryCode = SampleEntryCode::new(*b"encv");
    /// Essential Video Coding
    ///
    /// FourCC: `evc1`
    ///
    /// Specification: _NALu Video_
    pub const EVC1: SampleEntryCode = SampleEntryCode::new(*b"evc1");
    /// Essential Video Coding slice base track
    ///
    /// FourCC: `evm1`
    ///
    /// Specification: _NALu Video_
    pub const EVM1: SampleEntryCode = SampleEntryCode::new(*b"evm1");
    /// Essential Video Coding slice component track without parameter sets
    ///
    /// FourCC: `evs1`
    ///
    /// Specification: _NALu Video_
    pub const EVS1: SampleEntryCode = SampleEntryCode::new(*b"evs1");
    /// Essential Video Coding slice component track that may contain parameter sets
    ///
    /// FourCC: `evs2`
    ///
    /// Specification: _NALu Video_
    pub const EVS2: SampleEntryCode = SampleEntryCode::new(*b"evs2");
    /// Event message instance
    ///
    /// FourCC: `evte`
    ///
    /// Specification: _Event Message_
    pub const EVTE: SampleEntryCode = SampleEntryCode::new(*b"evte");
    /// File delivery hints
    ///
    /// FourCC: `fdp `
    ///
    /// Specification: _ISO_
    pub const FDP: SampleEntryCode = SampleEntryCode::new(*b"fdp ");
    /// An open lossless intra-frame video codec
    ///
    /// FourCC: `FFV1`
    ///
    /// Specification: _FF Video Codec_
    pub const FFV1: SampleEntryCode = SampleEntryCode::new(*b"FFV1");
    /// Free Lossless Audio Codec (FLAC)
    ///
    /// FourCC: `fLaC`
    ///
    /// Specification: _FLAC_
    pub const FLAC: SampleEntryCode = SampleEntryCode::new(*b"fLaC");
    /// ITU-T Recommendation G.719 (2008)
    ///
    /// FourCC: `g719`
    ///
    /// Specification: _ITU G.719_
    pub const G719: SampleEntryCode = SampleEntryCode::new(*b"g719");
    /// ITU-T Recommendation G.726 (1990)
    ///
    /// FourCC: `g726`
    ///
    /// Specification: _SDV_
    pub const G726: SampleEntryCode = SampleEntryCode::new(*b"g726");
    /// HEVC video with parameter sets in the Sample Entry or samples
    ///
    /// FourCC: `hev1`
    ///
    /// Specification: _NALu Video_
    pub const HEV1: SampleEntryCode = SampleEntryCode::new(*b"hev1");
    /// HEVC video with constrained extractors and/or aggregators and parameter sets in the Sample Entry or samples
    ///
    /// FourCC: `hev2`
    ///
    /// Specification: _NALu Video_
    pub const HEV2: SampleEntryCode = SampleEntryCode::new(*b"hev2");
    /// HEVC video with extractors and/or aggregators and parameter sets in the Sample Entry or samples
    ///
    /// FourCC: `hev3`
    ///
    /// Specification: _NALu Video_
    pub const HEV3: SampleEntryCode = SampleEntryCode::new(*b"hev3");
    /// HEVC video with parameter sets only in the Sample Entry
    ///
    /// FourCC: `hvc1`
    ///
    /// Specification: _NALu Video_
    pub const HVC1: SampleEntryCode = SampleEntryCode::new(*b"hvc1");
    /// HEVC video with constrained extractors and/or aggregators and parameter sets only in the Sample Entry
    ///
    /// FourCC: `hvc2`
    ///
    /// Specification: _NALu Video_
    pub const HVC2: SampleEntryCode = SampleEntryCode::new(*b"hvc2");
    /// HEVC video with extractors and/or aggregators and parameter sets only in the Sample Entry
    ///
    /// FourCC: `hvc3`
    ///
    /// Specification: _NALu Video_
    pub const HVC3: SampleEntryCode = SampleEntryCode::new(*b"hvc3");
    /// HEVC tile tracks
    ///
    /// FourCC: `hvt1`
    ///
    /// Specification: _NALu Video_
    pub const HVT1: SampleEntryCode = SampleEntryCode::new(*b"hvt1");
    /// HEVC slice segment data track
    ///
    /// FourCC: `hvt2`
    ///
    /// Specification: _NALu Video_
    pub const HVT2: SampleEntryCode = SampleEntryCode::new(*b"hvt2");
    /// HEVC Tile Track with Slice Segment Header Info
    ///
    /// FourCC: `hvt3`
    ///
    /// Specification: _NALu Video_
    pub const HVT3: SampleEntryCode = SampleEntryCode::new(*b"hvt3");
    /// Incomplete video
    ///
    /// FourCC: `icpv`
    ///
    /// Specification: _ISO_
    pub const ICPV: SampleEntryCode = SampleEntryCode::new(*b"icpv");
    /// Incomplete volumetric visual
    ///
    /// FourCC: `icp3`
    ///
    /// Specification: _ISO_
    pub const ICP3: SampleEntryCode = SampleEntryCode::new(*b"icp3");
    /// Incomplete audio
    ///
    /// FourCC: `icpa`
    ///
    /// Specification: _ISO_
    pub const ICPA: SampleEntryCode = SampleEntryCode::new(*b"icpa");
    /// Incomplete hint
    ///
    /// FourCC: `icph`
    ///
    /// Specification: _ISO_
    pub const ICPH: SampleEntryCode = SampleEntryCode::new(*b"icph");
    /// Incomplete timed metadata
    ///
    /// FourCC: `icpm`
    ///
    /// Specification: _ISO_
    pub const ICPM: SampleEntryCode = SampleEntryCode::new(*b"icpm");
    /// Incomplete haptics
    ///
    /// FourCC: `icpp`
    ///
    /// Specification: _ISO_
    pub const ICPP: SampleEntryCode = SampleEntryCode::new(*b"icpp");
    /// Incomplete system
    ///
    /// FourCC: `icps`
    ///
    /// Specification: _ISO_
    pub const ICPS: SampleEntryCode = SampleEntryCode::new(*b"icps");
    /// Incomplete text
    ///
    /// FourCC: `icpt`
    ///
    /// Specification: _ISO_
    pub const ICPT: SampleEntryCode = SampleEntryCode::new(*b"icpt");
    /// Initial viewing orientation
    ///
    /// FourCC: `invo`
    ///
    /// Specification: _OMAF_
    pub const INVO: SampleEntryCode = SampleEntryCode::new(*b"invo");
    /// Initial viewpoint
    ///
    /// FourCC: `invp`
    ///
    /// Specification: _OMAF_
    pub const INVP: SampleEntryCode = SampleEntryCode::new(*b"invp");
    /// DVB Track Level Index Track
    ///
    /// FourCC: `ixse`
    ///
    /// Specification: _DVB_
    pub const IXSE: SampleEntryCode = SampleEntryCode::new(*b"ixse");
    /// Sequence of JPEG 2000 Contiguous Codestream boxes as defined in Rec. ITU-T T.800 | ISO/IEC 15444-1
    ///
    /// FourCC: `j2ki`
    ///
    /// Specification: _J2KHEIF_
    pub const J2KI: SampleEntryCode = SampleEntryCode::new(*b"j2ki");
    /// Video and image sequences coded to the JPEG-XS coding format
    ///
    /// FourCC: `jxsm`
    ///
    /// Specification: _JPXS_
    pub const JXSM: SampleEntryCode = SampleEntryCode::new(*b"jxsm");
    /// Layered HEVC
    ///
    /// FourCC: `lhe1`
    ///
    /// Specification: _NALu Video_
    pub const LHE1: SampleEntryCode = SampleEntryCode::new(*b"lhe1");
    /// Layered HEVC tile tracks
    ///
    /// FourCC: `lht1`
    ///
    /// Specification: _NALu Video_
    pub const LHT1: SampleEntryCode = SampleEntryCode::new(*b"lht1");
    /// Layered HEVC
    ///
    /// FourCC: `lhv1`
    ///
    /// Specification: _NALu Video_
    pub const LHV1: SampleEntryCode = SampleEntryCode::new(*b"lhv1");
    /// MPEG-2 transport stream for DMB
    ///
    /// FourCC: `m2ts`
    ///
    /// Specification: _DMB-MAF_
    pub const M2TS: SampleEntryCode = SampleEntryCode::new(*b"m2ts");
    /// MPEG-4 Audio Enhancement
    ///
    /// FourCC: `m4ae`
    ///
    /// Specification: _MP4v2_
    pub const M4AE: SampleEntryCode = SampleEntryCode::new(*b"m4ae");
    /// Timed metadata multiplex
    ///
    /// FourCC: `mebx`
    ///
    /// Specification: _ISO_
    pub const MEBX: SampleEntryCode = SampleEntryCode::new(*b"mebx");
    /// Text timed metadata that is not XML
    ///
    /// FourCC: `mett`
    ///
    /// Specification: _ISO_
    pub const METT: SampleEntryCode = SampleEntryCode::new(*b"mett");
    /// XML timed metadata
    ///
    /// FourCC: `metx`
    ///
    /// Specification: _ISO_
    pub const METX: SampleEntryCode = SampleEntryCode::new(*b"metx");
    /// MPEG-H Audio (single stream, unencapsulated)
    ///
    /// FourCC: `mha1`
    ///
    /// Specification: _MPEG-H_
    pub const MHA1: SampleEntryCode = SampleEntryCode::new(*b"mha1");
    /// MPEG-H Audio (multi-stream, unencapsulated)
    ///
    /// FourCC: `mha2`
    ///
    /// Specification: _MPEG-H_
    pub const MHA2: SampleEntryCode = SampleEntryCode::new(*b"mha2");
    /// MPEG-H Audio (single stream, MHAS encapsulated)
    ///
    /// FourCC: `mhm1`
    ///
    /// Specification: _MPEG-H_
    pub const MHM1: SampleEntryCode = SampleEntryCode::new(*b"mhm1");
    /// MPEG-H Audio (multi-stream, MHAS encapsulated)
    ///
    /// FourCC: `mhm2`
    ///
    /// Specification: _MPEG-H_
    pub const MHM2: SampleEntryCode = SampleEntryCode::new(*b"mhm2");
    /// Motion JPEG 2000
    ///
    /// FourCC: `mjp2`
    ///
    /// Specification: _MJ2_
    pub const MJP2: SampleEntryCode = SampleEntryCode::new(*b"mjp2");
    /// JPEG image sequences
    ///
    /// FourCC: `mjpg`
    ///
    /// Specification: _HEIF_
    pub const MJPG: SampleEntryCode = SampleEntryCode::new(*b"mjpg");
    /// DVB Movie level index track
    ///
    /// FourCC: `mlix`
    ///
    /// Specification: _DVB_
    pub const MLIX: SampleEntryCode = SampleEntryCode::new(*b"mlix");
    /// MLP Audio
    ///
    /// FourCC: `mlpa`
    ///
    /// Specification: _Dolby MLP_
    pub const MLPA: SampleEntryCode = SampleEntryCode::new(*b"mlpa");
    /// MPEG-4 Audio
    ///
    /// FourCC: `mp4a`
    ///
    /// Specification: _MP4v2_
    pub const MP4A: SampleEntryCode = SampleEntryCode::new(*b"mp4a");
    /// MPEG-4 Systems
    ///
    /// FourCC: `mp4s`
    ///
    /// Specification: _MP4v2_
    pub const MP4S: SampleEntryCode = SampleEntryCode::new(*b"mp4s");
    /// MPEG-4 Visual
    ///
    /// FourCC: `mp4v`
    ///
    /// Specification: _MP4v2_
    pub const MP4V: SampleEntryCode = SampleEntryCode::new(*b"mp4v");
    /// Multiview coding
    ///
    /// FourCC: `mvc1`
    ///
    /// Specification: _NALu Video_
    pub const MVC1: SampleEntryCode = SampleEntryCode::new(*b"mvc1");
    /// Multiview coding
    ///
    /// FourCC: `mvc2`
    ///
    /// Specification: _NALu Video_
    pub const MVC2: SampleEntryCode = SampleEntryCode::new(*b"mvc2");
    /// Multiview coding
    ///
    /// FourCC: `mvc3`
    ///
    /// Specification: _NALu Video_
    pub const MVC3: SampleEntryCode = SampleEntryCode::new(*b"mvc3");
    /// Multiview coding
    ///
    /// FourCC: `mvc4`
    ///
    /// Specification: _NALu Video_
    pub const MVC4: SampleEntryCode = SampleEntryCode::new(*b"mvc4");
    /// MVD stream
    ///
    /// FourCC: `mvd1`
    ///
    /// Specification: _NALu Video_
    pub const MVD1: SampleEntryCode = SampleEntryCode::new(*b"mvd1");
    /// MVD stream
    ///
    /// FourCC: `mvd2`
    ///
    /// Specification: _NALu Video_
    pub const MVD2: SampleEntryCode = SampleEntryCode::new(*b"mvd2");
    /// MVD stream
    ///
    /// FourCC: `mvd3`
    ///
    /// Specification: _NALu Video_
    pub const MVD3: SampleEntryCode = SampleEntryCode::new(*b"mvd3");
    /// MVD stream
    ///
    /// FourCC: `mvd4`
    ///
    /// Specification: _NALu Video_
    pub const MVD4: SampleEntryCode = SampleEntryCode::new(*b"mvd4");
    /// Object centre points correspondence between viewpoints
    ///
    /// FourCC: `ocpc`
    ///
    /// Specification: _OMAF_
    pub const OCPC: SampleEntryCode = SampleEntryCode::new(*b"ocpc");
    /// OMA Keys
    ///
    /// FourCC: `oksd`
    ///
    /// Specification: _OMA DRM XBS_
    pub const OKSD: SampleEntryCode = SampleEntryCode::new(*b"oksd");
    /// Opus audio coding
    ///
    /// FourCC: `Opus`
    ///
    /// Specification: _Opus_
    pub const OPUS: SampleEntryCode = SampleEntryCode::new(*b"Opus");
    /// Protected MPEG-2 Transport
    ///
    /// FourCC: `pm2t`
    ///
    /// Specification: _ISO_
    pub const PM2T: SampleEntryCode = SampleEntryCode::new(*b"pm2t");
    /// Protected RTP Reception
    ///
    /// FourCC: `prtp`
    ///
    /// Specification: _ISO_
    pub const PRTP: SampleEntryCode = SampleEntryCode::new(*b"prtp");
    /// Uncompressed audio
    ///
    /// FourCC: `raw `
    ///
    /// Specification: _MJ2_
    pub const RAW: SampleEntryCode = SampleEntryCode::new(*b"raw ");
    /// Recommended viewport without indicating a viewpoint
    ///
    /// FourCC: `rcvp`
    ///
    /// Specification: _OMAF_
    pub const RCVP: SampleEntryCode = SampleEntryCode::new(*b"rcvp");
    /// Restricted Video
    ///
    /// FourCC: `resv`
    ///
    /// Specification: _ISO_
    pub const RESV: SampleEntryCode = SampleEntryCode::new(*b"resv");
    /// Restricted Audio
    ///
    /// FourCC: `resa`
    ///
    /// Specification: _ISO_
    pub const RESA: SampleEntryCode = SampleEntryCode::new(*b"resa");
    /// MPEG-2 Transport Reception
    ///
    /// FourCC: `rm2t`
    ///
    /// Specification: _ISO_
    pub const RM2T: SampleEntryCode = SampleEntryCode::new(*b"rm2t");
    /// RTP reception
    ///
    /// FourCC: `rrtp`
    ///
    /// Specification: _ISO_
    pub const RRTP: SampleEntryCode = SampleEntryCode::new(*b"rrtp");
    /// SRTP Reception
    ///
    /// FourCC: `rsrp`
    ///
    /// Specification: _ISO_
    pub const RSRP: SampleEntryCode = SampleEntryCode::new(*b"rsrp");
    /// RTCP reception hint track
    ///
    /// FourCC: `rtcp`
    ///
    /// Specification: _ISO_
    pub const RTCP: SampleEntryCode = SampleEntryCode::new(*b"rtcp");
    /// Real Time Metadata Sample Entry(XAVC Format)
    ///
    /// FourCC: `rtmd`
    ///
    /// Specification: _Sony_
    pub const RTMD: SampleEntryCode = SampleEntryCode::new(*b"rtmd");
    /// RTP Hints
    ///
    /// FourCC: `rtp `
    ///
    /// Specification: _ISO_
    pub const RTP: SampleEntryCode = SampleEntryCode::new(*b"rtp ");
    /// RealVideo Codec 11
    ///
    /// FourCC: `rv60`
    ///
    /// Specification: _RealHD_
    pub const RV60: SampleEntryCode = SampleEntryCode::new(*b"rv60");
    /// Recommended viewport concerning one or more indicated viewpoints
    ///
    /// FourCC: `rvp2`
    ///
    /// Specification: _OMAF_
    pub const RVP2: SampleEntryCode = SampleEntryCode::new(*b"rvp2");
    /// ITU H.263 video (3GPP format)
    ///
    /// FourCC: `s263`
    ///
    /// Specification: _3GPP_
    pub const S263: SampleEntryCode = SampleEntryCode::new(*b"s263");
    /// Narrowband AMR voice
    ///
    /// FourCC: `samr`
    ///
    /// Specification: _3GPP_
    pub const SAMR: SampleEntryCode = SampleEntryCode::new(*b"samr");
    /// Wideband AMR voice
    ///
    /// FourCC: `sawb`
    ///
    /// Specification: _3GPP_
    pub const SAWB: SampleEntryCode = SampleEntryCode::new(*b"sawb");
    /// Extended AMR-WB (AMR-WB+)
    ///
    /// FourCC: `sawp`
    ///
    /// Specification: _3GPP_
    pub const SAWP: SampleEntryCode = SampleEntryCode::new(*b"sawp");
    /// Text subtitles
    ///
    /// FourCC: `sbtt`
    ///
    /// Specification: _ISO_
    pub const SBTT: SampleEntryCode = SampleEntryCode::new(*b"sbtt");
    /// EVRC Voice
    ///
    /// FourCC: `sevc`
    ///
    /// Specification: _3GPP2_
    pub const SEVC: SampleEntryCode = SampleEntryCode::new(*b"sevc");
    /// Enhanced Voice Services (EVS)
    ///
    /// FourCC: `sevs`
    ///
    /// Specification: _3GPP_
    pub const SEVS: SampleEntryCode = SampleEntryCode::new(*b"sevs");
    /// MPEG-2 Transport Server
    ///
    /// FourCC: `sm2t`
    ///
    /// Specification: _ISO_
    pub const SM2T: SampleEntryCode = SampleEntryCode::new(*b"sm2t");
    /// 13K Voice
    ///
    /// FourCC: `sqcp`
    ///
    /// Specification: _3GPP2_
    pub const SQCP: SampleEntryCode = SampleEntryCode::new(*b"sqcp");
    /// SRTP Hints
    ///
    /// FourCC: `srtp`
    ///
    /// Specification: _ISO_
    pub const SRTP: SampleEntryCode = SampleEntryCode::new(*b"srtp");
    /// SMV Voice
    ///
    /// FourCC: `ssmv`
    ///
    /// Specification: _3GPP2_
    pub const SSMV: SampleEntryCode = SampleEntryCode::new(*b"ssmv");
    /// SRTCP reception hint track
    ///
    /// FourCC: `stcp`
    ///
    /// Specification: _ISO_
    pub const STCP: SampleEntryCode = SampleEntryCode::new(*b"stcp");
    /// Subtitle Sample Entry (HMMP)
    ///
    /// FourCC: `STGS`
    ///
    /// Specification: _Sony_
    pub const STGS: SampleEntryCode = SampleEntryCode::new(*b"STGS");
    /// Metadata for ERP (equirectangular projection) regions
    ///
    /// FourCC: `stmd`
    ///
    /// Specification: _OMAF_
    pub const STMD: SampleEntryCode = SampleEntryCode::new(*b"stmd");
    /// Subtitles (Timed Text)
    ///
    /// FourCC: `stpp`
    ///
    /// Specification: _ISO_
    pub const STPP: SampleEntryCode = SampleEntryCode::new(*b"stpp");
    /// Simple timed text
    ///
    /// FourCC: `stxt`
    ///
    /// Specification: _ISO_
    pub const STXT: SampleEntryCode = SampleEntryCode::new(*b"stxt");
    /// Scalable Video Coding
    ///
    /// FourCC: `svc1`
    ///
    /// Specification: _NALu Video_
    pub const SVC1: SampleEntryCode = SampleEntryCode::new(*b"svc1");
    /// Scalable Video Coding
    ///
    /// FourCC: `svc2`
    ///
    /// Specification: _NALu Video_
    pub const SVC2: SampleEntryCode = SampleEntryCode::new(*b"svc2");
    /// SVC metadata
    ///
    /// FourCC: `svcM`
    ///
    /// Specification: _NALu Video_
    pub const SVCM: SampleEntryCode = SampleEntryCode::new(*b"svcM");
    /// 64 bit timecode samples
    ///
    /// FourCC: `tc64`
    ///
    /// Specification: _Apple_
    pub const TC64: SampleEntryCode = SampleEntryCode::new(*b"tc64");
    /// 32 bit timecode samples
    ///
    /// FourCC: `tmcd`
    ///
    /// Specification: _Apple_
    pub const TMCD: SampleEntryCode = SampleEntryCode::new(*b"tmcd");
    /// Sphere location for timed text
    ///
    /// FourCC: `ttsl`
    ///
    /// Specification: _OMAF_
    pub const TTSL: SampleEntryCode = SampleEntryCode::new(*b"ttsl");
    /// Uncompressed 16-bit audio
    ///
    /// FourCC: `twos`
    ///
    /// Specification: _MJ2_
    pub const TWOS: SampleEntryCode = SampleEntryCode::new(*b"twos");
    /// Timed Text stream
    ///
    /// FourCC: `tx3g`
    ///
    /// Specification: _3GPP_
    pub const TX3G: SampleEntryCode = SampleEntryCode::new(*b"tx3g");
    /// Samples have been compressed using uLaw 2:1.
    ///
    /// FourCC: `ulaw`
    ///
    /// Specification: _QT_
    pub const ULAW: SampleEntryCode = SampleEntryCode::new(*b"ulaw");
    /// Dynamic Range Control (DRC) data
    ///
    /// FourCC: `unid`
    ///
    /// Specification: _DRC_
    pub const UNID: SampleEntryCode = SampleEntryCode::new(*b"unid");
    /// Binary timed metadata identified by URI
    ///
    /// FourCC: `urim`
    ///
    /// Specification: _ISO_
    pub const URIM: SampleEntryCode = SampleEntryCode::new(*b"urim");
    /// V3C atlas track with atlas parameter sets only in sample entries
    ///
    /// FourCC: `v3a1`
    ///
    /// Specification: _V3C-SYS_
    pub const V3A1: SampleEntryCode = SampleEntryCode::new(*b"v3a1");
    /// V3C atlas track with atlas parameter sets in sample entries or samples
    ///
    /// FourCC: `v3ag`
    ///
    /// Specification: _V3C-SYS_
    pub const V3AG: SampleEntryCode = SampleEntryCode::new(*b"v3ag");
    /// V3C atlas track with a single atlas and atlas parameter sets only in sample entries
    ///
    /// FourCC: `v3c1`
    ///
    /// Specification: _V3C-SYS_
    pub const V3C1: SampleEntryCode = SampleEntryCode::new(*b"v3c1");
    /// V3C atlas base track with common atlas data
    ///
    /// FourCC: `v3cb`
    ///
    /// Specification: _V3C-SYS_
    pub const V3CB: SampleEntryCode = SampleEntryCode::new(*b"v3cb");
    /// V3C atlas track with a single atlas and atlas parameter sets in sample entries or samples
    ///
    /// FourCC: `v3cg`
    ///
    /// Specification: _V3C-SYS_
    pub const V3CG: SampleEntryCode = SampleEntryCode::new(*b"v3cg");
    /// V3C bitstream track with atlas parameter sets only in sample entries
    ///
    /// FourCC: `v3e1`
    ///
    /// Specification: _V3C-SYS_
    pub const V3E1: SampleEntryCode = SampleEntryCode::new(*b"v3e1");
    /// V3C bitstream track with atlas parameter sets in sample entries or samples
    ///
    /// FourCC: `v3eg`
    ///
    /// Specification: _V3C-SYS_
    pub const V3EG: SampleEntryCode = SampleEntryCode::new(*b"v3eg");
    /// V3C atlas tile track with atlas tile data
    ///
    /// FourCC: `v3t1`
    ///
    /// Specification: _V3C-SYS_
    pub const V3T1: SampleEntryCode = SampleEntryCode::new(*b"v3t1");
    /// SMPTE VC-1
    ///
    /// FourCC: `vc-1`
    ///
    /// Specification: _SMPTE_
    pub const VC_1: SampleEntryCode = SampleEntryCode::new(*b"vc-1");
    /// VP8 video
    ///
    /// FourCC: `vp08`
    ///
    /// Specification: _VPxx_
    pub const VP08: SampleEntryCode = SampleEntryCode::new(*b"vp08");
    /// VP9 video
    ///
    /// FourCC: `vp09`
    ///
    /// Specification: _VPxx_
    pub const VP09: SampleEntryCode = SampleEntryCode::new(*b"vp09");
    /// Quality metrics
    ///
    /// FourCC: `vqme`
    ///
    /// Specification: _Metrics_
    pub const VQME: SampleEntryCode = SampleEntryCode::new(*b"vqme");
    /// Viewing space
    ///
    /// FourCC: `vrsp`
    ///
    /// Specification: _OMAF_
    pub const VRSP: SampleEntryCode = SampleEntryCode::new(*b"vrsp");
    /// Versatile Video Coding with non-VCL (Video Coding Layer) NAL (Network Abstraction Layer) units only
    ///
    /// FourCC: `vvcN`
    ///
    /// Specification: _NALu Video_
    pub const VVCN: SampleEntryCode = SampleEntryCode::new(*b"vvcN");
    /// Versatile Video Coding with parameter sets only in sample entries
    ///
    /// FourCC: `vvc1`
    ///
    /// Specification: _NALu Video_
    pub const VVC1: SampleEntryCode = SampleEntryCode::new(*b"vvc1");
    /// Versatile Video Coding with parameter sets in sample entries or samples
    ///
    /// FourCC: `vvi1`
    ///
    /// Specification: _NALu Video_
    pub const VVI1: SampleEntryCode = SampleEntryCode::new(*b"vvi1");
    /// Versatile Video Coding (VVC) subpicture track that does not contain a conforming VVC bitstream
    ///
    /// FourCC: `vvs1`
    ///
    /// Specification: _NALu Video_
    pub const VVS1: SampleEntryCode = SampleEntryCode::new(*b"vvs1");
    /// WebVTT data
    ///
    /// FourCC: `wvtt`
    ///
    /// Specification: _ISO-Text_
    pub const WVTT: SampleEntryCode = SampleEntryCode::new(*b"wvtt");
    /// Encrypted/protected subtitles
    ///
    /// FourCC: `encu`
    ///
    /// Specification: _ISO_
    pub const ENCU: SampleEntryCode = SampleEntryCode::new(*b"encu");
    /// Encrypted/protected haptics
    ///
    /// FourCC: `encp`
    ///
    /// Specification: _ISO_
    pub const ENCP: SampleEntryCode = SampleEntryCode::new(*b"encp");
    /// Encrypted/protected volumetric visual
    ///
    /// FourCC: `enc3`
    ///
    /// Specification: _ISO_
    pub const ENC3: SampleEntryCode = SampleEntryCode::new(*b"enc3");
    /// Uncompressed Video
    ///
    /// FourCC: `uncv`
    ///
    /// Specification: _UNCV_
    pub const UNCV: SampleEntryCode = SampleEntryCode::new(*b"uncv");
    /// Immersive Audio Model and Formats - Encapsulated IA Sequence
    ///
    /// FourCC: `iamf`
    ///
    /// Specification: _AOM-IAMF_
    pub const IAMF: SampleEntryCode = SampleEntryCode::new(*b"iamf");
    /// Integer based PCM format for audio
    ///
    /// FourCC: `ipcm`
    ///
    /// Specification: _ISO-UNCA_
    pub const IPCM: SampleEntryCode = SampleEntryCode::new(*b"ipcm");
    /// Floating-point based PCM format for audio
    ///
    /// FourCC: `fpcm`
    ///
    /// Specification: _ISO-UNCA_
    pub const FPCM: SampleEntryCode = SampleEntryCode::new(*b"fpcm");
    /// Dynamic Metadata Sample Entry(XF-AVC S/XF-HEVC S File Format)
    ///
    /// FourCC: `cndm`
    ///
    /// Specification: _Canon_
    pub const CNDM: SampleEntryCode = SampleEntryCode::new(*b"cndm");
    /// LCEVC video track
    ///
    /// FourCC: `lvc1`
    ///
    /// Specification: _NALu Video_
    pub const LVC1: SampleEntryCode = SampleEntryCode::new(*b"lvc1");
    /// Apple Positional Audio Codec
    ///
    /// FourCC: `apac`
    ///
    /// Specification: _Apple_
    pub const APAC: SampleEntryCode = SampleEntryCode::new(*b"apac");
    /// APV mezzanine video codec for storage exchange and editing of professional quality video
    ///
    /// FourCC: `apv1`
    ///
    /// Specification: _OpenAPV_
    pub const APV1: SampleEntryCode = SampleEntryCode::new(*b"apv1");
}
impl BoxCode {
    /// 2D region quality ranking
    ///
    /// FourCC: `2dqr`
    ///
    /// Specification: _OMAF_
    pub const TWO_DQR: BoxCode = BoxCode::new(*b"2dqr");
    /// spatial relationship 2D source
    ///
    /// FourCC: `2dss`
    ///
    /// Specification: _OMAF_
    pub const TWO_DSS: BoxCode = BoxCode::new(*b"2dss");
    /// Asset information to identify, license and play
    ///
    /// FourCC: `ainf`
    ///
    /// Specification: _DECE_
    pub const AINF: BoxCode = BoxCode::new(*b"ainf");
    /// alternative startup sequence properties
    ///
    /// FourCC: `assp`
    ///
    /// Specification: _ISO_
    pub const ASSP: BoxCode = BoxCode::new(*b"assp");
    /// Auxiliary track type information
    ///
    /// FourCC: `auxi`
    ///
    /// Specification: _HEIF_
    pub const AUXI: BoxCode = BoxCode::new(*b"auxi");
    /// AVC NAL Unit Storage Box
    ///
    /// FourCC: `avcn`
    ///
    /// Specification: _DECE_
    pub const AVCN: BoxCode = BoxCode::new(*b"avcn");
    /// Box Index
    ///
    /// FourCC: `bidx`
    ///
    /// Specification: _ISO-Partial_
    pub const BIDX: BoxCode = BoxCode::new(*b"bidx");
    /// Base location and purchase location for license acquisition
    ///
    /// FourCC: `bloc`
    ///
    /// Specification: _DECE_
    pub const BLOC: BoxCode = BoxCode::new(*b"bloc");
    /// Buffer Model Description
    ///
    /// FourCC: `bmdm`
    ///
    /// Specification: _JPXS_
    pub const BMDM: BoxCode = BoxCode::new(*b"bmdm");
    /// Bits per component
    ///
    /// FourCC: `bpcc`
    ///
    /// Specification: _JPEG2000_
    pub const BPCC: BoxCode = BoxCode::new(*b"bpcc");
    /// Brotli-compressed box
    ///
    /// FourCC: `brob`
    ///
    /// Specification: _JPEG XL_
    pub const BROB: BoxCode = BoxCode::new(*b"brob");
    /// Buffering information
    ///
    /// FourCC: `buff`
    ///
    /// Specification: _NALu Video_
    pub const BUFF: BoxCode = BoxCode::new(*b"buff");
    /// binary XML container
    ///
    /// FourCC: `bxml`
    ///
    /// Specification: _ISO_
    pub const BXML: BoxCode = BoxCode::new(*b"bxml");
    /// OMA DRM Content ID
    ///
    /// FourCC: `ccid`
    ///
    /// Specification: _OMA DRM 2.1_
    pub const CCID: BoxCode = BoxCode::new(*b"ccid");
    /// Coding constraints box
    ///
    /// FourCC: `ccst`
    ///
    /// Specification: _HEIF_
    pub const CCST: BoxCode = BoxCode::new(*b"ccst");
    /// type and ordering of the components within the codestream
    ///
    /// FourCC: `cdef`
    ///
    /// Specification: _JPEG2000_
    pub const CDEF: BoxCode = BoxCode::new(*b"cdef");
    /// complete track information
    ///
    /// FourCC: `cinf`
    ///
    /// Specification: _ISO_
    pub const CINF: BoxCode = BoxCode::new(*b"cinf");
    /// Reserved
    ///
    /// FourCC: `clip`
    ///
    /// Specification: _ISO_
    pub const CLIP: BoxCode = BoxCode::new(*b"clip");
    /// mapping between a palette and codestream components
    ///
    /// FourCC: `cmap`
    ///
    /// Specification: _JPEG2000_
    pub const CMAP: BoxCode = BoxCode::new(*b"cmap");
    /// 64-bit chunk offset
    ///
    /// FourCC: `co64`
    ///
    /// Specification: _ISO_
    pub const CO64: BoxCode = BoxCode::new(*b"co64");
    /// composition information
    ///
    /// FourCC: `coif`
    ///
    /// Specification: _OMAF_
    pub const COIF: BoxCode = BoxCode::new(*b"coif");
    /// Content Information Box
    ///
    /// FourCC: `coin`
    ///
    /// Specification: _DECE_
    pub const COIN: BoxCode = BoxCode::new(*b"coin");
    /// specifies the colourspace of the image
    ///
    /// FourCC: `colr`
    ///
    /// Specification: _JPEG2000_
    pub const COLR: BoxCode = BoxCode::new(*b"colr");
    /// coverage information
    ///
    /// FourCC: `covi`
    ///
    /// Specification: _OMAF_
    pub const COVI: BoxCode = BoxCode::new(*b"covi");
    /// Reserved
    ///
    /// FourCC: `crgn`
    ///
    /// Specification: _ISO_
    pub const CRGN: BoxCode = BoxCode::new(*b"crgn");
    /// reserved for ClockReferenceStream header
    ///
    /// FourCC: `crhd`
    ///
    /// Specification: _MP4v2_
    pub const CRHD: BoxCode = BoxCode::new(*b"crhd");
    /// compact sample to group
    ///
    /// FourCC: `csgp`
    ///
    /// Specification: _ISO_
    pub const CSGP: BoxCode = BoxCode::new(*b"csgp");
    /// composition to decode timeline mapping
    ///
    /// FourCC: `cslg`
    ///
    /// Specification: _ISO_
    pub const CSLG: BoxCode = BoxCode::new(*b"cslg");
    /// corrected wall clock start time
    ///
    /// FourCC: `cstb`
    ///
    /// Specification: _ONVIF_
    pub const CSTB: BoxCode = BoxCode::new(*b"cstb");
    /// Reserved
    ///
    /// FourCC: `ctab`
    ///
    /// Specification: _ISO_
    pub const CTAB: BoxCode = BoxCode::new(*b"ctab");
    /// (composition) time to sample
    ///
    /// FourCC: `ctts`
    ///
    /// Specification: _ISO_
    pub const CTTS: BoxCode = BoxCode::new(*b"ctts");
    /// OMA DRM Cover URI
    ///
    /// FourCC: `cvru`
    ///
    /// Specification: _OMA DRM 2.1_
    pub const CVRU: BoxCode = BoxCode::new(*b"cvru");
    /// Data Integrity Hash
    ///
    /// FourCC: `dihd`
    ///
    /// Specification: _ISO-Partial_
    pub const DIHD: BoxCode = BoxCode::new(*b"dihd");
    /// data information box, container
    ///
    /// FourCC: `dinf`
    ///
    /// Specification: _ISO_
    pub const DINF: BoxCode = BoxCode::new(*b"dinf");
    /// Data Integrity
    ///
    /// FourCC: `dint`
    ///
    /// Specification: _ISO-Partial_
    pub const DINT: BoxCode = BoxCode::new(*b"dint");
    /// Mastering Display Metadata
    ///
    /// FourCC: `dmon`
    ///
    /// Specification: _JPXS_
    pub const DMON: BoxCode = BoxCode::new(*b"dmon");
    /// data reference box, declares source(s) of media data in track
    ///
    /// FourCC: `dref`
    ///
    /// Specification: _ISO_
    pub const DREF: BoxCode = BoxCode::new(*b"dref");
    /// DVB Sample Group Description Box
    ///
    /// FourCC: `dsgd`
    ///
    /// Specification: _DVB_
    pub const DSGD: BoxCode = BoxCode::new(*b"dsgd");
    /// DVB Sample to Group Box
    ///
    /// FourCC: `dstg`
    ///
    /// Specification: _DVB_
    pub const DSTG: BoxCode = BoxCode::new(*b"dstg");
    /// edit list container
    ///
    /// FourCC: `edts`
    ///
    /// Specification: _ISO_
    pub const EDTS: BoxCode = BoxCode::new(*b"edts");
    /// an edit list
    ///
    /// FourCC: `elst`
    ///
    /// Specification: _ISO_
    pub const ELST: BoxCode = BoxCode::new(*b"elst");
    /// event message
    ///
    /// FourCC: `emsg`
    ///
    /// Specification: _DASH_
    pub const EMSG: BoxCode = BoxCode::new(*b"emsg");
    /// Event information
    ///
    /// FourCC: `evti`
    ///
    /// Specification: _ATSC 3.0 A337_
    pub const EVTI: BoxCode = BoxCode::new(*b"evti");
    /// extended type and type combination
    ///
    /// FourCC: `etyp`
    ///
    /// Specification: _ISO_
    pub const ETYP: BoxCode = BoxCode::new(*b"etyp");
    /// Exif Metadata
    ///
    /// FourCC: `Exif`
    ///
    /// Specification: _JPXS_
    pub const EXIF: BoxCode = BoxCode::new(*b"Exif");
    /// File delivery information (item info extension)
    ///
    /// FourCC: `fdel`
    ///
    /// Specification: _ISO_
    pub const FDEL: BoxCode = BoxCode::new(*b"fdel");
    /// FEC Informatiom
    ///
    /// FourCC: `feci`
    ///
    /// Specification: _ISO_
    pub const FECI: BoxCode = BoxCode::new(*b"feci");
    /// FEC Reservoir
    ///
    /// FourCC: `fecr`
    ///
    /// Specification: _ISO_
    pub const FECR: BoxCode = BoxCode::new(*b"fecr");
    /// Box File Index
    ///
    /// FourCC: `fidx`
    ///
    /// Specification: _ISO-Partial_
    pub const FIDX: BoxCode = BoxCode::new(*b"fidx");
    /// FD Item Information
    ///
    /// FourCC: `fiin`
    ///
    /// Specification: _ISO_
    pub const FIIN: BoxCode = BoxCode::new(*b"fiin");
    /// File Reservoir
    ///
    /// FourCC: `fire`
    ///
    /// Specification: _ISO_
    pub const FIRE: BoxCode = BoxCode::new(*b"fire");
    /// fisheye omnidirectional video
    ///
    /// FourCC: `fovd`
    ///
    /// Specification: _OMAF_
    pub const FOVD: BoxCode = BoxCode::new(*b"fovd");
    /// fisheye video essential info
    ///
    /// FourCC: `fovi`
    ///
    /// Specification: _OMAF_
    pub const FOVI: BoxCode = BoxCode::new(*b"fovi");
    /// File Partition
    ///
    /// FourCC: `fpar`
    ///
    /// Specification: _ISO_
    pub const FPAR: BoxCode = BoxCode::new(*b"fpar");
    /// free space
    ///
    /// FourCC: `free`
    ///
    /// Specification: _ISO_
    pub const FREE: BoxCode = BoxCode::new(*b"free");
    /// original format box
    ///
    /// FourCC: `frma`
    ///
    /// Specification: _ISO_
    pub const FRMA: BoxCode = BoxCode::new(*b"frma");
    /// Front Part
    ///
    /// FourCC: `frpa`
    ///
    /// Specification: _ISO-Partial_
    pub const FRPA: BoxCode = BoxCode::new(*b"frpa");
    /// file type and compatibility
    ///
    /// FourCC: `ftyp`
    ///
    /// Specification: _ISO_
    pub const FTYP: BoxCode = BoxCode::new(*b"ftyp");
    /// fisheye video supplemental info
    ///
    /// FourCC: `fvsi`
    ///
    /// Specification: _OMAF_
    pub const FVSI: BoxCode = BoxCode::new(*b"fvsi");
    /// Group ID to name
    ///
    /// FourCC: `gitn`
    ///
    /// Specification: _ISO_
    pub const GITN: BoxCode = BoxCode::new(*b"gitn");
    /// OMA DRM Group ID
    ///
    /// FourCC: `grpi`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const GRPI: BoxCode = BoxCode::new(*b"grpi");
    /// Groups list box
    ///
    /// FourCC: `grpl`
    ///
    /// Specification: _ISO_
    pub const GRPL: BoxCode = BoxCode::new(*b"grpl");
    /// handler, declares the media (handler) type
    ///
    /// FourCC: `hdlr`
    ///
    /// Specification: _ISO_
    pub const HDLR: BoxCode = BoxCode::new(*b"hdlr");
    /// hint media header, overall information (hint track only)
    ///
    /// FourCC: `hmhd`
    ///
    /// Specification: _ISO_
    pub const HMHD: BoxCode = BoxCode::new(*b"hmhd");
    /// Hipix Rich Picture (user-data or meta-data)
    ///
    /// FourCC: `hpix`
    ///
    /// Specification: _Hipix_
    pub const HPIX: BoxCode = BoxCode::new(*b"hpix");
    /// OMA DRM Icon URI
    ///
    /// FourCC: `icnu`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const ICNU: BoxCode = BoxCode::new(*b"icnu");
    /// ID3 version 2 container
    ///
    /// FourCC: `ID32`
    ///
    /// Specification: _id3v2_
    pub const ID32: BoxCode = BoxCode::new(*b"ID32");
    /// Item data
    ///
    /// FourCC: `idat`
    ///
    /// Specification: _ISO_
    pub const IDAT: BoxCode = BoxCode::new(*b"idat");
    /// Image Header
    ///
    /// FourCC: `ihdr`
    ///
    /// Specification: _JPEG2000_
    pub const IHDR: BoxCode = BoxCode::new(*b"ihdr");
    /// item information
    ///
    /// FourCC: `iinf`
    ///
    /// Specification: _ISO_
    pub const IINF: BoxCode = BoxCode::new(*b"iinf");
    /// item location
    ///
    /// FourCC: `iloc`
    ///
    /// Specification: _ISO_
    pub const ILOC: BoxCode = BoxCode::new(*b"iloc");
    /// Reserved
    ///
    /// FourCC: `imap`
    ///
    /// Specification: _ISO_
    pub const IMAP: BoxCode = BoxCode::new(*b"imap");
    /// Identified media data
    ///
    /// FourCC: `imda`
    ///
    /// Specification: _ISO_
    pub const IMDA: BoxCode = BoxCode::new(*b"imda");
    /// IPMP Information box
    ///
    /// FourCC: `imif`
    ///
    /// Specification: _ISO_
    pub const IMIF: BoxCode = BoxCode::new(*b"imif");
    /// Item information entry
    ///
    /// FourCC: `infe`
    ///
    /// Specification: _ISO_
    pub const INFE: BoxCode = BoxCode::new(*b"infe");
    /// OMA DRM Info URL
    ///
    /// FourCC: `infu`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const INFU: BoxCode = BoxCode::new(*b"infu");
    /// Object Descriptor container box
    ///
    /// FourCC: `iods`
    ///
    /// Specification: _MP4v2_
    pub const IODS: BoxCode = BoxCode::new(*b"iods");
    /// ItemPropertyContainerBox
    ///
    /// FourCC: `ipco`
    ///
    /// Specification: _ISO_
    pub const IPCO: BoxCode = BoxCode::new(*b"ipco");
    /// reserved for IPMP Stream header
    ///
    /// FourCC: `iphd`
    ///
    /// Specification: _MP4v2_
    pub const IPHD: BoxCode = BoxCode::new(*b"iphd");
    /// ItemPropertyAssociation
    ///
    /// FourCC: `ipma`
    ///
    /// Specification: _ISO_
    pub const IPMA: BoxCode = BoxCode::new(*b"ipma");
    /// IPMP Control Box
    ///
    /// FourCC: `ipmc`
    ///
    /// Specification: _ISO_
    pub const IPMC: BoxCode = BoxCode::new(*b"ipmc");
    /// item protection
    ///
    /// FourCC: `ipro`
    ///
    /// Specification: _ISO_
    pub const IPRO: BoxCode = BoxCode::new(*b"ipro");
    /// Item Properties Box
    ///
    /// FourCC: `iprp`
    ///
    /// Specification: _ISO_
    pub const IPRP: BoxCode = BoxCode::new(*b"iprp");
    /// Item reference
    ///
    /// FourCC: `iref`
    ///
    /// Specification: _ISO_
    pub const IREF: BoxCode = BoxCode::new(*b"iref");
    /// JPEG 2000 header info
    ///
    /// FourCC: `j2kH`
    ///
    /// Specification: _J2KHEIF_
    pub const J2KH: BoxCode = BoxCode::new(*b"j2kH");
    /// JPEG 2000 prefix
    ///
    /// FourCC: `j2kP`
    ///
    /// Specification: _J2KHEIF_
    pub const J2KP: BoxCode = BoxCode::new(*b"j2kP");
    /// JPEG bitstream reconstruction data
    ///
    /// FourCC: `jbrd`
    ///
    /// Specification: _JPEG XL_
    pub const JBRD: BoxCode = BoxCode::new(*b"jbrd");
    /// JPEG 2000 Signature
    ///
    /// FourCC: `jP  `
    ///
    /// Specification: _JPEG2000_
    pub const JP: BoxCode = BoxCode::new(*b"jP  ");
    /// JPEG 2000 contiguous codestream
    ///
    /// FourCC: `jp2c`
    ///
    /// Specification: _JPEG2000_
    pub const JP2C: BoxCode = BoxCode::new(*b"jp2c");
    /// Header
    ///
    /// FourCC: `jp2h`
    ///
    /// Specification: _JPEG2000_
    pub const JP2H: BoxCode = BoxCode::new(*b"jp2h");
    /// intellectual property information
    ///
    /// FourCC: `jp2i`
    ///
    /// Specification: _JPEG2000_
    pub const JP2I: BoxCode = BoxCode::new(*b"jp2i");
    /// JPEG Universal Metadata Box Format (JUMBF)
    ///
    /// FourCC: `jumb`
    ///
    /// Specification: _JPEG Systems_
    pub const JUMB: BoxCode = BoxCode::new(*b"jumb");
    /// JPEG XL Signature
    ///
    /// FourCC: `JXL `
    ///
    /// Specification: _JPEG XL_
    pub const JXL: BoxCode = BoxCode::new(*b"JXL ");
    /// JPEG XL Level
    ///
    /// FourCC: `jxll`
    ///
    /// Specification: _JPEG XL_
    pub const JXLL: BoxCode = BoxCode::new(*b"jxll");
    /// JPEG XL Codestream
    ///
    /// FourCC: `jxlc`
    ///
    /// Specification: _JPEG XL_
    pub const JXLC: BoxCode = BoxCode::new(*b"jxlc");
    /// JPEG XL Frame Index
    ///
    /// FourCC: `jxli`
    ///
    /// Specification: _JPEG XL_
    pub const JXLI: BoxCode = BoxCode::new(*b"jxli");
    /// JPEG XL Partial Codestream
    ///
    /// FourCC: `jxlp`
    ///
    /// Specification: _JPEG XL_
    pub const JXLP: BoxCode = BoxCode::new(*b"jxlp");
    /// JPEG XS Video Information
    ///
    /// FourCC: `jpvi`
    ///
    /// Specification: _JPXS_
    pub const JPVI: BoxCode = BoxCode::new(*b"jpvi");
    /// JPEG XS Video Support
    ///
    /// FourCC: `jpvs`
    ///
    /// Specification: _JPXS_
    pub const JPVS: BoxCode = BoxCode::new(*b"jpvs");
    /// JPEG XS Video Transport Parameter
    ///
    /// FourCC: `jptp`
    ///
    /// Specification: _JPXS_
    pub const JPTP: BoxCode = BoxCode::new(*b"jptp");
    /// JPEG XS Signature
    ///
    /// FourCC: `JXS `
    ///
    /// Specification: _JPXS_
    pub const JXS: BoxCode = BoxCode::new(*b"JXS ");
    /// JPEG XS Profile and Level
    ///
    /// FourCC: `jxpl`
    ///
    /// Specification: _JPXS_
    pub const JXPL: BoxCode = BoxCode::new(*b"jxpl");
    /// Reserved
    ///
    /// FourCC: `kmat`
    ///
    /// Specification: _ISO_
    pub const KMAT: BoxCode = BoxCode::new(*b"kmat");
    /// Leval assignment
    ///
    /// FourCC: `leva`
    ///
    /// Specification: _ISO_
    pub const LEVA: BoxCode = BoxCode::new(*b"leva");
    /// Reserved
    ///
    /// FourCC: `load`
    ///
    /// Specification: _ISO_
    pub const LOAD: BoxCode = BoxCode::new(*b"load");
    /// Looping behavior
    ///
    /// FourCC: `loop`
    ///
    /// Specification: _WhatsApp_
    pub const LOOP: BoxCode = BoxCode::new(*b"loop");
    /// OMA DRM Lyrics URI
    ///
    /// FourCC: `lrcu`
    ///
    /// Specification: _OMA DRM 2.1_
    pub const LRCU: BoxCode = BoxCode::new(*b"lrcu");
    /// reserved for MPEG7Stream header
    ///
    /// FourCC: `m7hd`
    ///
    /// Specification: _MP4v2_
    pub const M7HD: BoxCode = BoxCode::new(*b"m7hd");
    /// Reserved
    ///
    /// FourCC: `matt`
    ///
    /// Specification: _ISO_
    pub const MATT: BoxCode = BoxCode::new(*b"matt");
    /// MD5IntegrityBox
    ///
    /// FourCC: `md5i`
    ///
    /// Specification: _HEIF_
    pub const MD5I: BoxCode = BoxCode::new(*b"md5i");
    /// media data container
    ///
    /// FourCC: `mdat`
    ///
    /// Specification: _ISO_
    pub const MDAT: BoxCode = BoxCode::new(*b"mdat");
    /// media header, overall information about the media
    ///
    /// FourCC: `mdhd`
    ///
    /// Specification: _ISO_
    pub const MDHD: BoxCode = BoxCode::new(*b"mdhd");
    /// container for the media information in a track
    ///
    /// FourCC: `mdia`
    ///
    /// Specification: _ISO_
    pub const MDIA: BoxCode = BoxCode::new(*b"mdia");
    /// Mutable DRM information
    ///
    /// FourCC: `mdri`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const MDRI: BoxCode = BoxCode::new(*b"mdri");
    /// additional metadata container
    ///
    /// FourCC: `meco`
    ///
    /// Specification: _ISO_
    pub const MECO: BoxCode = BoxCode::new(*b"meco");
    /// movie extends header box
    ///
    /// FourCC: `mehd`
    ///
    /// Specification: _ISO_
    pub const MEHD: BoxCode = BoxCode::new(*b"mehd");
    /// media offset
    ///
    /// FourCC: `meof`
    ///
    /// Specification: _OMAF_
    pub const MEOF: BoxCode = BoxCode::new(*b"meof");
    /// metabox relation
    ///
    /// FourCC: `mere`
    ///
    /// Specification: _ISO_
    pub const MERE: BoxCode = BoxCode::new(*b"mere");
    /// mesh box
    ///
    /// FourCC: `mesh`
    ///
    /// Specification: _OMAF_
    pub const MESH: BoxCode = BoxCode::new(*b"mesh");
    /// Metadata container
    ///
    /// FourCC: `meta`
    ///
    /// Specification: _ISO_
    pub const META: BoxCode = BoxCode::new(*b"meta");
    /// movie fragment header
    ///
    /// FourCC: `mfhd`
    ///
    /// Specification: _ISO_
    pub const MFHD: BoxCode = BoxCode::new(*b"mfhd");
    /// Movie fragment random access
    ///
    /// FourCC: `mfra`
    ///
    /// Specification: _ISO_
    pub const MFRA: BoxCode = BoxCode::new(*b"mfra");
    /// Movie fragment random access offset
    ///
    /// FourCC: `mfro`
    ///
    /// Specification: _ISO_
    pub const MFRO: BoxCode = BoxCode::new(*b"mfro");
    /// media information container
    ///
    /// FourCC: `minf`
    ///
    /// Specification: _ISO_
    pub const MINF: BoxCode = BoxCode::new(*b"minf");
    /// reserved for MPEG-J Stream header
    ///
    /// FourCC: `mjhd`
    ///
    /// Specification: _MP4v2_
    pub const MJHD: BoxCode = BoxCode::new(*b"mjhd");
    /// multimap video
    ///
    /// FourCC: `mmvi`
    ///
    /// Specification: _V3C-SYS_
    pub const MMVI: BoxCode = BoxCode::new(*b"mmvi");
    /// movie fragment
    ///
    /// FourCC: `moof`
    ///
    /// Specification: _ISO_
    pub const MOOF: BoxCode = BoxCode::new(*b"moof");
    /// container for all the meta-data
    ///
    /// FourCC: `moov`
    ///
    /// Specification: _ISO_
    pub const MOOV: BoxCode = BoxCode::new(*b"moov");
    /// mesh omnidirectional video
    ///
    /// FourCC: `movd`
    ///
    /// Specification: _OMAF_
    pub const MOVD: BoxCode = BoxCode::new(*b"movd");
    /// MVC sub track view box
    ///
    /// FourCC: `mstv`
    ///
    /// Specification: _NALu Video_
    pub const MSTV: BoxCode = BoxCode::new(*b"mstv");
    /// Multiview group
    ///
    /// FourCC: `mvcg`
    ///
    /// Specification: _NALu Video_
    pub const MVCG: BoxCode = BoxCode::new(*b"mvcg");
    /// Multiview Information
    ///
    /// FourCC: `mvci`
    ///
    /// Specification: _NALu Video_
    pub const MVCI: BoxCode = BoxCode::new(*b"mvci");
    /// MVDDepthResolutionBox
    ///
    /// FourCC: `3dpr`
    ///
    /// Specification: _NALu Video_
    pub const THREE_DPR: BoxCode = BoxCode::new(*b"3dpr");
    /// movie extends box
    ///
    /// FourCC: `mvex`
    ///
    /// Specification: _ISO_
    pub const MVEX: BoxCode = BoxCode::new(*b"mvex");
    /// movie header, overall declarations
    ///
    /// FourCC: `mvhd`
    ///
    /// Specification: _ISO_
    pub const MVHD: BoxCode = BoxCode::new(*b"mvhd");
    /// Multiview Relation Attribute
    ///
    /// FourCC: `mvra`
    ///
    /// Specification: _NALu Video_
    pub const MVRA: BoxCode = BoxCode::new(*b"mvra");
    /// Null media header, overall information (some tracks only)
    ///
    /// FourCC: `nmhd`
    ///
    /// Specification: _ISO_
    pub const NMHD: BoxCode = BoxCode::new(*b"nmhd");
    /// reserved for ObjectContentInfoStream header
    ///
    /// FourCC: `ochd`
    ///
    /// Specification: _MP4v2_
    pub const OCHD: BoxCode = BoxCode::new(*b"ochd");
    /// OMA DRM Access Unit Format
    ///
    /// FourCC: `odaf`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const ODAF: BoxCode = BoxCode::new(*b"odaf");
    /// OMA DRM Content Object
    ///
    /// FourCC: `odda`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const ODDA: BoxCode = BoxCode::new(*b"odda");
    /// reserved for ObjectDescriptorStream header
    ///
    /// FourCC: `odhd`
    ///
    /// Specification: _MP4v2_
    pub const ODHD: BoxCode = BoxCode::new(*b"odhd");
    /// OMA DRM Discrete Media Headers
    ///
    /// FourCC: `odhe`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const ODHE: BoxCode = BoxCode::new(*b"odhe");
    /// OMA DRM Rights Object
    ///
    /// FourCC: `odrb`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const ODRB: BoxCode = BoxCode::new(*b"odrb");
    /// OMA DRM Container
    ///
    /// FourCC: `odrm`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const ODRM: BoxCode = BoxCode::new(*b"odrm");
    /// OMA DRM Transaction Tracking
    ///
    /// FourCC: `odtt`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const ODTT: BoxCode = BoxCode::new(*b"odtt");
    /// OMA DRM Common headers
    ///
    /// FourCC: `ohdr`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const OHDR: BoxCode = BoxCode::new(*b"ohdr");
    /// OMAF timed text configuration
    ///
    /// FourCC: `otcf`
    ///
    /// Specification: _OMAF_
    pub const OTCF: BoxCode = BoxCode::new(*b"otcf");
    /// Original file type
    ///
    /// FourCC: `otyp`
    ///
    /// Specification: _ISO_
    pub const OTYP: BoxCode = BoxCode::new(*b"otyp");
    /// overlay configuration
    ///
    /// FourCC: `ovly`
    ///
    /// Specification: _OMAF_
    pub const OVLY: BoxCode = BoxCode::new(*b"ovly");
    /// sample padding bits
    ///
    /// FourCC: `padb`
    ///
    /// Specification: _ISO_
    pub const PADB: BoxCode = BoxCode::new(*b"padb");
    /// Partition Entry
    ///
    /// FourCC: `paen`
    ///
    /// Specification: _ISO_
    pub const PAEN: BoxCode = BoxCode::new(*b"paen");
    /// palette which maps a single component in index space to a multiple- component image
    ///
    /// FourCC: `pclr`
    ///
    /// Specification: _JPEG2000_
    pub const PCLR: BoxCode = BoxCode::new(*b"pclr");
    /// Partial Data
    ///
    /// FourCC: `pdat`
    ///
    /// Specification: _ISO-Partial_
    pub const PDAT: BoxCode = BoxCode::new(*b"pdat");
    /// Progressive download information
    ///
    /// FourCC: `pdin`
    ///
    /// Specification: _ISO_
    pub const PDIN: BoxCode = BoxCode::new(*b"pdin");
    /// Partial File Header
    ///
    /// FourCC: `pfhd`
    ///
    /// Specification: _ISO-Partial_
    pub const PFHD: BoxCode = BoxCode::new(*b"pfhd");
    /// Partial File
    ///
    /// FourCC: `pfil`
    ///
    /// Specification: _ISO-Partial_
    pub const PFIL: BoxCode = BoxCode::new(*b"pfil");
    /// primary item reference
    ///
    /// FourCC: `pitm`
    ///
    /// Specification: _ISO_
    pub const PITM: BoxCode = BoxCode::new(*b"pitm");
    /// Partial Segment Location
    ///
    /// FourCC: `ploc`
    ///
    /// Specification: _ISO-Partial_
    pub const PLOC: BoxCode = BoxCode::new(*b"ploc");
    /// Reserved
    ///
    /// FourCC: `pnot`
    ///
    /// Specification: _ISO_
    pub const PNOT: BoxCode = BoxCode::new(*b"pnot");
    /// projected omnidirectional video
    ///
    /// FourCC: `povd`
    ///
    /// Specification: _OMAF_
    pub const POVD: BoxCode = BoxCode::new(*b"povd");
    /// projection format
    ///
    /// FourCC: `prfr`
    ///
    /// Specification: _OMAF_
    pub const PRFR: BoxCode = BoxCode::new(*b"prfr");
    /// Producer reference time
    ///
    /// FourCC: `prft`
    ///
    /// Specification: _ISO_
    pub const PRFT: BoxCode = BoxCode::new(*b"prft");
    /// Partial Segment
    ///
    /// FourCC: `pseg`
    ///
    /// Specification: _ISO-Partial_
    pub const PSEG: BoxCode = BoxCode::new(*b"pseg");
    /// Partial Segment Header
    ///
    /// FourCC: `pshd`
    ///
    /// Specification: _ISO-Partial_
    pub const PSHD: BoxCode = BoxCode::new(*b"pshd");
    /// Protection system specific header
    ///
    /// FourCC: `pssh`
    ///
    /// Specification: _ISO Common Encryption_
    pub const PSSH: BoxCode = BoxCode::new(*b"pssh");
    /// Partial Top Level Entry
    ///
    /// FourCC: `ptle`
    ///
    /// Specification: _ISO-Partial_
    pub const PTLE: BoxCode = BoxCode::new(*b"ptle");
    /// grid resolution
    ///
    /// FourCC: `res `
    ///
    /// Specification: _JPEG2000_
    pub const RES: BoxCode = BoxCode::new(*b"res ");
    /// grid resolution at which the image was captured
    ///
    /// FourCC: `resc`
    ///
    /// Specification: _JPEG2000_
    pub const RESC: BoxCode = BoxCode::new(*b"resc");
    /// default grid resolution at which the image should be displayed
    ///
    /// FourCC: `resd`
    ///
    /// Specification: _JPEG2000_
    pub const RESD: BoxCode = BoxCode::new(*b"resd");
    /// restricted scheme information box
    ///
    /// FourCC: `rinf`
    ///
    /// Specification: _ISO_
    pub const RINF: BoxCode = BoxCode::new(*b"rinf");
    /// rotation box
    ///
    /// FourCC: `rotn`
    ///
    /// Specification: _OMAF_
    pub const ROTN: BoxCode = BoxCode::new(*b"rotn");
    /// sphere region configuration
    ///
    /// FourCC: `rosc`
    ///
    /// Specification: _OMAF_
    pub const ROSC: BoxCode = BoxCode::new(*b"rosc");
    /// recommended viewport information
    ///
    /// FourCC: `rvif`
    ///
    /// Specification: _OMAF_
    pub const RVIF: BoxCode = BoxCode::new(*b"rvif");
    /// region-wise packing
    ///
    /// FourCC: `rwpk`
    ///
    /// Specification: _OMAF_
    pub const RWPK: BoxCode = BoxCode::new(*b"rwpk");
    /// Sample auxiliary information offsets
    ///
    /// FourCC: `saio`
    ///
    /// Specification: _ISO_
    pub const SAIO: BoxCode = BoxCode::new(*b"saio");
    /// Sample auxiliary information sizes
    ///
    /// FourCC: `saiz`
    ///
    /// Specification: _ISO_
    pub const SAIZ: BoxCode = BoxCode::new(*b"saiz");
    /// Sample to Group box
    ///
    /// FourCC: `sbgp`
    ///
    /// Specification: _ISO_
    pub const SBGP: BoxCode = BoxCode::new(*b"sbgp");
    /// scheme information box
    ///
    /// FourCC: `schi`
    ///
    /// Specification: _ISO_
    pub const SCHI: BoxCode = BoxCode::new(*b"schi");
    /// scheme type box
    ///
    /// FourCC: `schm`
    ///
    /// Specification: _ISO_
    pub const SCHM: BoxCode = BoxCode::new(*b"schm");
    /// Sample dependency
    ///
    /// FourCC: `sdep`
    ///
    /// Specification: _NALu Video_
    pub const SDEP: BoxCode = BoxCode::new(*b"sdep");
    /// reserved for SceneDescriptionStream header
    ///
    /// FourCC: `sdhd`
    ///
    /// Specification: _MP4v2_
    pub const SDHD: BoxCode = BoxCode::new(*b"sdhd");
    /// Independent and Disposable Samples Box
    ///
    /// FourCC: `sdtp`
    ///
    /// Specification: _ISO_
    pub const SDTP: BoxCode = BoxCode::new(*b"sdtp");
    /// SD Profile Box
    ///
    /// FourCC: `sdvp`
    ///
    /// Specification: _SDV_
    pub const SDVP: BoxCode = BoxCode::new(*b"sdvp");
    /// file delivery session group
    ///
    /// FourCC: `segr`
    ///
    /// Specification: _ISO_
    pub const SEGR: BoxCode = BoxCode::new(*b"segr");
    /// SEI information box
    ///
    /// FourCC: `seii`
    ///
    /// Specification: _NALu Video_
    pub const SEII: BoxCode = BoxCode::new(*b"seii");
    /// Sample specific encryption data
    ///
    /// FourCC: `senc`
    ///
    /// Specification: _ISO Common Encryption_
    pub const SENC: BoxCode = BoxCode::new(*b"senc");
    /// Sample group definition box
    ///
    /// FourCC: `sgpd`
    ///
    /// Specification: _ISO_
    pub const SGPD: BoxCode = BoxCode::new(*b"sgpd");
    /// Segment Index Box
    ///
    /// FourCC: `sidx`
    ///
    /// Specification: _ISO_
    pub const SIDX: BoxCode = BoxCode::new(*b"sidx");
    /// protection scheme information box
    ///
    /// FourCC: `sinf`
    ///
    /// Specification: _ISO_
    pub const SINF: BoxCode = BoxCode::new(*b"sinf");
    /// free space
    ///
    /// FourCC: `skip`
    ///
    /// Specification: _ISO_
    pub const SKIP: BoxCode = BoxCode::new(*b"skip");
    /// sound media header, overall information (sound track only)
    ///
    /// FourCC: `smhd`
    ///
    /// Specification: _ISO_
    pub const SMHD: BoxCode = BoxCode::new(*b"smhd");
    /// sub-picture region
    ///
    /// FourCC: `sprg`
    ///
    /// Specification: _OMAF_
    pub const SPRG: BoxCode = BoxCode::new(*b"sprg");
    /// System Renewability Message
    ///
    /// FourCC: `srmb`
    ///
    /// Specification: _DVB_
    pub const SRMB: BoxCode = BoxCode::new(*b"srmb");
    /// System Renewability Message container
    ///
    /// FourCC: `srmc`
    ///
    /// Specification: _DVB_
    pub const SRMC: BoxCode = BoxCode::new(*b"srmc");
    /// STRP Process
    ///
    /// FourCC: `srpp`
    ///
    /// Specification: _ISO_
    pub const SRPP: BoxCode = BoxCode::new(*b"srpp");
    /// sphere region quality ranking
    ///
    /// FourCC: `srqr`
    ///
    /// Specification: _OMAF_
    pub const SRQR: BoxCode = BoxCode::new(*b"srqr");
    /// Sub-sample index
    ///
    /// FourCC: `ssix`
    ///
    /// Specification: _ISO_
    pub const SSIX: BoxCode = BoxCode::new(*b"ssix");
    /// SVC sub track layer box
    ///
    /// FourCC: `sstl`
    ///
    /// Specification: _NALu Video_
    pub const SSTL: BoxCode = BoxCode::new(*b"sstl");
    /// sample table box, container for the time/space map
    ///
    /// FourCC: `stbl`
    ///
    /// Specification: _ISO_
    pub const STBL: BoxCode = BoxCode::new(*b"stbl");
    /// chunk offset, partial data-offset information
    ///
    /// FourCC: `stco`
    ///
    /// Specification: _ISO_
    pub const STCO: BoxCode = BoxCode::new(*b"stco");
    /// sample degradation priority
    ///
    /// FourCC: `stdp`
    ///
    /// Specification: _ISO_
    pub const STDP: BoxCode = BoxCode::new(*b"stdp");
    /// Subtitle Media Header Box
    ///
    /// FourCC: `sthd`
    ///
    /// Specification: _ISO_
    pub const STHD: BoxCode = BoxCode::new(*b"sthd");
    /// MVC sub track multiview group box
    ///
    /// FourCC: `stmg`
    ///
    /// Specification: _NALu Video_
    pub const STMG: BoxCode = BoxCode::new(*b"stmg");
    /// Sub-track definition
    ///
    /// FourCC: `strd`
    ///
    /// Specification: _ISO_
    pub const STRD: BoxCode = BoxCode::new(*b"strd");
    /// Sub-track information
    ///
    /// FourCC: `stri`
    ///
    /// Specification: _ISO_
    pub const STRI: BoxCode = BoxCode::new(*b"stri");
    /// sample-to-chunk, partial data-offset information
    ///
    /// FourCC: `stsc`
    ///
    /// Specification: _ISO_
    pub const STSC: BoxCode = BoxCode::new(*b"stsc");
    /// sample descriptions (codec types, initialization etc.)
    ///
    /// FourCC: `stsd`
    ///
    /// Specification: _ISO_
    pub const STSD: BoxCode = BoxCode::new(*b"stsd");
    /// Sub-track sample grouping
    ///
    /// FourCC: `stsg`
    ///
    /// Specification: _ISO_
    pub const STSG: BoxCode = BoxCode::new(*b"stsg");
    /// shadow sync sample table
    ///
    /// FourCC: `stsh`
    ///
    /// Specification: _ISO_
    pub const STSH: BoxCode = BoxCode::new(*b"stsh");
    /// sync sample table (random access points)
    ///
    /// FourCC: `stss`
    ///
    /// Specification: _ISO_
    pub const STSS: BoxCode = BoxCode::new(*b"stss");
    /// sample sizes (framing)
    ///
    /// FourCC: `stsz`
    ///
    /// Specification: _ISO_
    pub const STSZ: BoxCode = BoxCode::new(*b"stsz");
    /// Sub track tier box
    ///
    /// FourCC: `stti`
    ///
    /// Specification: _NALu Video_
    pub const STTI: BoxCode = BoxCode::new(*b"stti");
    /// (decoding) time-to-sample
    ///
    /// FourCC: `stts`
    ///
    /// Specification: _ISO_
    pub const STTS: BoxCode = BoxCode::new(*b"stts");
    /// Segment Type Box
    ///
    /// FourCC: `styp`
    ///
    /// Specification: _ISO_
    pub const STYP: BoxCode = BoxCode::new(*b"styp");
    /// compact sample sizes (framing)
    ///
    /// FourCC: `stz2`
    ///
    /// Specification: _ISO_
    pub const STZ2: BoxCode = BoxCode::new(*b"stz2");
    /// Sub-sample information
    ///
    /// FourCC: `subs`
    ///
    /// Specification: _ISO_
    pub const SUBS: BoxCode = BoxCode::new(*b"subs");
    /// signer identity information
    ///
    /// FourCC: `suep`
    ///
    /// Specification: _ONVIF_
    pub const SUEP: BoxCode = BoxCode::new(*b"suep");
    /// supplemental surveillance meta information
    ///
    /// FourCC: `sumi`
    ///
    /// Specification: _MPEG-VSAF_
    pub const SUMI: BoxCode = BoxCode::new(*b"sumi");
    /// Source URL
    ///
    /// FourCC: `surl`
    ///
    /// Specification: _ISO-Partial_
    pub const SURL: BoxCode = BoxCode::new(*b"surl");
    /// Multiview Group Relation
    ///
    /// FourCC: `swtc`
    ///
    /// Specification: _NALu Video_
    pub const SWTC: BoxCode = BoxCode::new(*b"swtc");
    /// Track Encryption
    ///
    /// FourCC: `tenc`
    ///
    /// Specification: _ISO Common Encryption_
    pub const TENC: BoxCode = BoxCode::new(*b"tenc");
    /// Track fragment adjustment box
    ///
    /// FourCC: `tfad`
    ///
    /// Specification: _3GPP_
    pub const TFAD: BoxCode = BoxCode::new(*b"tfad");
    /// Track fragment decode time
    ///
    /// FourCC: `tfdt`
    ///
    /// Specification: _ISO_
    pub const TFDT: BoxCode = BoxCode::new(*b"tfdt");
    /// Track fragment header
    ///
    /// FourCC: `tfhd`
    ///
    /// Specification: _ISO_
    pub const TFHD: BoxCode = BoxCode::new(*b"tfhd");
    /// Track fragment media adjustment box
    ///
    /// FourCC: `tfma`
    ///
    /// Specification: _3GPP_
    pub const TFMA: BoxCode = BoxCode::new(*b"tfma");
    /// Track fragment radom access
    ///
    /// FourCC: `tfra`
    ///
    /// Specification: _ISO_
    pub const TFRA: BoxCode = BoxCode::new(*b"tfra");
    /// Tier Bit rate
    ///
    /// FourCC: `tibr`
    ///
    /// Specification: _NALu Video_
    pub const TIBR: BoxCode = BoxCode::new(*b"tibr");
    /// Tier Information
    ///
    /// FourCC: `tiri`
    ///
    /// Specification: _NALu Video_
    pub const TIRI: BoxCode = BoxCode::new(*b"tiri");
    /// Track header, overall information about the track
    ///
    /// FourCC: `tkhd`
    ///
    /// Specification: _ISO_
    pub const TKHD: BoxCode = BoxCode::new(*b"tkhd");
    /// Track fragment
    ///
    /// FourCC: `traf`
    ///
    /// Specification: _ISO_
    pub const TRAF: BoxCode = BoxCode::new(*b"traf");
    /// container for an individual track or stream
    ///
    /// FourCC: `trak`
    ///
    /// Specification: _ISO_
    pub const TRAK: BoxCode = BoxCode::new(*b"trak");
    /// track reference container
    ///
    /// FourCC: `tref`
    ///
    /// Specification: _ISO_
    pub const TREF: BoxCode = BoxCode::new(*b"tref");
    /// track extension properties
    ///
    /// FourCC: `trep`
    ///
    /// Specification: _ISO_
    pub const TREP: BoxCode = BoxCode::new(*b"trep");
    /// track extends defaults
    ///
    /// FourCC: `trex`
    ///
    /// Specification: _ISO_
    pub const TREX: BoxCode = BoxCode::new(*b"trex");
    /// Track grouping information
    ///
    /// FourCC: `trgr`
    ///
    /// Specification: _ISO_
    pub const TRGR: BoxCode = BoxCode::new(*b"trgr");
    /// Facilitates random access and trick play modes
    ///
    /// FourCC: `trik`
    ///
    /// Specification: _DECE_
    pub const TRIK: BoxCode = BoxCode::new(*b"trik");
    /// track fragment run
    ///
    /// FourCC: `trun`
    ///
    /// Specification: _ISO_
    pub const TRUN: BoxCode = BoxCode::new(*b"trun");
    /// TileSubTrackGroupBox
    ///
    /// FourCC: `tstb`
    ///
    /// Specification: _NALu Video_
    pub const TSTB: BoxCode = BoxCode::new(*b"tstb");
    /// track type and compatibility
    ///
    /// FourCC: `ttyp`
    ///
    /// Specification: _ISO_
    pub const TTYP: BoxCode = BoxCode::new(*b"ttyp");
    /// type and-combination
    ///
    /// FourCC: `tyco`
    ///
    /// Specification: _ISO_
    pub const TYCO: BoxCode = BoxCode::new(*b"tyco");
    /// user-data
    ///
    /// FourCC: `udta`
    ///
    /// Specification: _ISO_
    pub const UDTA: BoxCode = BoxCode::new(*b"udta");
    /// a tool by which a vendor may provide access to additional information associated with a UUID
    ///
    /// FourCC: `uinf`
    ///
    /// Specification: _JPEG2000_
    pub const UINF: BoxCode = BoxCode::new(*b"uinf");
    /// Unique Identifier Technology Solution
    ///
    /// FourCC: `UITS`
    ///
    /// Specification: _Universal Music Group_
    pub const UITS: BoxCode = BoxCode::new(*b"UITS");
    /// a list of UUID’s
    ///
    /// FourCC: `ulst`
    ///
    /// Specification: _JPEG2000_
    pub const ULST: BoxCode = BoxCode::new(*b"ulst");
    /// a URL
    ///
    /// FourCC: `url `
    ///
    /// Specification: _JPEG2000_
    pub const URL: BoxCode = BoxCode::new(*b"url ");
    /// user-extension box
    ///
    /// FourCC: `uuid`
    ///
    /// Specification: _ISO_
    pub const UUID: BoxCode = BoxCode::new(*b"uuid");
    /// V3C spatial region collection
    ///
    /// FourCC: `v3sc`
    ///
    /// Specification: _V3C-SYS_
    pub const V3SC: BoxCode = BoxCode::new(*b"v3sc");
    /// video media header, overall information (video track only)
    ///
    /// FourCC: `vmhd`
    ///
    /// Specification: _ISO_
    pub const VMHD: BoxCode = BoxCode::new(*b"vmhd");
    /// Volumetric media bounding box
    ///
    /// FourCC: `vpbb`
    ///
    /// Specification: _V3C-SYS_
    pub const VPBB: BoxCode = BoxCode::new(*b"vpbb");
    /// viewing space
    ///
    /// FourCC: `vssn`
    ///
    /// Specification: _OMAF_
    pub const VSSN: BoxCode = BoxCode::new(*b"vssn");
    /// V3C unit header
    ///
    /// FourCC: `vunt`
    ///
    /// Specification: _V3C-SYS_
    pub const VUNT: BoxCode = BoxCode::new(*b"vunt");
    /// volumetric visual media header
    ///
    /// FourCC: `vvhd`
    ///
    /// Specification: _ISO_
    pub const VVHD: BoxCode = BoxCode::new(*b"vvhd");
    /// Multiview Scene Information
    ///
    /// FourCC: `vwdi`
    ///
    /// Specification: _NALu Video_
    pub const VWDI: BoxCode = BoxCode::new(*b"vwdi");
    /// segment position in the watermark pattern
    ///
    /// FourCC: `wmpi`
    ///
    /// Specification: _DASH-IF watermarking_
    pub const WMPI: BoxCode = BoxCode::new(*b"wmpi");
    /// XML container
    ///
    /// FourCC: `xml `
    ///
    /// Specification: _ISO_
    pub const XML: BoxCode = BoxCode::new(*b"xml ");
    /// Compressed movie fragment
    ///
    /// FourCC: `!mof`
    ///
    /// Specification: _ISO_
    pub const COMPRESSED_MOF: BoxCode = BoxCode::new(*b"!mof");
    /// Compressed movie
    ///
    /// FourCC: `!mov`
    ///
    /// Specification: _ISO_
    pub const COMPRESSED_MOV: BoxCode = BoxCode::new(*b"!mov");
    /// Compressed segment index
    ///
    /// FourCC: `!six`
    ///
    /// Specification: _ISO_
    pub const COMPRESSED_SIX: BoxCode = BoxCode::new(*b"!six");
    /// Compressed subsegment index
    ///
    /// FourCC: `!ssx`
    ///
    /// Specification: _ISO_
    pub const COMPRESSED_SSX: BoxCode = BoxCode::new(*b"!ssx");
    /// SVC region of interest box
    ///
    /// FourCC: `iroi`
    ///
    /// Specification: _NALu Video_
    pub const IROI: BoxCode = BoxCode::new(*b"iroi");
    /// Tier dependency box
    ///
    /// FourCC: `ldep`
    ///
    /// Specification: _NALu Video_
    pub const LDEP: BoxCode = BoxCode::new(*b"ldep");
    /// SVC rect region box
    ///
    /// FourCC: `rrgn`
    ///
    /// Specification: _NALu Video_
    pub const RRGN: BoxCode = BoxCode::new(*b"rrgn");
    /// SVC dependency range
    ///
    /// FourCC: `svdr`
    ///
    /// Specification: _NALu Video_
    pub const SVDR: BoxCode = BoxCode::new(*b"svdr");
    /// Initial parameter sets box for tiers
    ///
    /// FourCC: `svip`
    ///
    /// Specification: _NALu Video_
    pub const SVIP: BoxCode = BoxCode::new(*b"svip");
    /// Priority range
    ///
    /// FourCC: `svpr`
    ///
    /// Specification: _NALu Video_
    pub const SVPR: BoxCode = BoxCode::new(*b"svpr");
    /// SVC lightweight transcoding
    ///
    /// FourCC: `tran`
    ///
    /// Specification: _NALu Video_
    pub const TRAN: BoxCode = BoxCode::new(*b"tran");
    /// View priority Box
    ///
    /// FourCC: `vipr`
    ///
    /// Specification: _NALu Video_
    pub const VIPR: BoxCode = BoxCode::new(*b"vipr");
    /// SDP information
    ///
    /// FourCC: `sdp `
    ///
    /// Specification: _ISO_
    pub const SDP: BoxCode = BoxCode::new(*b"sdp ");
    /// Stereo Video Box
    ///
    /// FourCC: `stvi`
    ///
    /// Specification: _ISO_
    pub const STVI: BoxCode = BoxCode::new(*b"stvi");
    /// Sample packing information box
    ///
    /// FourCC: `spki`
    ///
    /// Specification: _ISO_
    pub const SPKI: BoxCode = BoxCode::new(*b"spki");
    /// audio element description
    ///
    /// FourCC: `aedb`
    ///
    /// Specification: _ISO_
    pub const AEDB: BoxCode = BoxCode::new(*b"aedb");
    /// audio element box
    ///
    /// FourCC: `aelm`
    ///
    /// Specification: _ISO_
    pub const AELM: BoxCode = BoxCode::new(*b"aelm");
    /// audio element positioning interactivity polar box
    ///
    /// FourCC: `aepp`
    ///
    /// Specification: _ISO_
    pub const AEPP: BoxCode = BoxCode::new(*b"aepp");
    /// audio element prominence interactivity box
    ///
    /// FourCC: `aepr`
    ///
    /// Specification: _ISO_
    pub const AEPR: BoxCode = BoxCode::new(*b"aepr");
    /// audio element selection box
    ///
    /// FourCC: `aesb`
    ///
    /// Specification: _ISO_
    pub const AESB: BoxCode = BoxCode::new(*b"aesb");
    /// audio element selection description box
    ///
    /// FourCC: `aesd`
    ///
    /// Specification: _ISO_
    pub const AESD: BoxCode = BoxCode::new(*b"aesd");
    /// audio rendering indication
    ///
    /// FourCC: `ardi`
    ///
    /// Specification: _ISO_
    pub const ARDI: BoxCode = BoxCode::new(*b"ardi");
    /// Label box
    ///
    /// FourCC: `labl`
    ///
    /// Specification: _ISO_
    pub const LABL: BoxCode = BoxCode::new(*b"labl");
    /// Extended Language Tag
    ///
    /// FourCC: `elng`
    ///
    /// Specification: _ISO_
    pub const ELNG: BoxCode = BoxCode::new(*b"elng");
    /// Redundant Sample Original Timing
    ///
    /// FourCC: `rsot`
    ///
    /// Specification: _ISO_
    pub const RSOT: BoxCode = BoxCode::new(*b"rsot");
    /// Sub-sample Reference Table Box
    ///
    /// FourCC: `ssrt`
    ///
    /// Specification: _ISO_
    pub const SSRT: BoxCode = BoxCode::new(*b"ssrt");
    /// The MinimizedImageBox provides a more compact representation of the MetaBox for a subset of use cases
    ///
    /// FourCC: `mini`
    ///
    /// Specification: _HEIF_
    pub const MINI: BoxCode = BoxCode::new(*b"mini");
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
    pub const SYSTEMS_ISO_IEC_14496_1_BIFS_V_2_CONFIG: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x02,
    );
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
    pub const SIMPLE_AGGREGATION_FORMAT_STREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x0A,
    );
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
    pub const VISUAL_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x21,
    );
    /// Parameter Sets for ITU-T Recommendation H.264 | ISO/IEC 14496-10
    ///
    /// Type value: `0x22`
    ///
    /// Specification: _MPEG-4_
    pub const PARAMETER_SETS_FOR_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x22,
    );
    /// Visual ISO/IEC 23008-2 | ITU-T Recommendation H.265
    ///
    /// Type value: `0x23`
    ///
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_23008_2_ITU_T_RECOMMENDATION_H_265: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x23,
    );
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
    pub const VISUAL_ISO_IEC_13818_2_SIMPLE_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x60,
    );
    /// Visual ISO/IEC 13818-2 Main Profile
    ///
    /// Type value: `0x61`
    ///
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_MAIN_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x61,
    );
    /// Visual ISO/IEC 13818-2 SNR Profile
    ///
    /// Type value: `0x62`
    ///
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_SNR_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x62,
    );
    /// Visual ISO/IEC 13818-2 Spatial Profile
    ///
    /// Type value: `0x63`
    ///
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_SPATIAL_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x63,
    );
    /// Visual ISO/IEC 13818-2 High Profile
    ///
    /// Type value: `0x64`
    ///
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_HIGH_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x64,
    );
    /// Visual ISO/IEC 13818-2 422 Profile
    ///
    /// Type value: `0x65`
    ///
    /// Specification: _MPEG-4_
    pub const VISUAL_ISO_IEC_13818_2_422_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x65,
    );
    /// Audio ISO/IEC 13818-7 Main Profile
    ///
    /// Type value: `0x66`
    ///
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_13818_7_MAIN_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x66,
    );
    /// Audio ISO/IEC 13818-7 LowComplexity Profile
    ///
    /// Type value: `0x67`
    ///
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_13818_7_LOW_COMPLEXITY_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x67,
    );
    /// Audio ISO/IEC 13818-7 Scaleable Sampling Rate Profile
    ///
    /// Type value: `0x68`
    ///
    /// Specification: _MPEG-4_
    pub const AUDIO_ISO_IEC_13818_7_SCALEABLE_SAMPLING_RATE_PROFILE: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x68,
    );
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
    pub const PORTABLE_NETWORK_GRAPHICS: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0x6D,
    );
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
    pub const THREE_GPP_2_COMPACT_MULTIMEDIA_FORMAT: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0xA2,
    );
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
    pub const CORE_SUBSTREAM_EXTENSION_SUBSTREAM: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0xAA,
    );
    /// Extension Substream containing only XLL
    ///
    /// Type value: `0xAB`
    ///
    /// Specification: _DTS-HD_
    pub const EXTENSION_SUBSTREAM_CONTAINING_ONLY_XLL: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0xAB,
    );
    /// Extension Substream containing only LBR
    ///
    /// Type value: `0xAC`
    ///
    /// Specification: _DTS-HD_
    pub const EXTENSION_SUBSTREAM_CONTAINING_ONLY_LBR: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0xAC,
    );
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
    pub const DTS_UHD_PROFILE_3_OR_HIGHER: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0xB3,
    );
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
    pub const NO_OBJECT_TYPE_SPECIFIED: ObjectTypeIdentifier = ObjectTypeIdentifier(
        0xFF,
    );
}
impl fmt::Debug for ObjectTypeIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match *self {
            ObjectTypeIdentifier::FORBIDDEN => "FORBIDDEN",
            ObjectTypeIdentifier::SYSTEMS_ISO_IEC_14496_1 => "SYSTEMS_ISO_IEC_14496_1",
            ObjectTypeIdentifier::SYSTEMS_ISO_IEC_14496_1_BIFS_V_2_CONFIG => {
                "SYSTEMS_ISO_IEC_14496_1_BIFS_V_2_CONFIG"
            }
            ObjectTypeIdentifier::INTERACTION_STREAM => "INTERACTION_STREAM",
            ObjectTypeIdentifier::EXTENDED_BIFS => "EXTENDED_BIFS",
            ObjectTypeIdentifier::AFX_STREAM => "AFX_STREAM",
            ObjectTypeIdentifier::FONT_DATA_STREAM => "FONT_DATA_STREAM",
            ObjectTypeIdentifier::SYNTHETISED_TEXTURE => "SYNTHETISED_TEXTURE",
            ObjectTypeIdentifier::TEXT_STREAM => "TEXT_STREAM",
            ObjectTypeIdentifier::LA_SE_R_STREAM => "LA_SE_R_STREAM",
            ObjectTypeIdentifier::SIMPLE_AGGREGATION_FORMAT_STREAM => {
                "SIMPLE_AGGREGATION_FORMAT_STREAM"
            }
            ObjectTypeIdentifier::VISUAL_ISO_IEC_14496_2 => "VISUAL_ISO_IEC_14496_2",
            ObjectTypeIdentifier::VISUAL_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10 => {
                "VISUAL_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10"
            }
            ObjectTypeIdentifier::PARAMETER_SETS_FOR_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10 => {
                "PARAMETER_SETS_FOR_ITU_T_RECOMMENDATION_H_264_ISO_IEC_14496_10"
            }
            ObjectTypeIdentifier::VISUAL_ISO_IEC_23008_2_ITU_T_RECOMMENDATION_H_265 => {
                "VISUAL_ISO_IEC_23008_2_ITU_T_RECOMMENDATION_H_265"
            }
            ObjectTypeIdentifier::AUDIO_ISO_IEC_14496_3 => "AUDIO_ISO_IEC_14496_3",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_SIMPLE_PROFILE => {
                "VISUAL_ISO_IEC_13818_2_SIMPLE_PROFILE"
            }
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_MAIN_PROFILE => {
                "VISUAL_ISO_IEC_13818_2_MAIN_PROFILE"
            }
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_SNR_PROFILE => {
                "VISUAL_ISO_IEC_13818_2_SNR_PROFILE"
            }
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_SPATIAL_PROFILE => {
                "VISUAL_ISO_IEC_13818_2_SPATIAL_PROFILE"
            }
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_HIGH_PROFILE => {
                "VISUAL_ISO_IEC_13818_2_HIGH_PROFILE"
            }
            ObjectTypeIdentifier::VISUAL_ISO_IEC_13818_2_422_PROFILE => {
                "VISUAL_ISO_IEC_13818_2_422_PROFILE"
            }
            ObjectTypeIdentifier::AUDIO_ISO_IEC_13818_7_MAIN_PROFILE => {
                "AUDIO_ISO_IEC_13818_7_MAIN_PROFILE"
            }
            ObjectTypeIdentifier::AUDIO_ISO_IEC_13818_7_LOW_COMPLEXITY_PROFILE => {
                "AUDIO_ISO_IEC_13818_7_LOW_COMPLEXITY_PROFILE"
            }
            ObjectTypeIdentifier::AUDIO_ISO_IEC_13818_7_SCALEABLE_SAMPLING_RATE_PROFILE => {
                "AUDIO_ISO_IEC_13818_7_SCALEABLE_SAMPLING_RATE_PROFILE"
            }
            ObjectTypeIdentifier::AUDIO_ISO_IEC_13818_3 => "AUDIO_ISO_IEC_13818_3",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_11172_2 => "VISUAL_ISO_IEC_11172_2",
            ObjectTypeIdentifier::AUDIO_ISO_IEC_11172_3 => "AUDIO_ISO_IEC_11172_3",
            ObjectTypeIdentifier::VISUAL_ISO_IEC_10918_1 => "VISUAL_ISO_IEC_10918_1",
            ObjectTypeIdentifier::PORTABLE_NETWORK_GRAPHICS => {
                "PORTABLE_NETWORK_GRAPHICS"
            }
            ObjectTypeIdentifier::VISUAL_ISO_IEC_15444_1 => "VISUAL_ISO_IEC_15444_1",
            ObjectTypeIdentifier::EVRC_VOICE => "EVRC_VOICE",
            ObjectTypeIdentifier::SMV_VOICE => "SMV_VOICE",
            ObjectTypeIdentifier::THREE_GPP_2_COMPACT_MULTIMEDIA_FORMAT => {
                "THREE_GPP_2_COMPACT_MULTIMEDIA_FORMAT"
            }
            ObjectTypeIdentifier::SMPTE_VC_1_VIDEO => "SMPTE_VC_1_VIDEO",
            ObjectTypeIdentifier::DIRAC_VIDEO_CODER => "DIRAC_VIDEO_CODER",
            ObjectTypeIdentifier(0xA5) => "WITHDRAWN",
            ObjectTypeIdentifier(0xA6) => "WITHDRAWN",
            ObjectTypeIdentifier::DRA_AUDIO => "DRA_AUDIO",
            ObjectTypeIdentifier::ITU_G_719_AUDIO => "ITU_G_719_AUDIO",
            ObjectTypeIdentifier::CORE_SUBSTREAM => "CORE_SUBSTREAM",
            ObjectTypeIdentifier::CORE_SUBSTREAM_EXTENSION_SUBSTREAM => {
                "CORE_SUBSTREAM_EXTENSION_SUBSTREAM"
            }
            ObjectTypeIdentifier::EXTENSION_SUBSTREAM_CONTAINING_ONLY_XLL => {
                "EXTENSION_SUBSTREAM_CONTAINING_ONLY_XLL"
            }
            ObjectTypeIdentifier::EXTENSION_SUBSTREAM_CONTAINING_ONLY_LBR => {
                "EXTENSION_SUBSTREAM_CONTAINING_ONLY_LBR"
            }
            ObjectTypeIdentifier::OPUS_AUDIO => "OPUS_AUDIO",
            ObjectTypeIdentifier(0xAE) => "WITHDRAWN",
            ObjectTypeIdentifier::AURO_CX_3_D_AUDIO => "AURO_CX_3_D_AUDIO",
            ObjectTypeIdentifier::REAL_VIDEO_CODEC_11 => "REAL_VIDEO_CODEC_11",
            ObjectTypeIdentifier::VP_9_VIDEO => "VP_9_VIDEO",
            ObjectTypeIdentifier::DTS_UHD_PROFILE_2 => "DTS_UHD_PROFILE_2",
            ObjectTypeIdentifier::DTS_UHD_PROFILE_3_OR_HIGHER => {
                "DTS_UHD_PROFILE_3_OR_HIGHER"
            }
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
    pub const ONE_PIC: BrandCode = BrandCode::new(*b"1pic");
    /// 3GPP2
    ///
    /// FourCC: `3g2a`
    ///
    /// Specification: _3GPP2_
    pub const THREE_G2A: BrandCode = BrandCode::new(*b"3g2a");
    /// 3GPP Release 6 extended-presentation Profile
    ///
    /// FourCC: `3ge6`
    ///
    /// Specification: _3GPP_
    pub const THREE_GE6: BrandCode = BrandCode::new(*b"3ge6");
    /// 3GPP Release 7 extended-presentation Profile
    ///
    /// FourCC: `3ge7`
    ///
    /// Specification: _3GPP_
    pub const THREE_GE7: BrandCode = BrandCode::new(*b"3ge7");
    /// 3GPP Release 9 Extended Presentation Profile
    ///
    /// FourCC: `3ge9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GE9: BrandCode = BrandCode::new(*b"3ge9");
    /// 3GPP Release 9 File-delivery Server Profile
    ///
    /// FourCC: `3gf9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GF9: BrandCode = BrandCode::new(*b"3gf9");
    /// 3GPP Release 6 General Profile
    ///
    /// FourCC: `3gg6`
    ///
    /// Specification: _3GPP_
    pub const THREE_GG6: BrandCode = BrandCode::new(*b"3gg6");
    /// 3GPP Release 9 General Profile
    ///
    /// FourCC: `3gg9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GG9: BrandCode = BrandCode::new(*b"3gg9");
    /// 3GPP Release 9 Adaptive Streaming Profile
    ///
    /// FourCC: `3gh9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GH9: BrandCode = BrandCode::new(*b"3gh9");
    /// 3GPP Release 9 Media Segment Profile
    ///
    /// FourCC: `3gm9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GM9: BrandCode = BrandCode::new(*b"3gm9");
    /// 3GPP Media Segment conforming to the Media Segment Format for 3GP DASH
    ///
    /// FourCC: `3gmA`
    ///
    /// Specification: _3GPP-DASH_
    pub const THREE_GMA: BrandCode = BrandCode::new(*b"3gmA");
    /// 3GPP Release 4
    ///
    /// FourCC: `3gp4`
    ///
    /// Specification: _3GPP_
    pub const THREE_GP4: BrandCode = BrandCode::new(*b"3gp4");
    /// 3GPP Release 5
    ///
    /// FourCC: `3gp5`
    ///
    /// Specification: _3GPP_
    pub const THREE_GP5: BrandCode = BrandCode::new(*b"3gp5");
    /// 3GPP Release 6 basic Profile
    ///
    /// FourCC: `3gp6`
    ///
    /// Specification: _3GPP_
    pub const THREE_GP6: BrandCode = BrandCode::new(*b"3gp6");
    /// 3GPP Release 7
    ///
    /// FourCC: `3gp7`
    ///
    /// Specification: _3GPP_
    pub const THREE_GP7: BrandCode = BrandCode::new(*b"3gp7");
    /// 3GPP Release 8
    ///
    /// FourCC: `3gp8`
    ///
    /// Specification: _3GPP_
    pub const THREE_GP8: BrandCode = BrandCode::new(*b"3gp8");
    /// 3GPP Release 9 Basic Profile
    ///
    /// FourCC: `3gp9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GP9: BrandCode = BrandCode::new(*b"3gp9");
    /// 3GPP Release 6 progressive-download Profile
    ///
    /// FourCC: `3gr6`
    ///
    /// Specification: _3GPP_
    pub const THREE_GR6: BrandCode = BrandCode::new(*b"3gr6");
    /// 3GPP Release 9 Progressive DownloadProfile
    ///
    /// FourCC: `3gr9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GR9: BrandCode = BrandCode::new(*b"3gr9");
    /// 3GPP Release 6 streaming-server Profile
    ///
    /// FourCC: `3gs6`
    ///
    /// Specification: _3GPP_
    pub const THREE_GS6: BrandCode = BrandCode::new(*b"3gs6");
    /// 3GPP Release 9 Streaming ServerProfile
    ///
    /// FourCC: `3gs9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GS9: BrandCode = BrandCode::new(*b"3gs9");
    /// 3GPP Release 8 Media Stream Recording Profile
    ///
    /// FourCC: `3gt8`
    ///
    /// Specification: _3GPP_
    pub const THREE_GT8: BrandCode = BrandCode::new(*b"3gt8");
    /// 3GPP Release 9 Media Stream Recording Profile
    ///
    /// FourCC: `3gt9`
    ///
    /// Specification: _3GPP_
    pub const THREE_GT9: BrandCode = BrandCode::new(*b"3gt9");
    /// 3GPP TV over 3GPP services
    ///
    /// FourCC: `3gtv`
    ///
    /// Specification: _3GPP-TVP_
    pub const THREE_GTV: BrandCode = BrandCode::new(*b"3gtv");
    /// 3GPP VR Presentation
    ///
    /// FourCC: `3gvr`
    ///
    /// Specification: _3GPP-VR_
    pub const THREE_GVR: BrandCode = BrandCode::new(*b"3gvr");
    /// 3GPP VR Advanced Video Media Profile
    ///
    /// FourCC: `3vra`
    ///
    /// Specification: _3GPP-VR_
    pub const THREE_VRA: BrandCode = BrandCode::new(*b"3vra");
    /// 3GPP VR Basic Video Media Profile
    ///
    /// FourCC: `3vrb`
    ///
    /// Specification: _3GPP-VR_
    pub const THREE_VRB: BrandCode = BrandCode::new(*b"3vrb");
    /// 3GPP VR Main Video Media Profile
    ///
    /// FourCC: `3vrm`
    ///
    /// Specification: _3GPP-VR_
    pub const THREE_VRM: BrandCode = BrandCode::new(*b"3vrm");
    /// Advanced tiling OMAF video profile
    ///
    /// FourCC: `adti`
    ///
    /// Specification: _OMAF_
    pub const ADTI: BrandCode = BrandCode::new(*b"adti");
    /// ID3 metadata carried as timed metadata in CMAF
    ///
    /// FourCC: `aid3`
    ///
    /// Specification: _CMAF-ID3_
    pub const AID3: BrandCode = BrandCode::new(*b"aid3");
    /// ARRI Digital Camera
    ///
    /// FourCC: `ARRI`
    ///
    /// Specification: _ARRI_
    pub const ARRI: BrandCode = BrandCode::new(*b"ARRI");
    /// AOM Video Codec
    ///
    /// FourCC: `av01`
    ///
    /// Specification: _AV1-ISOBMFF_
    pub const AV01: BrandCode = BrandCode::new(*b"av01");
    /// Advanced Video Coding extensions
    ///
    /// FourCC: `avc1`
    ///
    /// Specification: _ISO_
    pub const AVC1: BrandCode = BrandCode::new(*b"avc1");
    /// AVC image and image collection brands
    ///
    /// FourCC: `avci`
    ///
    /// Specification: _HEIF_
    pub const AVCI: BrandCode = BrandCode::new(*b"avci");
    /// AVC image sequence brands
    ///
    /// FourCC: `avcs`
    ///
    /// Specification: _HEIF_
    pub const AVCS: BrandCode = BrandCode::new(*b"avcs");
    /// AVC-based viewport-dependent OMAF video profile
    ///
    /// FourCC: `avde`
    ///
    /// Specification: _OMAF_
    pub const AVDE: BrandCode = BrandCode::new(*b"avde");
    /// AV1 image format brand
    ///
    /// FourCC: `avif`
    ///
    /// Specification: _AVIF_
    pub const AVIF: BrandCode = BrandCode::new(*b"avif");
    /// AV1 intra-only brand
    ///
    /// FourCC: `avio`
    ///
    /// Specification: _AVIF_
    pub const AVIO: BrandCode = BrandCode::new(*b"avio");
    /// AV1 image sequence brand
    ///
    /// FourCC: `avis`
    ///
    /// Specification: _AVIF_
    pub const AVIS: BrandCode = BrandCode::new(*b"avis");
    /// Blinkbox Master File: H.264 video and 16-bit little-endian LPCM audio
    ///
    /// FourCC: `bbxm`
    ///
    /// Specification: _Blinkbox_
    pub const BBXM: BrandCode = BrandCode::new(*b"bbxm");
    /// Canon Digital Camera
    ///
    /// FourCC: `CAEP`
    ///
    /// Specification: _Canon_
    pub const CAEP: BrandCode = BrandCode::new(*b"CAEP");
    /// Convergent Designs
    ///
    /// FourCC: `CDes`
    ///
    /// Specification: _Convergent_
    pub const CDES: BrandCode = BrandCode::new(*b"CDes");
    /// CMAF Media Profile - AC-4 Main
    ///
    /// FourCC: `ca4m`
    ///
    /// Specification: _ETSI AC-4_
    pub const CA4M: BrandCode = BrandCode::new(*b"ca4m");
    /// CMAF Media Profile - AC-4 Single Stream
    ///
    /// FourCC: `ca4s`
    ///
    /// Specification: _ETSI AC-4_
    pub const CA4S: BrandCode = BrandCode::new(*b"ca4s");
    /// CMAF Media Profile - AAC Adaptive Audio
    ///
    /// FourCC: `caaa`
    ///
    /// Specification: _CMAF_
    pub const CAAA: BrandCode = BrandCode::new(*b"caaa");
    /// CMAF Media Profile - AAC Core
    ///
    /// FourCC: `caac`
    ///
    /// Specification: _CMAF_
    pub const CAAC: BrandCode = BrandCode::new(*b"caac");
    /// CMAF Media Profile for OMAF 3D audio baseline profile
    ///
    /// FourCC: `cabl`
    ///
    /// Specification: _OMAF_
    pub const CABL: BrandCode = BrandCode::new(*b"cabl");
    /// CMAF Media Profile - AAC multichannel adaptive audio
    ///
    /// FourCC: `cama`
    ///
    /// Specification: _CMAF_
    pub const CAMA: BrandCode = BrandCode::new(*b"cama");
    /// CMAF Media Profile - AAC multichannel audio
    ///
    /// FourCC: `camc`
    ///
    /// Specification: _CMAF_
    pub const CAMC: BrandCode = BrandCode::new(*b"camc");
    /// Casio Digital Camera
    ///
    /// FourCC: `caqv`
    ///
    /// Specification: _Casio_
    pub const CAQV: BrandCode = BrandCode::new(*b"caqv");
    /// CMAF Media Profile - MPEG-D USAC audio
    ///
    /// FourCC: `casu`
    ///
    /// Specification: _CMAF_
    pub const CASU: BrandCode = BrandCode::new(*b"casu");
    /// CMAF Supplemental Data - CEA-608/708
    ///
    /// FourCC: `ccea`
    ///
    /// Specification: _CMAF_
    pub const CCEA: BrandCode = BrandCode::new(*b"ccea");
    /// Common container file format
    ///
    /// FourCC: `ccff`
    ///
    /// Specification: _DECE_
    pub const CCFF: BrandCode = BrandCode::new(*b"ccff");
    /// CMAF Media Profile - HEVC HDR10 (chd1) with SCTE Dynamic Metadata app #1 (ST2094-10)
    ///
    /// FourCC: `cdm1`
    ///
    /// Specification: _SCTE-215-1-1b_
    pub const CDM1: BrandCode = BrandCode::new(*b"cdm1");
    /// CMAF Media Profile compatibility to HDR10+ (ST2094-40)
    ///
    /// FourCC: `cdm4`
    ///
    /// Specification: _SCTE-215-1-1b_
    pub const CDM4: BrandCode = BrandCode::new(*b"cdm4");
    /// CMAF Media Profile - Enhanced AC-3
    ///
    /// FourCC: `ceac`
    ///
    /// Specification: _ETSI AC-3_
    pub const CEAC: BrandCode = BrandCode::new(*b"ceac");
    /// CMAF Media Profile - AVC HD
    ///
    /// FourCC: `cfhd`
    ///
    /// Specification: _CMAF_
    pub const CFHD: BrandCode = BrandCode::new(*b"cfhd");
    /// CMAF Media Profile - AVC SD
    ///
    /// FourCC: `cfsd`
    ///
    /// Specification: _CMAF_
    pub const CFSD: BrandCode = BrandCode::new(*b"cfsd");
    /// CMAF Media Profile - HEVC HDR10
    ///
    /// FourCC: `chd1`
    ///
    /// Specification: _CMAF_
    pub const CHD1: BrandCode = BrandCode::new(*b"chd1");
    /// CMAF Media Profile - AVC HDHF
    ///
    /// FourCC: `chdf`
    ///
    /// Specification: _CMAF_
    pub const CHDF: BrandCode = BrandCode::new(*b"chdf");
    /// CMAF Media Profile for the HEVC-based viewport-dependent OMAF video profile
    ///
    /// FourCC: `chev`
    ///
    /// Specification: _OMAF_
    pub const CHEV: BrandCode = BrandCode::new(*b"chev");
    /// CMAF High frame rate Media Profile - HEVC HDR10H
    ///
    /// FourCC: `chd2`
    ///
    /// Specification: _CMAF_
    pub const CHD2: BrandCode = BrandCode::new(*b"chd2");
    /// CMAF Media Profile - HEVC HHD10
    ///
    /// FourCC: `chh1`
    ///
    /// Specification: _CMAF_
    pub const CHH1: BrandCode = BrandCode::new(*b"chh1");
    /// CMAF Media Profile - HEVC HHD8
    ///
    /// FourCC: `chhd`
    ///
    /// Specification: _CMAF_
    pub const CHHD: BrandCode = BrandCode::new(*b"chhd");
    /// CMAF Interlaced Media Profile - INT10
    ///
    /// FourCC: `cint`
    ///
    /// Specification: _CMAF_
    pub const CINT: BrandCode = BrandCode::new(*b"cint");
    /// CMAF Media Profile - HEVC HLG10
    ///
    /// FourCC: `clg1`
    ///
    /// Specification: _CMAF_
    pub const CLG1: BrandCode = BrandCode::new(*b"clg1");
    /// CMAF High frame rate Media Profile - HEVC HLG10H
    ///
    /// FourCC: `clg2`
    ///
    /// Specification: _CMAF_
    pub const CLG2: BrandCode = BrandCode::new(*b"clg2");
    /// CMAF Structural Brand
    ///
    /// FourCC: `cmf2`
    ///
    /// Specification: _CMAF_
    pub const CMF2: BrandCode = BrandCode::new(*b"cmf2");
    /// CMAF Structural Brand
    ///
    /// FourCC: `cmfc`
    ///
    /// Specification: _CMAF_
    pub const CMFC: BrandCode = BrandCode::new(*b"cmfc");
    /// CMAF Fragment Format
    ///
    /// FourCC: `cmff`
    ///
    /// Specification: _CMAF_
    pub const CMFF: BrandCode = BrandCode::new(*b"cmff");
    /// CMAF Chunk Format
    ///
    /// FourCC: `cmfl`
    ///
    /// Specification: _CMAF_
    pub const CMFL: BrandCode = BrandCode::new(*b"cmfl");
    /// CMAF Segment Format
    ///
    /// FourCC: `cmfs`
    ///
    /// Specification: _CMAF_
    pub const CMFS: BrandCode = BrandCode::new(*b"cmfs");
    /// CMAF Media Profile - MPEG-H 3D audio LC (mhm2)
    ///
    /// FourCC: `cmhm`
    ///
    /// Specification: _CMAF_
    pub const CMHM: BrandCode = BrandCode::new(*b"cmhm");
    /// CMAF Media Profile - MPEG-H 3D audio LC (mhm1)
    ///
    /// FourCC: `cmhs`
    ///
    /// Specification: _CMAF_
    pub const CMHS: BrandCode = BrandCode::new(*b"cmhs");
    /// Compressed boxes
    ///
    /// FourCC: `comp`
    ///
    /// Specification: _ISO_
    pub const COMP: BrandCode = BrandCode::new(*b"comp");
    /// CMAF Media Profile - Scalable HEVC media profile
    ///
    /// FourCC: `csh1`
    ///
    /// Specification: _CMAF_
    pub const CSH1: BrandCode = BrandCode::new(*b"csh1");
    /// CMAF Media Profile - HEVC UHD10
    ///
    /// FourCC: `cud1`
    ///
    /// Specification: _CMAF_
    pub const CUD1: BrandCode = BrandCode::new(*b"cud1");
    /// CMAF High frame rate Media Profile - HEVC UHD10H
    ///
    /// FourCC: `cud2`
    ///
    /// Specification: _CMAF_
    pub const CUD2: BrandCode = BrandCode::new(*b"cud2");
    /// CMAF Media Profile - HEVC UHD8
    ///
    /// FourCC: `cud8`
    ///
    /// Specification: _CMAF_
    pub const CUD8: BrandCode = BrandCode::new(*b"cud8");
    /// CMAF High frame rate Media Profile - HEVC UHD8H
    ///
    /// FourCC: `cud9`
    ///
    /// Specification: _CMAF_
    pub const CUD9: BrandCode = BrandCode::new(*b"cud9");
    /// CMAF Media Profile for the unconstrained HEVC-based viewport-independent OMAF video profile
    ///
    /// FourCC: `cuvd`
    ///
    /// Specification: _OMAF_
    pub const CUVD: BrandCode = BrandCode::new(*b"cuvd");
    /// CMAF Media Profile for the HEVC-based viewport-independent OMAF video profile
    ///
    /// FourCC: `cvid`
    ///
    /// Specification: _OMAF_
    pub const CVID: BrandCode = BrandCode::new(*b"cvid");
    /// Baseline VVC media profile
    ///
    /// FourCC: `cvvc`
    ///
    /// Specification: _CMAF_
    pub const CVVC: BrandCode = BrandCode::new(*b"cvvc");
    /// CMAF Media Profile - WebVTT
    ///
    /// FourCC: `cwvt`
    ///
    /// Specification: _CMAF_
    pub const CWVT: BrandCode = BrandCode::new(*b"cwvt");
    /// DMB AF audio with MPEG Layer II audio, MOT slide show, DLS, JPG/PNG/MNG images
    ///
    /// FourCC: `da0a`
    ///
    /// Specification: _DMB-MAF_
    pub const DA0A: BrandCode = BrandCode::new(*b"da0a");
    /// DMB AF, extending da0a , with 3GPP timed text, DID, TVA, REL, IPMP
    ///
    /// FourCC: `da0b`
    ///
    /// Specification: _DMB-MAF_
    pub const DA0B: BrandCode = BrandCode::new(*b"da0b");
    /// DMB AF audio with ER-BSAC audio, JPG/PNG/MNG images
    ///
    /// FourCC: `da1a`
    ///
    /// Specification: _DMB-MAF_
    pub const DA1A: BrandCode = BrandCode::new(*b"da1a");
    /// DMB AF, extending da1a, with 3GPP timed text, DID, TVA, REL, IPMP
    ///
    /// FourCC: `da1b`
    ///
    /// Specification: _DMB-MAF_
    pub const DA1B: BrandCode = BrandCode::new(*b"da1b");
    /// DMB AF audio with HE-AAC v2 audio, MOT slide show, DLS, JPG/PNG/MNG images
    ///
    /// FourCC: `da2a`
    ///
    /// Specification: _DMB-MAF_
    pub const DA2A: BrandCode = BrandCode::new(*b"da2a");
    /// DMB AF extending da2a, with 3GPP timed text, DID, TVA, REL, IPMP
    ///
    /// FourCC: `da2b`
    ///
    /// Specification: _DMB-MAF_
    pub const DA2B: BrandCode = BrandCode::new(*b"da2b");
    /// DMB AF audio with HE-AAC, JPG/PNG/MNG images
    ///
    /// FourCC: `da3a`
    ///
    /// Specification: _DMB-MAF_
    pub const DA3A: BrandCode = BrandCode::new(*b"da3a");
    /// DMB AF extending da3a with BIFS, 3GPP timed text, DID, TVA, REL, IPMP
    ///
    /// FourCC: `da3b`
    ///
    /// Specification: _DMB-MAF_
    pub const DA3B: BrandCode = BrandCode::new(*b"da3b");
    /// ISO base media file format file specifically designed for DASH including movie fragments and Segment Index.
    ///
    /// FourCC: `dash`
    ///
    /// Specification: _DASH_
    pub const DASH: BrandCode = BrandCode::new(*b"dash");
    /// Dolby Vision cross-compatible with HDR10
    ///
    /// FourCC: `db1p`
    ///
    /// Specification: _Dolby_
    pub const DB1P: BrandCode = BrandCode::new(*b"db1p");
    /// Dolby Vision cross-compatible with SDR
    ///
    /// FourCC: `db2g`
    ///
    /// Specification: _Dolby_
    pub const DB2G: BrandCode = BrandCode::new(*b"db2g");
    /// Dolby Vision cross-compatible with HLG (VUI =18)
    ///
    /// FourCC: `db4h`
    ///
    /// Specification: _Dolby_
    pub const DB4H: BrandCode = BrandCode::new(*b"db4h");
    /// Dolby Vision cross-compatible with HLG (VUI=14)
    ///
    /// FourCC: `db4g`
    ///
    /// Specification: _Dolby_
    pub const DB4G: BrandCode = BrandCode::new(*b"db4g");
    /// MP4 files with Dolby content (e.g. Dolby AC-4, Dolby Digital Plus, Dolby TrueHD (Dolby MLP))
    ///
    /// FourCC: `dby1`
    ///
    /// Specification: _Dolby_
    pub const DBY1: BrandCode = BrandCode::new(*b"dby1");
    /// DMB AF supporting all the components defined in the specification
    ///
    /// FourCC: `dmb1`
    ///
    /// Specification: _DMB-MAF_
    pub const DMB1: BrandCode = BrandCode::new(*b"dmb1");
    /// Media Segment conforming to the DASH Self-Initializing Media Segment format type for ISO base media file format
    ///
    /// FourCC: `dsms`
    ///
    /// Specification: _DASH_
    pub const DSMS: BrandCode = BrandCode::new(*b"dsms");
    ///  CMAF media profile for audio codecs dtsc dtsh or dtse
    ///
    /// FourCC: `dts1`
    ///
    /// Specification: _DTS-HD_
    pub const DTS1: BrandCode = BrandCode::new(*b"dts1");
    ///  CMAF media profile for audio codec dtsx
    ///
    /// FourCC: `dts2`
    ///
    /// Specification: _DTS-UHD_
    pub const DTS2: BrandCode = BrandCode::new(*b"dts2");
    ///  CMAF media profile for audio codec dtsy
    ///
    /// FourCC: `dts3`
    ///
    /// Specification: _DTS-UHD_
    pub const DTS3: BrandCode = BrandCode::new(*b"dts3");
    /// DMB AF video with AVC video, ER-BSAC audio, BIFS, JPG/PNG/MNG images, TS
    ///
    /// FourCC: `dv1a`
    ///
    /// Specification: _DMB-MAF_
    pub const DV1A: BrandCode = BrandCode::new(*b"dv1a");
    /// DMB AF, extending dv1a, with 3GPP timed text, DID, TVA, REL, IPMP
    ///
    /// FourCC: `dv1b`
    ///
    /// Specification: _DMB-MAF_
    pub const DV1B: BrandCode = BrandCode::new(*b"dv1b");
    /// DMB AF video with AVC video, HE-AACv2 audio, BIFS, JPG/PNG/MNG images, TS
    ///
    /// FourCC: `dv2a`
    ///
    /// Specification: _DMB-MAF_
    pub const DV2A: BrandCode = BrandCode::new(*b"dv2a");
    /// DMB AF extending dv2a, with 3GPP timed text, DID, TVA, REL, IPMP
    ///
    /// FourCC: `dv2b`
    ///
    /// Specification: _DMB-MAF_
    pub const DV2B: BrandCode = BrandCode::new(*b"dv2b");
    /// DMB AF video with AVC video, HE-AAC audio, BIFS, JPG/PNG/MNG images, TS
    ///
    /// FourCC: `dv3a`
    ///
    /// Specification: _DMB-MAF_
    pub const DV3A: BrandCode = BrandCode::new(*b"dv3a");
    /// DMB AF extending dv3a with 3GPP timed text, DID, TVA, REL, IPMP
    ///
    /// FourCC: `dv3b`
    ///
    /// Specification: _DMB-MAF_
    pub const DV3B: BrandCode = BrandCode::new(*b"dv3b");
    /// DVB RTP
    ///
    /// FourCC: `dvr1`
    ///
    /// Specification: _DVB_
    pub const DVR1: BrandCode = BrandCode::new(*b"dvr1");
    /// DVB Transport Stream
    ///
    /// FourCC: `dvt1`
    ///
    /// Specification: _DVB_
    pub const DVT1: BrandCode = BrandCode::new(*b"dvt1");
    /// DxO ONE camera
    ///
    /// FourCC: `dxo `
    ///
    /// Specification: _DxO_
    pub const DXO: BrandCode = BrandCode::new(*b"dxo ");
    /// Event message box present
    ///
    /// FourCC: `emsg`
    ///
    /// Specification: _DASH_
    pub const EMSG: BrandCode = BrandCode::new(*b"emsg");
    /// EVC Baseline coded image
    ///
    /// FourCC: `evbi`
    ///
    /// Specification: _HEIF_
    pub const EVBI: BrandCode = BrandCode::new(*b"evbi");
    /// EVC Baseline coded image sequence
    ///
    /// FourCC: `evbs`
    ///
    /// Specification: _HEIF_
    pub const EVBS: BrandCode = BrandCode::new(*b"evbs");
    /// EVC Main coded image
    ///
    /// FourCC: `evmi`
    ///
    /// Specification: _HEIF_
    pub const EVMI: BrandCode = BrandCode::new(*b"evmi");
    /// EVC Main coded image sequenc
    ///
    /// FourCC: `evms`
    ///
    /// Specification: _HEIF_
    pub const EVMS: BrandCode = BrandCode::new(*b"evms");
    /// HEVC image and image collection brands
    ///
    /// FourCC: `heic`
    ///
    /// Specification: _HEIF_
    pub const HEIC: BrandCode = BrandCode::new(*b"heic");
    /// L-HEVC image and image collection brands
    ///
    /// FourCC: `heim`
    ///
    /// Specification: _HEIF_
    pub const HEIM: BrandCode = BrandCode::new(*b"heim");
    /// L-HEVC image and image collection brands
    ///
    /// FourCC: `heis`
    ///
    /// Specification: _HEIF_
    pub const HEIS: BrandCode = BrandCode::new(*b"heis");
    /// HEVC image and image collection brands
    ///
    /// FourCC: `heix`
    ///
    /// Specification: _HEIF_
    pub const HEIX: BrandCode = BrandCode::new(*b"heix");
    /// OMAF HEVC image profile
    ///
    /// FourCC: `heoi`
    ///
    /// Specification: _OMAF_
    pub const HEOI: BrandCode = BrandCode::new(*b"heoi");
    /// HEVC image sequence brands
    ///
    /// FourCC: `hevc`
    ///
    /// Specification: _HEIF_
    pub const HEVC: BrandCode = BrandCode::new(*b"hevc");
    /// HEVC-based viewport-dependent OMAF video profile
    ///
    /// FourCC: `hevd`
    ///
    /// Specification: _OMAF_
    pub const HEVD: BrandCode = BrandCode::new(*b"hevd");
    /// HEVC-based viewport-independent OMAF video profile
    ///
    /// FourCC: `hevi`
    ///
    /// Specification: _OMAF_
    pub const HEVI: BrandCode = BrandCode::new(*b"hevi");
    /// L-HEVC image sequence brands
    ///
    /// FourCC: `hevm`
    ///
    /// Specification: _HEIF_
    pub const HEVM: BrandCode = BrandCode::new(*b"hevm");
    /// L-HEVC image sequence brands
    ///
    /// FourCC: `hevs`
    ///
    /// Specification: _HEIF_
    pub const HEVS: BrandCode = BrandCode::new(*b"hevs");
    /// HEVC image sequence brands
    ///
    /// FourCC: `hevx`
    ///
    /// Specification: _HEIF_
    pub const HEVX: BrandCode = BrandCode::new(*b"hevx");
    /// L-HEVC explicit reconstruction
    ///
    /// FourCC: `hvce`
    ///
    /// Specification: _NALu Video_
    pub const HVCE: BrandCode = BrandCode::new(*b"hvce");
    /// L-HEVC implicit reconstruction
    ///
    /// FourCC: `hvci`
    ///
    /// Specification: _NALu Video_
    pub const HVCI: BrandCode = BrandCode::new(*b"hvci");
    /// L-HEVC extended explicit reconstruction brand
    ///
    /// FourCC: `hvcx`
    ///
    /// Specification: _NALu Video_
    pub const HVCX: BrandCode = BrandCode::new(*b"hvcx");
    /// HEVC Tile Track
    ///
    /// FourCC: `hvti`
    ///
    /// Specification: _NALu Video_
    pub const HVTI: BrandCode = BrandCode::new(*b"hvti");
    /// The IFE-SD Media Profile
    ///
    /// FourCC: `ifsd`
    ///
    /// Specification: _IFE_
    pub const IFSD: BrandCode = BrandCode::new(*b"ifsd");
    /// The IFE-HSD Media Profile
    ///
    /// FourCC: `ifhs`
    ///
    /// Specification: _IFE_
    pub const IFHS: BrandCode = BrandCode::new(*b"ifhs");
    /// The IFE-HD Media Profile
    ///
    /// FourCC: `ifhd`
    ///
    /// Specification: _IFE_
    pub const IFHD: BrandCode = BrandCode::new(*b"ifhd");
    /// The IFE-HHD10 Media Profile
    ///
    /// FourCC: `ifhx`
    ///
    /// Specification: _IFE_
    pub const IFHX: BrandCode = BrandCode::new(*b"ifhx");
    /// The IFE-HDHDR Media Profile
    ///
    /// FourCC: `ifhh`
    ///
    /// Specification: _IFE_
    pub const IFHH: BrandCode = BrandCode::new(*b"ifhh");
    /// The IFE-UHD10 Media Profile
    ///
    /// FourCC: `ifhu`
    ///
    /// Specification: _IFE_
    pub const IFHU: BrandCode = BrandCode::new(*b"ifhu");
    /// The IFE-HDR10 Media Profile
    ///
    /// FourCC: `ifhr`
    ///
    /// Specification: _IFE_
    pub const IFHR: BrandCode = BrandCode::new(*b"ifhr");
    /// The IFE-AAC Core Media Profile
    ///
    /// FourCC: `ifaa`
    ///
    /// Specification: _IFE_
    pub const IFAA: BrandCode = BrandCode::new(*b"ifaa");
    /// The IFE-AV1-SD Media Profile
    ///
    /// FourCC: `ifas`
    ///
    /// Specification: _IFE_
    pub const IFAS: BrandCode = BrandCode::new(*b"ifas");
    /// The IFE-AV1-HD Media Profile
    ///
    /// FourCC: `ifah`
    ///
    /// Specification: _IFE_
    pub const IFAH: BrandCode = BrandCode::new(*b"ifah");
    /// The IFE-AV1-HDHDR Media Profile
    ///
    /// FourCC: `ifai`
    ///
    /// Specification: _IFE_
    pub const IFAI: BrandCode = BrandCode::new(*b"ifai");
    /// The IFE-AV1-UHD10 Media Profile
    ///
    /// FourCC: `ifau`
    ///
    /// Specification: _IFE_
    pub const IFAU: BrandCode = BrandCode::new(*b"ifau");
    /// The IFE-AV1-HDR10 Media Profile
    ///
    /// FourCC: `ifav`
    ///
    /// Specification: _IFE_
    pub const IFAV: BrandCode = BrandCode::new(*b"ifav");
    /// Apple iFrame Specification, Version 8.1 Jan 2013
    ///
    /// FourCC: `ifrm`
    ///
    /// Specification: _Apple_
    pub const IFRM: BrandCode = BrandCode::new(*b"ifrm");
    /// CMAF Media Profile - IMSC1 Image
    ///
    /// FourCC: `im1i`
    ///
    /// Specification: _CMAF_
    pub const IM1I: BrandCode = BrandCode::new(*b"im1i");
    /// CMAF Media Profile - IMSC1 Text
    ///
    /// FourCC: `im1t`
    ///
    /// Specification: _CMAF_
    pub const IM1T: BrandCode = BrandCode::new(*b"im1t");
    /// CMAF Media Profile - IMSC1.1 Image
    ///
    /// FourCC: `im2i`
    ///
    /// Specification: _CMAF_
    pub const IM2I: BrandCode = BrandCode::new(*b"im2i");
    /// CMAF Media Profile - IMSC1.1 Text
    ///
    /// FourCC: `im2t`
    ///
    /// Specification: _CMAF_
    pub const IM2T: BrandCode = BrandCode::new(*b"im2t");
    /// Files encrypted according to ISMACryp 2.0
    ///
    /// FourCC: `isc2`
    ///
    /// Specification: _ISMACryp2_
    pub const ISC2: BrandCode = BrandCode::new(*b"isc2");
    /// All files based on the 2004 edition of the ISO file format
    ///
    /// FourCC: `iso2`
    ///
    /// Specification: _ISO_
    pub const ISO2: BrandCode = BrandCode::new(*b"iso2");
    /// Version of the ISO file format
    ///
    /// FourCC: `iso3`
    ///
    /// Specification: _ISO_
    pub const ISO3: BrandCode = BrandCode::new(*b"iso3");
    /// Version of the ISO file format
    ///
    /// FourCC: `iso4`
    ///
    /// Specification: _ISO_
    pub const ISO4: BrandCode = BrandCode::new(*b"iso4");
    /// Version of the ISO file format
    ///
    /// FourCC: `iso5`
    ///
    /// Specification: _ISO_
    pub const ISO5: BrandCode = BrandCode::new(*b"iso5");
    /// Version of the ISO file format
    ///
    /// FourCC: `iso6`
    ///
    /// Specification: _ISO_
    pub const ISO6: BrandCode = BrandCode::new(*b"iso6");
    /// Version of the ISO file format
    ///
    /// FourCC: `iso7`
    ///
    /// Specification: _ISO_
    pub const ISO7: BrandCode = BrandCode::new(*b"iso7");
    /// Version of the ISO file format
    ///
    /// FourCC: `iso8`
    ///
    /// Specification: _ISO_
    pub const ISO8: BrandCode = BrandCode::new(*b"iso8");
    /// Version of the ISO file format
    ///
    /// FourCC: `iso9`
    ///
    /// Specification: _ISO_
    pub const ISO9: BrandCode = BrandCode::new(*b"iso9");
    /// Version of the ISO file format
    ///
    /// FourCC: `isoa`
    ///
    /// Specification: _ISO_
    pub const ISOA: BrandCode = BrandCode::new(*b"isoa");
    /// Version of the ISO file format
    ///
    /// FourCC: `isob`
    ///
    /// Specification: _ISO_
    pub const ISOB: BrandCode = BrandCode::new(*b"isob");
    /// Version of the ISO file format
    ///
    /// FourCC: `isoc`
    ///
    /// Specification: _ISO_
    pub const ISOC: BrandCode = BrandCode::new(*b"isoc");
    /// All files based on the ISO Base Media File Format
    ///
    /// FourCC: `isom`
    ///
    /// Specification: _ISO_
    pub const ISOM: BrandCode = BrandCode::new(*b"isom");
    /// JPEG 2000 image and image collections in ISO/IEC 23008-12 files
    ///
    /// FourCC: `j2ki`
    ///
    /// Specification: _J2KHEIF_
    pub const J2KI: BrandCode = BrandCode::new(*b"j2ki");
    /// Motion JPEG 2000 in ISO/IEC 23008-12 files
    ///
    /// FourCC: `j2ks`
    ///
    /// Specification: _J2KHEIF_
    pub const J2KS: BrandCode = BrandCode::new(*b"j2ks");
    /// JPEG 2000 image sequence in ISO/IEC 23008-12 files
    ///
    /// FourCC: `j2is`
    ///
    /// Specification: _J2KHEIF_
    pub const J2IS: BrandCode = BrandCode::new(*b"j2is");
    /// JPEG2000 Profile 0
    ///
    /// FourCC: `J2P0`
    ///
    /// Specification: _JPEG2000_
    pub const J2P0: BrandCode = BrandCode::new(*b"J2P0");
    /// JPEG2000 Profile 1
    ///
    /// FourCC: `J2P1`
    ///
    /// Specification: _JPEG2000_
    pub const J2P1: BrandCode = BrandCode::new(*b"J2P1");
    /// JPEG2000 Part 1
    ///
    /// FourCC: `jp2 `
    ///
    /// Specification: _JPEG2000_
    pub const JP2: BrandCode = BrandCode::new(*b"jp2 ");
    /// JPEG-specific still image brand
    ///
    /// FourCC: `jpeg`
    ///
    /// Specification: _HEIF_
    pub const JPEG: BrandCode = BrandCode::new(*b"jpeg");
    /// JPEG image sequence brands
    ///
    /// FourCC: `jpgs`
    ///
    /// Specification: _HEIF_
    pub const JPGS: BrandCode = BrandCode::new(*b"jpgs");
    /// JPEG 2000 Part 6 Compound Images
    ///
    /// FourCC: `jpm `
    ///
    /// Specification: _JPM_
    pub const JPM: BrandCode = BrandCode::new(*b"jpm ");
    /// OMAF legacy image profile
    ///
    /// FourCC: `jpoi`
    ///
    /// Specification: _OMAF_
    pub const JPOI: BrandCode = BrandCode::new(*b"jpoi");
    /// The JPSearch data interchange format, for the exchange of image collections and respective metadata
    ///
    /// FourCC: `jpsi`
    ///
    /// Specification: _JPSearch_
    pub const JPSI: BrandCode = BrandCode::new(*b"jpsi");
    /// JPEG2000 Part 2
    ///
    /// FourCC: `jpx `
    ///
    /// Specification: _JPX_
    pub const JPX: BrandCode = BrandCode::new(*b"jpx ");
    /// JPEG XR
    ///
    /// FourCC: `jpxb`
    ///
    /// Specification: _JPXR_
    pub const JPXB: BrandCode = BrandCode::new(*b"jpxb");
    /// JPEG XL
    ///
    /// FourCC: `jxl `
    ///
    /// Specification: _JPEG XL_
    pub const JXL: BrandCode = BrandCode::new(*b"jxl ");
    /// Still Image File Format for JPEG XS
    ///
    /// FourCC: `jxs `
    ///
    /// Specification: _JPXS_
    pub const JXS: BrandCode = BrandCode::new(*b"jxs ");
    /// Codestream for JPEG XS
    ///
    /// FourCC: `jxsc`
    ///
    /// Specification: _JPXS_
    pub const JXSC: BrandCode = BrandCode::new(*b"jxsc");
    /// JPEG XS image and image collections for HEIF
    ///
    /// FourCC: `jxsi`
    ///
    /// Specification: _JPXS_
    pub const JXSI: BrandCode = BrandCode::new(*b"jxsi");
    /// JPEG XS image sequences for HEIF
    ///
    /// FourCC: `jxss`
    ///
    /// Specification: _JPXS_
    pub const JXSS: BrandCode = BrandCode::new(*b"jxss");
    /// Leica digital camera
    ///
    /// FourCC: `LCAG`
    ///
    /// Specification: _Leica_
    pub const LCAG: BrandCode = BrandCode::new(*b"LCAG");
    /// L-HEVC Tile Track Explicit brand
    ///
    /// FourCC: `lhte`
    ///
    /// Specification: _NALu Video_
    pub const LHTE: BrandCode = BrandCode::new(*b"lhte");
    /// L-HEVC Tile Track Implicit brand
    ///
    /// FourCC: `lhti`
    ///
    /// Specification: _NALu Video_
    pub const LHTI: BrandCode = BrandCode::new(*b"lhti");
    /// last Media Segment indicator for ISO base media file format.
    ///
    /// FourCC: `lmsg`
    ///
    /// Specification: _DASH_
    pub const LMSG: BrandCode = BrandCode::new(*b"lmsg");
    /// iTunes MPEG-4 audio protected or not, can contain audio + video + 3g text track + chapter track
    ///
    /// FourCC: `M4A `
    ///
    /// Specification: _iTunes_
    pub const M4A: BrandCode = BrandCode::new(*b"M4A ");
    /// iTunes AudioBook protected or not, can contain audio + video + 3g text track + chapter track
    ///
    /// FourCC: `M4B `
    ///
    /// Specification: _iTunes_
    pub const M4B: BrandCode = BrandCode::new(*b"M4B ");
    /// MPEG-4 protected audio
    ///
    /// FourCC: `M4P `
    ///
    /// Specification: _iTunes_
    pub const M4P: BrandCode = BrandCode::new(*b"M4P ");
    /// MPEG-4 protected audio+video
    ///
    /// FourCC: `M4V `
    ///
    /// Specification: _iTunes_
    pub const M4V: BrandCode = BrandCode::new(*b"M4V ");
    /// AVIF Baseline Profile
    ///
    /// FourCC: `MA1B`
    ///
    /// Specification: _AVIF_
    pub const MA1B: BrandCode = BrandCode::new(*b"MA1B");
    /// AVIF Advanced Profile
    ///
    /// FourCC: `MA1A`
    ///
    /// Specification: _AVIF_
    pub const MA1A: BrandCode = BrandCode::new(*b"MA1A");
    /// Media File for Samsung video Metadata
    ///
    /// FourCC: `MFSM`
    ///
    /// Specification: _Samsung_
    pub const MFSM: BrandCode = BrandCode::new(*b"MFSM");
    /// Home and Mobile Multimedia Platform (HMMP)
    ///
    /// FourCC: `MGSV`
    ///
    /// Specification: _Sony_
    pub const MGSV: BrandCode = BrandCode::new(*b"MGSV");
    /// Multi-Image Application format brand for MIAF AVC Basic Profile
    ///
    /// FourCC: `MiAB`
    ///
    /// Specification: _MIAF_
    pub const MIAB: BrandCode = BrandCode::new(*b"MiAB");
    /// Multi-Image Application format brand for fragmented alpha video
    ///
    /// FourCC: `MiAC`
    ///
    /// Specification: _MIAF_
    pub const MIAC: BrandCode = BrandCode::new(*b"MiAC");
    /// Multi-Image Application format brand for general MIAF requirements
    ///
    /// FourCC: `miaf`
    ///
    /// Specification: _MIAF_
    pub const MIAF: BrandCode = BrandCode::new(*b"miaf");
    /// Mutli-Image Application format brand for animation
    ///
    /// FourCC: `MiAn`
    ///
    /// Specification: _MIAF_
    pub const MIAN: BrandCode = BrandCode::new(*b"MiAn");
    /// Multi-Image Application format brand for burst capture
    ///
    /// FourCC: `MiBu`
    ///
    /// Specification: _MIAF_
    pub const MIBU: BrandCode = BrandCode::new(*b"MiBu");
    /// Multi-Image Application format brand for CMAF compatibility
    ///
    /// FourCC: `MiCm`
    ///
    /// Specification: _MIAF_
    pub const MICM: BrandCode = BrandCode::new(*b"MiCm");
    /// Image file format structural brand
    ///
    /// FourCC: `mif1`
    ///
    /// Specification: _HEIF_
    pub const MIF1: BrandCode = BrandCode::new(*b"mif1");
    /// Image file format structural brand CICP alpha and depth
    ///
    /// FourCC: `mif2`
    ///
    /// Specification: _HEIF_
    pub const MIF2: BrandCode = BrandCode::new(*b"mif2");
    /// Multi-Image Application format brand for MIAF HEVC Advanced Profile
    ///
    /// FourCC: `MiHA`
    ///
    /// Specification: _MIAF_
    pub const MIHA: BrandCode = BrandCode::new(*b"MiHA");
    /// Multi-Image Application format brand for MIAF HEVC Basic Profile
    ///
    /// FourCC: `MiHB`
    ///
    /// Specification: _MIAF_
    pub const MIHB: BrandCode = BrandCode::new(*b"MiHB");
    /// Multi-Image Application format brand for MIAF HEVC Extended Profile
    ///
    /// FourCC: `MiHE`
    ///
    /// Specification: _MIAF_
    pub const MIHE: BrandCode = BrandCode::new(*b"MiHE");
    /// Multi-Image Application format brand for progressive decoding and rendering
    ///
    /// FourCC: `MiPr`
    ///
    /// Specification: _MIAF_
    pub const MIPR: BrandCode = BrandCode::new(*b"MiPr");
    /// Motion JPEG 2000 simple profile
    ///
    /// FourCC: `mj2s`
    ///
    /// Specification: _MJ2_
    pub const MJ2S: BrandCode = BrandCode::new(*b"mj2s");
    /// Motion JPEG 2000, general profile
    ///
    /// FourCC: `mjp2`
    ///
    /// Specification: _MJ2_
    pub const MJP2: BrandCode = BrandCode::new(*b"mjp2");
    /// MPEG-21
    ///
    /// FourCC: `mp21`
    ///
    /// Specification: _MPEG-21_
    pub const MP21: BrandCode = BrandCode::new(*b"mp21");
    /// MP4 version 1
    ///
    /// FourCC: `mp41`
    ///
    /// Specification: _MP4v2_
    pub const MP41: BrandCode = BrandCode::new(*b"mp41");
    /// MP4 version 2
    ///
    /// FourCC: `mp42`
    ///
    /// Specification: _MP4v2_
    pub const MP42: BrandCode = BrandCode::new(*b"mp42");
    /// MPEG-7 file-level metadata
    ///
    /// FourCC: `mp71`
    ///
    /// Specification: _ISO_
    pub const MP71: BrandCode = BrandCode::new(*b"mp71");
    /// Photo Player Multimedia Application Format
    ///
    /// FourCC: `MPPI`
    ///
    /// Specification: _ISO-MAF_
    pub const MPPI: BrandCode = BrandCode::new(*b"MPPI");
    /// Compliance with the MMT Processing Unit format
    ///
    /// FourCC: `mpuf`
    ///
    /// Specification: _MMT_
    pub const MPUF: BrandCode = BrandCode::new(*b"mpuf");
    /// Image file format structural brand
    ///
    /// FourCC: `msf1`
    ///
    /// Specification: _HEIF_
    pub const MSF1: BrandCode = BrandCode::new(*b"msf1");
    /// Media Segment conforming to the general format type for ISO base media file format.
    ///
    /// FourCC: `msdh`
    ///
    /// Specification: _DASH_
    pub const MSDH: BrandCode = BrandCode::new(*b"msdh");
    /// Media Segment conforming to the Indexed Media Segment format type for ISO base media file format.
    ///
    /// FourCC: `msix`
    ///
    /// Specification: _DASH_
    pub const MSIX: BrandCode = BrandCode::new(*b"msix");
    /// Portable multimedia CE products using MP4 file format with AVC video codec and AAC audio codec
    ///
    /// FourCC: `MSNV`
    ///
    /// Specification: _IEC 62592_
    pub const MSNV: BrandCode = BrandCode::new(*b"MSNV");
    /// Nikon Digital Camera
    ///
    /// FourCC: `niko`
    ///
    /// Specification: _Nikon_
    pub const NIKO: BrandCode = BrandCode::new(*b"niko");
    /// Non-linear storyline toolset brand
    ///
    /// FourCC: `nlsl`
    ///
    /// Specification: _OMAF_
    pub const NLSL: BrandCode = BrandCode::new(*b"nlsl");
    /// No Leading Picture Sync Brand
    ///
    /// FourCC: `nras`
    ///
    /// Specification: _NALu Video_
    pub const NRAS: BrandCode = BrandCode::new(*b"nras");
    /// OMAF 2D audio legacy profile
    ///
    /// FourCC: `oa2d`
    ///
    /// Specification: _OMAF_
    pub const OA2D: BrandCode = BrandCode::new(*b"oa2d");
    /// OMAF 3D audio baseline profile
    ///
    /// FourCC: `oabl`
    ///
    /// Specification: _OMAF_
    pub const OABL: BrandCode = BrandCode::new(*b"oabl");
    /// OMA DCF (DRM Content Format)
    ///
    /// FourCC: `odcf`
    ///
    /// Specification: _OMA DRM 2.0_
    pub const ODCF: BrandCode = BrandCode::new(*b"odcf");
    /// OMAF viewport-independent baseline presentation profile
    ///
    /// FourCC: `ompp`
    ///
    /// Specification: _OMAF_
    pub const OMPP: BrandCode = BrandCode::new(*b"ompp");
    /// OMA PDCF (DRM Content Format)
    ///
    /// FourCC: `opf2`
    ///
    /// Specification: _OMA DRM 2.1_
    pub const OPF2: BrandCode = BrandCode::new(*b"opf2");
    /// OMA Adapted PDCF
    ///
    /// FourCC: `opx2`
    ///
    /// Specification: _OMA DRM XBS_
    pub const OPX2: BrandCode = BrandCode::new(*b"opx2");
    /// OMAF viewport-dependent baseline presentation profile
    ///
    /// FourCC: `ovdp`
    ///
    /// Specification: _OMAF_
    pub const OVDP: BrandCode = BrandCode::new(*b"ovdp");
    /// Overlay toolset brand
    ///
    /// FourCC: `ovly`
    ///
    /// Specification: _OMAF_
    pub const OVLY: BrandCode = BrandCode::new(*b"ovly");
    /// Generic Partial File
    ///
    /// FourCC: `paff`
    ///
    /// Specification: _ISO-Partial_
    pub const PAFF: BrandCode = BrandCode::new(*b"paff");
    /// Panasonic Digital Camera
    ///
    /// FourCC: `pana`
    ///
    /// Specification: _Panasonic_
    pub const PANA: BrandCode = BrandCode::new(*b"pana");
    /// Protected Interoperable File Format
    ///
    /// FourCC: `piff`
    ///
    /// Specification: _PIFF_
    pub const PIFF: BrandCode = BrandCode::new(*b"piff");
    /// Mixed Partial File
    ///
    /// FourCC: `pmff`
    ///
    /// Specification: _ISO-Partial_
    pub const PMFF: BrandCode = BrandCode::new(*b"pmff");
    /// Panasonic Video Intercom
    ///
    /// FourCC: `pnvi`
    ///
    /// Specification: _Panasonic Video Intercom_
    pub const PNVI: BrandCode = BrandCode::new(*b"pnvi");
    /// Image file format brand for predictively coded image items
    ///
    /// FourCC: `pred`
    ///
    /// Specification: _HEIF_
    pub const PRED: BrandCode = BrandCode::new(*b"pred");
    /// QuickTime
    ///
    /// FourCC: `qt  `
    ///
    /// Specification: _QT_
    pub const QT: BrandCode = BrandCode::new(*b"qt  ");
    /// combination brand to indicate relative addressing
    ///
    /// FourCC: `relo`
    ///
    /// Specification: _ISO_
    pub const RELO: BrandCode = BrandCode::new(*b"relo");
    /// Representation Index Segment used to index MPEG-2 TS based Media Segments.
    ///
    /// FourCC: `risx`
    ///
    /// Specification: _DASH_
    pub const RISX: BrandCode = BrandCode::new(*b"risx");
    /// Ross Video
    ///
    /// FourCC: `ROSS`
    ///
    /// Specification: _Ross_
    pub const ROSS: BrandCode = BrandCode::new(*b"ROSS");
    /// SD Video
    ///
    /// FourCC: `sdv `
    ///
    /// Specification: _SDV_
    pub const SDV: BrandCode = BrandCode::new(*b"sdv ");
    /// Home and Mobile Multimedia Platform (HMMP)
    ///
    /// FourCC: `SEAU`
    ///
    /// Specification: _Sony_
    pub const SEAU: BrandCode = BrandCode::new(*b"SEAU");
    /// Home and Mobile Multimedia Platform (HMMP)
    ///
    /// FourCC: `SEBK`
    ///
    /// Specification: _Sony_
    pub const SEBK: BrandCode = BrandCode::new(*b"SEBK");
    /// Video contents Sony Entertainment Network provides by using MP4 file format
    ///
    /// FourCC: `senv`
    ///
    /// Specification: _Sony_
    pub const SENV: BrandCode = BrandCode::new(*b"senv");
    /// Media Segment conforming to the Sub-Indexed Media Segment format type for ISO base media file format.
    ///
    /// FourCC: `sims`
    ///
    /// Specification: _DASH_
    pub const SIMS: BrandCode = BrandCode::new(*b"sims");
    /// Single Index Segment used to index MPEG-2 TS based Media Segments.
    ///
    /// FourCC: `sisx`
    ///
    /// Specification: _DASH_
    pub const SISX: BrandCode = BrandCode::new(*b"sisx");
    /// HEVC-based simple tiling OMAF video profile
    ///
    /// FourCC: `siti`
    ///
    /// Specification: _OMAF_
    pub const SITI: BrandCode = BrandCode::new(*b"siti");
    /// VVC-based simple tiling OMAF video profile
    ///
    /// FourCC: `sitv`
    ///
    /// Specification: _OMAF_
    pub const SITV: BrandCode = BrandCode::new(*b"sitv");
    /// Dynamic metadata for Single Layer SDR-compatible HDR video streams
    ///
    /// FourCC: `slh1`
    ///
    /// Specification: _SL-HDR_
    pub const SLH1: BrandCode = BrandCode::new(*b"slh1");
    /// Dynamic metadata for Single Layer PQ-based HDR video streams
    ///
    /// FourCC: `slh2`
    ///
    /// Specification: _SL-HDR_
    pub const SLH2: BrandCode = BrandCode::new(*b"slh2");
    /// Dynamic metadata for Single Layer HLG-based HDR video streams
    ///
    /// FourCC: `slh3`
    ///
    /// Specification: _SL-HDR_
    pub const SLH3: BrandCode = BrandCode::new(*b"slh3");
    /// Subsegment Index Segment used to index MPEG-2 TS based Media Segments.
    ///
    /// FourCC: `ssss`
    ///
    /// Specification: _DASH_
    pub const SSSS: BrandCode = BrandCode::new(*b"ssss");
    /// OMAF IMSC1 timed text profile
    ///
    /// FourCC: `ttml`
    ///
    /// Specification: _OMAF_
    pub const TTML: BrandCode = BrandCode::new(*b"ttml");
    /// OMAF WebVTT timed text profile
    ///
    /// FourCC: `ttwv`
    ///
    /// Specification: _OMAF_
    pub const TTWV: BrandCode = BrandCode::new(*b"ttwv");
    /// Unconstrained HEVC-based viewport-independent OMAF video profile
    ///
    /// FourCC: `uhvi`
    ///
    /// Specification: _OMAF_
    pub const UHVI: BrandCode = BrandCode::new(*b"uhvi");
    /// Unified identifiers
    ///
    /// FourCC: `unif`
    ///
    /// Specification: _ISO_
    pub const UNIF: BrandCode = BrandCode::new(*b"unif");
    /// UltraViolet file brand – conforming to the DECE Common File Format spec, Annex E.
    ///
    /// FourCC: `uvvu`
    ///
    /// Specification: _DECE_
    pub const UVVU: BrandCode = BrandCode::new(*b"uvvu");
    /// Multi-track encapsulation mode for V3C data with partial access support
    ///
    /// FourCC: `v3mp`
    ///
    /// Specification: _V3C-SYS_
    pub const V3MP: BrandCode = BrandCode::new(*b"v3mp");
    /// Multi-track encapsulation mode for V3C data
    ///
    /// FourCC: `v3mt`
    ///
    /// Specification: _V3C-SYS_
    pub const V3MT: BrandCode = BrandCode::new(*b"v3mt");
    /// Non-timed encpasulation mode for V3C data
    ///
    /// FourCC: `v3nt`
    ///
    /// Specification: _V3C-SYS_
    pub const V3NT: BrandCode = BrandCode::new(*b"v3nt");
    /// Single-track encapsulation mode for V3C data
    ///
    /// FourCC: `v3st`
    ///
    /// Specification: _V3C-SYS_
    pub const V3ST: BrandCode = BrandCode::new(*b"v3st");
    /// VVC-based viewport-independent OMAF video profile
    ///
    /// FourCC: `vvci`
    ///
    /// Specification: _OMAF_
    pub const VVCI: BrandCode = BrandCode::new(*b"vvci");
    /// VVC coded image item
    ///
    /// FourCC: `vvic`
    ///
    /// Specification: _HEIF_
    pub const VVIC: BrandCode = BrandCode::new(*b"vvic");
    /// VVC coded image sequence
    ///
    /// FourCC: `vvis`
    ///
    /// Specification: _HEIF_
    pub const VVIS: BrandCode = BrandCode::new(*b"vvis");
    /// OMAF VVC image profile
    ///
    /// FourCC: `vvoi`
    ///
    /// Specification: _OMAF_
    pub const VVOI: BrandCode = BrandCode::new(*b"vvoi");
    /// Viewpoint toolset brand
    ///
    /// FourCC: `vwpt`
    ///
    /// Specification: _OMAF_
    pub const VWPT: BrandCode = BrandCode::new(*b"vwpt");
    /// XAVC File Format
    ///
    /// FourCC: `XAVC`
    ///
    /// Specification: _Sony_
    pub const XAVC: BrandCode = BrandCode::new(*b"XAVC");
    /// Google specification for use by YouTube apps
    ///
    /// FourCC: `yt4 `
    ///
    /// Specification: _Youtube_
    pub const YT4: BrandCode = BrandCode::new(*b"yt4 ");
    /// Immersive Audio Model and Formats - Encapsulated IA Sequence
    ///
    /// FourCC: `iamf`
    ///
    /// Specification: _AOM-IAMF_
    pub const IAMF: BrandCode = BrandCode::new(*b"iamf");
    /// Apple HEVC Video with Alpha
    ///
    /// FourCC: `muxa`
    ///
    /// Specification: _Apple HEVC Alpha_
    pub const MUXA: BrandCode = BrandCode::new(*b"muxa");
    /// CMAF Media Profile for Constrained Multilayer VVC
    ///
    /// FourCC: `cvvm`
    ///
    /// Specification: _ATSC 3.0 A345_
    pub const CVVM: BrandCode = BrandCode::new(*b"cvvm");
    /// CMAF Media Profile for ATSC Full Range Constrained Baseline VVC
    ///
    /// FourCC: `cafr`
    ///
    /// Specification: _ATSC 3.0 A345_
    pub const CAFR: BrandCode = BrandCode::new(*b"cafr");
    /// XF-AVC S/XF-HEVC S File Format
    ///
    /// FourCC: `XFVC`
    ///
    /// Specification: _Canon_
    pub const XFVC: BrandCode = BrandCode::new(*b"XFVC");
    /// CMAF Media Profile – AVS3-P3
    ///
    /// FourCC: `ca3a`
    ///
    /// Specification: _T-AI-109.7_
    pub const CA3A: BrandCode = BrandCode::new(*b"ca3a");
    /// CMAF Media Profile for MPEG-5 LCEVC
    ///
    /// FourCC: `clv1`
    ///
    /// Specification: _CMAF_
    pub const CLV1: BrandCode = BrandCode::new(*b"clv1");
    /// Version of the ISO file format
    ///
    /// FourCC: `isod`
    ///
    /// Specification: _ISO_
    pub const ISOD: BrandCode = BrandCode::new(*b"isod");
    /// CMAF EVC Baseline Media Profile
    ///
    /// FourCC: `cevb`
    ///
    /// Specification: _CMAF_
    pub const CEVB: BrandCode = BrandCode::new(*b"cevb");
    /// CMAF EVC Main Media Profile
    ///
    /// FourCC: `cevm`
    ///
    /// Specification: _CMAF_
    pub const CEVM: BrandCode = BrandCode::new(*b"cevm");
    /// CMAF Random Access chunk
    ///
    /// FourCC: `cmfr`
    ///
    /// Specification: _CMAF_
    pub const CMFR: BrandCode = BrandCode::new(*b"cmfr");
    /// MPEG-H audio BL single-stream media profile
    ///
    /// FourCC: `cmh1`
    ///
    /// Specification: _CMAF_
    pub const CMH1: BrandCode = BrandCode::new(*b"cmh1");
    /// MPEG-H audio BL multi-stream media profile
    ///
    /// FourCC: `cmh2`
    ///
    /// Specification: _CMAF_
    pub const CMH2: BrandCode = BrandCode::new(*b"cmh2");
    /// SampleAuxiliaryInformationSizesBox with version 1 and 2
    ///
    /// FourCC: `saie`
    ///
    /// Specification: _ISO_
    pub const SAIE: BrandCode = BrandCode::new(*b"saie");
}
impl TrackReferenceCode {
    /// Additional audio track
    ///
    /// FourCC: `adda`
    ///
    /// Specification: _ISO_
    pub const ADDA: TrackReferenceCode = TrackReferenceCode::new(*b"adda");
    /// DRC metadata track
    ///
    /// FourCC: `adrc`
    ///
    /// Specification: _ISO_
    pub const ADRC: TrackReferenceCode = TrackReferenceCode::new(*b"adrc");
    /// Auxiliary track reference
    ///
    /// FourCC: `auxl`
    ///
    /// Specification: _ISO_
    pub const AUXL: TrackReferenceCode = TrackReferenceCode::new(*b"auxl");
    /// AVC parameter set stream link
    ///
    /// FourCC: `avcp`
    ///
    /// Specification: _NALu Video_
    pub const AVCP: TrackReferenceCode = TrackReferenceCode::new(*b"avcp");
    /// this track describes the referenced track.
    ///
    /// FourCC: `cdsc`
    ///
    /// Specification: _ISO_
    pub const CDSC: TrackReferenceCode = TrackReferenceCode::new(*b"cdsc");
    /// this track describes the referenced tracks and track groups collectively
    ///
    /// FourCC: `cdtg`
    ///
    /// Specification: _OMAF_
    pub const CDTG: TrackReferenceCode = TrackReferenceCode::new(*b"cdtg");
    /// track containing the depth view
    ///
    /// FourCC: `deps`
    ///
    /// Specification: _NALu Video_
    pub const DEPS: TrackReferenceCode = TrackReferenceCode::new(*b"deps");
    /// this track has an MPEG-4 dependency on the referenced track
    ///
    /// FourCC: `dpnd`
    ///
    /// Specification: _MP4v2_
    pub const DPND: TrackReferenceCode = TrackReferenceCode::new(*b"dpnd");
    /// EVC slice base track
    ///
    /// FourCC: `evcr`
    ///
    /// Specification: _NALu Video_
    pub const EVCR: TrackReferenceCode = TrackReferenceCode::new(*b"evcr");
    /// this track uses fonts carried/defined in the referenced track
    ///
    /// FourCC: `font`
    ///
    /// Specification: _ISO_
    pub const FONT: TrackReferenceCode = TrackReferenceCode::new(*b"font");
    /// Hint dependency
    ///
    /// FourCC: `hind`
    ///
    /// Specification: _ISO_
    pub const HIND: TrackReferenceCode = TrackReferenceCode::new(*b"hind");
    /// links hint track to original media track
    ///
    /// FourCC: `hint`
    ///
    /// Specification: _ISO_
    pub const HINT: TrackReferenceCode = TrackReferenceCode::new(*b"hint");
    /// this track contains IPI declarations for the referenced track
    ///
    /// FourCC: `ipir`
    ///
    /// Specification: _MP4v2_
    pub const IPIR: TrackReferenceCode = TrackReferenceCode::new(*b"ipir");
    /// Audio layer track dependency
    ///
    /// FourCC: `lyra`
    ///
    /// Specification: _DTS_
    pub const LYRA: TrackReferenceCode = TrackReferenceCode::new(*b"lyra");
    /// used in indicating combinations that result into mixed network abstraction layer unit types in a coded picture of VVC
    ///
    /// FourCC: `mixn`
    ///
    /// Specification: _NALu Video_
    pub const MIXN: TrackReferenceCode = TrackReferenceCode::new(*b"mixn");
    /// this track is an OD track which uses the referenced track as an included elementary stream track
    ///
    /// FourCC: `mpod`
    ///
    /// Specification: _MP4v2_
    pub const MPOD: TrackReferenceCode = TrackReferenceCode::new(*b"mpod");
    /// track that contains an 'oref' sample group
    ///
    /// FourCC: `oref`
    ///
    /// Specification: _NALu Video_
    pub const OREF: TrackReferenceCode = TrackReferenceCode::new(*b"oref");
    /// resolved by extracting an indicated subset of the referenced VVC track to reconstruct a VVC bitstream
    ///
    /// FourCC: `recr`
    ///
    /// Specification: _NALu Video_
    pub const RECR: TrackReferenceCode = TrackReferenceCode::new(*b"recr");
    /// HEVC Tile Track
    ///
    /// FourCC: `sabt`
    ///
    /// Specification: _NALu Video_
    pub const SABT: TrackReferenceCode = TrackReferenceCode::new(*b"sabt");
    /// Scalable base
    ///
    /// FourCC: `sbas`
    ///
    /// Specification: _NALu Video_
    pub const SBAS: TrackReferenceCode = TrackReferenceCode::new(*b"sbas");
    /// Scalable extraction
    ///
    /// FourCC: `scal`
    ///
    /// Specification: _NALu Video_
    pub const SCAL: TrackReferenceCode = TrackReferenceCode::new(*b"scal");
    /// reference to a shadow sync sample track
    ///
    /// FourCC: `shsc`
    ///
    /// Specification: _OMAF_
    pub const SHSC: TrackReferenceCode = TrackReferenceCode::new(*b"shsc");
    /// the referenced VVC subpicture tracks or 'alte' track groups of VVC subpicture tracks are used to reconstruct a VVC bitstream
    ///
    /// FourCC: `subp`
    ///
    /// Specification: _NALu Video_
    pub const SUBP: TrackReferenceCode = TrackReferenceCode::new(*b"subp");
    /// subtitle or timed text or overlay graphical information
    ///
    /// FourCC: `subt`
    ///
    /// Specification: _ISO_
    pub const SUBT: TrackReferenceCode = TrackReferenceCode::new(*b"subt");
    /// AVC Switch from
    ///
    /// FourCC: `swfr`
    ///
    /// Specification: _NALu Video_
    pub const SWFR: TrackReferenceCode = TrackReferenceCode::new(*b"swfr");
    /// AVC Switch to
    ///
    /// FourCC: `swto`
    ///
    /// Specification: _NALu Video_
    pub const SWTO: TrackReferenceCode = TrackReferenceCode::new(*b"swto");
    /// this track uses the referenced track as its synchronization source.
    ///
    /// FourCC: `sync`
    ///
    /// Specification: _MP4v2_
    pub const SYNC: TrackReferenceCode = TrackReferenceCode::new(*b"sync");
    /// HEVC Tile track base
    ///
    /// FourCC: `tbas`
    ///
    /// Specification: _NALu Video_
    pub const TBAS: TrackReferenceCode = TrackReferenceCode::new(*b"tbas");
    /// Thumbnail track reference
    ///
    /// FourCC: `thmb`
    ///
    /// Specification: _ISO_
    pub const THMB: TrackReferenceCode = TrackReferenceCode::new(*b"thmb");
    /// Time code. Usually references a time code track.
    ///
    /// FourCC: `tmcd`
    ///
    /// Specification: _Apple_
    pub const TMCD: TrackReferenceCode = TrackReferenceCode::new(*b"tmcd");
    /// V3C atlas track
    ///
    /// FourCC: `v3cs`
    ///
    /// Specification: _V3C-SYS_
    pub const V3CS: TrackReferenceCode = TrackReferenceCode::new(*b"v3cs");
    /// V3C atlas tile track
    ///
    /// FourCC: `v3ct`
    ///
    /// Specification: _V3C-SYS_
    pub const V3CT: TrackReferenceCode = TrackReferenceCode::new(*b"v3ct");
    /// V3C attribute video track
    ///
    /// FourCC: `v3va`
    ///
    /// Specification: _V3C-SYS_
    pub const V3VA: TrackReferenceCode = TrackReferenceCode::new(*b"v3va");
    /// V3C geometry video track
    ///
    /// FourCC: `v3vg`
    ///
    /// Specification: _V3C-SYS_
    pub const V3VG: TrackReferenceCode = TrackReferenceCode::new(*b"v3vg");
    /// V3C occupancy video track
    ///
    /// FourCC: `v3vo`
    ///
    /// Specification: _V3C-SYS_
    pub const V3VO: TrackReferenceCode = TrackReferenceCode::new(*b"v3vo");
    /// Auxiliary video depth
    ///
    /// FourCC: `vdep`
    ///
    /// Specification: _ISO_
    pub const VDEP: TrackReferenceCode = TrackReferenceCode::new(*b"vdep");
    /// Auxiliary video parallax
    ///
    /// FourCC: `vplx`
    ///
    /// Specification: _ISO_
    pub const VPLX: TrackReferenceCode = TrackReferenceCode::new(*b"vplx");
    /// reference to a track that contains a 'vopi' sample group for VVC video
    ///
    /// FourCC: `vref`
    ///
    /// Specification: _NALu Video_
    pub const VREF: TrackReferenceCode = TrackReferenceCode::new(*b"vref");
    /// reference to a VVC operating point entity group
    ///
    /// FourCC: `vreg`
    ///
    /// Specification: _NALu Video_
    pub const VREG: TrackReferenceCode = TrackReferenceCode::new(*b"vreg");
    /// the referenced track is a non video coding layer track of VVC
    ///
    /// FourCC: `vvcN`
    ///
    /// Specification: _NALu Video_
    pub const VVCN: TrackReferenceCode = TrackReferenceCode::new(*b"vvcN");
    /// The referenced tracks provide supplementary video to achieve picture-in-picture functionality
    ///
    /// FourCC: `supm`
    ///
    /// Specification: _NALu Video_
    pub const SUPM: TrackReferenceCode = TrackReferenceCode::new(*b"supm");
    /// associated external stream track
    ///
    /// FourCC: `aest`
    ///
    /// Specification: _ISO_
    pub const AEST: TrackReferenceCode = TrackReferenceCode::new(*b"aest");
    /// Rendering-related metadata
    ///
    /// FourCC: `rndr`
    ///
    /// Specification: _Apple_
    pub const RNDR: TrackReferenceCode = TrackReferenceCode::new(*b"rndr");
}
