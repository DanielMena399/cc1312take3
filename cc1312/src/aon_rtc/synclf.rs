#[doc = "Reader of register SYNCLF"]
pub type R = crate::R<u32, super::SYNCLF>;
#[doc = "Reader of field `PHASE`"]
pub type PHASE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PHASE"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 0x01) != 0)
    }
}
