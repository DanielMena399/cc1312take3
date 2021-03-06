#[doc = "Reader of register STIM9"]
pub type R = crate::R<u32, super::STIM9>;
#[doc = "Writer for register STIM9"]
pub type W = crate::W<u32, super::STIM9>;
#[doc = "Register STIM9 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM9`"]
pub type STIM9_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM9`"]
pub struct STIM9_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM9"]
    #[inline(always)]
    pub fn stim9(&self) -> STIM9_R {
        STIM9_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM9"]
    #[inline(always)]
    pub fn stim9(&mut self) -> STIM9_W {
        STIM9_W { w: self }
    }
}
