#[doc = "Reader of register ID_PFR1"]
pub type R = crate::R<u32, super::ID_PFR1>;
#[doc = "Reader of field `MICROCONTROLLER_PROGRAMMERS_MODEL`"]
pub type MICROCONTROLLER_PROGRAMMERS_MODEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:11 - MICROCONTROLLER_PROGRAMMERS_MODEL"]
    #[inline(always)]
    pub fn microcontroller_programmers_model(&self) -> MICROCONTROLLER_PROGRAMMERS_MODEL_R {
        MICROCONTROLLER_PROGRAMMERS_MODEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
