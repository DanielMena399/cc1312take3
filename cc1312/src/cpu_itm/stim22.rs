#[doc = "Reader of register STIM22"]
pub type R = crate::R<u32, super::STIM22>;
#[doc = "Writer for register STIM22"]
pub type W = crate::W<u32, super::STIM22>;
#[doc = "Register STIM22 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM22`"]
pub type STIM22_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM22`"]
pub struct STIM22_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM22"]
    #[inline(always)]
    pub fn stim22(&self) -> STIM22_R {
        STIM22_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM22"]
    #[inline(always)]
    pub fn stim22(&mut self) -> STIM22_W {
        STIM22_W { w: self }
    }
}
