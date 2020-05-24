#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 8 kB ULL SRAM"]
    pub bank1: [BANK1; 2048],
}
#[doc = "8 kB ULL SRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank1](bank1) module"]
pub type BANK1 = crate::Reg<u32, _BANK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BANK1;
#[doc = "`read()` method returns [bank1::R](bank1::R) reader structure"]
impl crate::Readable for BANK1 {}
#[doc = "`write(|w| ..)` method takes [bank1::W](bank1::W) writer structure"]
impl crate::Writable for BANK1 {}
#[doc = "8 kB ULL SRAM"]
pub mod bank1;
