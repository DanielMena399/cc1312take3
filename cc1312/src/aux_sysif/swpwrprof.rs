#[doc = "Reader of register SWPWRPROF"]
pub type R = crate::R<u32, super::SWPWRPROF>;
#[doc = "Writer for register SWPWRPROF"]
pub type W = crate::W<u32, super::SWPWRPROF>;
#[doc = "Register SWPWRPROF `reset()`'s with value 0"]
impl crate::ResetValue for super::SWPWRPROF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STAT`"]
pub type STAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STAT`"]
pub struct STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - STAT"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - STAT"]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W {
        STAT_W { w: self }
    }
}
