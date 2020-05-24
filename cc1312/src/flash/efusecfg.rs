#[doc = "Reader of register EFUSECFG"]
pub type R = crate::R<u32, super::EFUSECFG>;
#[doc = "Writer for register EFUSECFG"]
pub type W = crate::W<u32, super::EFUSECFG>;
#[doc = "Register EFUSECFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::EFUSECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `IDLEGATING`"]
pub type IDLEGATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLEGATING`"]
pub struct IDLEGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEGATING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SLAVEPOWER`"]
pub type SLAVEPOWER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLAVEPOWER`"]
pub struct SLAVEPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVEPOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `GATING`"]
pub type GATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GATING`"]
pub struct GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> GATING_W<'a> {
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
    #[doc = "Bit 8 - IDLEGATING"]
    #[inline(always)]
    pub fn idlegating(&self) -> IDLEGATING_R {
        IDLEGATING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - SLAVEPOWER"]
    #[inline(always)]
    pub fn slavepower(&self) -> SLAVEPOWER_R {
        SLAVEPOWER_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GATING"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IDLEGATING"]
    #[inline(always)]
    pub fn idlegating(&mut self) -> IDLEGATING_W {
        IDLEGATING_W { w: self }
    }
    #[doc = "Bits 3:4 - SLAVEPOWER"]
    #[inline(always)]
    pub fn slavepower(&mut self) -> SLAVEPOWER_W {
        SLAVEPOWER_W { w: self }
    }
    #[doc = "Bit 0 - GATING"]
    #[inline(always)]
    pub fn gating(&mut self) -> GATING_W {
        GATING_W { w: self }
    }
}
