#[doc = "Reader of register DMAHWVER"]
pub type R = crate::R<u32, super::DMAHWVER>;
#[doc = "Reader of field `HW_MAJOR_VERSION`"]
pub type HW_MAJOR_VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `HW_MINOR_VERSION`"]
pub type HW_MINOR_VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `HW_PATCH_LEVEL`"]
pub type HW_PATCH_LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EIP_NUMBER_COMPL`"]
pub type EIP_NUMBER_COMPL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EIP_NUMBER`"]
pub type EIP_NUMBER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:27 - HW_MAJOR_VERSION"]
    #[inline(always)]
    pub fn hw_major_version(&self) -> HW_MAJOR_VERSION_R {
        HW_MAJOR_VERSION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - HW_MINOR_VERSION"]
    #[inline(always)]
    pub fn hw_minor_version(&self) -> HW_MINOR_VERSION_R {
        HW_MINOR_VERSION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - HW_PATCH_LEVEL"]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVEL_R {
        HW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - EIP_NUMBER_COMPL"]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EIP_NUMBER_COMPL_R {
        EIP_NUMBER_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - EIP_NUMBER"]
    #[inline(always)]
    pub fn eip_number(&self) -> EIP_NUMBER_R {
        EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
