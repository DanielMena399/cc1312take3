#[doc = "Reader of register EVOBSCFG"]
pub type R = crate::R<u32, super::EVOBSCFG>;
#[doc = "Writer for register EVOBSCFG"]
pub type W = crate::W<u32, super::EVOBSCFG>;
#[doc = "Register EVOBSCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::EVOBSCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVOBS_SEL`"]
pub type EVOBS_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EVOBS_SEL`"]
pub struct EVOBS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EVOBS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - EVOBS_SEL"]
    #[inline(always)]
    pub fn evobs_sel(&self) -> EVOBS_SEL_R {
        EVOBS_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - EVOBS_SEL"]
    #[inline(always)]
    pub fn evobs_sel(&mut self) -> EVOBS_SEL_W {
        EVOBS_SEL_W { w: self }
    }
}
