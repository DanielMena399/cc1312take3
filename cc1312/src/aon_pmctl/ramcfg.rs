#[doc = "Reader of register RAMCFG"]
pub type R = crate::R<u32, super::RAMCFG>;
#[doc = "Writer for register RAMCFG"]
pub type W = crate::W<u32, super::RAMCFG>;
#[doc = "Register RAMCFG `reset()`'s with value 0x0001_000f"]
impl crate::ResetValue for super::RAMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_000f
    }
}
#[doc = "Reader of field `AUX_SRAM_PWR_OFF`"]
pub type AUX_SRAM_PWR_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_SRAM_PWR_OFF`"]
pub struct AUX_SRAM_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_SRAM_PWR_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `AUX_SRAM_RET_EN`"]
pub type AUX_SRAM_RET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_SRAM_RET_EN`"]
pub struct AUX_SRAM_RET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_SRAM_RET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `BUS_SRAM_RET_EN`"]
pub type BUS_SRAM_RET_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUS_SRAM_RET_EN`"]
pub struct BUS_SRAM_RET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_SRAM_RET_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - AUX_SRAM_PWR_OFF"]
    #[inline(always)]
    pub fn aux_sram_pwr_off(&self) -> AUX_SRAM_PWR_OFF_R {
        AUX_SRAM_PWR_OFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AUX_SRAM_RET_EN"]
    #[inline(always)]
    pub fn aux_sram_ret_en(&self) -> AUX_SRAM_RET_EN_R {
        AUX_SRAM_RET_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - BUS_SRAM_RET_EN"]
    #[inline(always)]
    pub fn bus_sram_ret_en(&self) -> BUS_SRAM_RET_EN_R {
        BUS_SRAM_RET_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 17 - AUX_SRAM_PWR_OFF"]
    #[inline(always)]
    pub fn aux_sram_pwr_off(&mut self) -> AUX_SRAM_PWR_OFF_W {
        AUX_SRAM_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 16 - AUX_SRAM_RET_EN"]
    #[inline(always)]
    pub fn aux_sram_ret_en(&mut self) -> AUX_SRAM_RET_EN_W {
        AUX_SRAM_RET_EN_W { w: self }
    }
    #[doc = "Bits 0:3 - BUS_SRAM_RET_EN"]
    #[inline(always)]
    pub fn bus_sram_ret_en(&mut self) -> BUS_SRAM_RET_EN_W {
        BUS_SRAM_RET_EN_W { w: self }
    }
}
