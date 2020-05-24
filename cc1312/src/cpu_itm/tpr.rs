#[doc = "Reader of register TPR"]
pub type R = crate::R<u32, super::TPR>;
#[doc = "Writer for register TPR"]
pub type W = crate::W<u32, super::TPR>;
#[doc = "Register TPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIVMASK`"]
pub type PRIVMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIVMASK`"]
pub struct PRIVMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PRIVMASK"]
    #[inline(always)]
    pub fn privmask(&self) -> PRIVMASK_R {
        PRIVMASK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRIVMASK"]
    #[inline(always)]
    pub fn privmask(&mut self) -> PRIVMASK_W {
        PRIVMASK_W { w: self }
    }
}
