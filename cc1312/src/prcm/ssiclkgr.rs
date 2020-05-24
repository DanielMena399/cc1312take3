#[doc = "Reader of register SSICLKGR"]
pub type R = crate::R<u32, super::SSICLKGR>;
#[doc = "Writer for register SSICLKGR"]
pub type W = crate::W<u32, super::SSICLKGR>;
#[doc = "Register SSICLKGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSICLKGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSI1_AM_CLK_EN`"]
pub type SSI1_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI1_AM_CLK_EN`"]
pub struct SSI1_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI1_AM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SSI0_AM_CLK_EN`"]
pub type SSI0_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI0_AM_CLK_EN`"]
pub struct SSI0_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI0_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `SSI1_CLK_EN`"]
pub type SSI1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI1_CLK_EN`"]
pub struct SSI1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `SSI0_CLK_EN`"]
pub type SSI0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI0_CLK_EN`"]
pub struct SSI0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI0_CLK_EN_W<'a> {
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
    #[doc = "Bit 9 - Enable clock for SSI1 for all modes"]
    #[inline(always)]
    pub fn ssi1_am_clk_en(&self) -> SSI1_AM_CLK_EN_R {
        SSI1_AM_CLK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable clock for SSI0 for all modes"]
    #[inline(always)]
    pub fn ssi0_am_clk_en(&self) -> SSI0_AM_CLK_EN_R {
        SSI0_AM_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable clock for SSI1"]
    #[inline(always)]
    pub fn ssi1_clk_en(&self) -> SSI1_CLK_EN_R {
        SSI1_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable clock for SSI0"]
    #[inline(always)]
    pub fn ssi0_clk_en(&self) -> SSI0_CLK_EN_R {
        SSI0_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Enable clock for SSI1 for all modes"]
    #[inline(always)]
    pub fn ssi1_am_clk_en(&mut self) -> SSI1_AM_CLK_EN_W {
        SSI1_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8 - Enable clock for SSI0 for all modes"]
    #[inline(always)]
    pub fn ssi0_am_clk_en(&mut self) -> SSI0_AM_CLK_EN_W {
        SSI0_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable clock for SSI1"]
    #[inline(always)]
    pub fn ssi1_clk_en(&mut self) -> SSI1_CLK_EN_W {
        SSI1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable clock for SSI0"]
    #[inline(always)]
    pub fn ssi0_clk_en(&mut self) -> SSI0_CLK_EN_W {
        SSI0_CLK_EN_W { w: self }
    }
}
