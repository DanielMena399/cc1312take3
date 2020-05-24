#[doc = "Reader of register SLEEPCNT"]
pub type R = crate::R<u32, super::SLEEPCNT>;
#[doc = "Writer for register SLEEPCNT"]
pub type W = crate::W<u32, super::SLEEPCNT>;
#[doc = "Register SLEEPCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::SLEEPCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLEEPCNT`"]
pub type SLEEPCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLEEPCNT`"]
pub struct SLEEPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SLEEPCNT"]
    #[inline(always)]
    pub fn sleepcnt(&self) -> SLEEPCNT_R {
        SLEEPCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SLEEPCNT"]
    #[inline(always)]
    pub fn sleepcnt(&mut self) -> SLEEPCNT_W {
        SLEEPCNT_W { w: self }
    }
}
