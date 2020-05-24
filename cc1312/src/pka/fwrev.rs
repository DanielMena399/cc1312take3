#[doc = "Reader of register FWREV"]
pub type R = crate::R<u32, super::FWREV>;
#[doc = "Reader of field `FW_CAPABILITIES`"]
pub type FW_CAPABILITIES_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR_FW_REVISION`"]
pub type MAJOR_FW_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `MINOR_FW_REVISION`"]
pub type MINOR_FW_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `FW_PATCH_LEVEL`"]
pub type FW_PATCH_LEVEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - FW_CAPABILITIES"]
    #[inline(always)]
    pub fn fw_capabilities(&self) -> FW_CAPABILITIES_R {
        FW_CAPABILITIES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - MAJOR_FW_REVISION"]
    #[inline(always)]
    pub fn major_fw_revision(&self) -> MAJOR_FW_REVISION_R {
        MAJOR_FW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MINOR_FW_REVISION"]
    #[inline(always)]
    pub fn minor_fw_revision(&self) -> MINOR_FW_REVISION_R {
        MINOR_FW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - FW_PATCH_LEVEL"]
    #[inline(always)]
    pub fn fw_patch_level(&self) -> FW_PATCH_LEVEL_R {
        FW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
