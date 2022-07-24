#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc PA00_SEL"]
    pub pa00_sel: crate::Reg<pa00_sel::PA00_SEL_SPEC>,
    #[doc = "0x04 - desc PA01_SEL"]
    pub pa01_sel: crate::Reg<pa01_sel::PA01_SEL_SPEC>,
    #[doc = "0x08 - desc PA02_SEL"]
    pub pa02_sel: crate::Reg<pa02_sel::PA02_SEL_SPEC>,
    #[doc = "0x0c - desc PA03_SEL"]
    pub pa03_sel: crate::Reg<pa03_sel::PA03_SEL_SPEC>,
    #[doc = "0x10 - desc PA04_SEL"]
    pub pa04_sel: crate::Reg<pa04_sel::PA04_SEL_SPEC>,
    #[doc = "0x14 - desc PA05_SEL"]
    pub pa05_sel: crate::Reg<pa05_sel::PA05_SEL_SPEC>,
    #[doc = "0x18 - desc PA06_SEL"]
    pub pa06_sel: crate::Reg<pa06_sel::PA06_SEL_SPEC>,
    #[doc = "0x1c - desc PA07_SEL"]
    pub pa07_sel: crate::Reg<pa07_sel::PA07_SEL_SPEC>,
    #[doc = "0x20 - desc PA08_SEL"]
    pub pa08_sel: crate::Reg<pa08_sel::PA08_SEL_SPEC>,
    #[doc = "0x24 - desc PA09_SEL"]
    pub pa09_sel: crate::Reg<pa09_sel::PA09_SEL_SPEC>,
    #[doc = "0x28 - desc PA10_SEL"]
    pub pa10_sel: crate::Reg<pa10_sel::PA10_SEL_SPEC>,
    #[doc = "0x2c - desc PA11_SEL"]
    pub pa11_sel: crate::Reg<pa11_sel::PA11_SEL_SPEC>,
    #[doc = "0x30 - desc PA12_SEL"]
    pub pa12_sel: crate::Reg<pa12_sel::PA12_SEL_SPEC>,
    #[doc = "0x34 - desc PA13_SEL"]
    pub pa13_sel: crate::Reg<pa13_sel::PA13_SEL_SPEC>,
    #[doc = "0x38 - desc PA14_SEL"]
    pub pa14_sel: crate::Reg<pa14_sel::PA14_SEL_SPEC>,
    #[doc = "0x3c - desc PA15_SEL"]
    pub pa15_sel: crate::Reg<pa15_sel::PA15_SEL_SPEC>,
    #[doc = "0x40 - desc PB00_SEL"]
    pub pb00_sel: crate::Reg<pb00_sel::PB00_SEL_SPEC>,
    #[doc = "0x44 - desc PB01_SEL"]
    pub pb01_sel: crate::Reg<pb01_sel::PB01_SEL_SPEC>,
    #[doc = "0x48 - desc PB02_SEL"]
    pub pb02_sel: crate::Reg<pb02_sel::PB02_SEL_SPEC>,
    #[doc = "0x4c - desc PB03_SEL"]
    pub pb03_sel: crate::Reg<pb03_sel::PB03_SEL_SPEC>,
    #[doc = "0x50 - desc PB04_SEL"]
    pub pb04_sel: crate::Reg<pb04_sel::PB04_SEL_SPEC>,
    #[doc = "0x54 - desc PB05_SEL"]
    pub pb05_sel: crate::Reg<pb05_sel::PB05_SEL_SPEC>,
    #[doc = "0x58 - desc PB06_SEL"]
    pub pb06_sel: crate::Reg<pb06_sel::PB06_SEL_SPEC>,
    #[doc = "0x5c - desc PB07_SEL"]
    pub pb07_sel: crate::Reg<pb07_sel::PB07_SEL_SPEC>,
    #[doc = "0x60 - desc PB08_SEL"]
    pub pb08_sel: crate::Reg<pb08_sel::PB08_SEL_SPEC>,
    #[doc = "0x64 - desc PB09_SEL"]
    pub pb09_sel: crate::Reg<pb09_sel::PB09_SEL_SPEC>,
    #[doc = "0x68 - desc PB10_SEL"]
    pub pb10_sel: crate::Reg<pb10_sel::PB10_SEL_SPEC>,
    #[doc = "0x6c - desc PB11_SEL"]
    pub pb11_sel: crate::Reg<pb11_sel::PB11_SEL_SPEC>,
    #[doc = "0x70 - desc PB12_SEL"]
    pub pb12_sel: crate::Reg<pb12_sel::PB12_SEL_SPEC>,
    #[doc = "0x74 - desc PB13_SEL"]
    pub pb13_sel: crate::Reg<pb13_sel::PB13_SEL_SPEC>,
    #[doc = "0x78 - desc PB14_SEL"]
    pub pb14_sel: crate::Reg<pb14_sel::PB14_SEL_SPEC>,
    #[doc = "0x7c - desc PB15_SEL"]
    pub pb15_sel: crate::Reg<pb15_sel::PB15_SEL_SPEC>,
    #[doc = "0x80 - desc PC00_SEL"]
    pub pc00_sel: crate::Reg<pc00_sel::PC00_SEL_SPEC>,
    #[doc = "0x84 - desc PC01_SEL"]
    pub pc01_sel: crate::Reg<pc01_sel::PC01_SEL_SPEC>,
    #[doc = "0x88 - desc PC02_SEL"]
    pub pc02_sel: crate::Reg<pc02_sel::PC02_SEL_SPEC>,
    #[doc = "0x8c - desc PC03_SEL"]
    pub pc03_sel: crate::Reg<pc03_sel::PC03_SEL_SPEC>,
    #[doc = "0x90 - desc PC04_SEL"]
    pub pc04_sel: crate::Reg<pc04_sel::PC04_SEL_SPEC>,
    #[doc = "0x94 - desc PC05_SEL"]
    pub pc05_sel: crate::Reg<pc05_sel::PC05_SEL_SPEC>,
    #[doc = "0x98 - desc PC06_SEL"]
    pub pc06_sel: crate::Reg<pc06_sel::PC06_SEL_SPEC>,
    #[doc = "0x9c - desc PC07_SEL"]
    pub pc07_sel: crate::Reg<pc07_sel::PC07_SEL_SPEC>,
    #[doc = "0xa0 - desc PC08_SEL"]
    pub pc08_sel: crate::Reg<pc08_sel::PC08_SEL_SPEC>,
    #[doc = "0xa4 - desc PC09_SEL"]
    pub pc09_sel: crate::Reg<pc09_sel::PC09_SEL_SPEC>,
    #[doc = "0xa8 - desc PC10_SEL"]
    pub pc10_sel: crate::Reg<pc10_sel::PC10_SEL_SPEC>,
    #[doc = "0xac - desc PC11_SEL"]
    pub pc11_sel: crate::Reg<pc11_sel::PC11_SEL_SPEC>,
    #[doc = "0xb0 - desc PC12_SEL"]
    pub pc12_sel: crate::Reg<pc12_sel::PC12_SEL_SPEC>,
    #[doc = "0xb4 - desc PC13_SEL"]
    pub pc13_sel: crate::Reg<pc13_sel::PC13_SEL_SPEC>,
    #[doc = "0xb8 - desc PC14_SEL"]
    pub pc14_sel: crate::Reg<pc14_sel::PC14_SEL_SPEC>,
    #[doc = "0xbc - desc PC15_SEL"]
    pub pc15_sel: crate::Reg<pc15_sel::PC15_SEL_SPEC>,
    #[doc = "0xc0 - desc PD00_SEL"]
    pub pd00_sel: crate::Reg<pd00_sel::PD00_SEL_SPEC>,
    #[doc = "0xc4 - desc PD01_SEL"]
    pub pd01_sel: crate::Reg<pd01_sel::PD01_SEL_SPEC>,
    #[doc = "0xc8 - desc PD02_SEL"]
    pub pd02_sel: crate::Reg<pd02_sel::PD02_SEL_SPEC>,
    #[doc = "0xcc - desc PD03_SEL"]
    pub pd03_sel: crate::Reg<pd03_sel::PD03_SEL_SPEC>,
    #[doc = "0xd0 - desc PD04_SEL"]
    pub pd04_sel: crate::Reg<pd04_sel::PD04_SEL_SPEC>,
    #[doc = "0xd4 - desc PD05_SEL"]
    pub pd05_sel: crate::Reg<pd05_sel::PD05_SEL_SPEC>,
    #[doc = "0xd8 - desc PD06_SEL"]
    pub pd06_sel: crate::Reg<pd06_sel::PD06_SEL_SPEC>,
    #[doc = "0xdc - desc PD07_SEL"]
    pub pd07_sel: crate::Reg<pd07_sel::PD07_SEL_SPEC>,
    _reserved56: [u8; 0x20],
    #[doc = "0x100 - desc PADIR"]
    pub padir: crate::Reg<padir::PADIR_SPEC>,
    #[doc = "0x104 - desc PAIN"]
    pub pain: crate::Reg<pain::PAIN_SPEC>,
    #[doc = "0x108 - desc PAOUT"]
    pub paout: crate::Reg<paout::PAOUT_SPEC>,
    #[doc = "0x10c - desc PAADS"]
    pub paads: crate::Reg<paads::PAADS_SPEC>,
    #[doc = "0x110 - desc PABSET"]
    pub pabset: crate::Reg<pabset::PABSET_SPEC>,
    #[doc = "0x114 - desc PABCLR"]
    pub pabclr: crate::Reg<pabclr::PABCLR_SPEC>,
    #[doc = "0x118 - desc PABSETCLR"]
    pub pabsetclr: crate::Reg<pabsetclr::PABSETCLR_SPEC>,
    #[doc = "0x11c - desc PADR"]
    pub padr: crate::Reg<padr::PADR_SPEC>,
    #[doc = "0x120 - desc PAPU"]
    pub papu: crate::Reg<papu::PAPU_SPEC>,
    #[doc = "0x124 - desc PAPD"]
    pub papd: crate::Reg<papd::PAPD_SPEC>,
    _reserved66: [u8; 0x04],
    #[doc = "0x12c - desc PAOD"]
    pub paod: crate::Reg<paod::PAOD_SPEC>,
    #[doc = "0x130 - desc PAHIE"]
    pub pahie: crate::Reg<pahie::PAHIE_SPEC>,
    #[doc = "0x134 - desc PALIE"]
    pub palie: crate::Reg<palie::PALIE_SPEC>,
    #[doc = "0x138 - desc PARIE"]
    pub parie: crate::Reg<parie::PARIE_SPEC>,
    #[doc = "0x13c - desc PAFIE"]
    pub pafie: crate::Reg<pafie::PAFIE_SPEC>,
    #[doc = "0x140 - desc PBDIR"]
    pub pbdir: crate::Reg<pbdir::PBDIR_SPEC>,
    #[doc = "0x144 - desc PBIN"]
    pub pbin: crate::Reg<pbin::PBIN_SPEC>,
    #[doc = "0x148 - desc PBOUT"]
    pub pbout: crate::Reg<pbout::PBOUT_SPEC>,
    #[doc = "0x14c - desc PBADS"]
    pub pbads: crate::Reg<pbads::PBADS_SPEC>,
    #[doc = "0x150 - desc PBBSET"]
    pub pbbset: crate::Reg<pbbset::PBBSET_SPEC>,
    #[doc = "0x154 - desc PBBCLR"]
    pub pbbclr: crate::Reg<pbbclr::PBBCLR_SPEC>,
    #[doc = "0x158 - desc PBBSETCLR"]
    pub pbbsetclr: crate::Reg<pbbsetclr::PBBSETCLR_SPEC>,
    #[doc = "0x15c - desc PBDR"]
    pub pbdr: crate::Reg<pbdr::PBDR_SPEC>,
    #[doc = "0x160 - desc PBPU"]
    pub pbpu: crate::Reg<pbpu::PBPU_SPEC>,
    #[doc = "0x164 - desc PBPD"]
    pub pbpd: crate::Reg<pbpd::PBPD_SPEC>,
    _reserved81: [u8; 0x04],
    #[doc = "0x16c - desc PBOD"]
    pub pbod: crate::Reg<pbod::PBOD_SPEC>,
    #[doc = "0x170 - desc PBHIE"]
    pub pbhie: crate::Reg<pbhie::PBHIE_SPEC>,
    #[doc = "0x174 - desc PBLIE"]
    pub pblie: crate::Reg<pblie::PBLIE_SPEC>,
    #[doc = "0x178 - desc PBRIE"]
    pub pbrie: crate::Reg<pbrie::PBRIE_SPEC>,
    #[doc = "0x17c - desc PBFIE"]
    pub pbfie: crate::Reg<pbfie::PBFIE_SPEC>,
    #[doc = "0x180 - desc PCDIR"]
    pub pcdir: crate::Reg<pcdir::PCDIR_SPEC>,
    #[doc = "0x184 - desc PCIN"]
    pub pcin: crate::Reg<pcin::PCIN_SPEC>,
    #[doc = "0x188 - desc PCOUT"]
    pub pcout: crate::Reg<pcout::PCOUT_SPEC>,
    #[doc = "0x18c - desc PCADS"]
    pub pcads: crate::Reg<pcads::PCADS_SPEC>,
    #[doc = "0x190 - desc PCBSET"]
    pub pcbset: crate::Reg<pcbset::PCBSET_SPEC>,
    #[doc = "0x194 - desc PCBCLR"]
    pub pcbclr: crate::Reg<pcbclr::PCBCLR_SPEC>,
    #[doc = "0x198 - desc PCBSETCLR"]
    pub pcbsetclr: crate::Reg<pcbsetclr::PCBSETCLR_SPEC>,
    #[doc = "0x19c - desc PCDR"]
    pub pcdr: crate::Reg<pcdr::PCDR_SPEC>,
    #[doc = "0x1a0 - desc PCPU"]
    pub pcpu: crate::Reg<pcpu::PCPU_SPEC>,
    #[doc = "0x1a4 - desc PCPD"]
    pub pcpd: crate::Reg<pcpd::PCPD_SPEC>,
    _reserved96: [u8; 0x04],
    #[doc = "0x1ac - desc PCOD"]
    pub pcod: crate::Reg<pcod::PCOD_SPEC>,
    #[doc = "0x1b0 - desc PCHIE"]
    pub pchie: crate::Reg<pchie::PCHIE_SPEC>,
    #[doc = "0x1b4 - desc PCLIE"]
    pub pclie: crate::Reg<pclie::PCLIE_SPEC>,
    #[doc = "0x1b8 - desc PCRIE"]
    pub pcrie: crate::Reg<pcrie::PCRIE_SPEC>,
    #[doc = "0x1bc - desc PCFIE"]
    pub pcfie: crate::Reg<pcfie::PCFIE_SPEC>,
    #[doc = "0x1c0 - desc PDDIR"]
    pub pddir: crate::Reg<pddir::PDDIR_SPEC>,
    #[doc = "0x1c4 - desc PDIN"]
    pub pdin: crate::Reg<pdin::PDIN_SPEC>,
    #[doc = "0x1c8 - desc PDOUT"]
    pub pdout: crate::Reg<pdout::PDOUT_SPEC>,
    #[doc = "0x1cc - desc PDADS"]
    pub pdads: crate::Reg<pdads::PDADS_SPEC>,
    #[doc = "0x1d0 - desc PDBSET"]
    pub pdbset: crate::Reg<pdbset::PDBSET_SPEC>,
    #[doc = "0x1d4 - desc PDBCLR"]
    pub pdbclr: crate::Reg<pdbclr::PDBCLR_SPEC>,
    #[doc = "0x1d8 - desc PDBSETCLR"]
    pub pdbsetclr: crate::Reg<pdbsetclr::PDBSETCLR_SPEC>,
    #[doc = "0x1dc - desc PDDR"]
    pub pddr: crate::Reg<pddr::PDDR_SPEC>,
    #[doc = "0x1e0 - desc PDPU"]
    pub pdpu: crate::Reg<pdpu::PDPU_SPEC>,
    #[doc = "0x1e4 - desc PDPD"]
    pub pdpd: crate::Reg<pdpd::PDPD_SPEC>,
    _reserved111: [u8; 0x04],
    #[doc = "0x1ec - desc PDOD"]
    pub pdod: crate::Reg<pdod::PDOD_SPEC>,
    #[doc = "0x1f0 - desc PDHIE"]
    pub pdhie: crate::Reg<pdhie::PDHIE_SPEC>,
    #[doc = "0x1f4 - desc PDLIE"]
    pub pdlie: crate::Reg<pdlie::PDLIE_SPEC>,
    #[doc = "0x1f8 - desc PDRIE"]
    pub pdrie: crate::Reg<pdrie::PDRIE_SPEC>,
    #[doc = "0x1fc - desc PDFIE"]
    pub pdfie: crate::Reg<pdfie::PDFIE_SPEC>,
    #[doc = "0x200 - desc PA_STAT"]
    pub pa_stat: crate::Reg<pa_stat::PA_STAT_SPEC>,
    _reserved117: [u8; 0x0c],
    #[doc = "0x210 - desc PA_ICLR"]
    pub pa_iclr: crate::Reg<pa_iclr::PA_ICLR_SPEC>,
    _reserved118: [u8; 0x2c],
    #[doc = "0x240 - desc PB_STAT"]
    pub pb_stat: crate::Reg<pb_stat::PB_STAT_SPEC>,
    _reserved119: [u8; 0x0c],
    #[doc = "0x250 - desc PB_ICLR"]
    pub pb_iclr: crate::Reg<pb_iclr::PB_ICLR_SPEC>,
    _reserved120: [u8; 0x2c],
    #[doc = "0x280 - desc PC_STAT"]
    pub pc_stat: crate::Reg<pc_stat::PC_STAT_SPEC>,
    _reserved121: [u8; 0x0c],
    #[doc = "0x290 - desc PC_ICLR"]
    pub pc_iclr: crate::Reg<pc_iclr::PC_ICLR_SPEC>,
    _reserved122: [u8; 0x2c],
    #[doc = "0x2c0 - desc PD_STAT"]
    pub pd_stat: crate::Reg<pd_stat::PD_STAT_SPEC>,
    _reserved123: [u8; 0x0c],
    #[doc = "0x2d0 - desc PD_ICLR"]
    pub pd_iclr: crate::Reg<pd_iclr::PD_ICLR_SPEC>,
    _reserved124: [u8; 0x2c],
    #[doc = "0x300 - desc CTRL0"]
    pub ctrl0: crate::Reg<ctrl0::CTRL0_SPEC>,
    #[doc = "0x304 - desc CTRL1"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x308 - desc CTRL2"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x30c - desc TIMGS"]
    pub timgs: crate::Reg<timgs::TIMGS_SPEC>,
    #[doc = "0x310 - desc TIMES"]
    pub times: crate::Reg<times::TIMES_SPEC>,
    #[doc = "0x314 - desc TIMCPS"]
    pub timcps: crate::Reg<timcps::TIMCPS_SPEC>,
    #[doc = "0x318 - desc PCAS"]
    pub pcas: crate::Reg<pcas::PCAS_SPEC>,
}
#[doc = "PA00_SEL register accessor: an alias for `Reg<PA00_SEL_SPEC>`"]
pub type PA00_SEL = crate::Reg<pa00_sel::PA00_SEL_SPEC>;
#[doc = "desc PA00_SEL"]
pub mod pa00_sel;
#[doc = "PA01_SEL register accessor: an alias for `Reg<PA01_SEL_SPEC>`"]
pub type PA01_SEL = crate::Reg<pa01_sel::PA01_SEL_SPEC>;
#[doc = "desc PA01_SEL"]
pub mod pa01_sel;
#[doc = "PA02_SEL register accessor: an alias for `Reg<PA02_SEL_SPEC>`"]
pub type PA02_SEL = crate::Reg<pa02_sel::PA02_SEL_SPEC>;
#[doc = "desc PA02_SEL"]
pub mod pa02_sel;
#[doc = "PA03_SEL register accessor: an alias for `Reg<PA03_SEL_SPEC>`"]
pub type PA03_SEL = crate::Reg<pa03_sel::PA03_SEL_SPEC>;
#[doc = "desc PA03_SEL"]
pub mod pa03_sel;
#[doc = "PA04_SEL register accessor: an alias for `Reg<PA04_SEL_SPEC>`"]
pub type PA04_SEL = crate::Reg<pa04_sel::PA04_SEL_SPEC>;
#[doc = "desc PA04_SEL"]
pub mod pa04_sel;
#[doc = "PA05_SEL register accessor: an alias for `Reg<PA05_SEL_SPEC>`"]
pub type PA05_SEL = crate::Reg<pa05_sel::PA05_SEL_SPEC>;
#[doc = "desc PA05_SEL"]
pub mod pa05_sel;
#[doc = "PA06_SEL register accessor: an alias for `Reg<PA06_SEL_SPEC>`"]
pub type PA06_SEL = crate::Reg<pa06_sel::PA06_SEL_SPEC>;
#[doc = "desc PA06_SEL"]
pub mod pa06_sel;
#[doc = "PA07_SEL register accessor: an alias for `Reg<PA07_SEL_SPEC>`"]
pub type PA07_SEL = crate::Reg<pa07_sel::PA07_SEL_SPEC>;
#[doc = "desc PA07_SEL"]
pub mod pa07_sel;
#[doc = "PA08_SEL register accessor: an alias for `Reg<PA08_SEL_SPEC>`"]
pub type PA08_SEL = crate::Reg<pa08_sel::PA08_SEL_SPEC>;
#[doc = "desc PA08_SEL"]
pub mod pa08_sel;
#[doc = "PA09_SEL register accessor: an alias for `Reg<PA09_SEL_SPEC>`"]
pub type PA09_SEL = crate::Reg<pa09_sel::PA09_SEL_SPEC>;
#[doc = "desc PA09_SEL"]
pub mod pa09_sel;
#[doc = "PA10_SEL register accessor: an alias for `Reg<PA10_SEL_SPEC>`"]
pub type PA10_SEL = crate::Reg<pa10_sel::PA10_SEL_SPEC>;
#[doc = "desc PA10_SEL"]
pub mod pa10_sel;
#[doc = "PA11_SEL register accessor: an alias for `Reg<PA11_SEL_SPEC>`"]
pub type PA11_SEL = crate::Reg<pa11_sel::PA11_SEL_SPEC>;
#[doc = "desc PA11_SEL"]
pub mod pa11_sel;
#[doc = "PA12_SEL register accessor: an alias for `Reg<PA12_SEL_SPEC>`"]
pub type PA12_SEL = crate::Reg<pa12_sel::PA12_SEL_SPEC>;
#[doc = "desc PA12_SEL"]
pub mod pa12_sel;
#[doc = "PA13_SEL register accessor: an alias for `Reg<PA13_SEL_SPEC>`"]
pub type PA13_SEL = crate::Reg<pa13_sel::PA13_SEL_SPEC>;
#[doc = "desc PA13_SEL"]
pub mod pa13_sel;
#[doc = "PA14_SEL register accessor: an alias for `Reg<PA14_SEL_SPEC>`"]
pub type PA14_SEL = crate::Reg<pa14_sel::PA14_SEL_SPEC>;
#[doc = "desc PA14_SEL"]
pub mod pa14_sel;
#[doc = "PA15_SEL register accessor: an alias for `Reg<PA15_SEL_SPEC>`"]
pub type PA15_SEL = crate::Reg<pa15_sel::PA15_SEL_SPEC>;
#[doc = "desc PA15_SEL"]
pub mod pa15_sel;
#[doc = "PB00_SEL register accessor: an alias for `Reg<PB00_SEL_SPEC>`"]
pub type PB00_SEL = crate::Reg<pb00_sel::PB00_SEL_SPEC>;
#[doc = "desc PB00_SEL"]
pub mod pb00_sel;
#[doc = "PB01_SEL register accessor: an alias for `Reg<PB01_SEL_SPEC>`"]
pub type PB01_SEL = crate::Reg<pb01_sel::PB01_SEL_SPEC>;
#[doc = "desc PB01_SEL"]
pub mod pb01_sel;
#[doc = "PB02_SEL register accessor: an alias for `Reg<PB02_SEL_SPEC>`"]
pub type PB02_SEL = crate::Reg<pb02_sel::PB02_SEL_SPEC>;
#[doc = "desc PB02_SEL"]
pub mod pb02_sel;
#[doc = "PB03_SEL register accessor: an alias for `Reg<PB03_SEL_SPEC>`"]
pub type PB03_SEL = crate::Reg<pb03_sel::PB03_SEL_SPEC>;
#[doc = "desc PB03_SEL"]
pub mod pb03_sel;
#[doc = "PB04_SEL register accessor: an alias for `Reg<PB04_SEL_SPEC>`"]
pub type PB04_SEL = crate::Reg<pb04_sel::PB04_SEL_SPEC>;
#[doc = "desc PB04_SEL"]
pub mod pb04_sel;
#[doc = "PB05_SEL register accessor: an alias for `Reg<PB05_SEL_SPEC>`"]
pub type PB05_SEL = crate::Reg<pb05_sel::PB05_SEL_SPEC>;
#[doc = "desc PB05_SEL"]
pub mod pb05_sel;
#[doc = "PB06_SEL register accessor: an alias for `Reg<PB06_SEL_SPEC>`"]
pub type PB06_SEL = crate::Reg<pb06_sel::PB06_SEL_SPEC>;
#[doc = "desc PB06_SEL"]
pub mod pb06_sel;
#[doc = "PB07_SEL register accessor: an alias for `Reg<PB07_SEL_SPEC>`"]
pub type PB07_SEL = crate::Reg<pb07_sel::PB07_SEL_SPEC>;
#[doc = "desc PB07_SEL"]
pub mod pb07_sel;
#[doc = "PB08_SEL register accessor: an alias for `Reg<PB08_SEL_SPEC>`"]
pub type PB08_SEL = crate::Reg<pb08_sel::PB08_SEL_SPEC>;
#[doc = "desc PB08_SEL"]
pub mod pb08_sel;
#[doc = "PB09_SEL register accessor: an alias for `Reg<PB09_SEL_SPEC>`"]
pub type PB09_SEL = crate::Reg<pb09_sel::PB09_SEL_SPEC>;
#[doc = "desc PB09_SEL"]
pub mod pb09_sel;
#[doc = "PB10_SEL register accessor: an alias for `Reg<PB10_SEL_SPEC>`"]
pub type PB10_SEL = crate::Reg<pb10_sel::PB10_SEL_SPEC>;
#[doc = "desc PB10_SEL"]
pub mod pb10_sel;
#[doc = "PB11_SEL register accessor: an alias for `Reg<PB11_SEL_SPEC>`"]
pub type PB11_SEL = crate::Reg<pb11_sel::PB11_SEL_SPEC>;
#[doc = "desc PB11_SEL"]
pub mod pb11_sel;
#[doc = "PB12_SEL register accessor: an alias for `Reg<PB12_SEL_SPEC>`"]
pub type PB12_SEL = crate::Reg<pb12_sel::PB12_SEL_SPEC>;
#[doc = "desc PB12_SEL"]
pub mod pb12_sel;
#[doc = "PB13_SEL register accessor: an alias for `Reg<PB13_SEL_SPEC>`"]
pub type PB13_SEL = crate::Reg<pb13_sel::PB13_SEL_SPEC>;
#[doc = "desc PB13_SEL"]
pub mod pb13_sel;
#[doc = "PB14_SEL register accessor: an alias for `Reg<PB14_SEL_SPEC>`"]
pub type PB14_SEL = crate::Reg<pb14_sel::PB14_SEL_SPEC>;
#[doc = "desc PB14_SEL"]
pub mod pb14_sel;
#[doc = "PB15_SEL register accessor: an alias for `Reg<PB15_SEL_SPEC>`"]
pub type PB15_SEL = crate::Reg<pb15_sel::PB15_SEL_SPEC>;
#[doc = "desc PB15_SEL"]
pub mod pb15_sel;
#[doc = "PC00_SEL register accessor: an alias for `Reg<PC00_SEL_SPEC>`"]
pub type PC00_SEL = crate::Reg<pc00_sel::PC00_SEL_SPEC>;
#[doc = "desc PC00_SEL"]
pub mod pc00_sel;
#[doc = "PC01_SEL register accessor: an alias for `Reg<PC01_SEL_SPEC>`"]
pub type PC01_SEL = crate::Reg<pc01_sel::PC01_SEL_SPEC>;
#[doc = "desc PC01_SEL"]
pub mod pc01_sel;
#[doc = "PC02_SEL register accessor: an alias for `Reg<PC02_SEL_SPEC>`"]
pub type PC02_SEL = crate::Reg<pc02_sel::PC02_SEL_SPEC>;
#[doc = "desc PC02_SEL"]
pub mod pc02_sel;
#[doc = "PC03_SEL register accessor: an alias for `Reg<PC03_SEL_SPEC>`"]
pub type PC03_SEL = crate::Reg<pc03_sel::PC03_SEL_SPEC>;
#[doc = "desc PC03_SEL"]
pub mod pc03_sel;
#[doc = "PC04_SEL register accessor: an alias for `Reg<PC04_SEL_SPEC>`"]
pub type PC04_SEL = crate::Reg<pc04_sel::PC04_SEL_SPEC>;
#[doc = "desc PC04_SEL"]
pub mod pc04_sel;
#[doc = "PC05_SEL register accessor: an alias for `Reg<PC05_SEL_SPEC>`"]
pub type PC05_SEL = crate::Reg<pc05_sel::PC05_SEL_SPEC>;
#[doc = "desc PC05_SEL"]
pub mod pc05_sel;
#[doc = "PC06_SEL register accessor: an alias for `Reg<PC06_SEL_SPEC>`"]
pub type PC06_SEL = crate::Reg<pc06_sel::PC06_SEL_SPEC>;
#[doc = "desc PC06_SEL"]
pub mod pc06_sel;
#[doc = "PC07_SEL register accessor: an alias for `Reg<PC07_SEL_SPEC>`"]
pub type PC07_SEL = crate::Reg<pc07_sel::PC07_SEL_SPEC>;
#[doc = "desc PC07_SEL"]
pub mod pc07_sel;
#[doc = "PC08_SEL register accessor: an alias for `Reg<PC08_SEL_SPEC>`"]
pub type PC08_SEL = crate::Reg<pc08_sel::PC08_SEL_SPEC>;
#[doc = "desc PC08_SEL"]
pub mod pc08_sel;
#[doc = "PC09_SEL register accessor: an alias for `Reg<PC09_SEL_SPEC>`"]
pub type PC09_SEL = crate::Reg<pc09_sel::PC09_SEL_SPEC>;
#[doc = "desc PC09_SEL"]
pub mod pc09_sel;
#[doc = "PC10_SEL register accessor: an alias for `Reg<PC10_SEL_SPEC>`"]
pub type PC10_SEL = crate::Reg<pc10_sel::PC10_SEL_SPEC>;
#[doc = "desc PC10_SEL"]
pub mod pc10_sel;
#[doc = "PC11_SEL register accessor: an alias for `Reg<PC11_SEL_SPEC>`"]
pub type PC11_SEL = crate::Reg<pc11_sel::PC11_SEL_SPEC>;
#[doc = "desc PC11_SEL"]
pub mod pc11_sel;
#[doc = "PC12_SEL register accessor: an alias for `Reg<PC12_SEL_SPEC>`"]
pub type PC12_SEL = crate::Reg<pc12_sel::PC12_SEL_SPEC>;
#[doc = "desc PC12_SEL"]
pub mod pc12_sel;
#[doc = "PC13_SEL register accessor: an alias for `Reg<PC13_SEL_SPEC>`"]
pub type PC13_SEL = crate::Reg<pc13_sel::PC13_SEL_SPEC>;
#[doc = "desc PC13_SEL"]
pub mod pc13_sel;
#[doc = "PC14_SEL register accessor: an alias for `Reg<PC14_SEL_SPEC>`"]
pub type PC14_SEL = crate::Reg<pc14_sel::PC14_SEL_SPEC>;
#[doc = "desc PC14_SEL"]
pub mod pc14_sel;
#[doc = "PC15_SEL register accessor: an alias for `Reg<PC15_SEL_SPEC>`"]
pub type PC15_SEL = crate::Reg<pc15_sel::PC15_SEL_SPEC>;
#[doc = "desc PC15_SEL"]
pub mod pc15_sel;
#[doc = "PD00_SEL register accessor: an alias for `Reg<PD00_SEL_SPEC>`"]
pub type PD00_SEL = crate::Reg<pd00_sel::PD00_SEL_SPEC>;
#[doc = "desc PD00_SEL"]
pub mod pd00_sel;
#[doc = "PD01_SEL register accessor: an alias for `Reg<PD01_SEL_SPEC>`"]
pub type PD01_SEL = crate::Reg<pd01_sel::PD01_SEL_SPEC>;
#[doc = "desc PD01_SEL"]
pub mod pd01_sel;
#[doc = "PD02_SEL register accessor: an alias for `Reg<PD02_SEL_SPEC>`"]
pub type PD02_SEL = crate::Reg<pd02_sel::PD02_SEL_SPEC>;
#[doc = "desc PD02_SEL"]
pub mod pd02_sel;
#[doc = "PD03_SEL register accessor: an alias for `Reg<PD03_SEL_SPEC>`"]
pub type PD03_SEL = crate::Reg<pd03_sel::PD03_SEL_SPEC>;
#[doc = "desc PD03_SEL"]
pub mod pd03_sel;
#[doc = "PD04_SEL register accessor: an alias for `Reg<PD04_SEL_SPEC>`"]
pub type PD04_SEL = crate::Reg<pd04_sel::PD04_SEL_SPEC>;
#[doc = "desc PD04_SEL"]
pub mod pd04_sel;
#[doc = "PD05_SEL register accessor: an alias for `Reg<PD05_SEL_SPEC>`"]
pub type PD05_SEL = crate::Reg<pd05_sel::PD05_SEL_SPEC>;
#[doc = "desc PD05_SEL"]
pub mod pd05_sel;
#[doc = "PD06_SEL register accessor: an alias for `Reg<PD06_SEL_SPEC>`"]
pub type PD06_SEL = crate::Reg<pd06_sel::PD06_SEL_SPEC>;
#[doc = "desc PD06_SEL"]
pub mod pd06_sel;
#[doc = "PD07_SEL register accessor: an alias for `Reg<PD07_SEL_SPEC>`"]
pub type PD07_SEL = crate::Reg<pd07_sel::PD07_SEL_SPEC>;
#[doc = "desc PD07_SEL"]
pub mod pd07_sel;
#[doc = "PADIR register accessor: an alias for `Reg<PADIR_SPEC>`"]
pub type PADIR = crate::Reg<padir::PADIR_SPEC>;
#[doc = "desc PADIR"]
pub mod padir;
#[doc = "PAIN register accessor: an alias for `Reg<PAIN_SPEC>`"]
pub type PAIN = crate::Reg<pain::PAIN_SPEC>;
#[doc = "desc PAIN"]
pub mod pain;
#[doc = "PAOUT register accessor: an alias for `Reg<PAOUT_SPEC>`"]
pub type PAOUT = crate::Reg<paout::PAOUT_SPEC>;
#[doc = "desc PAOUT"]
pub mod paout;
#[doc = "PAADS register accessor: an alias for `Reg<PAADS_SPEC>`"]
pub type PAADS = crate::Reg<paads::PAADS_SPEC>;
#[doc = "desc PAADS"]
pub mod paads;
#[doc = "PABSET register accessor: an alias for `Reg<PABSET_SPEC>`"]
pub type PABSET = crate::Reg<pabset::PABSET_SPEC>;
#[doc = "desc PABSET"]
pub mod pabset;
#[doc = "PABCLR register accessor: an alias for `Reg<PABCLR_SPEC>`"]
pub type PABCLR = crate::Reg<pabclr::PABCLR_SPEC>;
#[doc = "desc PABCLR"]
pub mod pabclr;
#[doc = "PABSETCLR register accessor: an alias for `Reg<PABSETCLR_SPEC>`"]
pub type PABSETCLR = crate::Reg<pabsetclr::PABSETCLR_SPEC>;
#[doc = "desc PABSETCLR"]
pub mod pabsetclr;
#[doc = "PADR register accessor: an alias for `Reg<PADR_SPEC>`"]
pub type PADR = crate::Reg<padr::PADR_SPEC>;
#[doc = "desc PADR"]
pub mod padr;
#[doc = "PAPU register accessor: an alias for `Reg<PAPU_SPEC>`"]
pub type PAPU = crate::Reg<papu::PAPU_SPEC>;
#[doc = "desc PAPU"]
pub mod papu;
#[doc = "PAPD register accessor: an alias for `Reg<PAPD_SPEC>`"]
pub type PAPD = crate::Reg<papd::PAPD_SPEC>;
#[doc = "desc PAPD"]
pub mod papd;
#[doc = "PAOD register accessor: an alias for `Reg<PAOD_SPEC>`"]
pub type PAOD = crate::Reg<paod::PAOD_SPEC>;
#[doc = "desc PAOD"]
pub mod paod;
#[doc = "PAHIE register accessor: an alias for `Reg<PAHIE_SPEC>`"]
pub type PAHIE = crate::Reg<pahie::PAHIE_SPEC>;
#[doc = "desc PAHIE"]
pub mod pahie;
#[doc = "PALIE register accessor: an alias for `Reg<PALIE_SPEC>`"]
pub type PALIE = crate::Reg<palie::PALIE_SPEC>;
#[doc = "desc PALIE"]
pub mod palie;
#[doc = "PARIE register accessor: an alias for `Reg<PARIE_SPEC>`"]
pub type PARIE = crate::Reg<parie::PARIE_SPEC>;
#[doc = "desc PARIE"]
pub mod parie;
#[doc = "PAFIE register accessor: an alias for `Reg<PAFIE_SPEC>`"]
pub type PAFIE = crate::Reg<pafie::PAFIE_SPEC>;
#[doc = "desc PAFIE"]
pub mod pafie;
#[doc = "PBDIR register accessor: an alias for `Reg<PBDIR_SPEC>`"]
pub type PBDIR = crate::Reg<pbdir::PBDIR_SPEC>;
#[doc = "desc PBDIR"]
pub mod pbdir;
#[doc = "PBIN register accessor: an alias for `Reg<PBIN_SPEC>`"]
pub type PBIN = crate::Reg<pbin::PBIN_SPEC>;
#[doc = "desc PBIN"]
pub mod pbin;
#[doc = "PBOUT register accessor: an alias for `Reg<PBOUT_SPEC>`"]
pub type PBOUT = crate::Reg<pbout::PBOUT_SPEC>;
#[doc = "desc PBOUT"]
pub mod pbout;
#[doc = "PBADS register accessor: an alias for `Reg<PBADS_SPEC>`"]
pub type PBADS = crate::Reg<pbads::PBADS_SPEC>;
#[doc = "desc PBADS"]
pub mod pbads;
#[doc = "PBBSET register accessor: an alias for `Reg<PBBSET_SPEC>`"]
pub type PBBSET = crate::Reg<pbbset::PBBSET_SPEC>;
#[doc = "desc PBBSET"]
pub mod pbbset;
#[doc = "PBBCLR register accessor: an alias for `Reg<PBBCLR_SPEC>`"]
pub type PBBCLR = crate::Reg<pbbclr::PBBCLR_SPEC>;
#[doc = "desc PBBCLR"]
pub mod pbbclr;
#[doc = "PBBSETCLR register accessor: an alias for `Reg<PBBSETCLR_SPEC>`"]
pub type PBBSETCLR = crate::Reg<pbbsetclr::PBBSETCLR_SPEC>;
#[doc = "desc PBBSETCLR"]
pub mod pbbsetclr;
#[doc = "PBDR register accessor: an alias for `Reg<PBDR_SPEC>`"]
pub type PBDR = crate::Reg<pbdr::PBDR_SPEC>;
#[doc = "desc PBDR"]
pub mod pbdr;
#[doc = "PBPU register accessor: an alias for `Reg<PBPU_SPEC>`"]
pub type PBPU = crate::Reg<pbpu::PBPU_SPEC>;
#[doc = "desc PBPU"]
pub mod pbpu;
#[doc = "PBPD register accessor: an alias for `Reg<PBPD_SPEC>`"]
pub type PBPD = crate::Reg<pbpd::PBPD_SPEC>;
#[doc = "desc PBPD"]
pub mod pbpd;
#[doc = "PBOD register accessor: an alias for `Reg<PBOD_SPEC>`"]
pub type PBOD = crate::Reg<pbod::PBOD_SPEC>;
#[doc = "desc PBOD"]
pub mod pbod;
#[doc = "PBHIE register accessor: an alias for `Reg<PBHIE_SPEC>`"]
pub type PBHIE = crate::Reg<pbhie::PBHIE_SPEC>;
#[doc = "desc PBHIE"]
pub mod pbhie;
#[doc = "PBLIE register accessor: an alias for `Reg<PBLIE_SPEC>`"]
pub type PBLIE = crate::Reg<pblie::PBLIE_SPEC>;
#[doc = "desc PBLIE"]
pub mod pblie;
#[doc = "PBRIE register accessor: an alias for `Reg<PBRIE_SPEC>`"]
pub type PBRIE = crate::Reg<pbrie::PBRIE_SPEC>;
#[doc = "desc PBRIE"]
pub mod pbrie;
#[doc = "PBFIE register accessor: an alias for `Reg<PBFIE_SPEC>`"]
pub type PBFIE = crate::Reg<pbfie::PBFIE_SPEC>;
#[doc = "desc PBFIE"]
pub mod pbfie;
#[doc = "PCDIR register accessor: an alias for `Reg<PCDIR_SPEC>`"]
pub type PCDIR = crate::Reg<pcdir::PCDIR_SPEC>;
#[doc = "desc PCDIR"]
pub mod pcdir;
#[doc = "PCIN register accessor: an alias for `Reg<PCIN_SPEC>`"]
pub type PCIN = crate::Reg<pcin::PCIN_SPEC>;
#[doc = "desc PCIN"]
pub mod pcin;
#[doc = "PCOUT register accessor: an alias for `Reg<PCOUT_SPEC>`"]
pub type PCOUT = crate::Reg<pcout::PCOUT_SPEC>;
#[doc = "desc PCOUT"]
pub mod pcout;
#[doc = "PCADS register accessor: an alias for `Reg<PCADS_SPEC>`"]
pub type PCADS = crate::Reg<pcads::PCADS_SPEC>;
#[doc = "desc PCADS"]
pub mod pcads;
#[doc = "PCBSET register accessor: an alias for `Reg<PCBSET_SPEC>`"]
pub type PCBSET = crate::Reg<pcbset::PCBSET_SPEC>;
#[doc = "desc PCBSET"]
pub mod pcbset;
#[doc = "PCBCLR register accessor: an alias for `Reg<PCBCLR_SPEC>`"]
pub type PCBCLR = crate::Reg<pcbclr::PCBCLR_SPEC>;
#[doc = "desc PCBCLR"]
pub mod pcbclr;
#[doc = "PCBSETCLR register accessor: an alias for `Reg<PCBSETCLR_SPEC>`"]
pub type PCBSETCLR = crate::Reg<pcbsetclr::PCBSETCLR_SPEC>;
#[doc = "desc PCBSETCLR"]
pub mod pcbsetclr;
#[doc = "PCDR register accessor: an alias for `Reg<PCDR_SPEC>`"]
pub type PCDR = crate::Reg<pcdr::PCDR_SPEC>;
#[doc = "desc PCDR"]
pub mod pcdr;
#[doc = "PCPU register accessor: an alias for `Reg<PCPU_SPEC>`"]
pub type PCPU = crate::Reg<pcpu::PCPU_SPEC>;
#[doc = "desc PCPU"]
pub mod pcpu;
#[doc = "PCPD register accessor: an alias for `Reg<PCPD_SPEC>`"]
pub type PCPD = crate::Reg<pcpd::PCPD_SPEC>;
#[doc = "desc PCPD"]
pub mod pcpd;
#[doc = "PCOD register accessor: an alias for `Reg<PCOD_SPEC>`"]
pub type PCOD = crate::Reg<pcod::PCOD_SPEC>;
#[doc = "desc PCOD"]
pub mod pcod;
#[doc = "PCHIE register accessor: an alias for `Reg<PCHIE_SPEC>`"]
pub type PCHIE = crate::Reg<pchie::PCHIE_SPEC>;
#[doc = "desc PCHIE"]
pub mod pchie;
#[doc = "PCLIE register accessor: an alias for `Reg<PCLIE_SPEC>`"]
pub type PCLIE = crate::Reg<pclie::PCLIE_SPEC>;
#[doc = "desc PCLIE"]
pub mod pclie;
#[doc = "PCRIE register accessor: an alias for `Reg<PCRIE_SPEC>`"]
pub type PCRIE = crate::Reg<pcrie::PCRIE_SPEC>;
#[doc = "desc PCRIE"]
pub mod pcrie;
#[doc = "PCFIE register accessor: an alias for `Reg<PCFIE_SPEC>`"]
pub type PCFIE = crate::Reg<pcfie::PCFIE_SPEC>;
#[doc = "desc PCFIE"]
pub mod pcfie;
#[doc = "PDDIR register accessor: an alias for `Reg<PDDIR_SPEC>`"]
pub type PDDIR = crate::Reg<pddir::PDDIR_SPEC>;
#[doc = "desc PDDIR"]
pub mod pddir;
#[doc = "PDIN register accessor: an alias for `Reg<PDIN_SPEC>`"]
pub type PDIN = crate::Reg<pdin::PDIN_SPEC>;
#[doc = "desc PDIN"]
pub mod pdin;
#[doc = "PDOUT register accessor: an alias for `Reg<PDOUT_SPEC>`"]
pub type PDOUT = crate::Reg<pdout::PDOUT_SPEC>;
#[doc = "desc PDOUT"]
pub mod pdout;
#[doc = "PDADS register accessor: an alias for `Reg<PDADS_SPEC>`"]
pub type PDADS = crate::Reg<pdads::PDADS_SPEC>;
#[doc = "desc PDADS"]
pub mod pdads;
#[doc = "PDBSET register accessor: an alias for `Reg<PDBSET_SPEC>`"]
pub type PDBSET = crate::Reg<pdbset::PDBSET_SPEC>;
#[doc = "desc PDBSET"]
pub mod pdbset;
#[doc = "PDBCLR register accessor: an alias for `Reg<PDBCLR_SPEC>`"]
pub type PDBCLR = crate::Reg<pdbclr::PDBCLR_SPEC>;
#[doc = "desc PDBCLR"]
pub mod pdbclr;
#[doc = "PDBSETCLR register accessor: an alias for `Reg<PDBSETCLR_SPEC>`"]
pub type PDBSETCLR = crate::Reg<pdbsetclr::PDBSETCLR_SPEC>;
#[doc = "desc PDBSETCLR"]
pub mod pdbsetclr;
#[doc = "PDDR register accessor: an alias for `Reg<PDDR_SPEC>`"]
pub type PDDR = crate::Reg<pddr::PDDR_SPEC>;
#[doc = "desc PDDR"]
pub mod pddr;
#[doc = "PDPU register accessor: an alias for `Reg<PDPU_SPEC>`"]
pub type PDPU = crate::Reg<pdpu::PDPU_SPEC>;
#[doc = "desc PDPU"]
pub mod pdpu;
#[doc = "PDPD register accessor: an alias for `Reg<PDPD_SPEC>`"]
pub type PDPD = crate::Reg<pdpd::PDPD_SPEC>;
#[doc = "desc PDPD"]
pub mod pdpd;
#[doc = "PDOD register accessor: an alias for `Reg<PDOD_SPEC>`"]
pub type PDOD = crate::Reg<pdod::PDOD_SPEC>;
#[doc = "desc PDOD"]
pub mod pdod;
#[doc = "PDHIE register accessor: an alias for `Reg<PDHIE_SPEC>`"]
pub type PDHIE = crate::Reg<pdhie::PDHIE_SPEC>;
#[doc = "desc PDHIE"]
pub mod pdhie;
#[doc = "PDLIE register accessor: an alias for `Reg<PDLIE_SPEC>`"]
pub type PDLIE = crate::Reg<pdlie::PDLIE_SPEC>;
#[doc = "desc PDLIE"]
pub mod pdlie;
#[doc = "PDRIE register accessor: an alias for `Reg<PDRIE_SPEC>`"]
pub type PDRIE = crate::Reg<pdrie::PDRIE_SPEC>;
#[doc = "desc PDRIE"]
pub mod pdrie;
#[doc = "PDFIE register accessor: an alias for `Reg<PDFIE_SPEC>`"]
pub type PDFIE = crate::Reg<pdfie::PDFIE_SPEC>;
#[doc = "desc PDFIE"]
pub mod pdfie;
#[doc = "PA_STAT register accessor: an alias for `Reg<PA_STAT_SPEC>`"]
pub type PA_STAT = crate::Reg<pa_stat::PA_STAT_SPEC>;
#[doc = "desc PA_STAT"]
pub mod pa_stat;
#[doc = "PA_ICLR register accessor: an alias for `Reg<PA_ICLR_SPEC>`"]
pub type PA_ICLR = crate::Reg<pa_iclr::PA_ICLR_SPEC>;
#[doc = "desc PA_ICLR"]
pub mod pa_iclr;
#[doc = "PB_STAT register accessor: an alias for `Reg<PB_STAT_SPEC>`"]
pub type PB_STAT = crate::Reg<pb_stat::PB_STAT_SPEC>;
#[doc = "desc PB_STAT"]
pub mod pb_stat;
#[doc = "PB_ICLR register accessor: an alias for `Reg<PB_ICLR_SPEC>`"]
pub type PB_ICLR = crate::Reg<pb_iclr::PB_ICLR_SPEC>;
#[doc = "desc PB_ICLR"]
pub mod pb_iclr;
#[doc = "PC_STAT register accessor: an alias for `Reg<PC_STAT_SPEC>`"]
pub type PC_STAT = crate::Reg<pc_stat::PC_STAT_SPEC>;
#[doc = "desc PC_STAT"]
pub mod pc_stat;
#[doc = "PC_ICLR register accessor: an alias for `Reg<PC_ICLR_SPEC>`"]
pub type PC_ICLR = crate::Reg<pc_iclr::PC_ICLR_SPEC>;
#[doc = "desc PC_ICLR"]
pub mod pc_iclr;
#[doc = "PD_STAT register accessor: an alias for `Reg<PD_STAT_SPEC>`"]
pub type PD_STAT = crate::Reg<pd_stat::PD_STAT_SPEC>;
#[doc = "desc PD_STAT"]
pub mod pd_stat;
#[doc = "PD_ICLR register accessor: an alias for `Reg<PD_ICLR_SPEC>`"]
pub type PD_ICLR = crate::Reg<pd_iclr::PD_ICLR_SPEC>;
#[doc = "desc PD_ICLR"]
pub mod pd_iclr;
#[doc = "CTRL0 register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "desc CTRL0"]
pub mod ctrl0;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "desc CTRL1"]
pub mod ctrl1;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "desc CTRL2"]
pub mod ctrl2;
#[doc = "TIMGS register accessor: an alias for `Reg<TIMGS_SPEC>`"]
pub type TIMGS = crate::Reg<timgs::TIMGS_SPEC>;
#[doc = "desc TIMGS"]
pub mod timgs;
#[doc = "TIMES register accessor: an alias for `Reg<TIMES_SPEC>`"]
pub type TIMES = crate::Reg<times::TIMES_SPEC>;
#[doc = "desc TIMES"]
pub mod times;
#[doc = "TIMCPS register accessor: an alias for `Reg<TIMCPS_SPEC>`"]
pub type TIMCPS = crate::Reg<timcps::TIMCPS_SPEC>;
#[doc = "desc TIMCPS"]
pub mod timcps;
#[doc = "PCAS register accessor: an alias for `Reg<PCAS_SPEC>`"]
pub type PCAS = crate::Reg<pcas::PCAS_SPEC>;
#[doc = "desc PCAS"]
pub mod pcas;
