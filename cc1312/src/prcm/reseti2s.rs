#[doc = "Reader of register RESETI2S"]
pub type R = crate::R<u32, super::RESETI2S>;
#[doc = "Writer for register RESETI2S"]
pub type W = crate::W<u32, super::RESETI2S>;
#[doc = "Register RESETI2S `reset()`'s with value 0"]
impl crate::ResetValue for super::RESETI2S {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S`"]
pub type I2S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S`"]
pub struct I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_W<'a> {
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
    #[doc = "Bit 0 - I2S"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S"]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2S_W {
        I2S_W { w: self }
    }
}
