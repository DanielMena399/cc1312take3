#[doc = "Writer for register ACCSHIFT"]
pub type W = crate::W<u32, super::ACCSHIFT>;
#[doc = "Register ACCSHIFT `reset()`'s with value 0"]
impl crate::ResetValue for super::ACCSHIFT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LSL1`"]
pub struct LSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> LSL1_W<'a> {
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
#[doc = "Write proxy for field `LSR1`"]
pub struct LSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> LSR1_W<'a> {
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
#[doc = "Write proxy for field `ASR1`"]
pub struct ASR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASR1_W<'a> {
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
impl W {
    #[doc = "Bit 2 - LSL1"]
    #[inline(always)]
    pub fn lsl1(&mut self) -> LSL1_W {
        LSL1_W { w: self }
    }
    #[doc = "Bit 1 - LSR1"]
    #[inline(always)]
    pub fn lsr1(&mut self) -> LSR1_W {
        LSR1_W { w: self }
    }
    #[doc = "Bit 0 - ASR1"]
    #[inline(always)]
    pub fn asr1(&mut self) -> ASR1_W {
        ASR1_W { w: self }
    }
}
