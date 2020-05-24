#[doc = "Reader of register CH2EVCFG"]
pub type R = crate::R<u32, super::CH2EVCFG>;
#[doc = "Writer for register CH2EVCFG"]
pub type W = crate::W<u32, super::CH2EVCFG>;
#[doc = "Register CH2EVCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2EVCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EV3_GEN`"]
pub type EV3_GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV3_GEN`"]
pub struct EV3_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV3_GEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EV2_GEN`"]
pub type EV2_GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV2_GEN`"]
pub struct EV2_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2_GEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EV1_GEN`"]
pub type EV1_GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV1_GEN`"]
pub struct EV1_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1_GEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EV0_GEN`"]
pub type EV0_GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EV0_GEN`"]
pub struct EV0_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_GEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CCACT`"]
pub type CCACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCACT`"]
pub struct CCACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CCACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - EV3_GEN"]
    #[inline(always)]
    pub fn ev3_gen(&self) -> EV3_GEN_R {
        EV3_GEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EV2_GEN"]
    #[inline(always)]
    pub fn ev2_gen(&self) -> EV2_GEN_R {
        EV2_GEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EV1_GEN"]
    #[inline(always)]
    pub fn ev1_gen(&self) -> EV1_GEN_R {
        EV1_GEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EV0_GEN"]
    #[inline(always)]
    pub fn ev0_gen(&self) -> EV0_GEN_R {
        EV0_GEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - CCACT"]
    #[inline(always)]
    pub fn ccact(&self) -> CCACT_R {
        CCACT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - EV3_GEN"]
    #[inline(always)]
    pub fn ev3_gen(&mut self) -> EV3_GEN_W {
        EV3_GEN_W { w: self }
    }
    #[doc = "Bit 6 - EV2_GEN"]
    #[inline(always)]
    pub fn ev2_gen(&mut self) -> EV2_GEN_W {
        EV2_GEN_W { w: self }
    }
    #[doc = "Bit 5 - EV1_GEN"]
    #[inline(always)]
    pub fn ev1_gen(&mut self) -> EV1_GEN_W {
        EV1_GEN_W { w: self }
    }
    #[doc = "Bit 4 - EV0_GEN"]
    #[inline(always)]
    pub fn ev0_gen(&mut self) -> EV0_GEN_W {
        EV0_GEN_W { w: self }
    }
    #[doc = "Bits 0:3 - CCACT"]
    #[inline(always)]
    pub fn ccact(&mut self) -> CCACT_W {
        CCACT_W { w: self }
    }
}
