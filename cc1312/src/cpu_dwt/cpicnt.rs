#[doc = "Reader of register CPICNT"]
pub type R = crate::R<u32, super::CPICNT>;
#[doc = "Writer for register CPICNT"]
pub type W = crate::W<u32, super::CPICNT>;
#[doc = "Register CPICNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CPICNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPICNT`"]
pub type CPICNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPICNT`"]
pub struct CPICNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPICNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CPICNT"]
    #[inline(always)]
    pub fn cpicnt(&self) -> CPICNT_R {
        CPICNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CPICNT"]
    #[inline(always)]
    pub fn cpicnt(&mut self) -> CPICNT_W {
        CPICNT_W { w: self }
    }
}
