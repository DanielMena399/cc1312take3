#[doc = "Reader of register LSUCNT"]
pub type R = crate::R<u32, super::LSUCNT>;
#[doc = "Writer for register LSUCNT"]
pub type W = crate::W<u32, super::LSUCNT>;
#[doc = "Register LSUCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::LSUCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSUCNT`"]
pub type LSUCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSUCNT`"]
pub struct LSUCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSUCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LSUCNT"]
    #[inline(always)]
    pub fn lsucnt(&self) -> LSUCNT_R {
        LSUCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LSUCNT"]
    #[inline(always)]
    pub fn lsucnt(&mut self) -> LSUCNT_W {
        LSUCNT_W { w: self }
    }
}
