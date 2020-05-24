#[doc = "Reader of register TIMER2CLKSTAT"]
pub type R = crate::R<u32, super::TIMER2CLKSTAT>;
#[doc = "Reader of field `STAT`"]
pub type STAT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - STAT"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0x07) as u8)
    }
}
