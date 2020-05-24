#[doc = "Reader of register ID_MMFR2"]
pub type R = crate::R<u32, super::ID_MMFR2>;
#[doc = "Reader of field `WAIT_FOR_INTERRUPT_STALLING`"]
pub type WAIT_FOR_INTERRUPT_STALLING_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 24 - WAIT_FOR_INTERRUPT_STALLING"]
    #[inline(always)]
    pub fn wait_for_interrupt_stalling(&self) -> WAIT_FOR_INTERRUPT_STALLING_R {
        WAIT_FOR_INTERRUPT_STALLING_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
