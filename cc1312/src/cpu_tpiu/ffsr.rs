#[doc = "Reader of register FFSR"]
pub type R = crate::R<u32, super::FFSR>;
#[doc = "Reader of field `FTNONSTOP`"]
pub type FTNONSTOP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - FTNONSTOP"]
    #[inline(always)]
    pub fn ftnonstop(&self) -> FTNONSTOP_R {
        FTNONSTOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
