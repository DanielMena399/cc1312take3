#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Direct Access for ADI Byte Offsets 0 to 3"]
    pub dir03: DIR03,
    #[doc = "0x04 - Direct Access for ADI Byte Offsets 4 to 7"]
    pub dir47: DIR47,
    #[doc = "0x08 - Direct Access for ADI Byte Offsets 8 to 11"]
    pub dir811: DIR811,
    #[doc = "0x0c - Direct Access for ADI Byte Offsets 12 to 15"]
    pub dir1215: DIR1215,
    #[doc = "0x10 - Set for ADI Byte Offsets 0 to 3"]
    pub set03: SET03,
    #[doc = "0x14 - Set for ADI Byte Offsets 4 to 7"]
    pub set47: SET47,
    #[doc = "0x18 - Set for ADI Byte Offsets 8 to 11"]
    pub set811: SET811,
    #[doc = "0x1c - Set for ADI Byte Offsets 12 to 15"]
    pub set1215: SET1215,
    #[doc = "0x20 - Clear for ADI Byte Offsets 0 to 3"]
    pub clr03: CLR03,
    #[doc = "0x24 - Clear for ADI Byte Offsets 4 to 7"]
    pub clr47: CLR47,
    #[doc = "0x28 - Clear for ADI Byte Offsets 8 to 11"]
    pub clr811: CLR811,
    #[doc = "0x2c - Clear for ADI Byte Offsets 12 to 15"]
    pub clr1215: CLR1215,
    #[doc = "0x30 - Internal. Only to be used through TI provided API."]
    pub sync: SYNC,
    _reserved13: [u8; 4usize],
    #[doc = "0x38 - Internal. Only to be used through TI provided API."]
    pub cfg: CFG,
    _reserved14: [u8; 4usize],
    #[doc = "0x40 - Masked Access (4m/4d), Byte Offsets 0 and 1"]
    pub mask4b01: MASK4B01,
    #[doc = "0x44 - Masked Access (4m/4d), Byte Offsets 2 and 3"]
    pub mask4b23: MASK4B23,
    #[doc = "0x48 - Masked Access (4m/4d), Byte Offsets 4 and 5"]
    pub mask4b45: MASK4B45,
    #[doc = "0x4c - Masked Access (4m/4d), Byte Offsets 6 and 7"]
    pub mask4b67: MASK4B67,
    #[doc = "0x50 - Masked Access (4m/4d), Byte Offsets 8 and 9"]
    pub mask4b89: MASK4B89,
    #[doc = "0x54 - Masked Access (4m/4d), Byte Offsets 10 and 11"]
    pub mask4b1011: MASK4B1011,
    #[doc = "0x58 - Masked Access (4m/4d), Byte Offsets 12 and 13"]
    pub mask4b1213: MASK4B1213,
    #[doc = "0x5c - Masked Access (4m/4d), Byte Offsets 14 and 15"]
    pub mask4b1415: MASK4B1415,
    #[doc = "0x60 - Masked Access (8m/8d), Byte Offsets 0 and 1"]
    pub mask8b01: MASK8B01,
    #[doc = "0x64 - Masked Access (8m/8d), Byte Offsets 2 and 3"]
    pub mask8b23: MASK8B23,
    #[doc = "0x68 - Masked Access (8m/8d), Byte Offsets 4 and 5"]
    pub mask8b45: MASK8B45,
    #[doc = "0x6c - Masked Access (8m/8d), Byte Offsets 6 and 7"]
    pub mask8b67: MASK8B67,
    #[doc = "0x70 - Masked Access (8m/8d), Byte Offsets 8 and 9"]
    pub mask8b89: MASK8B89,
    #[doc = "0x74 - Masked Access (8m/8d), Byte Offsets 10 and 11"]
    pub mask8b1011: MASK8B1011,
    #[doc = "0x78 - Masked Access (8m/8d), Byte Offsets 12 and 13"]
    pub mask8b1213: MASK8B1213,
    #[doc = "0x7c - Masked Access (8m/8d), Byte Offsets 14 and 15"]
    pub mask8b1415: MASK8B1415,
    #[doc = "0x80 - Masked Access (16m/16d), Byte Offsets 0 and 1"]
    pub mask16b01: MASK16B01,
    #[doc = "0x84 - Masked Access (16m/16d), Byte Offsets 2 and 3"]
    pub mask16b23: MASK16B23,
    #[doc = "0x88 - Masked Access (16m/16d), Byte Offsets 4 and 5"]
    pub mask16b45: MASK16B45,
    #[doc = "0x8c - Masked Access (16m/16d), Byte Offsets 6 and 7"]
    pub mask16b67: MASK16B67,
    #[doc = "0x90 - Masked Access (16m/16d), Byte Offsets 8 and 9"]
    pub mask16b89: MASK16B89,
    #[doc = "0x94 - Masked Access (16m/16d), Byte Offsets 10 and 11"]
    pub mask16b1011: MASK16B1011,
    #[doc = "0x98 - Masked Access (16m/16d), Byte Offsets 12 and 13"]
    pub mask16b1213: MASK16B1213,
    #[doc = "0x9c - Masked Access (16m/16d), Byte Offsets 14 and 15"]
    pub mask16b1415: MASK16B1415,
}
#[doc = "Direct Access for ADI Byte Offsets 0 to 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir03](dir03) module"]
pub type DIR03 = crate::Reg<u32, _DIR03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR03;
#[doc = "`read()` method returns [dir03::R](dir03::R) reader structure"]
impl crate::Readable for DIR03 {}
#[doc = "`write(|w| ..)` method takes [dir03::W](dir03::W) writer structure"]
impl crate::Writable for DIR03 {}
#[doc = "Direct Access for ADI Byte Offsets 0 to 3"]
pub mod dir03;
#[doc = "Direct Access for ADI Byte Offsets 4 to 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir47](dir47) module"]
pub type DIR47 = crate::Reg<u32, _DIR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR47;
#[doc = "`read()` method returns [dir47::R](dir47::R) reader structure"]
impl crate::Readable for DIR47 {}
#[doc = "`write(|w| ..)` method takes [dir47::W](dir47::W) writer structure"]
impl crate::Writable for DIR47 {}
#[doc = "Direct Access for ADI Byte Offsets 4 to 7"]
pub mod dir47;
#[doc = "Direct Access for ADI Byte Offsets 8 to 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir811](dir811) module"]
pub type DIR811 = crate::Reg<u32, _DIR811>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR811;
#[doc = "`read()` method returns [dir811::R](dir811::R) reader structure"]
impl crate::Readable for DIR811 {}
#[doc = "`write(|w| ..)` method takes [dir811::W](dir811::W) writer structure"]
impl crate::Writable for DIR811 {}
#[doc = "Direct Access for ADI Byte Offsets 8 to 11"]
pub mod dir811;
#[doc = "Direct Access for ADI Byte Offsets 12 to 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir1215](dir1215) module"]
pub type DIR1215 = crate::Reg<u32, _DIR1215>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR1215;
#[doc = "`read()` method returns [dir1215::R](dir1215::R) reader structure"]
impl crate::Readable for DIR1215 {}
#[doc = "`write(|w| ..)` method takes [dir1215::W](dir1215::W) writer structure"]
impl crate::Writable for DIR1215 {}
#[doc = "Direct Access for ADI Byte Offsets 12 to 15"]
pub mod dir1215;
#[doc = "Set for ADI Byte Offsets 0 to 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set03](set03) module"]
pub type SET03 = crate::Reg<u32, _SET03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET03;
#[doc = "`write(|w| ..)` method takes [set03::W](set03::W) writer structure"]
impl crate::Writable for SET03 {}
#[doc = "Set for ADI Byte Offsets 0 to 3"]
pub mod set03;
#[doc = "Set for ADI Byte Offsets 4 to 7\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set47](set47) module"]
pub type SET47 = crate::Reg<u32, _SET47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET47;
#[doc = "`write(|w| ..)` method takes [set47::W](set47::W) writer structure"]
impl crate::Writable for SET47 {}
#[doc = "Set for ADI Byte Offsets 4 to 7"]
pub mod set47;
#[doc = "Set for ADI Byte Offsets 8 to 11\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set811](set811) module"]
pub type SET811 = crate::Reg<u32, _SET811>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET811;
#[doc = "`write(|w| ..)` method takes [set811::W](set811::W) writer structure"]
impl crate::Writable for SET811 {}
#[doc = "Set for ADI Byte Offsets 8 to 11"]
pub mod set811;
#[doc = "Set for ADI Byte Offsets 12 to 15\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set1215](set1215) module"]
pub type SET1215 = crate::Reg<u32, _SET1215>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET1215;
#[doc = "`write(|w| ..)` method takes [set1215::W](set1215::W) writer structure"]
impl crate::Writable for SET1215 {}
#[doc = "Set for ADI Byte Offsets 12 to 15"]
pub mod set1215;
#[doc = "Clear for ADI Byte Offsets 0 to 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr03](clr03) module"]
pub type CLR03 = crate::Reg<u32, _CLR03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR03;
#[doc = "`write(|w| ..)` method takes [clr03::W](clr03::W) writer structure"]
impl crate::Writable for CLR03 {}
#[doc = "Clear for ADI Byte Offsets 0 to 3"]
pub mod clr03;
#[doc = "Clear for ADI Byte Offsets 4 to 7\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr47](clr47) module"]
pub type CLR47 = crate::Reg<u32, _CLR47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR47;
#[doc = "`write(|w| ..)` method takes [clr47::W](clr47::W) writer structure"]
impl crate::Writable for CLR47 {}
#[doc = "Clear for ADI Byte Offsets 4 to 7"]
pub mod clr47;
#[doc = "Clear for ADI Byte Offsets 8 to 11\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr811](clr811) module"]
pub type CLR811 = crate::Reg<u32, _CLR811>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR811;
#[doc = "`write(|w| ..)` method takes [clr811::W](clr811::W) writer structure"]
impl crate::Writable for CLR811 {}
#[doc = "Clear for ADI Byte Offsets 8 to 11"]
pub mod clr811;
#[doc = "Clear for ADI Byte Offsets 12 to 15\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr1215](clr1215) module"]
pub type CLR1215 = crate::Reg<u32, _CLR1215>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR1215;
#[doc = "`write(|w| ..)` method takes [clr1215::W](clr1215::W) writer structure"]
impl crate::Writable for CLR1215 {}
#[doc = "Clear for ADI Byte Offsets 12 to 15"]
pub mod clr1215;
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
