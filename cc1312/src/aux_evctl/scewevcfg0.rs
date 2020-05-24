#[doc = "Reader of register SCEWEVCFG0"]
pub type R = crate::R<u32, super::SCEWEVCFG0>;
#[doc = "Writer for register SCEWEVCFG0"]
pub type W = crate::W<u32, super::SCEWEVCFG0>;
#[doc = "Register SCEWEVCFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCEWEVCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMB_EV_EN`"]
pub type COMB_EV_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMB_EV_EN`"]
pub struct COMB_EV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMB_EV_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EV0_SEL`"]
pub type EV0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EV0_SEL`"]
pub struct EV0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - COMB_EV_EN"]
    #[inline(always)]
    pub fn comb_ev_en(&self) -> COMB_EV_EN_R {
        COMB_EV_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - EV0_SEL"]
    #[inline(always)]
    pub fn ev0_sel(&self) -> EV0_SEL_R {
        EV0_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - COMB_EV_EN"]
    #[inline(always)]
    pub fn comb_ev_en(&mut self) -> COMB_EV_EN_W {
        COMB_EV_EN_W { w: self }
    }
    #[doc = "Bits 0:5 - EV0_SEL"]
    #[inline(always)]
    pub fn ev0_sel(&mut self) -> EV0_SEL_W {
        EV0_SEL_W { w: self }
    }
}
