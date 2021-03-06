#[doc = "Reader of register RECHARGETRIG"]
pub type R = crate::R<u32, super::RECHARGETRIG>;
#[doc = "Writer for register RECHARGETRIG"]
pub type W = crate::W<u32, super::RECHARGETRIG>;
#[doc = "Register RECHARGETRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::RECHARGETRIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIG`"]
pub type TRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG`"]
pub struct TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TRIG"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG"]
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W {
        TRIG_W { w: self }
    }
}
