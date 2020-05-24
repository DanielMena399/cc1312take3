#[doc = "Reader of register CCFG_TAP_DAP_1"]
pub type R = crate::R<u32, super::CCFG_TAP_DAP_1>;
#[doc = "Reader of field `PBIST2_TAP_ENABLE`"]
pub type PBIST2_TAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PBIST1_TAP_ENABLE`"]
pub type PBIST1_TAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `AON_TAP_ENABLE`"]
pub type AON_TAP_ENABLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:23 - PBIST2_TAP_ENABLE"]
    #[inline(always)]
    pub fn pbist2_tap_enable(&self) -> PBIST2_TAP_ENABLE_R {
        PBIST2_TAP_ENABLE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PBIST1_TAP_ENABLE"]
    #[inline(always)]
    pub fn pbist1_tap_enable(&self) -> PBIST1_TAP_ENABLE_R {
        PBIST1_TAP_ENABLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - AON_TAP_ENABLE"]
    #[inline(always)]
    pub fn aon_tap_enable(&self) -> AON_TAP_ENABLE_R {
        AON_TAP_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
