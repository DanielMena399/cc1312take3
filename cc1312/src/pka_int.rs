#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4088usize],
    #[doc = "0xff8 - PKA Options register"]
    pub options: OPTIONS,
    #[doc = "0xffc - PKA hardware revision register his register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
    pub revision: REVISION,
}
#[doc = "PKA Options register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options](options) module"]
pub type OPTIONS = crate::Reg<u32, _OPTIONS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTIONS;
#[doc = "`read()` method returns [options::R](options::R) reader structure"]
impl crate::Readable for OPTIONS {}
#[doc = "PKA Options register"]
pub mod options;
#[doc = "PKA hardware revision register his register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u32, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "PKA hardware revision register his register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
pub mod revision;
