#[doc = "Reader of register MISOCFG"]
pub type R = crate::R<u32, super::MISOCFG>;
#[doc = "Writer for register MISOCFG"]
pub type W = crate::W<u32, super::MISOCFG>;
#[doc = "Register MISOCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MISOCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUXIO`"]
pub type AUXIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AUXIO`"]
pub struct AUXIO_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - AUXIO"]
    #[inline(always)]
    pub fn auxio(&self) -> AUXIO_R {
        AUXIO_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - AUXIO"]
    #[inline(always)]
    pub fn auxio(&mut self) -> AUXIO_W {
        AUXIO_W { w: self }
    }
}
