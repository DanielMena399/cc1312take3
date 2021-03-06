#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub iostrmin: IOSTRMIN,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub iostrmed: IOSTRMED,
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    pub iostrmax: IOSTRMAX,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - SCLK_LF External Output Control"]
    pub clk32kctl: CLK32KCTL,
    #[doc = "0x14 - TCK IO Pin Control"]
    pub tckctl: TCKCTL,
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iostrmin](iostrmin) module"]
pub type IOSTRMIN = crate::Reg<u32, _IOSTRMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOSTRMIN;
#[doc = "`read()` method returns [iostrmin::R](iostrmin::R) reader structure"]
impl crate::Readable for IOSTRMIN {}
#[doc = "`write(|w| ..)` method takes [iostrmin::W](iostrmin::W) writer structure"]
impl crate::Writable for IOSTRMIN {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmin;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iostrmed](iostrmed) module"]
pub type IOSTRMED = crate::Reg<u32, _IOSTRMED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOSTRMED;
#[doc = "`read()` method returns [iostrmed::R](iostrmed::R) reader structure"]
impl crate::Readable for IOSTRMED {}
#[doc = "`write(|w| ..)` method takes [iostrmed::W](iostrmed::W) writer structure"]
impl crate::Writable for IOSTRMED {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmed;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iostrmax](iostrmax) module"]
pub type IOSTRMAX = crate::Reg<u32, _IOSTRMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOSTRMAX;
#[doc = "`read()` method returns [iostrmax::R](iostrmax::R) reader structure"]
impl crate::Readable for IOSTRMAX {}
#[doc = "`write(|w| ..)` method takes [iostrmax::W](iostrmax::W) writer structure"]
impl crate::Writable for IOSTRMAX {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmax;
#[doc = "SCLK_LF External Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk32kctl](clk32kctl) module"]
pub type CLK32KCTL = crate::Reg<u32, _CLK32KCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK32KCTL;
#[doc = "`read()` method returns [clk32kctl::R](clk32kctl::R) reader structure"]
impl crate::Readable for CLK32KCTL {}
#[doc = "`write(|w| ..)` method takes [clk32kctl::W](clk32kctl::W) writer structure"]
impl crate::Writable for CLK32KCTL {}
#[doc = "SCLK_LF External Output Control"]
pub mod clk32kctl;
#[doc = "TCK IO Pin Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tckctl](tckctl) module"]
pub type TCKCTL = crate::Reg<u32, _TCKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCKCTL;
#[doc = "`read()` method returns [tckctl::R](tckctl::R) reader structure"]
impl crate::Readable for TCKCTL {}
#[doc = "`write(|w| ..)` method takes [tckctl::W](tckctl::W) writer structure"]
impl crate::Writable for TCKCTL {}
#[doc = "TCK IO Pin Control"]
pub mod tckctl;
