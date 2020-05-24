#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH3_RESET`"]
pub type CH3_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3_RESET`"]
pub struct CH3_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_RESET_W<'a> {
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
#[doc = "Reader of field `CH2_RESET`"]
pub type CH2_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_RESET`"]
pub struct CH2_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CH1_RESET`"]
pub type CH1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_RESET`"]
pub struct CH1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH0_RESET`"]
pub type CH0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_RESET`"]
pub struct CH0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_RESET_W<'a> {
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
#[doc = "Reader of field `TARGET_EN`"]
pub type TARGET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TARGET_EN`"]
pub struct TARGET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET_EN_W<'a> {
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
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - CH3_RESET"]
    #[inline(always)]
    pub fn ch3_reset(&self) -> CH3_RESET_R {
        CH3_RESET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH2_RESET"]
    #[inline(always)]
    pub fn ch2_reset(&self) -> CH2_RESET_R {
        CH2_RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH1_RESET"]
    #[inline(always)]
    pub fn ch1_reset(&self) -> CH1_RESET_R {
        CH1_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH0_RESET"]
    #[inline(always)]
    pub fn ch0_reset(&self) -> CH0_RESET_R {
        CH0_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TARGET_EN"]
    #[inline(always)]
    pub fn target_en(&self) -> TARGET_EN_R {
        TARGET_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - CH3_RESET"]
    #[inline(always)]
    pub fn ch3_reset(&mut self) -> CH3_RESET_W {
        CH3_RESET_W { w: self }
    }
    #[doc = "Bit 5 - CH2_RESET"]
    #[inline(always)]
    pub fn ch2_reset(&mut self) -> CH2_RESET_W {
        CH2_RESET_W { w: self }
    }
    #[doc = "Bit 4 - CH1_RESET"]
    #[inline(always)]
    pub fn ch1_reset(&mut self) -> CH1_RESET_W {
        CH1_RESET_W { w: self }
    }
    #[doc = "Bit 3 - CH0_RESET"]
    #[inline(always)]
    pub fn ch0_reset(&mut self) -> CH0_RESET_W {
        CH0_RESET_W { w: self }
    }
    #[doc = "Bit 2 - TARGET_EN"]
    #[inline(always)]
    pub fn target_en(&mut self) -> TARGET_EN_W {
        TARGET_EN_W { w: self }
    }
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
