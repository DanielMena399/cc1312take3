#[doc = "Reader of register VECCFG2"]
pub type R = crate::R<u32, super::VECCFG2>;
#[doc = "Writer for register VECCFG2"]
pub type W = crate::W<u32, super::VECCFG2>;
#[doc = "Register VECCFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::VECCFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VEC_EV`"]
pub type VEC_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VEC_EV`"]
pub struct VEC_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - VEC_EV"]
    #[inline(always)]
    pub fn vec_ev(&self) -> VEC_EV_R {
        VEC_EV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VEC_EV"]
    #[inline(always)]
    pub fn vec_ev(&mut self) -> VEC_EV_W {
        VEC_EV_W { w: self }
    }
}
