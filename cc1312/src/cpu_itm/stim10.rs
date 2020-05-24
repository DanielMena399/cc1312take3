#[doc = "Reader of register STIM10"]
pub type R = crate::R<u32, super::STIM10>;
#[doc = "Writer for register STIM10"]
pub type W = crate::W<u32, super::STIM10>;
#[doc = "Register STIM10 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM10`"]
pub type STIM10_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM10`"]
pub struct STIM10_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM10"]
    #[inline(always)]
    pub fn stim10(&self) -> STIM10_R {
        STIM10_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM10"]
    #[inline(always)]
    pub fn stim10(&mut self) -> STIM10_W {
        STIM10_W { w: self }
    }
}
