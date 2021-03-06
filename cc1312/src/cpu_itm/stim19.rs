#[doc = "Reader of register STIM19"]
pub type R = crate::R<u32, super::STIM19>;
#[doc = "Writer for register STIM19"]
pub type W = crate::W<u32, super::STIM19>;
#[doc = "Register STIM19 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM19 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM19`"]
pub type STIM19_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM19`"]
pub struct STIM19_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM19"]
    #[inline(always)]
    pub fn stim19(&self) -> STIM19_R {
        STIM19_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM19"]
    #[inline(always)]
    pub fn stim19(&mut self) -> STIM19_W {
        STIM19_W { w: self }
    }
}
