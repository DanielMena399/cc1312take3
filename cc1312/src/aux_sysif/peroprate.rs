#[doc = "Reader of register PEROPRATE"]
pub type R = crate::R<u32, super::PEROPRATE>;
#[doc = "Writer for register PEROPRATE"]
pub type W = crate::W<u32, super::PEROPRATE>;
#[doc = "Register PEROPRATE `reset()`'s with value 0"]
impl crate::ResetValue for super::PEROPRATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ANAIF_DAC_OP_RATE`"]
pub type ANAIF_DAC_OP_RATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANAIF_DAC_OP_RATE`"]
pub struct ANAIF_DAC_OP_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAIF_DAC_OP_RATE_W<'a> {
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
#[doc = "Reader of field `TIMER01_OP_RATE`"]
pub type TIMER01_OP_RATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER01_OP_RATE`"]
pub struct TIMER01_OP_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER01_OP_RATE_W<'a> {
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
#[doc = "Reader of field `SPIM_OP_RATE`"]
pub type SPIM_OP_RATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIM_OP_RATE`"]
pub struct SPIM_OP_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIM_OP_RATE_W<'a> {
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
#[doc = "Reader of field `MAC_OP_RATE`"]
pub type MAC_OP_RATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAC_OP_RATE`"]
pub struct MAC_OP_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_OP_RATE_W<'a> {
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
    #[doc = "Bit 3 - ANAIF_DAC_OP_RATE"]
    #[inline(always)]
    pub fn anaif_dac_op_rate(&self) -> ANAIF_DAC_OP_RATE_R {
        ANAIF_DAC_OP_RATE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIMER01_OP_RATE"]
    #[inline(always)]
    pub fn timer01_op_rate(&self) -> TIMER01_OP_RATE_R {
        TIMER01_OP_RATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPIM_OP_RATE"]
    #[inline(always)]
    pub fn spim_op_rate(&self) -> SPIM_OP_RATE_R {
        SPIM_OP_RATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MAC_OP_RATE"]
    #[inline(always)]
    pub fn mac_op_rate(&self) -> MAC_OP_RATE_R {
        MAC_OP_RATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ANAIF_DAC_OP_RATE"]
    #[inline(always)]
    pub fn anaif_dac_op_rate(&mut self) -> ANAIF_DAC_OP_RATE_W {
        ANAIF_DAC_OP_RATE_W { w: self }
    }
    #[doc = "Bit 2 - TIMER01_OP_RATE"]
    #[inline(always)]
    pub fn timer01_op_rate(&mut self) -> TIMER01_OP_RATE_W {
        TIMER01_OP_RATE_W { w: self }
    }
    #[doc = "Bit 1 - SPIM_OP_RATE"]
    #[inline(always)]
    pub fn spim_op_rate(&mut self) -> SPIM_OP_RATE_W {
        SPIM_OP_RATE_W { w: self }
    }
    #[doc = "Bit 0 - MAC_OP_RATE"]
    #[inline(always)]
    pub fn mac_op_rate(&mut self) -> MAC_OP_RATE_W {
        MAC_OP_RATE_W { w: self }
    }
}
