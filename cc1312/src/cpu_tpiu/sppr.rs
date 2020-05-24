#[doc = "Reader of register SPPR"]
pub type R = crate::R<u32, super::SPPR>;
#[doc = "Writer for register SPPR"]
pub type W = crate::W<u32, super::SPPR>;
#[doc = "Register SPPR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SPPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `PROTOCOL`"]
pub type PROTOCOL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROTOCOL`"]
pub struct PROTOCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTOCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PROTOCOL"]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PROTOCOL"]
    #[inline(always)]
    pub fn protocol(&mut self) -> PROTOCOL_W {
        PROTOCOL_W { w: self }
    }
}
