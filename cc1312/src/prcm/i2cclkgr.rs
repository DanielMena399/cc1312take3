#[doc = "Reader of register I2CCLKGR"]
pub type R = crate::R<u32, super::I2CCLKGR>;
#[doc = "Writer for register I2CCLKGR"]
pub type W = crate::W<u32, super::I2CCLKGR>;
#[doc = "Register I2CCLKGR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CCLKGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C0_AM_CLK_EN`"]
pub type I2C0_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0_AM_CLK_EN`"]
pub struct I2C0_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `I2C0_CLK_EN`"]
pub type I2C0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0_CLK_EN`"]
pub struct I2C0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_CLK_EN_W<'a> {
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
    #[doc = "Bit 8 - Enable clock for I2C0 for all modes"]
    #[inline(always)]
    pub fn i2c0_am_clk_en(&self) -> I2C0_AM_CLK_EN_R {
        I2C0_AM_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable clock for I2C0"]
    #[inline(always)]
    pub fn i2c0_clk_en(&self) -> I2C0_CLK_EN_R {
        I2C0_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable clock for I2C0 for all modes"]
    #[inline(always)]
    pub fn i2c0_am_clk_en(&mut self) -> I2C0_AM_CLK_EN_W {
        I2C0_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable clock for I2C0"]
    #[inline(always)]
    pub fn i2c0_clk_en(&mut self) -> I2C0_CLK_EN_W {
        I2C0_CLK_EN_W { w: self }
    }
}
