#[doc = "Reader of register DWT"]
pub type R = crate::R<u32, super::DWT>;
#[doc = "Reader of field `DWT`"]
pub type DWT_R = crate::R<u32, u32>;
#[doc = "Reader of field `DWT_PRESENT`"]
pub type DWT_PRESENT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 1:31 - DWT"]
    #[inline(always)]
    pub fn dwt(&self) -> DWT_R {
        DWT_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - DWT_PRESENT"]
    #[inline(always)]
    pub fn dwt_present(&self) -> DWT_PRESENT_R {
        DWT_PRESENT_R::new((self.bits & 0x01) != 0)
    }
}
