#[doc = "Reader of register ETM"]
pub type R = crate::R<u32, super::ETM>;
#[doc = "Reader of field `ETM`"]
pub type ETM_R = crate::R<u32, u32>;
#[doc = "Reader of field `ETM_PRESENT`"]
pub type ETM_PRESENT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 1:31 - ETM"]
    #[inline(always)]
    pub fn etm(&self) -> ETM_R {
        ETM_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - ETM_PRESENT"]
    #[inline(always)]
    pub fn etm_present(&self) -> ETM_PRESENT_R {
        ETM_PRESENT_R::new((self.bits & 0x01) != 0)
    }
}
