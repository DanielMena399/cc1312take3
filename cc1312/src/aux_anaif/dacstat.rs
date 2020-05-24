#[doc = "Reader of register DACSTAT"]
pub type R = crate::R<u32, super::DACSTAT>;
#[doc = "Reader of field `SETUP_ACTIVE`"]
pub type SETUP_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HOLD_ACTIVE`"]
pub type HOLD_ACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - SETUP_ACTIVE"]
    #[inline(always)]
    pub fn setup_active(&self) -> SETUP_ACTIVE_R {
        SETUP_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - HOLD_ACTIVE"]
    #[inline(always)]
    pub fn hold_active(&self) -> HOLD_ACTIVE_R {
        HOLD_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
