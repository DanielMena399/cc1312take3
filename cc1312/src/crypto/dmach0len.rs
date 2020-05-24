#[doc = "Reader of register DMACH0LEN"]
pub type R = crate::R<u32, super::DMACH0LEN>;
#[doc = "Writer for register DMACH0LEN"]
pub type W = crate::W<u32, super::DMACH0LEN>;
#[doc = "Register DMACH0LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACH0LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMALEN`"]
pub type DMALEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMALEN`"]
pub struct DMALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMALEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DMALEN"]
    #[inline(always)]
    pub fn dmalen(&self) -> DMALEN_R {
        DMALEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMALEN"]
    #[inline(always)]
    pub fn dmalen(&mut self) -> DMALEN_W {
        DMALEN_W { w: self }
    }
}
