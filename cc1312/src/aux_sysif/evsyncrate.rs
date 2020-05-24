#[doc = "Reader of register EVSYNCRATE"]
pub type R = crate::R<u32, super::EVSYNCRATE>;
#[doc = "Writer for register EVSYNCRATE"]
pub type W = crate::W<u32, super::EVSYNCRATE>;
#[doc = "Register EVSYNCRATE `reset()`'s with value 0"]
impl crate::ResetValue for super::EVSYNCRATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUX_COMPA_SYNC_RATE`"]
pub type AUX_COMPA_SYNC_RATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_COMPA_SYNC_RATE`"]
pub struct AUX_COMPA_SYNC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_SYNC_RATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AUX_COMPB_SYNC_RATE`"]
pub type AUX_COMPB_SYNC_RATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_COMPB_SYNC_RATE`"]
pub struct AUX_COMPB_SYNC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_SYNC_RATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AUX_TIMER2_SYNC_RATE`"]
pub type AUX_TIMER2_SYNC_RATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER2_SYNC_RATE`"]
pub struct AUX_TIMER2_SYNC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_SYNC_RATE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - AUX_COMPA_SYNC_RATE"]
    #[inline(always)]
    pub fn aux_compa_sync_rate(&self) -> AUX_COMPA_SYNC_RATE_R {
        AUX_COMPA_SYNC_RATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AUX_COMPB_SYNC_RATE"]
    #[inline(always)]
    pub fn aux_compb_sync_rate(&self) -> AUX_COMPB_SYNC_RATE_R {
        AUX_COMPB_SYNC_RATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AUX_TIMER2_SYNC_RATE"]
    #[inline(always)]
    pub fn aux_timer2_sync_rate(&self) -> AUX_TIMER2_SYNC_RATE_R {
        AUX_TIMER2_SYNC_RATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AUX_COMPA_SYNC_RATE"]
    #[inline(always)]
    pub fn aux_compa_sync_rate(&mut self) -> AUX_COMPA_SYNC_RATE_W {
        AUX_COMPA_SYNC_RATE_W { w: self }
    }
    #[doc = "Bit 1 - AUX_COMPB_SYNC_RATE"]
    #[inline(always)]
    pub fn aux_compb_sync_rate(&mut self) -> AUX_COMPB_SYNC_RATE_W {
        AUX_COMPB_SYNC_RATE_W { w: self }
    }
    #[doc = "Bit 0 - AUX_TIMER2_SYNC_RATE"]
    #[inline(always)]
    pub fn aux_timer2_sync_rate(&mut self) -> AUX_TIMER2_SYNC_RATE_W {
        AUX_TIMER2_SYNC_RATE_W { w: self }
    }
}
