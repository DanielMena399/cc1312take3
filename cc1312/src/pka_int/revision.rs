#[doc = "Reader of register REVISION"]
pub type R = crate::R<u32, super::REVISION>;
#[doc = "Reader of field `MAJOR_REVISION`"]
pub type MAJOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `MINOR_REVISION`"]
pub type MINOR_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `PATCH_LEVEL`"]
pub type PATCH_LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `COMP_EIP_NUM`"]
pub type COMP_EIP_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EIP_NUM`"]
pub type EIP_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:27 - MAJOR_REVISION"]
    #[inline(always)]
    pub fn major_revision(&self) -> MAJOR_REVISION_R {
        MAJOR_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MINOR_REVISION"]
    #[inline(always)]
    pub fn minor_revision(&self) -> MINOR_REVISION_R {
        MINOR_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PATCH_LEVEL"]
    #[inline(always)]
    pub fn patch_level(&self) -> PATCH_LEVEL_R {
        PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - COMP_EIP_NUM"]
    #[inline(always)]
    pub fn comp_eip_num(&self) -> COMP_EIP_NUM_R {
        COMP_EIP_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - EIP_NUM"]
    #[inline(always)]
    pub fn eip_num(&self) -> EIP_NUM_R {
        EIP_NUM_R::new((self.bits & 0xff) as u8)
    }
}
