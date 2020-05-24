#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PKA RAM"]
    pub bank0: [BANK0; 512],
}
#[doc = "PKA RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank0](bank0) module"]
pub type BANK0 = crate::Reg<u32, _BANK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BANK0;
#[doc = "`read()` method returns [bank0::R](bank0::R) reader structure"]
impl crate::Readable for BANK0 {}
#[doc = "`write(|w| ..)` method takes [bank0::W](bank0::W) writer structure"]
impl crate::Writable for BANK0 {}
#[doc = "PKA RAM"]
pub mod bank0;
