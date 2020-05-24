#[doc = "Reader of register FOLDCNT"]
pub type R = crate::R<u32, super::FOLDCNT>;
#[doc = "Writer for register FOLDCNT"]
pub type W = crate::W<u32, super::FOLDCNT>;
#[doc = "Register FOLDCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::FOLDCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FOLDCNT`"]
pub type FOLDCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FOLDCNT`"]
pub struct FOLDCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FOLDCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - FOLDCNT"]
    #[inline(always)]
    pub fn foldcnt(&self) -> FOLDCNT_R {
        FOLDCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FOLDCNT"]
    #[inline(always)]
    pub fn foldcnt(&mut self) -> FOLDCNT_W {
        FOLDCNT_W { w: self }
    }
}
