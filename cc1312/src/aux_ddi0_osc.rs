#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Direct Access for DDI Byte Offsets 0 to 3"]
    pub dir03: DIR03,
    #[doc = "0x04 - Direct Access for DDI Byte Offsets 4 to 7"]
    pub dir47: DIR47,
    #[doc = "0x08 - Direct Access for DDI Byte Offsets 8 to 11"]
    pub dir811: DIR811,
    #[doc = "0x0c - Direct Access for DDI Byte Offsets 12 to 15"]
    pub dir1215: DIR1215,
    #[doc = "0x10 - Direct Access for DDI Byte Offsets 16 to 19"]
    pub dir1619: DIR1619,
    #[doc = "0x14 - Direct Access for DDI Byte Offsets 20 to 23"]
    pub dir2023: DIR2023,
    #[doc = "0x18 - Direct Access for DDI Byte Offsets 24 to 27"]
    pub dir2427: DIR2427,
    #[doc = "0x1c - Direct Access for DDI Byte Offsets 28 to 31"]
    pub dir2831: DIR2831,
    #[doc = "0x20 - Direct Access for DDI Byte Offsets 32 to 35"]
    pub dir3235: DIR3235,
    #[doc = "0x24 - Direct Access for DDI Byte Offsets 36 to 39"]
    pub dir3639: DIR3639,
    #[doc = "0x28 - Direct Access for DDI Byte Offsets 40 to 43"]
    pub dir4043: DIR4043,
    #[doc = "0x2c - Direct Access for DDI Byte Offsets 44 to 47"]
    pub dir4447: DIR4447,
    #[doc = "0x30 - Direct Access for DDI Byte Offsets 48 to 51"]
    pub dir4851: DIR4851,
    #[doc = "0x34 - Direct Access for DDI Byte Offsets 52 to 55"]
    pub dir5255: DIR5255,
    #[doc = "0x38 - Direct Access for DDI Byte Offsets 56 to 59"]
    pub dir5659: DIR5659,
    #[doc = "0x3c - Direct Access for DDI Byte Offsets 60 to 63"]
    pub dir6063: DIR6063,
    #[doc = "0x40 - Direct Access for DDI Byte Offsets 64 to 67"]
    pub dir6467: DIR6467,
    #[doc = "0x44 - Direct Access for DDI Byte Offsets 68 to 71"]
    pub dir6871: DIR6871,
    _reserved18: [u8; 56usize],
    #[doc = "0x80 - Set for DDI Byte Offsets 0 to 3"]
    pub set03: SET03,
    #[doc = "0x84 - Set for DDI Byte Offsets 4 to 7"]
    pub set47: SET47,
    #[doc = "0x88 - Set for DDI Byte Offsets 8 to 11"]
    pub set811: SET811,
    #[doc = "0x8c - Set for DDI Byte Offsets 12 to 15"]
    pub set1215: SET1215,
    #[doc = "0x90 - Set for DDI Byte Offsets 16 to 19"]
    pub set1619: SET1619,
    #[doc = "0x94 - Set for DDI Byte Offsets 20 to 23"]
    pub set2023: SET2023,
    #[doc = "0x98 - Set for DDI Byte Offsets 24 to 27"]
    pub set2427: SET2427,
    #[doc = "0x9c - Set for DDI Byte Offsets 28 to 31"]
    pub set2831: SET2831,
    #[doc = "0xa0 - Set for DDI Byte Offsets 32 to 35"]
    pub set3235: SET3235,
    #[doc = "0xa4 - Set for DDI Byte Offsets 36 to 39"]
    pub set3639: SET3639,
    #[doc = "0xa8 - Set for DDI Byte Offsets 40 to 43"]
    pub set4043: SET4043,
    #[doc = "0xac - Set for DDI Byte Offsets 44 to 47"]
    pub set4447: SET4447,
    #[doc = "0xb0 - Set for DDI Byte Offsets 48 to 51"]
    pub set4851: SET4851,
    #[doc = "0xb4 - Set for DDI Byte Offsets 52 to 55"]
    pub set5255: SET5255,
    #[doc = "0xb8 - Set for DDI Byte Offsets 56 to 59"]
    pub set5659: SET5659,
    #[doc = "0xbc - Set for DDI Byte Offsets 60 to 63"]
    pub set6063: SET6063,
    #[doc = "0xc0 - Set for DDI Byte Offsets 64 to 67"]
    pub set6467: SET6467,
    #[doc = "0xc4 - Set for DDI Byte Offsets 68 to 71"]
    pub set6871: SET6871,
    _reserved36: [u8; 56usize],
    #[doc = "0x100 - Clear for DDI Byte Offsets 0 to 3"]
    pub clr03: CLR03,
    #[doc = "0x104 - Clear for DDI Byte Offsets 4 to 7"]
    pub clr47: CLR47,
    #[doc = "0x108 - Clear for DDI Byte Offsets 8 to 11"]
    pub clr811: CLR811,
    #[doc = "0x10c - Clear for DDI Byte Offsets 12 to 15"]
    pub clr1215: CLR1215,
    #[doc = "0x110 - Clear for DDI Byte Offsets 16 to 19"]
    pub clr1619: CLR1619,
    #[doc = "0x114 - Clear for DDI Byte Offsets 20 to 23"]
    pub clr2023: CLR2023,
    #[doc = "0x118 - Clear for DDI Byte Offsets 24 to 27"]
    pub clr2427: CLR2427,
    #[doc = "0x11c - Clear for DDI Byte Offsets 28 to 31"]
    pub clr2831: CLR2831,
    #[doc = "0x120 - Clear for DDI Byte Offsets 32 to 35"]
    pub clr3235: CLR3235,
    #[doc = "0x124 - Clear for DDI Byte Offsets 36 to 39"]
    pub clr3639: CLR3639,
    #[doc = "0x128 - Clear for DDI Byte Offsets 40 to 43"]
    pub clr4043: CLR4043,
    #[doc = "0x12c - Clear for DDI Byte Offsets 44 to 47"]
    pub clr4447: CLR4447,
    #[doc = "0x130 - Clear for DDI Byte Offsets 48 to 51"]
    pub clr4851: CLR4851,
    #[doc = "0x134 - Clear for DDI Byte Offsets 52 to 55"]
    pub clr5255: CLR5255,
    #[doc = "0x138 - Clear for DDI Byte Offsets 56 to 59"]
    pub clr5659: CLR5659,
    #[doc = "0x13c - Clear for DDI Byte Offsets 60 to 63"]
    pub clr6063: CLR6063,
    #[doc = "0x140 - Clear for DDI Byte Offsets 64 to 67"]
    pub clr6467: CLR6467,
    #[doc = "0x144 - Clear for DDI Byte Offsets 68 to 71"]
    pub clr6871: CLR6871,
    _reserved54: [u8; 56usize],
    #[doc = "0x180 - Internal. Only to be used through TI provided API."]
    pub sync: SYNC,
    _reserved55: [u8; 4usize],
    #[doc = "0x188 - Internal. Only to be used through TI provided API."]
    pub cfg: CFG,
    _reserved56: [u8; 116usize],
    #[doc = "0x200 - Masked Access (4m/4d), Byte Offsets 0 and 1"]
    pub mask4b01: MASK4B01,
    #[doc = "0x204 - Masked Access (4m/4d), Byte Offsets 2 and 3"]
    pub mask4b23: MASK4B23,
    #[doc = "0x208 - Masked Access (4m/4d), Byte Offsets 4 and 5"]
    pub mask4b45: MASK4B45,
    #[doc = "0x20c - Masked Access (4m/4d), Byte Offsets 6 and 7"]
    pub mask4b67: MASK4B67,
    #[doc = "0x210 - Masked Access (4m/4d), Byte Offsets 8 and 9"]
    pub mask4b89: MASK4B89,
    #[doc = "0x214 - Masked Access (4m/4d), Byte Offsets 10 and 11"]
    pub mask4b1011: MASK4B1011,
    #[doc = "0x218 - Masked Access (4m/4d), Byte Offsets 12 and 13"]
    pub mask4b1213: MASK4B1213,
    #[doc = "0x21c - Masked Access (4m/4d), Byte Offsets 14 and 15"]
    pub mask4b1415: MASK4B1415,
    #[doc = "0x220 - Masked Access (4m/4d), Byte Offsets 16 and 17"]
    pub mask4b1617: MASK4B1617,
    #[doc = "0x224 - Masked Access (4m/4d), Byte Offsets 18 and 19"]
    pub mask4b1819: MASK4B1819,
    #[doc = "0x228 - Masked Access (4m/4d), Byte Offsets 20 and 21"]
    pub mask4b2021: MASK4B2021,
    #[doc = "0x22c - Masked Access (4m/4d), Byte Offsets 22 and 23"]
    pub mask4b2223: MASK4B2223,
    #[doc = "0x230 - Masked Access (4m/4d), Byte Offsets 24 and 25"]
    pub mask4b2425: MASK4B2425,
    #[doc = "0x234 - Masked Access (4m/4d), Byte Offsets 26 and 27"]
    pub mask4b2627: MASK4B2627,
    #[doc = "0x238 - Masked Access (4m/4d), Byte Offsets 28 and 29"]
    pub mask4b2829: MASK4B2829,
    #[doc = "0x23c - Masked Access (4m/4d), Byte Offsets 30 and 31"]
    pub mask4b3031: MASK4B3031,
    #[doc = "0x240 - Masked Access (4m/4d), Byte Offsets 32 and 33"]
    pub mask4b3233: MASK4B3233,
    #[doc = "0x244 - Masked Access (4m/4d), Byte Offsets 34 and 35"]
    pub mask4b3435: MASK4B3435,
    #[doc = "0x248 - Masked Access (4m/4d), Byte Offsets 36 and 37"]
    pub mask4b3637: MASK4B3637,
    #[doc = "0x24c - Masked Access (4m/4d), Byte Offsets 38 and 39"]
    pub mask4b3839: MASK4B3839,
    #[doc = "0x250 - Masked Access (4m/4d), Byte Offsets 40 and 41"]
    pub mask4b4041: MASK4B4041,
    #[doc = "0x254 - Masked Access (4m/4d), Byte Offsets 42 and 43"]
    pub mask4b4243: MASK4B4243,
    #[doc = "0x258 - Masked Access (4m/4d), Byte Offsets 44 and 45"]
    pub mask4b4445: MASK4B4445,
    #[doc = "0x25c - Masked Access (4m/4d), Byte Offsets 46 and 47"]
    pub mask4b4647: MASK4B4647,
    #[doc = "0x260 - Masked Access (4m/4d), Byte Offsets 48 and 49"]
    pub mask4b4849: MASK4B4849,
    #[doc = "0x264 - Masked Access (4m/4d), Byte Offsets 50 and 51"]
    pub mask4b5051: MASK4B5051,
    #[doc = "0x268 - Masked Access (4m/4d), Byte Offsets 52 and 53"]
    pub mask4b5253: MASK4B5253,
    #[doc = "0x26c - Masked Access (4m/4d), Byte Offsets 54 and 55"]
    pub mask4b5455: MASK4B5455,
    #[doc = "0x270 - Masked Access (4m/4d), Byte Offsets 56 and 57"]
    pub mask4b5657: MASK4B5657,
    #[doc = "0x274 - Masked Access (4m/4d), Byte Offsets 58 and 59"]
    pub mask4b5859: MASK4B5859,
    #[doc = "0x278 - Masked Access (4m/4d), Byte Offsets 60 and 61"]
    pub mask4b6061: MASK4B6061,
    #[doc = "0x27c - Masked Access (4m/4d), Byte Offsets 62 and 63"]
    pub mask4b6263: MASK4B6263,
    #[doc = "0x280 - Masked Access (4m/4d), Byte Offsets 64 and 65"]
    pub mask4b6465: MASK4B6465,
    #[doc = "0x284 - Masked Access (4m/4d), Byte Offsets 66 and 67"]
    pub mask4b6667: MASK4B6667,
    #[doc = "0x288 - Masked Access (4m/4d), Byte Offsets 68 and 69"]
    pub mask4b6869: MASK4B6869,
    #[doc = "0x28c - Masked Access (4m/4d), Byte Offsets 70 and 71"]
    pub mask4b7071: MASK4B7071,
    _reserved92: [u8; 112usize],
    #[doc = "0x300 - Masked Access (8m/8d), Byte Offsets 0 and 1"]
    pub mask8b01: MASK8B01,
    #[doc = "0x304 - Masked Access (8m/8d), Byte Offsets 2 and 3"]
    pub mask8b23: MASK8B23,
    #[doc = "0x308 - Masked Access (8m/8d), Byte Offsets 4 and 5"]
    pub mask8b45: MASK8B45,
    #[doc = "0x30c - Masked Access (8m/8d), Byte Offsets 6 and 7"]
    pub mask8b67: MASK8B67,
    #[doc = "0x310 - Masked Access (8m/8d), Byte Offsets 8 and 9"]
    pub mask8b89: MASK8B89,
    #[doc = "0x314 - Masked Access (8m/8d), Byte Offsets 10 and 11"]
    pub mask8b1011: MASK8B1011,
    #[doc = "0x318 - Masked Access (8m/8d), Byte Offsets 12 and 13"]
    pub mask8b1213: MASK8B1213,
    #[doc = "0x31c - Masked Access (8m/8d), Byte Offsets 14 and 15"]
    pub mask8b1415: MASK8B1415,
    #[doc = "0x320 - Masked Access (8m/8d), Byte Offsets 16 and 17"]
    pub mask8b1617: MASK8B1617,
    #[doc = "0x324 - Masked Access (8m/8d), Byte Offsets 18 and 19"]
    pub mask8b1819: MASK8B1819,
    #[doc = "0x328 - Masked Access (8m/8d), Byte Offsets 20 and 21"]
    pub mask8b2021: MASK8B2021,
    #[doc = "0x32c - Masked Access (8m/8d), Byte Offsets 22 and 23"]
    pub mask8b2223: MASK8B2223,
    #[doc = "0x330 - Masked Access (8m/8d), Byte Offsets 24 and 25"]
    pub mask8b2425: MASK8B2425,
    #[doc = "0x334 - Masked Access (8m/8d), Byte Offsets 26 and 27"]
    pub mask8b2627: MASK8B2627,
    #[doc = "0x338 - Masked Access (8m/8d), Byte Offsets 28 and 29"]
    pub mask8b2829: MASK8B2829,
    #[doc = "0x33c - Masked Access (8m/8d), Byte Offsets 30 and 31"]
    pub mask8b3031: MASK8B3031,
    #[doc = "0x340 - Masked Access (8m/8d), Byte Offsets 32 and 33"]
    pub mask8b3233: MASK8B3233,
    #[doc = "0x344 - Masked Access (8m/8d), Byte Offsets 34 and 35"]
    pub mask8b3435: MASK8B3435,
    #[doc = "0x348 - Masked Access (8m/8d), Byte Offsets 36 and 37"]
    pub mask8b3637: MASK8B3637,
    #[doc = "0x34c - Masked Access (8m/8d), Byte Offsets 38 and 39"]
    pub mask8b3839: MASK8B3839,
    #[doc = "0x350 - Masked Access (8m/8d), Byte Offsets 40 and 41"]
    pub mask8b4041: MASK8B4041,
    #[doc = "0x354 - Masked Access (8m/8d), Byte Offsets 42 and 43"]
    pub mask8b4243: MASK8B4243,
    #[doc = "0x358 - Masked Access (8m/8d), Byte Offsets 44 and 45"]
    pub mask8b4445: MASK8B4445,
    #[doc = "0x35c - Masked Access (8m/8d), Byte Offsets 46 and 47"]
    pub mask8b4647: MASK8B4647,
    #[doc = "0x360 - Masked Access (8m/8d), Byte Offsets 48 and 49"]
    pub mask8b4849: MASK8B4849,
    #[doc = "0x364 - Masked Access (8m/8d), Byte Offsets 50 and 51"]
    pub mask8b5051: MASK8B5051,
    #[doc = "0x368 - Masked Access (8m/8d), Byte Offsets 52 and 53"]
    pub mask8b5253: MASK8B5253,
    #[doc = "0x36c - Masked Access (8m/8d), Byte Offsets 54 and 55"]
    pub mask8b5455: MASK8B5455,
    #[doc = "0x370 - Masked Access (8m/8d), Byte Offsets 56 and 57"]
    pub mask8b5657: MASK8B5657,
    #[doc = "0x374 - Masked Access (8m/8d), Byte Offsets 58 and 59"]
    pub mask8b5859: MASK8B5859,
    #[doc = "0x378 - Masked Access (8m/8d), Byte Offsets 60 and 61"]
    pub mask8b6061: MASK8B6061,
    #[doc = "0x37c - Masked Access (8m/8d), Byte Offsets 62 and 63"]
    pub mask8b6263: MASK8B6263,
    #[doc = "0x380 - Masked Access (8m/8d), Byte Offsets 64 and 65"]
    pub mask8b6465: MASK8B6465,
    #[doc = "0x384 - Masked Access (8m/8d), Byte Offsets 66 and 67"]
    pub mask8b6667: MASK8B6667,
    #[doc = "0x388 - Masked Access (8m/8d), Byte Offsets 68 and 69"]
    pub mask8b6869: MASK8B6869,
    #[doc = "0x38c - Masked Access (8m/8d), Byte Offsets 70 and 71"]
    pub mask8b7071: MASK8B7071,
    _reserved128: [u8; 112usize],
    #[doc = "0x400 - Masked Access (16m/16d), Byte Offsets 0 and 1"]
    pub mask16b01: MASK16B01,
    #[doc = "0x404 - Masked Access (16m/16d), Byte Offsets 2 and 3"]
    pub mask16b23: MASK16B23,
    #[doc = "0x408 - Masked Access (16m/16d), Byte Offsets 4 and 5"]
    pub mask16b45: MASK16B45,
    #[doc = "0x40c - Masked Access (16m/16d), Byte Offsets 6 and 7"]
    pub mask16b67: MASK16B67,
    #[doc = "0x410 - Masked Access (16m/16d), Byte Offsets 8 and 9"]
    pub mask16b89: MASK16B89,
    #[doc = "0x414 - Masked Access (16m/16d), Byte Offsets 10 and 11"]
    pub mask16b1011: MASK16B1011,
    #[doc = "0x418 - Masked Access (16m/16d), Byte Offsets 12 and 13"]
    pub mask16b1213: MASK16B1213,
    #[doc = "0x41c - Masked Access (16m/16d), Byte Offsets 14 and 15"]
    pub mask16b1415: MASK16B1415,
    #[doc = "0x420 - Masked Access (16m/16d), Byte Offsets 16 and 17"]
    pub mask16b1617: MASK16B1617,
    #[doc = "0x424 - Masked Access (16m/16d), Byte Offsets 18 and 19"]
    pub mask16b1819: MASK16B1819,
    #[doc = "0x428 - Masked Access (16m/16d), Byte Offsets 20 and 21"]
    pub mask16b2021: MASK16B2021,
    #[doc = "0x42c - Masked Access (16m/16d), Byte Offsets 22 and 23"]
    pub mask16b2223: MASK16B2223,
    #[doc = "0x430 - Masked Access (16m/16d), Byte Offsets 24 and 25"]
    pub mask16b2425: MASK16B2425,
    #[doc = "0x434 - Masked Access (16m/16d), Byte Offsets 26 and 27"]
    pub mask16b2627: MASK16B2627,
    #[doc = "0x438 - Masked Access (16m/16d), Byte Offsets 28 and 29"]
    pub mask16b2829: MASK16B2829,
    #[doc = "0x43c - Masked Access (16m/16d), Byte Offsets 30 and 31"]
    pub mask16b3031: MASK16B3031,
    #[doc = "0x440 - Masked Access (16m/16d), Byte Offsets 32 and 33"]
    pub mask16b3233: MASK16B3233,
    #[doc = "0x444 - Masked Access (16m/16d), Byte Offsets 34 and 35"]
    pub mask16b3435: MASK16B3435,
    #[doc = "0x448 - Masked Access (16m/16d), Byte Offsets 36 and 37"]
    pub mask16b3637: MASK16B3637,
    #[doc = "0x44c - Masked Access (16m/16d), Byte Offsets 38 and 39"]
    pub mask16b3839: MASK16B3839,
    #[doc = "0x450 - Masked Access (16m/16d), Byte Offsets 40 and 41"]
    pub mask16b4041: MASK16B4041,
    #[doc = "0x454 - Masked Access (16m/16d), Byte Offsets 42 and 43"]
    pub mask16b4243: MASK16B4243,
    #[doc = "0x458 - Masked Access (16m/16d), Byte Offsets 44 and 45"]
    pub mask16b4445: MASK16B4445,
    #[doc = "0x45c - Masked Access (16m/16d), Byte Offsets 46 and 47"]
    pub mask16b4647: MASK16B4647,
    #[doc = "0x460 - Masked Access (16m/16d), Byte Offsets 48 and 49"]
    pub mask16b4849: MASK16B4849,
    #[doc = "0x464 - Masked Access (16m/16d), Byte Offsets 50 and 51"]
    pub mask16b5051: MASK16B5051,
    #[doc = "0x468 - Masked Access (16m/16d), Byte Offsets 52 and 53"]
    pub mask16b5253: MASK16B5253,
    #[doc = "0x46c - Masked Access (16m/16d), Byte Offsets 54 and 55"]
    pub mask16b5455: MASK16B5455,
    #[doc = "0x470 - Masked Access (16m/16d), Byte Offsets 56 and 57"]
    pub mask16b5657: MASK16B5657,
    #[doc = "0x474 - Masked Access (16m/16d), Byte Offsets 58 and 59"]
    pub mask16b5859: MASK16B5859,
    #[doc = "0x478 - Masked Access (16m/16d), Byte Offsets 60 and 61"]
    pub mask16b6061: MASK16B6061,
    #[doc = "0x47c - Masked Access (16m/16d), Byte Offsets 62 and 63"]
    pub mask16b6263: MASK16B6263,
    #[doc = "0x480 - Masked Access (16m/16d), Byte Offsets 64 and 65"]
    pub mask16b6465: MASK16B6465,
    #[doc = "0x484 - Masked Access (16m/16d), Byte Offsets 66 and 67"]
    pub mask16b6667: MASK16B6667,
    #[doc = "0x488 - Masked Access (16m/16d), Byte Offsets 68 and 69"]
    pub mask16b6869: MASK16B6869,
    #[doc = "0x48c - Masked Access (16m/16d), Byte Offsets 70 and 71"]
    pub mask16b7071: MASK16B7071,
}
#[doc = "Direct Access for DDI Byte Offsets 0 to 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir03](dir03) module"]
pub type DIR03 = crate::Reg<u32, _DIR03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR03;
#[doc = "`read()` method returns [dir03::R](dir03::R) reader structure"]
impl crate::Readable for DIR03 {}
#[doc = "`write(|w| ..)` method takes [dir03::W](dir03::W) writer structure"]
impl crate::Writable for DIR03 {}
#[doc = "Direct Access for DDI Byte Offsets 0 to 3"]
pub mod dir03;
#[doc = "Direct Access for DDI Byte Offsets 4 to 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir47](dir47) module"]
pub type DIR47 = crate::Reg<u32, _DIR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR47;
#[doc = "`read()` method returns [dir47::R](dir47::R) reader structure"]
impl crate::Readable for DIR47 {}
#[doc = "`write(|w| ..)` method takes [dir47::W](dir47::W) writer structure"]
impl crate::Writable for DIR47 {}
#[doc = "Direct Access for DDI Byte Offsets 4 to 7"]
pub mod dir47;
#[doc = "Direct Access for DDI Byte Offsets 8 to 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir811](dir811) module"]
pub type DIR811 = crate::Reg<u32, _DIR811>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR811;
#[doc = "`read()` method returns [dir811::R](dir811::R) reader structure"]
impl crate::Readable for DIR811 {}
#[doc = "`write(|w| ..)` method takes [dir811::W](dir811::W) writer structure"]
impl crate::Writable for DIR811 {}
#[doc = "Direct Access for DDI Byte Offsets 8 to 11"]
pub mod dir811;
#[doc = "Direct Access for DDI Byte Offsets 12 to 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir1215](dir1215) module"]
pub type DIR1215 = crate::Reg<u32, _DIR1215>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR1215;
#[doc = "`read()` method returns [dir1215::R](dir1215::R) reader structure"]
impl crate::Readable for DIR1215 {}
#[doc = "`write(|w| ..)` method takes [dir1215::W](dir1215::W) writer structure"]
impl crate::Writable for DIR1215 {}
#[doc = "Direct Access for DDI Byte Offsets 12 to 15"]
pub mod dir1215;
#[doc = "Direct Access for DDI Byte Offsets 16 to 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir1619](dir1619) module"]
pub type DIR1619 = crate::Reg<u32, _DIR1619>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR1619;
#[doc = "`read()` method returns [dir1619::R](dir1619::R) reader structure"]
impl crate::Readable for DIR1619 {}
#[doc = "`write(|w| ..)` method takes [dir1619::W](dir1619::W) writer structure"]
impl crate::Writable for DIR1619 {}
#[doc = "Direct Access for DDI Byte Offsets 16 to 19"]
pub mod dir1619;
#[doc = "Direct Access for DDI Byte Offsets 20 to 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir2023](dir2023) module"]
pub type DIR2023 = crate::Reg<u32, _DIR2023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR2023;
#[doc = "`read()` method returns [dir2023::R](dir2023::R) reader structure"]
impl crate::Readable for DIR2023 {}
#[doc = "`write(|w| ..)` method takes [dir2023::W](dir2023::W) writer structure"]
impl crate::Writable for DIR2023 {}
#[doc = "Direct Access for DDI Byte Offsets 20 to 23"]
pub mod dir2023;
#[doc = "Direct Access for DDI Byte Offsets 24 to 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir2427](dir2427) module"]
pub type DIR2427 = crate::Reg<u32, _DIR2427>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR2427;
#[doc = "`read()` method returns [dir2427::R](dir2427::R) reader structure"]
impl crate::Readable for DIR2427 {}
#[doc = "`write(|w| ..)` method takes [dir2427::W](dir2427::W) writer structure"]
impl crate::Writable for DIR2427 {}
#[doc = "Direct Access for DDI Byte Offsets 24 to 27"]
pub mod dir2427;
#[doc = "Direct Access for DDI Byte Offsets 28 to 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir2831](dir2831) module"]
pub type DIR2831 = crate::Reg<u32, _DIR2831>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR2831;
#[doc = "`read()` method returns [dir2831::R](dir2831::R) reader structure"]
impl crate::Readable for DIR2831 {}
#[doc = "`write(|w| ..)` method takes [dir2831::W](dir2831::W) writer structure"]
impl crate::Writable for DIR2831 {}
#[doc = "Direct Access for DDI Byte Offsets 28 to 31"]
pub mod dir2831;
#[doc = "Direct Access for DDI Byte Offsets 32 to 35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir3235](dir3235) module"]
pub type DIR3235 = crate::Reg<u32, _DIR3235>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR3235;
#[doc = "`read()` method returns [dir3235::R](dir3235::R) reader structure"]
impl crate::Readable for DIR3235 {}
#[doc = "`write(|w| ..)` method takes [dir3235::W](dir3235::W) writer structure"]
impl crate::Writable for DIR3235 {}
#[doc = "Direct Access for DDI Byte Offsets 32 to 35"]
pub mod dir3235;
#[doc = "Direct Access for DDI Byte Offsets 36 to 39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir3639](dir3639) module"]
pub type DIR3639 = crate::Reg<u32, _DIR3639>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR3639;
#[doc = "`read()` method returns [dir3639::R](dir3639::R) reader structure"]
impl crate::Readable for DIR3639 {}
#[doc = "`write(|w| ..)` method takes [dir3639::W](dir3639::W) writer structure"]
impl crate::Writable for DIR3639 {}
#[doc = "Direct Access for DDI Byte Offsets 36 to 39"]
pub mod dir3639;
#[doc = "Direct Access for DDI Byte Offsets 40 to 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir4043](dir4043) module"]
pub type DIR4043 = crate::Reg<u32, _DIR4043>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR4043;
#[doc = "`read()` method returns [dir4043::R](dir4043::R) reader structure"]
impl crate::Readable for DIR4043 {}
#[doc = "`write(|w| ..)` method takes [dir4043::W](dir4043::W) writer structure"]
impl crate::Writable for DIR4043 {}
#[doc = "Direct Access for DDI Byte Offsets 40 to 43"]
pub mod dir4043;
#[doc = "Direct Access for DDI Byte Offsets 44 to 47\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir4447](dir4447) module"]
pub type DIR4447 = crate::Reg<u32, _DIR4447>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR4447;
#[doc = "`read()` method returns [dir4447::R](dir4447::R) reader structure"]
impl crate::Readable for DIR4447 {}
#[doc = "`write(|w| ..)` method takes [dir4447::W](dir4447::W) writer structure"]
impl crate::Writable for DIR4447 {}
#[doc = "Direct Access for DDI Byte Offsets 44 to 47"]
pub mod dir4447;
#[doc = "Direct Access for DDI Byte Offsets 48 to 51\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir4851](dir4851) module"]
pub type DIR4851 = crate::Reg<u32, _DIR4851>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR4851;
#[doc = "`read()` method returns [dir4851::R](dir4851::R) reader structure"]
impl crate::Readable for DIR4851 {}
#[doc = "`write(|w| ..)` method takes [dir4851::W](dir4851::W) writer structure"]
impl crate::Writable for DIR4851 {}
#[doc = "Direct Access for DDI Byte Offsets 48 to 51"]
pub mod dir4851;
#[doc = "Direct Access for DDI Byte Offsets 52 to 55\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir5255](dir5255) module"]
pub type DIR5255 = crate::Reg<u32, _DIR5255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR5255;
#[doc = "`read()` method returns [dir5255::R](dir5255::R) reader structure"]
impl crate::Readable for DIR5255 {}
#[doc = "`write(|w| ..)` method takes [dir5255::W](dir5255::W) writer structure"]
impl crate::Writable for DIR5255 {}
#[doc = "Direct Access for DDI Byte Offsets 52 to 55"]
pub mod dir5255;
#[doc = "Direct Access for DDI Byte Offsets 56 to 59\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir5659](dir5659) module"]
pub type DIR5659 = crate::Reg<u32, _DIR5659>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR5659;
#[doc = "`read()` method returns [dir5659::R](dir5659::R) reader structure"]
impl crate::Readable for DIR5659 {}
#[doc = "`write(|w| ..)` method takes [dir5659::W](dir5659::W) writer structure"]
impl crate::Writable for DIR5659 {}
#[doc = "Direct Access for DDI Byte Offsets 56 to 59"]
pub mod dir5659;
#[doc = "Direct Access for DDI Byte Offsets 60 to 63\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir6063](dir6063) module"]
pub type DIR6063 = crate::Reg<u32, _DIR6063>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR6063;
#[doc = "`read()` method returns [dir6063::R](dir6063::R) reader structure"]
impl crate::Readable for DIR6063 {}
#[doc = "`write(|w| ..)` method takes [dir6063::W](dir6063::W) writer structure"]
impl crate::Writable for DIR6063 {}
#[doc = "Direct Access for DDI Byte Offsets 60 to 63"]
pub mod dir6063;
#[doc = "Direct Access for DDI Byte Offsets 64 to 67\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir6467](dir6467) module"]
pub type DIR6467 = crate::Reg<u32, _DIR6467>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR6467;
#[doc = "`read()` method returns [dir6467::R](dir6467::R) reader structure"]
impl crate::Readable for DIR6467 {}
#[doc = "`write(|w| ..)` method takes [dir6467::W](dir6467::W) writer structure"]
impl crate::Writable for DIR6467 {}
#[doc = "Direct Access for DDI Byte Offsets 64 to 67"]
pub mod dir6467;
#[doc = "Direct Access for DDI Byte Offsets 68 to 71\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir6871](dir6871) module"]
pub type DIR6871 = crate::Reg<u32, _DIR6871>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR6871;
#[doc = "`read()` method returns [dir6871::R](dir6871::R) reader structure"]
impl crate::Readable for DIR6871 {}
#[doc = "`write(|w| ..)` method takes [dir6871::W](dir6871::W) writer structure"]
impl crate::Writable for DIR6871 {}
#[doc = "Direct Access for DDI Byte Offsets 68 to 71"]
pub mod dir6871;
#[doc = "Set for DDI Byte Offsets 0 to 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set03](set03) module"]
pub type SET03 = crate::Reg<u32, _SET03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET03;
#[doc = "`write(|w| ..)` method takes [set03::W](set03::W) writer structure"]
impl crate::Writable for SET03 {}
#[doc = "Set for DDI Byte Offsets 0 to 3"]
pub mod set03;
#[doc = "Set for DDI Byte Offsets 4 to 7\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set47](set47) module"]
pub type SET47 = crate::Reg<u32, _SET47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET47;
#[doc = "`write(|w| ..)` method takes [set47::W](set47::W) writer structure"]
impl crate::Writable for SET47 {}
#[doc = "Set for DDI Byte Offsets 4 to 7"]
pub mod set47;
#[doc = "Set for DDI Byte Offsets 8 to 11\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set811](set811) module"]
pub type SET811 = crate::Reg<u32, _SET811>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET811;
#[doc = "`write(|w| ..)` method takes [set811::W](set811::W) writer structure"]
impl crate::Writable for SET811 {}
#[doc = "Set for DDI Byte Offsets 8 to 11"]
pub mod set811;
#[doc = "Set for DDI Byte Offsets 12 to 15\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set1215](set1215) module"]
pub type SET1215 = crate::Reg<u32, _SET1215>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET1215;
#[doc = "`write(|w| ..)` method takes [set1215::W](set1215::W) writer structure"]
impl crate::Writable for SET1215 {}
#[doc = "Set for DDI Byte Offsets 12 to 15"]
pub mod set1215;
#[doc = "Set for DDI Byte Offsets 16 to 19\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set1619](set1619) module"]
pub type SET1619 = crate::Reg<u32, _SET1619>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET1619;
#[doc = "`write(|w| ..)` method takes [set1619::W](set1619::W) writer structure"]
impl crate::Writable for SET1619 {}
#[doc = "Set for DDI Byte Offsets 16 to 19"]
pub mod set1619;
#[doc = "Set for DDI Byte Offsets 20 to 23\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set2023](set2023) module"]
pub type SET2023 = crate::Reg<u32, _SET2023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET2023;
#[doc = "`write(|w| ..)` method takes [set2023::W](set2023::W) writer structure"]
impl crate::Writable for SET2023 {}
#[doc = "Set for DDI Byte Offsets 20 to 23"]
pub mod set2023;
#[doc = "Set for DDI Byte Offsets 24 to 27\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set2427](set2427) module"]
pub type SET2427 = crate::Reg<u32, _SET2427>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET2427;
#[doc = "`write(|w| ..)` method takes [set2427::W](set2427::W) writer structure"]
impl crate::Writable for SET2427 {}
#[doc = "Set for DDI Byte Offsets 24 to 27"]
pub mod set2427;
#[doc = "Set for DDI Byte Offsets 28 to 31\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set2831](set2831) module"]
pub type SET2831 = crate::Reg<u32, _SET2831>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET2831;
#[doc = "`write(|w| ..)` method takes [set2831::W](set2831::W) writer structure"]
impl crate::Writable for SET2831 {}
#[doc = "Set for DDI Byte Offsets 28 to 31"]
pub mod set2831;
#[doc = "Set for DDI Byte Offsets 32 to 35\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set3235](set3235) module"]
pub type SET3235 = crate::Reg<u32, _SET3235>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET3235;
#[doc = "`write(|w| ..)` method takes [set3235::W](set3235::W) writer structure"]
impl crate::Writable for SET3235 {}
#[doc = "Set for DDI Byte Offsets 32 to 35"]
pub mod set3235;
#[doc = "Set for DDI Byte Offsets 36 to 39\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set3639](set3639) module"]
pub type SET3639 = crate::Reg<u32, _SET3639>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET3639;
#[doc = "`write(|w| ..)` method takes [set3639::W](set3639::W) writer structure"]
impl crate::Writable for SET3639 {}
#[doc = "Set for DDI Byte Offsets 36 to 39"]
pub mod set3639;
#[doc = "Set for DDI Byte Offsets 40 to 43\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set4043](set4043) module"]
pub type SET4043 = crate::Reg<u32, _SET4043>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET4043;
#[doc = "`write(|w| ..)` method takes [set4043::W](set4043::W) writer structure"]
impl crate::Writable for SET4043 {}
#[doc = "Set for DDI Byte Offsets 40 to 43"]
pub mod set4043;
#[doc = "Set for DDI Byte Offsets 44 to 47\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set4447](set4447) module"]
pub type SET4447 = crate::Reg<u32, _SET4447>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET4447;
#[doc = "`write(|w| ..)` method takes [set4447::W](set4447::W) writer structure"]
impl crate::Writable for SET4447 {}
#[doc = "Set for DDI Byte Offsets 44 to 47"]
pub mod set4447;
#[doc = "Set for DDI Byte Offsets 48 to 51\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set4851](set4851) module"]
pub type SET4851 = crate::Reg<u32, _SET4851>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET4851;
#[doc = "`write(|w| ..)` method takes [set4851::W](set4851::W) writer structure"]
impl crate::Writable for SET4851 {}
#[doc = "Set for DDI Byte Offsets 48 to 51"]
pub mod set4851;
#[doc = "Set for DDI Byte Offsets 52 to 55\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set5255](set5255) module"]
pub type SET5255 = crate::Reg<u32, _SET5255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET5255;
#[doc = "`write(|w| ..)` method takes [set5255::W](set5255::W) writer structure"]
impl crate::Writable for SET5255 {}
#[doc = "Set for DDI Byte Offsets 52 to 55"]
pub mod set5255;
#[doc = "Set for DDI Byte Offsets 56 to 59\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set5659](set5659) module"]
pub type SET5659 = crate::Reg<u32, _SET5659>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET5659;
#[doc = "`write(|w| ..)` method takes [set5659::W](set5659::W) writer structure"]
impl crate::Writable for SET5659 {}
#[doc = "Set for DDI Byte Offsets 56 to 59"]
pub mod set5659;
#[doc = "Set for DDI Byte Offsets 60 to 63\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set6063](set6063) module"]
pub type SET6063 = crate::Reg<u32, _SET6063>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET6063;
#[doc = "`write(|w| ..)` method takes [set6063::W](set6063::W) writer structure"]
impl crate::Writable for SET6063 {}
#[doc = "Set for DDI Byte Offsets 60 to 63"]
pub mod set6063;
#[doc = "Set for DDI Byte Offsets 64 to 67\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set6467](set6467) module"]
pub type SET6467 = crate::Reg<u32, _SET6467>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET6467;
#[doc = "`write(|w| ..)` method takes [set6467::W](set6467::W) writer structure"]
impl crate::Writable for SET6467 {}
#[doc = "Set for DDI Byte Offsets 64 to 67"]
pub mod set6467;
#[doc = "Set for DDI Byte Offsets 68 to 71\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set6871](set6871) module"]
pub type SET6871 = crate::Reg<u32, _SET6871>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET6871;
#[doc = "`write(|w| ..)` method takes [set6871::W](set6871::W) writer structure"]
impl crate::Writable for SET6871 {}
#[doc = "Set for DDI Byte Offsets 68 to 71"]
pub mod set6871;
#[doc = "Clear for DDI Byte Offsets 0 to 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr03](clr03) module"]
pub type CLR03 = crate::Reg<u32, _CLR03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR03;
#[doc = "`write(|w| ..)` method takes [clr03::W](clr03::W) writer structure"]
impl crate::Writable for CLR03 {}
#[doc = "Clear for DDI Byte Offsets 0 to 3"]
pub mod clr03;
#[doc = "Clear for DDI Byte Offsets 4 to 7\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr47](clr47) module"]
pub type CLR47 = crate::Reg<u32, _CLR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR47;
#[doc = "`write(|w| ..)` method takes [clr47::W](clr47::W) writer structure"]
impl crate::Writable for CLR47 {}
#[doc = "Clear for DDI Byte Offsets 4 to 7"]
pub mod clr47;
#[doc = "Clear for DDI Byte Offsets 8 to 11\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr811](clr811) module"]
pub type CLR811 = crate::Reg<u32, _CLR811>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR811;
#[doc = "`write(|w| ..)` method takes [clr811::W](clr811::W) writer structure"]
impl crate::Writable for CLR811 {}
#[doc = "Clear for DDI Byte Offsets 8 to 11"]
pub mod clr811;
#[doc = "Clear for DDI Byte Offsets 12 to 15\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr1215](clr1215) module"]
pub type CLR1215 = crate::Reg<u32, _CLR1215>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR1215;
#[doc = "`write(|w| ..)` method takes [clr1215::W](clr1215::W) writer structure"]
impl crate::Writable for CLR1215 {}
#[doc = "Clear for DDI Byte Offsets 12 to 15"]
pub mod clr1215;
#[doc = "Clear for DDI Byte Offsets 16 to 19\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr1619](clr1619) module"]
pub type CLR1619 = crate::Reg<u32, _CLR1619>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR1619;
#[doc = "`write(|w| ..)` method takes [clr1619::W](clr1619::W) writer structure"]
impl crate::Writable for CLR1619 {}
#[doc = "Clear for DDI Byte Offsets 16 to 19"]
pub mod clr1619;
#[doc = "Clear for DDI Byte Offsets 20 to 23\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr2023](clr2023) module"]
pub type CLR2023 = crate::Reg<u32, _CLR2023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR2023;
#[doc = "`write(|w| ..)` method takes [clr2023::W](clr2023::W) writer structure"]
impl crate::Writable for CLR2023 {}
#[doc = "Clear for DDI Byte Offsets 20 to 23"]
pub mod clr2023;
#[doc = "Clear for DDI Byte Offsets 24 to 27\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr2427](clr2427) module"]
pub type CLR2427 = crate::Reg<u32, _CLR2427>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR2427;
#[doc = "`write(|w| ..)` method takes [clr2427::W](clr2427::W) writer structure"]
impl crate::Writable for CLR2427 {}
#[doc = "Clear for DDI Byte Offsets 24 to 27"]
pub mod clr2427;
#[doc = "Clear for DDI Byte Offsets 28 to 31\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr2831](clr2831) module"]
pub type CLR2831 = crate::Reg<u32, _CLR2831>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR2831;
#[doc = "`write(|w| ..)` method takes [clr2831::W](clr2831::W) writer structure"]
impl crate::Writable for CLR2831 {}
#[doc = "Clear for DDI Byte Offsets 28 to 31"]
pub mod clr2831;
#[doc = "Clear for DDI Byte Offsets 32 to 35\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr3235](clr3235) module"]
pub type CLR3235 = crate::Reg<u32, _CLR3235>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR3235;
#[doc = "`write(|w| ..)` method takes [clr3235::W](clr3235::W) writer structure"]
impl crate::Writable for CLR3235 {}
#[doc = "Clear for DDI Byte Offsets 32 to 35"]
pub mod clr3235;
#[doc = "Clear for DDI Byte Offsets 36 to 39\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr3639](clr3639) module"]
pub type CLR3639 = crate::Reg<u32, _CLR3639>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR3639;
#[doc = "`write(|w| ..)` method takes [clr3639::W](clr3639::W) writer structure"]
impl crate::Writable for CLR3639 {}
#[doc = "Clear for DDI Byte Offsets 36 to 39"]
pub mod clr3639;
#[doc = "Clear for DDI Byte Offsets 40 to 43\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr4043](clr4043) module"]
pub type CLR4043 = crate::Reg<u32, _CLR4043>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR4043;
#[doc = "`write(|w| ..)` method takes [clr4043::W](clr4043::W) writer structure"]
impl crate::Writable for CLR4043 {}
#[doc = "Clear for DDI Byte Offsets 40 to 43"]
pub mod clr4043;
#[doc = "Clear for DDI Byte Offsets 44 to 47\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr4447](clr4447) module"]
pub type CLR4447 = crate::Reg<u32, _CLR4447>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR4447;
#[doc = "`write(|w| ..)` method takes [clr4447::W](clr4447::W) writer structure"]
impl crate::Writable for CLR4447 {}
#[doc = "Clear for DDI Byte Offsets 44 to 47"]
pub mod clr4447;
#[doc = "Clear for DDI Byte Offsets 48 to 51\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr4851](clr4851) module"]
pub type CLR4851 = crate::Reg<u32, _CLR4851>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR4851;
#[doc = "`write(|w| ..)` method takes [clr4851::W](clr4851::W) writer structure"]
impl crate::Writable for CLR4851 {}
#[doc = "Clear for DDI Byte Offsets 48 to 51"]
pub mod clr4851;
#[doc = "Clear for DDI Byte Offsets 52 to 55\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr5255](clr5255) module"]
pub type CLR5255 = crate::Reg<u32, _CLR5255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR5255;
#[doc = "`write(|w| ..)` method takes [clr5255::W](clr5255::W) writer structure"]
impl crate::Writable for CLR5255 {}
#[doc = "Clear for DDI Byte Offsets 52 to 55"]
pub mod clr5255;
#[doc = "Clear for DDI Byte Offsets 56 to 59\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr5659](clr5659) module"]
pub type CLR5659 = crate::Reg<u32, _CLR5659>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR5659;
#[doc = "`write(|w| ..)` method takes [clr5659::W](clr5659::W) writer structure"]
impl crate::Writable for CLR5659 {}
#[doc = "Clear for DDI Byte Offsets 56 to 59"]
pub mod clr5659;
#[doc = "Clear for DDI Byte Offsets 60 to 63\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr6063](clr6063) module"]
pub type CLR6063 = crate::Reg<u32, _CLR6063>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR6063;
#[doc = "`write(|w| ..)` method takes [clr6063::W](clr6063::W) writer structure"]
impl crate::Writable for CLR6063 {}
#[doc = "Clear for DDI Byte Offsets 60 to 63"]
pub mod clr6063;
#[doc = "Clear for DDI Byte Offsets 64 to 67\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr6467](clr6467) module"]
pub type CLR6467 = crate::Reg<u32, _CLR6467>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR6467;
#[doc = "`write(|w| ..)` method takes [clr6467::W](clr6467::W) writer structure"]
impl crate::Writable for CLR6467 {}
#[doc = "Clear for DDI Byte Offsets 64 to 67"]
pub mod clr6467;
#[doc = "Clear for DDI Byte Offsets 68 to 71\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr6871](clr6871) module"]
pub type CLR6871 = crate::Reg<u32, _CLR6871>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR6871;
#[doc = "`write(|w| ..)` method takes [clr6871::W](clr6871::W) writer structure"]
impl crate::Writable for CLR6871 {}
#[doc = "Clear for DDI Byte Offsets 68 to 71"]
pub mod clr6871;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod sync;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cfg;
#[doc = "Masked Access (4m/4d), Byte Offsets 0 and 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b01](mask4b01) module"]
pub type MASK4B01 = crate::Reg<u32, _MASK4B01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B01;
#[doc = "`write(|w| ..)` method takes [mask4b01::W](mask4b01::W) writer structure"]
impl crate::Writable for MASK4B01 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 0 and 1"]
pub mod mask4b01;
#[doc = "Masked Access (4m/4d), Byte Offsets 2 and 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b23](mask4b23) module"]
pub type MASK4B23 = crate::Reg<u32, _MASK4B23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B23;
#[doc = "`write(|w| ..)` method takes [mask4b23::W](mask4b23::W) writer structure"]
impl crate::Writable for MASK4B23 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 2 and 3"]
pub mod mask4b23;
#[doc = "Masked Access (4m/4d), Byte Offsets 4 and 5\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b45](mask4b45) module"]
pub type MASK4B45 = crate::Reg<u32, _MASK4B45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B45;
#[doc = "`write(|w| ..)` method takes [mask4b45::W](mask4b45::W) writer structure"]
impl crate::Writable for MASK4B45 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 4 and 5"]
pub mod mask4b45;
#[doc = "Masked Access (4m/4d), Byte Offsets 6 and 7\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b67](mask4b67) module"]
pub type MASK4B67 = crate::Reg<u32, _MASK4B67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B67;
#[doc = "`write(|w| ..)` method takes [mask4b67::W](mask4b67::W) writer structure"]
impl crate::Writable for MASK4B67 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 6 and 7"]
pub mod mask4b67;
#[doc = "Masked Access (4m/4d), Byte Offsets 8 and 9\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b89](mask4b89) module"]
pub type MASK4B89 = crate::Reg<u32, _MASK4B89>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B89;
#[doc = "`write(|w| ..)` method takes [mask4b89::W](mask4b89::W) writer structure"]
impl crate::Writable for MASK4B89 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 8 and 9"]
pub mod mask4b89;
#[doc = "Masked Access (4m/4d), Byte Offsets 10 and 11\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b1011](mask4b1011) module"]
pub type MASK4B1011 = crate::Reg<u32, _MASK4B1011>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B1011;
#[doc = "`write(|w| ..)` method takes [mask4b1011::W](mask4b1011::W) writer structure"]
impl crate::Writable for MASK4B1011 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 10 and 11"]
pub mod mask4b1011;
#[doc = "Masked Access (4m/4d), Byte Offsets 12 and 13\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b1213](mask4b1213) module"]
pub type MASK4B1213 = crate::Reg<u32, _MASK4B1213>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B1213;
#[doc = "`write(|w| ..)` method takes [mask4b1213::W](mask4b1213::W) writer structure"]
impl crate::Writable for MASK4B1213 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 12 and 13"]
pub mod mask4b1213;
#[doc = "Masked Access (4m/4d), Byte Offsets 14 and 15\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b1415](mask4b1415) module"]
pub type MASK4B1415 = crate::Reg<u32, _MASK4B1415>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B1415;
#[doc = "`write(|w| ..)` method takes [mask4b1415::W](mask4b1415::W) writer structure"]
impl crate::Writable for MASK4B1415 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 14 and 15"]
pub mod mask4b1415;
#[doc = "Masked Access (4m/4d), Byte Offsets 16 and 17\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b1617](mask4b1617) module"]
pub type MASK4B1617 = crate::Reg<u32, _MASK4B1617>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B1617;
#[doc = "`write(|w| ..)` method takes [mask4b1617::W](mask4b1617::W) writer structure"]
impl crate::Writable for MASK4B1617 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 16 and 17"]
pub mod mask4b1617;
#[doc = "Masked Access (4m/4d), Byte Offsets 18 and 19\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b1819](mask4b1819) module"]
pub type MASK4B1819 = crate::Reg<u32, _MASK4B1819>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B1819;
#[doc = "`write(|w| ..)` method takes [mask4b1819::W](mask4b1819::W) writer structure"]
impl crate::Writable for MASK4B1819 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 18 and 19"]
pub mod mask4b1819;
#[doc = "Masked Access (4m/4d), Byte Offsets 20 and 21\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b2021](mask4b2021) module"]
pub type MASK4B2021 = crate::Reg<u32, _MASK4B2021>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B2021;
#[doc = "`write(|w| ..)` method takes [mask4b2021::W](mask4b2021::W) writer structure"]
impl crate::Writable for MASK4B2021 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 20 and 21"]
pub mod mask4b2021;
#[doc = "Masked Access (4m/4d), Byte Offsets 22 and 23\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b2223](mask4b2223) module"]
pub type MASK4B2223 = crate::Reg<u32, _MASK4B2223>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B2223;
#[doc = "`write(|w| ..)` method takes [mask4b2223::W](mask4b2223::W) writer structure"]
impl crate::Writable for MASK4B2223 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 22 and 23"]
pub mod mask4b2223;
#[doc = "Masked Access (4m/4d), Byte Offsets 24 and 25\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b2425](mask4b2425) module"]
pub type MASK4B2425 = crate::Reg<u32, _MASK4B2425>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B2425;
#[doc = "`write(|w| ..)` method takes [mask4b2425::W](mask4b2425::W) writer structure"]
impl crate::Writable for MASK4B2425 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 24 and 25"]
pub mod mask4b2425;
#[doc = "Masked Access (4m/4d), Byte Offsets 26 and 27\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b2627](mask4b2627) module"]
pub type MASK4B2627 = crate::Reg<u32, _MASK4B2627>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B2627;
#[doc = "`write(|w| ..)` method takes [mask4b2627::W](mask4b2627::W) writer structure"]
impl crate::Writable for MASK4B2627 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 26 and 27"]
pub mod mask4b2627;
#[doc = "Masked Access (4m/4d), Byte Offsets 28 and 29\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b2829](mask4b2829) module"]
pub type MASK4B2829 = crate::Reg<u32, _MASK4B2829>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B2829;
#[doc = "`write(|w| ..)` method takes [mask4b2829::W](mask4b2829::W) writer structure"]
impl crate::Writable for MASK4B2829 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 28 and 29"]
pub mod mask4b2829;
#[doc = "Masked Access (4m/4d), Byte Offsets 30 and 31\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b3031](mask4b3031) module"]
pub type MASK4B3031 = crate::Reg<u32, _MASK4B3031>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B3031;
#[doc = "`write(|w| ..)` method takes [mask4b3031::W](mask4b3031::W) writer structure"]
impl crate::Writable for MASK4B3031 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 30 and 31"]
pub mod mask4b3031;
#[doc = "Masked Access (4m/4d), Byte Offsets 32 and 33\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b3233](mask4b3233) module"]
pub type MASK4B3233 = crate::Reg<u32, _MASK4B3233>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B3233;
#[doc = "`write(|w| ..)` method takes [mask4b3233::W](mask4b3233::W) writer structure"]
impl crate::Writable for MASK4B3233 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 32 and 33"]
pub mod mask4b3233;
#[doc = "Masked Access (4m/4d), Byte Offsets 34 and 35\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b3435](mask4b3435) module"]
pub type MASK4B3435 = crate::Reg<u32, _MASK4B3435>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B3435;
#[doc = "`write(|w| ..)` method takes [mask4b3435::W](mask4b3435::W) writer structure"]
impl crate::Writable for MASK4B3435 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 34 and 35"]
pub mod mask4b3435;
#[doc = "Masked Access (4m/4d), Byte Offsets 36 and 37\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b3637](mask4b3637) module"]
pub type MASK4B3637 = crate::Reg<u32, _MASK4B3637>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B3637;
#[doc = "`write(|w| ..)` method takes [mask4b3637::W](mask4b3637::W) writer structure"]
impl crate::Writable for MASK4B3637 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 36 and 37"]
pub mod mask4b3637;
#[doc = "Masked Access (4m/4d), Byte Offsets 38 and 39\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b3839](mask4b3839) module"]
pub type MASK4B3839 = crate::Reg<u32, _MASK4B3839>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B3839;
#[doc = "`write(|w| ..)` method takes [mask4b3839::W](mask4b3839::W) writer structure"]
impl crate::Writable for MASK4B3839 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 38 and 39"]
pub mod mask4b3839;
#[doc = "Masked Access (4m/4d), Byte Offsets 40 and 41\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b4041](mask4b4041) module"]
pub type MASK4B4041 = crate::Reg<u32, _MASK4B4041>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B4041;
#[doc = "`write(|w| ..)` method takes [mask4b4041::W](mask4b4041::W) writer structure"]
impl crate::Writable for MASK4B4041 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 40 and 41"]
pub mod mask4b4041;
#[doc = "Masked Access (4m/4d), Byte Offsets 42 and 43\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b4243](mask4b4243) module"]
pub type MASK4B4243 = crate::Reg<u32, _MASK4B4243>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B4243;
#[doc = "`write(|w| ..)` method takes [mask4b4243::W](mask4b4243::W) writer structure"]
impl crate::Writable for MASK4B4243 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 42 and 43"]
pub mod mask4b4243;
#[doc = "Masked Access (4m/4d), Byte Offsets 44 and 45\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b4445](mask4b4445) module"]
pub type MASK4B4445 = crate::Reg<u32, _MASK4B4445>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B4445;
#[doc = "`write(|w| ..)` method takes [mask4b4445::W](mask4b4445::W) writer structure"]
impl crate::Writable for MASK4B4445 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 44 and 45"]
pub mod mask4b4445;
#[doc = "Masked Access (4m/4d), Byte Offsets 46 and 47\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b4647](mask4b4647) module"]
pub type MASK4B4647 = crate::Reg<u32, _MASK4B4647>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B4647;
#[doc = "`write(|w| ..)` method takes [mask4b4647::W](mask4b4647::W) writer structure"]
impl crate::Writable for MASK4B4647 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 46 and 47"]
pub mod mask4b4647;
#[doc = "Masked Access (4m/4d), Byte Offsets 48 and 49\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b4849](mask4b4849) module"]
pub type MASK4B4849 = crate::Reg<u32, _MASK4B4849>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B4849;
#[doc = "`write(|w| ..)` method takes [mask4b4849::W](mask4b4849::W) writer structure"]
impl crate::Writable for MASK4B4849 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 48 and 49"]
pub mod mask4b4849;
#[doc = "Masked Access (4m/4d), Byte Offsets 50 and 51\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b5051](mask4b5051) module"]
pub type MASK4B5051 = crate::Reg<u32, _MASK4B5051>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B5051;
#[doc = "`write(|w| ..)` method takes [mask4b5051::W](mask4b5051::W) writer structure"]
impl crate::Writable for MASK4B5051 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 50 and 51"]
pub mod mask4b5051;
#[doc = "Masked Access (4m/4d), Byte Offsets 52 and 53\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b5253](mask4b5253) module"]
pub type MASK4B5253 = crate::Reg<u32, _MASK4B5253>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B5253;
#[doc = "`write(|w| ..)` method takes [mask4b5253::W](mask4b5253::W) writer structure"]
impl crate::Writable for MASK4B5253 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 52 and 53"]
pub mod mask4b5253;
#[doc = "Masked Access (4m/4d), Byte Offsets 54 and 55\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b5455](mask4b5455) module"]
pub type MASK4B5455 = crate::Reg<u32, _MASK4B5455>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B5455;
#[doc = "`write(|w| ..)` method takes [mask4b5455::W](mask4b5455::W) writer structure"]
impl crate::Writable for MASK4B5455 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 54 and 55"]
pub mod mask4b5455;
#[doc = "Masked Access (4m/4d), Byte Offsets 56 and 57\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b5657](mask4b5657) module"]
pub type MASK4B5657 = crate::Reg<u32, _MASK4B5657>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B5657;
#[doc = "`write(|w| ..)` method takes [mask4b5657::W](mask4b5657::W) writer structure"]
impl crate::Writable for MASK4B5657 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 56 and 57"]
pub mod mask4b5657;
#[doc = "Masked Access (4m/4d), Byte Offsets 58 and 59\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b5859](mask4b5859) module"]
pub type MASK4B5859 = crate::Reg<u32, _MASK4B5859>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B5859;
#[doc = "`write(|w| ..)` method takes [mask4b5859::W](mask4b5859::W) writer structure"]
impl crate::Writable for MASK4B5859 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 58 and 59"]
pub mod mask4b5859;
#[doc = "Masked Access (4m/4d), Byte Offsets 60 and 61\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b6061](mask4b6061) module"]
pub type MASK4B6061 = crate::Reg<u32, _MASK4B6061>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B6061;
#[doc = "`write(|w| ..)` method takes [mask4b6061::W](mask4b6061::W) writer structure"]
impl crate::Writable for MASK4B6061 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 60 and 61"]
pub mod mask4b6061;
#[doc = "Masked Access (4m/4d), Byte Offsets 62 and 63\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b6263](mask4b6263) module"]
pub type MASK4B6263 = crate::Reg<u32, _MASK4B6263>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B6263;
#[doc = "`write(|w| ..)` method takes [mask4b6263::W](mask4b6263::W) writer structure"]
impl crate::Writable for MASK4B6263 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 62 and 63"]
pub mod mask4b6263;
#[doc = "Masked Access (4m/4d), Byte Offsets 64 and 65\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b6465](mask4b6465) module"]
pub type MASK4B6465 = crate::Reg<u32, _MASK4B6465>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B6465;
#[doc = "`write(|w| ..)` method takes [mask4b6465::W](mask4b6465::W) writer structure"]
impl crate::Writable for MASK4B6465 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 64 and 65"]
pub mod mask4b6465;
#[doc = "Masked Access (4m/4d), Byte Offsets 66 and 67\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b6667](mask4b6667) module"]
pub type MASK4B6667 = crate::Reg<u32, _MASK4B6667>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B6667;
#[doc = "`write(|w| ..)` method takes [mask4b6667::W](mask4b6667::W) writer structure"]
impl crate::Writable for MASK4B6667 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 66 and 67"]
pub mod mask4b6667;
#[doc = "Masked Access (4m/4d), Byte Offsets 68 and 69\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b6869](mask4b6869) module"]
pub type MASK4B6869 = crate::Reg<u32, _MASK4B6869>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B6869;
#[doc = "`write(|w| ..)` method takes [mask4b6869::W](mask4b6869::W) writer structure"]
impl crate::Writable for MASK4B6869 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 68 and 69"]
pub mod mask4b6869;
#[doc = "Masked Access (4m/4d), Byte Offsets 70 and 71\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask4b7071](mask4b7071) module"]
pub type MASK4B7071 = crate::Reg<u32, _MASK4B7071>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK4B7071;
#[doc = "`write(|w| ..)` method takes [mask4b7071::W](mask4b7071::W) writer structure"]
impl crate::Writable for MASK4B7071 {}
#[doc = "Masked Access (4m/4d), Byte Offsets 70 and 71"]
pub mod mask4b7071;
#[doc = "Masked Access (8m/8d), Byte Offsets 0 and 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b01](mask8b01) module"]
pub type MASK8B01 = crate::Reg<u32, _MASK8B01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B01;
#[doc = "`write(|w| ..)` method takes [mask8b01::W](mask8b01::W) writer structure"]
impl crate::Writable for MASK8B01 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 0 and 1"]
pub mod mask8b01;
#[doc = "Masked Access (8m/8d), Byte Offsets 2 and 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b23](mask8b23) module"]
pub type MASK8B23 = crate::Reg<u32, _MASK8B23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B23;
#[doc = "`write(|w| ..)` method takes [mask8b23::W](mask8b23::W) writer structure"]
impl crate::Writable for MASK8B23 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 2 and 3"]
pub mod mask8b23;
#[doc = "Masked Access (8m/8d), Byte Offsets 4 and 5\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b45](mask8b45) module"]
pub type MASK8B45 = crate::Reg<u32, _MASK8B45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B45;
#[doc = "`write(|w| ..)` method takes [mask8b45::W](mask8b45::W) writer structure"]
impl crate::Writable for MASK8B45 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 4 and 5"]
pub mod mask8b45;
#[doc = "Masked Access (8m/8d), Byte Offsets 6 and 7\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b67](mask8b67) module"]
pub type MASK8B67 = crate::Reg<u32, _MASK8B67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B67;
#[doc = "`write(|w| ..)` method takes [mask8b67::W](mask8b67::W) writer structure"]
impl crate::Writable for MASK8B67 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 6 and 7"]
pub mod mask8b67;
#[doc = "Masked Access (8m/8d), Byte Offsets 8 and 9\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b89](mask8b89) module"]
pub type MASK8B89 = crate::Reg<u32, _MASK8B89>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B89;
#[doc = "`write(|w| ..)` method takes [mask8b89::W](mask8b89::W) writer structure"]
impl crate::Writable for MASK8B89 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 8 and 9"]
pub mod mask8b89;
#[doc = "Masked Access (8m/8d), Byte Offsets 10 and 11\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b1011](mask8b1011) module"]
pub type MASK8B1011 = crate::Reg<u32, _MASK8B1011>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B1011;
#[doc = "`write(|w| ..)` method takes [mask8b1011::W](mask8b1011::W) writer structure"]
impl crate::Writable for MASK8B1011 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 10 and 11"]
pub mod mask8b1011;
#[doc = "Masked Access (8m/8d), Byte Offsets 12 and 13\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b1213](mask8b1213) module"]
pub type MASK8B1213 = crate::Reg<u32, _MASK8B1213>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B1213;
#[doc = "`write(|w| ..)` method takes [mask8b1213::W](mask8b1213::W) writer structure"]
impl crate::Writable for MASK8B1213 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 12 and 13"]
pub mod mask8b1213;
#[doc = "Masked Access (8m/8d), Byte Offsets 14 and 15\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b1415](mask8b1415) module"]
pub type MASK8B1415 = crate::Reg<u32, _MASK8B1415>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B1415;
#[doc = "`write(|w| ..)` method takes [mask8b1415::W](mask8b1415::W) writer structure"]
impl crate::Writable for MASK8B1415 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 14 and 15"]
pub mod mask8b1415;
#[doc = "Masked Access (8m/8d), Byte Offsets 16 and 17\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b1617](mask8b1617) module"]
pub type MASK8B1617 = crate::Reg<u32, _MASK8B1617>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B1617;
#[doc = "`write(|w| ..)` method takes [mask8b1617::W](mask8b1617::W) writer structure"]
impl crate::Writable for MASK8B1617 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 16 and 17"]
pub mod mask8b1617;
#[doc = "Masked Access (8m/8d), Byte Offsets 18 and 19\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b1819](mask8b1819) module"]
pub type MASK8B1819 = crate::Reg<u32, _MASK8B1819>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B1819;
#[doc = "`write(|w| ..)` method takes [mask8b1819::W](mask8b1819::W) writer structure"]
impl crate::Writable for MASK8B1819 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 18 and 19"]
pub mod mask8b1819;
#[doc = "Masked Access (8m/8d), Byte Offsets 20 and 21\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b2021](mask8b2021) module"]
pub type MASK8B2021 = crate::Reg<u32, _MASK8B2021>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B2021;
#[doc = "`write(|w| ..)` method takes [mask8b2021::W](mask8b2021::W) writer structure"]
impl crate::Writable for MASK8B2021 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 20 and 21"]
pub mod mask8b2021;
#[doc = "Masked Access (8m/8d), Byte Offsets 22 and 23\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b2223](mask8b2223) module"]
pub type MASK8B2223 = crate::Reg<u32, _MASK8B2223>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B2223;
#[doc = "`write(|w| ..)` method takes [mask8b2223::W](mask8b2223::W) writer structure"]
impl crate::Writable for MASK8B2223 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 22 and 23"]
pub mod mask8b2223;
#[doc = "Masked Access (8m/8d), Byte Offsets 24 and 25\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b2425](mask8b2425) module"]
pub type MASK8B2425 = crate::Reg<u32, _MASK8B2425>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B2425;
#[doc = "`write(|w| ..)` method takes [mask8b2425::W](mask8b2425::W) writer structure"]
impl crate::Writable for MASK8B2425 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 24 and 25"]
pub mod mask8b2425;
#[doc = "Masked Access (8m/8d), Byte Offsets 26 and 27\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b2627](mask8b2627) module"]
pub type MASK8B2627 = crate::Reg<u32, _MASK8B2627>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B2627;
#[doc = "`write(|w| ..)` method takes [mask8b2627::W](mask8b2627::W) writer structure"]
impl crate::Writable for MASK8B2627 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 26 and 27"]
pub mod mask8b2627;
#[doc = "Masked Access (8m/8d), Byte Offsets 28 and 29\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b2829](mask8b2829) module"]
pub type MASK8B2829 = crate::Reg<u32, _MASK8B2829>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B2829;
#[doc = "`write(|w| ..)` method takes [mask8b2829::W](mask8b2829::W) writer structure"]
impl crate::Writable for MASK8B2829 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 28 and 29"]
pub mod mask8b2829;
#[doc = "Masked Access (8m/8d), Byte Offsets 30 and 31\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b3031](mask8b3031) module"]
pub type MASK8B3031 = crate::Reg<u32, _MASK8B3031>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B3031;
#[doc = "`write(|w| ..)` method takes [mask8b3031::W](mask8b3031::W) writer structure"]
impl crate::Writable for MASK8B3031 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 30 and 31"]
pub mod mask8b3031;
#[doc = "Masked Access (8m/8d), Byte Offsets 32 and 33\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b3233](mask8b3233) module"]
pub type MASK8B3233 = crate::Reg<u32, _MASK8B3233>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B3233;
#[doc = "`write(|w| ..)` method takes [mask8b3233::W](mask8b3233::W) writer structure"]
impl crate::Writable for MASK8B3233 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 32 and 33"]
pub mod mask8b3233;
#[doc = "Masked Access (8m/8d), Byte Offsets 34 and 35\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b3435](mask8b3435) module"]
pub type MASK8B3435 = crate::Reg<u32, _MASK8B3435>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B3435;
#[doc = "`write(|w| ..)` method takes [mask8b3435::W](mask8b3435::W) writer structure"]
impl crate::Writable for MASK8B3435 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 34 and 35"]
pub mod mask8b3435;
#[doc = "Masked Access (8m/8d), Byte Offsets 36 and 37\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b3637](mask8b3637) module"]
pub type MASK8B3637 = crate::Reg<u32, _MASK8B3637>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B3637;
#[doc = "`write(|w| ..)` method takes [mask8b3637::W](mask8b3637::W) writer structure"]
impl crate::Writable for MASK8B3637 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 36 and 37"]
pub mod mask8b3637;
#[doc = "Masked Access (8m/8d), Byte Offsets 38 and 39\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b3839](mask8b3839) module"]
pub type MASK8B3839 = crate::Reg<u32, _MASK8B3839>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B3839;
#[doc = "`write(|w| ..)` method takes [mask8b3839::W](mask8b3839::W) writer structure"]
impl crate::Writable for MASK8B3839 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 38 and 39"]
pub mod mask8b3839;
#[doc = "Masked Access (8m/8d), Byte Offsets 40 and 41\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b4041](mask8b4041) module"]
pub type MASK8B4041 = crate::Reg<u32, _MASK8B4041>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B4041;
#[doc = "`write(|w| ..)` method takes [mask8b4041::W](mask8b4041::W) writer structure"]
impl crate::Writable for MASK8B4041 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 40 and 41"]
pub mod mask8b4041;
#[doc = "Masked Access (8m/8d), Byte Offsets 42 and 43\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b4243](mask8b4243) module"]
pub type MASK8B4243 = crate::Reg<u32, _MASK8B4243>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B4243;
#[doc = "`write(|w| ..)` method takes [mask8b4243::W](mask8b4243::W) writer structure"]
impl crate::Writable for MASK8B4243 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 42 and 43"]
pub mod mask8b4243;
#[doc = "Masked Access (8m/8d), Byte Offsets 44 and 45\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b4445](mask8b4445) module"]
pub type MASK8B4445 = crate::Reg<u32, _MASK8B4445>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B4445;
#[doc = "`write(|w| ..)` method takes [mask8b4445::W](mask8b4445::W) writer structure"]
impl crate::Writable for MASK8B4445 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 44 and 45"]
pub mod mask8b4445;
#[doc = "Masked Access (8m/8d), Byte Offsets 46 and 47\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b4647](mask8b4647) module"]
pub type MASK8B4647 = crate::Reg<u32, _MASK8B4647>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B4647;
#[doc = "`write(|w| ..)` method takes [mask8b4647::W](mask8b4647::W) writer structure"]
impl crate::Writable for MASK8B4647 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 46 and 47"]
pub mod mask8b4647;
#[doc = "Masked Access (8m/8d), Byte Offsets 48 and 49\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b4849](mask8b4849) module"]
pub type MASK8B4849 = crate::Reg<u32, _MASK8B4849>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B4849;
#[doc = "`write(|w| ..)` method takes [mask8b4849::W](mask8b4849::W) writer structure"]
impl crate::Writable for MASK8B4849 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 48 and 49"]
pub mod mask8b4849;
#[doc = "Masked Access (8m/8d), Byte Offsets 50 and 51\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b5051](mask8b5051) module"]
pub type MASK8B5051 = crate::Reg<u32, _MASK8B5051>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B5051;
#[doc = "`write(|w| ..)` method takes [mask8b5051::W](mask8b5051::W) writer structure"]
impl crate::Writable for MASK8B5051 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 50 and 51"]
pub mod mask8b5051;
#[doc = "Masked Access (8m/8d), Byte Offsets 52 and 53\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b5253](mask8b5253) module"]
pub type MASK8B5253 = crate::Reg<u32, _MASK8B5253>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B5253;
#[doc = "`write(|w| ..)` method takes [mask8b5253::W](mask8b5253::W) writer structure"]
impl crate::Writable for MASK8B5253 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 52 and 53"]
pub mod mask8b5253;
#[doc = "Masked Access (8m/8d), Byte Offsets 54 and 55\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b5455](mask8b5455) module"]
pub type MASK8B5455 = crate::Reg<u32, _MASK8B5455>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B5455;
#[doc = "`write(|w| ..)` method takes [mask8b5455::W](mask8b5455::W) writer structure"]
impl crate::Writable for MASK8B5455 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 54 and 55"]
pub mod mask8b5455;
#[doc = "Masked Access (8m/8d), Byte Offsets 56 and 57\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b5657](mask8b5657) module"]
pub type MASK8B5657 = crate::Reg<u32, _MASK8B5657>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B5657;
#[doc = "`write(|w| ..)` method takes [mask8b5657::W](mask8b5657::W) writer structure"]
impl crate::Writable for MASK8B5657 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 56 and 57"]
pub mod mask8b5657;
#[doc = "Masked Access (8m/8d), Byte Offsets 58 and 59\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b5859](mask8b5859) module"]
pub type MASK8B5859 = crate::Reg<u32, _MASK8B5859>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B5859;
#[doc = "`write(|w| ..)` method takes [mask8b5859::W](mask8b5859::W) writer structure"]
impl crate::Writable for MASK8B5859 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 58 and 59"]
pub mod mask8b5859;
#[doc = "Masked Access (8m/8d), Byte Offsets 60 and 61\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b6061](mask8b6061) module"]
pub type MASK8B6061 = crate::Reg<u32, _MASK8B6061>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B6061;
#[doc = "`write(|w| ..)` method takes [mask8b6061::W](mask8b6061::W) writer structure"]
impl crate::Writable for MASK8B6061 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 60 and 61"]
pub mod mask8b6061;
#[doc = "Masked Access (8m/8d), Byte Offsets 62 and 63\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b6263](mask8b6263) module"]
pub type MASK8B6263 = crate::Reg<u32, _MASK8B6263>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B6263;
#[doc = "`write(|w| ..)` method takes [mask8b6263::W](mask8b6263::W) writer structure"]
impl crate::Writable for MASK8B6263 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 62 and 63"]
pub mod mask8b6263;
#[doc = "Masked Access (8m/8d), Byte Offsets 64 and 65\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b6465](mask8b6465) module"]
pub type MASK8B6465 = crate::Reg<u32, _MASK8B6465>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B6465;
#[doc = "`write(|w| ..)` method takes [mask8b6465::W](mask8b6465::W) writer structure"]
impl crate::Writable for MASK8B6465 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 64 and 65"]
pub mod mask8b6465;
#[doc = "Masked Access (8m/8d), Byte Offsets 66 and 67\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b6667](mask8b6667) module"]
pub type MASK8B6667 = crate::Reg<u32, _MASK8B6667>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B6667;
#[doc = "`write(|w| ..)` method takes [mask8b6667::W](mask8b6667::W) writer structure"]
impl crate::Writable for MASK8B6667 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 66 and 67"]
pub mod mask8b6667;
#[doc = "Masked Access (8m/8d), Byte Offsets 68 and 69\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b6869](mask8b6869) module"]
pub type MASK8B6869 = crate::Reg<u32, _MASK8B6869>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B6869;
#[doc = "`write(|w| ..)` method takes [mask8b6869::W](mask8b6869::W) writer structure"]
impl crate::Writable for MASK8B6869 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 68 and 69"]
pub mod mask8b6869;
#[doc = "Masked Access (8m/8d), Byte Offsets 70 and 71\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask8b7071](mask8b7071) module"]
pub type MASK8B7071 = crate::Reg<u32, _MASK8B7071>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK8B7071;
#[doc = "`write(|w| ..)` method takes [mask8b7071::W](mask8b7071::W) writer structure"]
impl crate::Writable for MASK8B7071 {}
#[doc = "Masked Access (8m/8d), Byte Offsets 70 and 71"]
pub mod mask8b7071;
#[doc = "Masked Access (16m/16d), Byte Offsets 0 and 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b01](mask16b01) module"]
pub type MASK16B01 = crate::Reg<u32, _MASK16B01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B01;
#[doc = "`write(|w| ..)` method takes [mask16b01::W](mask16b01::W) writer structure"]
impl crate::Writable for MASK16B01 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 0 and 1"]
pub mod mask16b01;
#[doc = "Masked Access (16m/16d), Byte Offsets 2 and 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b23](mask16b23) module"]
pub type MASK16B23 = crate::Reg<u32, _MASK16B23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B23;
#[doc = "`write(|w| ..)` method takes [mask16b23::W](mask16b23::W) writer structure"]
impl crate::Writable for MASK16B23 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 2 and 3"]
pub mod mask16b23;
#[doc = "Masked Access (16m/16d), Byte Offsets 4 and 5\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b45](mask16b45) module"]
pub type MASK16B45 = crate::Reg<u32, _MASK16B45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B45;
#[doc = "`write(|w| ..)` method takes [mask16b45::W](mask16b45::W) writer structure"]
impl crate::Writable for MASK16B45 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 4 and 5"]
pub mod mask16b45;
#[doc = "Masked Access (16m/16d), Byte Offsets 6 and 7\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b67](mask16b67) module"]
pub type MASK16B67 = crate::Reg<u32, _MASK16B67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B67;
#[doc = "`write(|w| ..)` method takes [mask16b67::W](mask16b67::W) writer structure"]
impl crate::Writable for MASK16B67 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 6 and 7"]
pub mod mask16b67;
#[doc = "Masked Access (16m/16d), Byte Offsets 8 and 9\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b89](mask16b89) module"]
pub type MASK16B89 = crate::Reg<u32, _MASK16B89>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B89;
#[doc = "`write(|w| ..)` method takes [mask16b89::W](mask16b89::W) writer structure"]
impl crate::Writable for MASK16B89 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 8 and 9"]
pub mod mask16b89;
#[doc = "Masked Access (16m/16d), Byte Offsets 10 and 11\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b1011](mask16b1011) module"]
pub type MASK16B1011 = crate::Reg<u32, _MASK16B1011>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B1011;
#[doc = "`write(|w| ..)` method takes [mask16b1011::W](mask16b1011::W) writer structure"]
impl crate::Writable for MASK16B1011 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 10 and 11"]
pub mod mask16b1011;
#[doc = "Masked Access (16m/16d), Byte Offsets 12 and 13\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b1213](mask16b1213) module"]
pub type MASK16B1213 = crate::Reg<u32, _MASK16B1213>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B1213;
#[doc = "`write(|w| ..)` method takes [mask16b1213::W](mask16b1213::W) writer structure"]
impl crate::Writable for MASK16B1213 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 12 and 13"]
pub mod mask16b1213;
#[doc = "Masked Access (16m/16d), Byte Offsets 14 and 15\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b1415](mask16b1415) module"]
pub type MASK16B1415 = crate::Reg<u32, _MASK16B1415>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B1415;
#[doc = "`write(|w| ..)` method takes [mask16b1415::W](mask16b1415::W) writer structure"]
impl crate::Writable for MASK16B1415 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 14 and 15"]
pub mod mask16b1415;
#[doc = "Masked Access (16m/16d), Byte Offsets 16 and 17\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b1617](mask16b1617) module"]
pub type MASK16B1617 = crate::Reg<u32, _MASK16B1617>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B1617;
#[doc = "`write(|w| ..)` method takes [mask16b1617::W](mask16b1617::W) writer structure"]
impl crate::Writable for MASK16B1617 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 16 and 17"]
pub mod mask16b1617;
#[doc = "Masked Access (16m/16d), Byte Offsets 18 and 19\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b1819](mask16b1819) module"]
pub type MASK16B1819 = crate::Reg<u32, _MASK16B1819>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B1819;
#[doc = "`write(|w| ..)` method takes [mask16b1819::W](mask16b1819::W) writer structure"]
impl crate::Writable for MASK16B1819 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 18 and 19"]
pub mod mask16b1819;
#[doc = "Masked Access (16m/16d), Byte Offsets 20 and 21\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b2021](mask16b2021) module"]
pub type MASK16B2021 = crate::Reg<u32, _MASK16B2021>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B2021;
#[doc = "`write(|w| ..)` method takes [mask16b2021::W](mask16b2021::W) writer structure"]
impl crate::Writable for MASK16B2021 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 20 and 21"]
pub mod mask16b2021;
#[doc = "Masked Access (16m/16d), Byte Offsets 22 and 23\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b2223](mask16b2223) module"]
pub type MASK16B2223 = crate::Reg<u32, _MASK16B2223>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B2223;
#[doc = "`write(|w| ..)` method takes [mask16b2223::W](mask16b2223::W) writer structure"]
impl crate::Writable for MASK16B2223 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 22 and 23"]
pub mod mask16b2223;
#[doc = "Masked Access (16m/16d), Byte Offsets 24 and 25\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b2425](mask16b2425) module"]
pub type MASK16B2425 = crate::Reg<u32, _MASK16B2425>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B2425;
#[doc = "`write(|w| ..)` method takes [mask16b2425::W](mask16b2425::W) writer structure"]
impl crate::Writable for MASK16B2425 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 24 and 25"]
pub mod mask16b2425;
#[doc = "Masked Access (16m/16d), Byte Offsets 26 and 27\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b2627](mask16b2627) module"]
pub type MASK16B2627 = crate::Reg<u32, _MASK16B2627>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B2627;
#[doc = "`write(|w| ..)` method takes [mask16b2627::W](mask16b2627::W) writer structure"]
impl crate::Writable for MASK16B2627 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 26 and 27"]
pub mod mask16b2627;
#[doc = "Masked Access (16m/16d), Byte Offsets 28 and 29\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b2829](mask16b2829) module"]
pub type MASK16B2829 = crate::Reg<u32, _MASK16B2829>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B2829;
#[doc = "`write(|w| ..)` method takes [mask16b2829::W](mask16b2829::W) writer structure"]
impl crate::Writable for MASK16B2829 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 28 and 29"]
pub mod mask16b2829;
#[doc = "Masked Access (16m/16d), Byte Offsets 30 and 31\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b3031](mask16b3031) module"]
pub type MASK16B3031 = crate::Reg<u32, _MASK16B3031>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B3031;
#[doc = "`write(|w| ..)` method takes [mask16b3031::W](mask16b3031::W) writer structure"]
impl crate::Writable for MASK16B3031 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 30 and 31"]
pub mod mask16b3031;
#[doc = "Masked Access (16m/16d), Byte Offsets 32 and 33\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b3233](mask16b3233) module"]
pub type MASK16B3233 = crate::Reg<u32, _MASK16B3233>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B3233;
#[doc = "`write(|w| ..)` method takes [mask16b3233::W](mask16b3233::W) writer structure"]
impl crate::Writable for MASK16B3233 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 32 and 33"]
pub mod mask16b3233;
#[doc = "Masked Access (16m/16d), Byte Offsets 34 and 35\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b3435](mask16b3435) module"]
pub type MASK16B3435 = crate::Reg<u32, _MASK16B3435>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B3435;
#[doc = "`write(|w| ..)` method takes [mask16b3435::W](mask16b3435::W) writer structure"]
impl crate::Writable for MASK16B3435 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 34 and 35"]
pub mod mask16b3435;
#[doc = "Masked Access (16m/16d), Byte Offsets 36 and 37\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b3637](mask16b3637) module"]
pub type MASK16B3637 = crate::Reg<u32, _MASK16B3637>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B3637;
#[doc = "`write(|w| ..)` method takes [mask16b3637::W](mask16b3637::W) writer structure"]
impl crate::Writable for MASK16B3637 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 36 and 37"]
pub mod mask16b3637;
#[doc = "Masked Access (16m/16d), Byte Offsets 38 and 39\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b3839](mask16b3839) module"]
pub type MASK16B3839 = crate::Reg<u32, _MASK16B3839>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B3839;
#[doc = "`write(|w| ..)` method takes [mask16b3839::W](mask16b3839::W) writer structure"]
impl crate::Writable for MASK16B3839 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 38 and 39"]
pub mod mask16b3839;
#[doc = "Masked Access (16m/16d), Byte Offsets 40 and 41\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b4041](mask16b4041) module"]
pub type MASK16B4041 = crate::Reg<u32, _MASK16B4041>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B4041;
#[doc = "`write(|w| ..)` method takes [mask16b4041::W](mask16b4041::W) writer structure"]
impl crate::Writable for MASK16B4041 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 40 and 41"]
pub mod mask16b4041;
#[doc = "Masked Access (16m/16d), Byte Offsets 42 and 43\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b4243](mask16b4243) module"]
pub type MASK16B4243 = crate::Reg<u32, _MASK16B4243>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B4243;
#[doc = "`write(|w| ..)` method takes [mask16b4243::W](mask16b4243::W) writer structure"]
impl crate::Writable for MASK16B4243 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 42 and 43"]
pub mod mask16b4243;
#[doc = "Masked Access (16m/16d), Byte Offsets 44 and 45\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b4445](mask16b4445) module"]
pub type MASK16B4445 = crate::Reg<u32, _MASK16B4445>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B4445;
#[doc = "`write(|w| ..)` method takes [mask16b4445::W](mask16b4445::W) writer structure"]
impl crate::Writable for MASK16B4445 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 44 and 45"]
pub mod mask16b4445;
#[doc = "Masked Access (16m/16d), Byte Offsets 46 and 47\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b4647](mask16b4647) module"]
pub type MASK16B4647 = crate::Reg<u32, _MASK16B4647>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B4647;
#[doc = "`write(|w| ..)` method takes [mask16b4647::W](mask16b4647::W) writer structure"]
impl crate::Writable for MASK16B4647 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 46 and 47"]
pub mod mask16b4647;
#[doc = "Masked Access (16m/16d), Byte Offsets 48 and 49\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b4849](mask16b4849) module"]
pub type MASK16B4849 = crate::Reg<u32, _MASK16B4849>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B4849;
#[doc = "`write(|w| ..)` method takes [mask16b4849::W](mask16b4849::W) writer structure"]
impl crate::Writable for MASK16B4849 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 48 and 49"]
pub mod mask16b4849;
#[doc = "Masked Access (16m/16d), Byte Offsets 50 and 51\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b5051](mask16b5051) module"]
pub type MASK16B5051 = crate::Reg<u32, _MASK16B5051>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B5051;
#[doc = "`write(|w| ..)` method takes [mask16b5051::W](mask16b5051::W) writer structure"]
impl crate::Writable for MASK16B5051 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 50 and 51"]
pub mod mask16b5051;
#[doc = "Masked Access (16m/16d), Byte Offsets 52 and 53\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b5253](mask16b5253) module"]
pub type MASK16B5253 = crate::Reg<u32, _MASK16B5253>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B5253;
#[doc = "`write(|w| ..)` method takes [mask16b5253::W](mask16b5253::W) writer structure"]
impl crate::Writable for MASK16B5253 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 52 and 53"]
pub mod mask16b5253;
#[doc = "Masked Access (16m/16d), Byte Offsets 54 and 55\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b5455](mask16b5455) module"]
pub type MASK16B5455 = crate::Reg<u32, _MASK16B5455>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B5455;
#[doc = "`write(|w| ..)` method takes [mask16b5455::W](mask16b5455::W) writer structure"]
impl crate::Writable for MASK16B5455 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 54 and 55"]
pub mod mask16b5455;
#[doc = "Masked Access (16m/16d), Byte Offsets 56 and 57\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b5657](mask16b5657) module"]
pub type MASK16B5657 = crate::Reg<u32, _MASK16B5657>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B5657;
#[doc = "`write(|w| ..)` method takes [mask16b5657::W](mask16b5657::W) writer structure"]
impl crate::Writable for MASK16B5657 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 56 and 57"]
pub mod mask16b5657;
#[doc = "Masked Access (16m/16d), Byte Offsets 58 and 59\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b5859](mask16b5859) module"]
pub type MASK16B5859 = crate::Reg<u32, _MASK16B5859>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B5859;
#[doc = "`write(|w| ..)` method takes [mask16b5859::W](mask16b5859::W) writer structure"]
impl crate::Writable for MASK16B5859 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 58 and 59"]
pub mod mask16b5859;
#[doc = "Masked Access (16m/16d), Byte Offsets 60 and 61\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b6061](mask16b6061) module"]
pub type MASK16B6061 = crate::Reg<u32, _MASK16B6061>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B6061;
#[doc = "`write(|w| ..)` method takes [mask16b6061::W](mask16b6061::W) writer structure"]
impl crate::Writable for MASK16B6061 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 60 and 61"]
pub mod mask16b6061;
#[doc = "Masked Access (16m/16d), Byte Offsets 62 and 63\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b6263](mask16b6263) module"]
pub type MASK16B6263 = crate::Reg<u32, _MASK16B6263>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B6263;
#[doc = "`write(|w| ..)` method takes [mask16b6263::W](mask16b6263::W) writer structure"]
impl crate::Writable for MASK16B6263 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 62 and 63"]
pub mod mask16b6263;
#[doc = "Masked Access (16m/16d), Byte Offsets 64 and 65\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b6465](mask16b6465) module"]
pub type MASK16B6465 = crate::Reg<u32, _MASK16B6465>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B6465;
#[doc = "`write(|w| ..)` method takes [mask16b6465::W](mask16b6465::W) writer structure"]
impl crate::Writable for MASK16B6465 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 64 and 65"]
pub mod mask16b6465;
#[doc = "Masked Access (16m/16d), Byte Offsets 66 and 67\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b6667](mask16b6667) module"]
pub type MASK16B6667 = crate::Reg<u32, _MASK16B6667>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B6667;
#[doc = "`write(|w| ..)` method takes [mask16b6667::W](mask16b6667::W) writer structure"]
impl crate::Writable for MASK16B6667 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 66 and 67"]
pub mod mask16b6667;
#[doc = "Masked Access (16m/16d), Byte Offsets 68 and 69\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b6869](mask16b6869) module"]
pub type MASK16B6869 = crate::Reg<u32, _MASK16B6869>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B6869;
#[doc = "`write(|w| ..)` method takes [mask16b6869::W](mask16b6869::W) writer structure"]
impl crate::Writable for MASK16B6869 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 68 and 69"]
pub mod mask16b6869;
#[doc = "Masked Access (16m/16d), Byte Offsets 70 and 71\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask16b7071](mask16b7071) module"]
pub type MASK16B7071 = crate::Reg<u32, _MASK16B7071>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK16B7071;
#[doc = "`write(|w| ..)` method takes [mask16b7071::W](mask16b7071::W) writer structure"]
impl crate::Writable for MASK16B7071 {}
#[doc = "Masked Access (16m/16d), Byte Offsets 70 and 71"]
pub mod mask16b7071;
