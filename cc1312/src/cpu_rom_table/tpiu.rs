#[doc = "Reader of register TPIU"]
pub type R = crate::R<u32, super::TPIU>;
#[doc = "Reader of field `TPIU`"]
pub type TPIU_R = crate::R<u32, u32>;
#[doc = "Reader of field `TPIU_PRESENT`"]
pub type TPIU_PRESENT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 1:31 - TPIU"]
    #[inline(always)]
    pub fn tpiu(&self) -> TPIU_R {
        TPIU_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - TPIU_PRESENT"]
    #[inline(always)]
    pub fn tpiu_present(&self) -> TPIU_PRESENT_R {
        TPIU_PRESENT_R::new((self.bits & 0x01) != 0)
    }
}
