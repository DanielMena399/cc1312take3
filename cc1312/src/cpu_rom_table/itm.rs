#[doc = "Reader of register ITM"]
pub type R = crate::R<u32, super::ITM>;
#[doc = "Reader of field `ITM`"]
pub type ITM_R = crate::R<u32, u32>;
#[doc = "Reader of field `ITM_PRESENT`"]
pub type ITM_PRESENT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 1:31 - ITM"]
    #[inline(always)]
    pub fn itm(&self) -> ITM_R {
        ITM_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - ITM_PRESENT"]
    #[inline(always)]
    pub fn itm_present(&self) -> ITM_PRESENT_R {
        ITM_PRESENT_R::new((self.bits & 0x01) != 0)
    }
}
