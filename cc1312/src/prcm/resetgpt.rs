#[doc = "Reader of register RESETGPT"]
pub type R = crate::R<u32, super::RESETGPT>;
#[doc = "Writer for register RESETGPT"]
pub type W = crate::W<u32, super::RESETGPT>;
#[doc = "Register RESETGPT `reset()`'s with value 0"]
impl crate::ResetValue for super::RESETGPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPT`"]
pub type GPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT`"]
pub struct GPT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT_W<'a> {
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
    #[doc = "Bit 0 - GPT"]
    #[inline(always)]
    pub fn gpt(&self) -> GPT_R {
        GPT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPT"]
    #[inline(always)]
    pub fn gpt(&mut self) -> GPT_W {
        GPT_W { w: self }
    }
}
