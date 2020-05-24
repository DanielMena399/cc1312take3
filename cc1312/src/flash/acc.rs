#[doc = "Reader of register ACC"]
pub type R = crate::R<u32, super::ACC>;
#[doc = "Reader of field `ACCUMULATOR`"]
pub type ACCUMULATOR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - ACCUMULATOR"]
    #[inline(always)]
    pub fn accumulator(&self) -> ACCUMULATOR_R {
        ACCUMULATOR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
