#[doc = "Reader of register TIMER2CLKCTL"]
pub type R = crate::R<u32, super::TIMER2CLKCTL>;
#[doc = "Writer for register TIMER2CLKCTL"]
pub type W = crate::W<u32, super::TIMER2CLKCTL>;
#[doc = "Register TIMER2CLKCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER2CLKCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SRC"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SRC"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
