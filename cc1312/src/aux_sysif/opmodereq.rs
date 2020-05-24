#[doc = "Reader of register OPMODEREQ"]
pub type R = crate::R<u32, super::OPMODEREQ>;
#[doc = "Writer for register OPMODEREQ"]
pub type W = crate::W<u32, super::OPMODEREQ>;
#[doc = "Register OPMODEREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::OPMODEREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REQ`"]
pub type REQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REQ`"]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - REQ"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - REQ"]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
}
