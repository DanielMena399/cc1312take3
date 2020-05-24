#[doc = "Reader of register STIM26"]
pub type R = crate::R<u32, super::STIM26>;
#[doc = "Writer for register STIM26"]
pub type W = crate::W<u32, super::STIM26>;
#[doc = "Register STIM26 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM26`"]
pub type STIM26_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM26`"]
pub struct STIM26_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM26"]
    #[inline(always)]
    pub fn stim26(&self) -> STIM26_R {
        STIM26_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM26"]
    #[inline(always)]
    pub fn stim26(&mut self) -> STIM26_W {
        STIM26_W { w: self }
    }
}
