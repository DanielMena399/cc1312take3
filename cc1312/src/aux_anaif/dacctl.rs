#[doc = "Reader of register DACCTL"]
pub type R = crate::R<u32, super::DACCTL>;
#[doc = "Writer for register DACCTL"]
pub type W = crate::W<u32, super::DACCTL>;
#[doc = "Register DACCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DACCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAC_EN`"]
pub type DAC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_EN`"]
pub struct DAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_EN_W<'a> {
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
#[doc = "Reader of field `DAC_BUFFER_EN`"]
pub type DAC_BUFFER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_BUFFER_EN`"]
pub struct DAC_BUFFER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_BUFFER_EN_W<'a> {
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
#[doc = "Reader of field `DAC_PRECHARGE_EN`"]
pub type DAC_PRECHARGE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_PRECHARGE_EN`"]
pub struct DAC_PRECHARGE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_PRECHARGE_EN_W<'a> {
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
#[doc = "Reader of field `DAC_VOUT_SEL`"]
pub type DAC_VOUT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_VOUT_SEL`"]
pub struct DAC_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - DAC_EN"]
    #[inline(always)]
    pub fn dac_en(&self) -> DAC_EN_R {
        DAC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DAC_BUFFER_EN"]
    #[inline(always)]
    pub fn dac_buffer_en(&self) -> DAC_BUFFER_EN_R {
        DAC_BUFFER_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DAC_PRECHARGE_EN"]
    #[inline(always)]
    pub fn dac_precharge_en(&self) -> DAC_PRECHARGE_EN_R {
        DAC_PRECHARGE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - DAC_VOUT_SEL"]
    #[inline(always)]
    pub fn dac_vout_sel(&self) -> DAC_VOUT_SEL_R {
        DAC_VOUT_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - DAC_EN"]
    #[inline(always)]
    pub fn dac_en(&mut self) -> DAC_EN_W {
        DAC_EN_W { w: self }
    }
    #[doc = "Bit 4 - DAC_BUFFER_EN"]
    #[inline(always)]
    pub fn dac_buffer_en(&mut self) -> DAC_BUFFER_EN_W {
        DAC_BUFFER_EN_W { w: self }
    }
    #[doc = "Bit 3 - DAC_PRECHARGE_EN"]
    #[inline(always)]
    pub fn dac_precharge_en(&mut self) -> DAC_PRECHARGE_EN_W {
        DAC_PRECHARGE_EN_W { w: self }
    }
    #[doc = "Bits 0:2 - DAC_VOUT_SEL"]
    #[inline(always)]
    pub fn dac_vout_sel(&mut self) -> DAC_VOUT_SEL_W {
        DAC_VOUT_SEL_W { w: self }
    }
}
