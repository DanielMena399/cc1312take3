#[doc = "Reader of register EVSTAT1H"]
pub type R = crate::R<u32, super::EVSTAT1H>;
#[doc = "Reader of field `ALIAS_EV`"]
pub type ALIAS_EV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - ALIAS_EV"]
    #[inline(always)]
    pub fn alias_ev(&self) -> ALIAS_EV_R {
        ALIAS_EV_R::new((self.bits & 0xff) as u8)
    }
}
