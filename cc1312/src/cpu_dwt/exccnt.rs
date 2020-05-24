#[doc = "Reader of register EXCCNT"]
pub type R = crate::R<u32, super::EXCCNT>;
#[doc = "Writer for register EXCCNT"]
pub type W = crate::W<u32, super::EXCCNT>;
#[doc = "Register EXCCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::EXCCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXCCNT`"]
pub type EXCCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXCCNT`"]
pub struct EXCCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXCCNT"]
    #[inline(always)]
    pub fn exccnt(&self) -> EXCCNT_R {
        EXCCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXCCNT"]
    #[inline(always)]
    pub fn exccnt(&mut self) -> EXCCNT_W {
        EXCCNT_W { w: self }
    }
}
