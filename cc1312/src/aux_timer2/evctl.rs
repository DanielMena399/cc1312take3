#[doc = "Writer for register EVCTL"]
pub type W = crate::W<u32, super::EVCTL>;
#[doc = "Register EVCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EV3_SET`"]
pub struct EV3_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EV3_SET_W<'a> {
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
#[doc = "Write proxy for field `EV3_CLR`"]
pub struct EV3_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EV3_CLR_W<'a> {
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
#[doc = "Write proxy for field `EV2_SET`"]
pub struct EV2_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2_SET_W<'a> {
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
#[doc = "Write proxy for field `EV2_CLR`"]
pub struct EV2_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EV2_CLR_W<'a> {
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
#[doc = "Write proxy for field `EV1_SET`"]
pub struct EV1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `EV1_CLR`"]
pub struct EV1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EV1_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `EV0_SET`"]
pub struct EV0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `EV0_CLR`"]
pub struct EV0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EV0_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 7 - EV3_SET"]
    #[inline(always)]
    pub fn ev3_set(&mut self) -> EV3_SET_W {
        EV3_SET_W { w: self }
    }
    #[doc = "Bit 6 - EV3_CLR"]
    #[inline(always)]
    pub fn ev3_clr(&mut self) -> EV3_CLR_W {
        EV3_CLR_W { w: self }
    }
    #[doc = "Bit 5 - EV2_SET"]
    #[inline(always)]
    pub fn ev2_set(&mut self) -> EV2_SET_W {
        EV2_SET_W { w: self }
    }
    #[doc = "Bit 4 - EV2_CLR"]
    #[inline(always)]
    pub fn ev2_clr(&mut self) -> EV2_CLR_W {
        EV2_CLR_W { w: self }
    }
    #[doc = "Bit 3 - EV1_SET"]
    #[inline(always)]
    pub fn ev1_set(&mut self) -> EV1_SET_W {
        EV1_SET_W { w: self }
    }
    #[doc = "Bit 2 - EV1_CLR"]
    #[inline(always)]
    pub fn ev1_clr(&mut self) -> EV1_CLR_W {
        EV1_CLR_W { w: self }
    }
    #[doc = "Bit 1 - EV0_SET"]
    #[inline(always)]
    pub fn ev0_set(&mut self) -> EV0_SET_W {
        EV0_SET_W { w: self }
    }
    #[doc = "Bit 0 - EV0_CLR"]
    #[inline(always)]
    pub fn ev0_clr(&mut self) -> EV0_CLR_W {
        EV0_CLR_W { w: self }
    }
}
