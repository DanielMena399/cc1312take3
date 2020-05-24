#[doc = "Reader of register TIMER2CLKSWITCH"]
pub type R = crate::R<u32, super::TIMER2CLKSWITCH>;
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RDY"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 0x01) != 0)
    }
}
