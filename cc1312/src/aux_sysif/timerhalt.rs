#[doc = "Reader of register TIMERHALT"]
pub type R = crate::R<u32, super::TIMERHALT>;
#[doc = "Writer for register TIMERHALT"]
pub type W = crate::W<u32, super::TIMERHALT>;
#[doc = "Register TIMERHALT `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMERHALT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROGDLY`"]
pub type PROGDLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROGDLY`"]
pub struct PROGDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGDLY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `AUX_TIMER2`"]
pub type AUX_TIMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER2`"]
pub struct AUX_TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER2_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER1`"]
pub type AUX_TIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER1`"]
pub struct AUX_TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER1_W<'a> {
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
#[doc = "Reader of field `AUX_TIMER0`"]
pub type AUX_TIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_TIMER0`"]
pub struct AUX_TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_TIMER0_W<'a> {
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
    #[doc = "Bit 3 - PROGDLY"]
    #[inline(always)]
    pub fn progdly(&self) -> PROGDLY_R {
        PROGDLY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AUX_TIMER2"]
    #[inline(always)]
    pub fn aux_timer2(&self) -> AUX_TIMER2_R {
        AUX_TIMER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AUX_TIMER1"]
    #[inline(always)]
    pub fn aux_timer1(&self) -> AUX_TIMER1_R {
        AUX_TIMER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AUX_TIMER0"]
    #[inline(always)]
    pub fn aux_timer0(&self) -> AUX_TIMER0_R {
        AUX_TIMER0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PROGDLY"]
    #[inline(always)]
    pub fn progdly(&mut self) -> PROGDLY_W {
        PROGDLY_W { w: self }
    }
    #[doc = "Bit 2 - AUX_TIMER2"]
    #[inline(always)]
    pub fn aux_timer2(&mut self) -> AUX_TIMER2_W {
        AUX_TIMER2_W { w: self }
    }
    #[doc = "Bit 1 - AUX_TIMER1"]
    #[inline(always)]
    pub fn aux_timer1(&mut self) -> AUX_TIMER1_W {
        AUX_TIMER1_W { w: self }
    }
    #[doc = "Bit 0 - AUX_TIMER0"]
    #[inline(always)]
    pub fn aux_timer0(&mut self) -> AUX_TIMER0_W {
        AUX_TIMER0_W { w: self }
    }
}
