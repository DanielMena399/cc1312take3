#[doc = "Reader of register APTR"]
pub type R = crate::R<u32, super::APTR>;
#[doc = "Writer for register APTR"]
pub type W = crate::W<u32, super::APTR>;
#[doc = "Register APTR `reset()`'s with value 0"]
impl crate::ResetValue for super::APTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APTR`"]
pub type APTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APTR`"]
pub struct APTR_W<'a> {
    w: &'a mut W,
}
impl<'a> APTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - APTR"]
    #[inline(always)]
    pub fn aptr(&self) -> APTR_R {
        APTR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - APTR"]
    #[inline(always)]
    pub fn aptr(&mut self) -> APTR_W {
        APTR_W { w: self }
    }
}
