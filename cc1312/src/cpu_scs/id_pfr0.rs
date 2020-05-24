#[doc = "Reader of register ID_PFR0"]
pub type R = crate::R<u32, super::ID_PFR0>;
#[doc = "Reader of field `STATE1`"]
pub type STATE1_R = crate::R<u8, u8>;
#[doc = "Reader of field `STATE0`"]
pub type STATE0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 4:7 - STATE1"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - STATE0"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x0f) as u8)
    }
}
