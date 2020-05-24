#[doc = "Reader of register ID_DFR0"]
pub type R = crate::R<u32, super::ID_DFR0>;
#[doc = "Reader of field `MICROCONTROLLER_DEBUG_MODEL`"]
pub type MICROCONTROLLER_DEBUG_MODEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 20:23 - MICROCONTROLLER_DEBUG_MODEL"]
    #[inline(always)]
    pub fn microcontroller_debug_model(&self) -> MICROCONTROLLER_DEBUG_MODEL_R {
        MICROCONTROLLER_DEBUG_MODEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
