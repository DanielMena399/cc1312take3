#[doc = "Reader of register HWREV"]
pub type R = crate::R<u32, super::HWREV>;
#[doc = "Reader of field `MAJOR_HW_REVISION`"]
pub type MAJOR_HW_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `MINOR_HW_REVISION`"]
pub type MINOR_HW_REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `HW_PATCH_LEVEL`"]
pub type HW_PATCH_LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `COMPLEMENT_OF_BASIC_EIP_NUMBER`"]
pub type COMPLEMENT_OF_BASIC_EIP_NUMBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `BASIC_EIP_NUMBER`"]
pub type BASIC_EIP_NUMBER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:27 - MAJOR_HW_REVISION"]
    #[inline(always)]
    pub fn major_hw_revision(&self) -> MAJOR_HW_REVISION_R {
        MAJOR_HW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MINOR_HW_REVISION"]
    #[inline(always)]
    pub fn minor_hw_revision(&self) -> MINOR_HW_REVISION_R {
        MINOR_HW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - HW_PATCH_LEVEL"]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVEL_R {
        HW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - COMPLEMENT_OF_BASIC_EIP_NUMBER"]
    #[inline(always)]
    pub fn complement_of_basic_eip_number(&self) -> COMPLEMENT_OF_BASIC_EIP_NUMBER_R {
        COMPLEMENT_OF_BASIC_EIP_NUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BASIC_EIP_NUMBER"]
    #[inline(always)]
    pub fn basic_eip_number(&self) -> BASIC_EIP_NUMBER_R {
        BASIC_EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
