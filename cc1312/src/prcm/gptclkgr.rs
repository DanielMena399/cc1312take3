#[doc = "Reader of register GPTCLKGR"]
pub type R = crate::R<u32, super::GPTCLKGR>;
#[doc = "Writer for register GPTCLKGR"]
pub type W = crate::W<u32, super::GPTCLKGR>;
#[doc = "Register GPTCLKGR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTCLKGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPT3_AM_CLK_EN`"]
pub type GPT3_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT3_AM_CLK_EN`"]
pub struct GPT3_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT3_AM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `GPT2_AM_CLK_EN`"]
pub type GPT2_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT2_AM_CLK_EN`"]
pub struct GPT2_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT2_AM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `GPT1_AM_CLK_EN`"]
pub type GPT1_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT1_AM_CLK_EN`"]
pub struct GPT1_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT1_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `GPT0_AM_CLK_EN`"]
pub type GPT0_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT0_AM_CLK_EN`"]
pub struct GPT0_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT0_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `GPT3_CLK_EN`"]
pub type GPT3_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT3_CLK_EN`"]
pub struct GPT3_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT3_CLK_EN_W<'a> {
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
#[doc = "Reader of field `GPT2_CLK_EN`"]
pub type GPT2_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT2_CLK_EN`"]
pub struct GPT2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT2_CLK_EN_W<'a> {
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
#[doc = "Reader of field `GPT1_CLK_EN`"]
pub type GPT1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT1_CLK_EN`"]
pub struct GPT1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `GPT0_CLK_EN`"]
pub type GPT0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPT0_CLK_EN`"]
pub struct GPT0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT0_CLK_EN_W<'a> {
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
    #[doc = "Bit 11 - Enable clock for GPT3 in all modes"]
    #[inline(always)]
    pub fn gpt3_am_clk_en(&self) -> GPT3_AM_CLK_EN_R {
        GPT3_AM_CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable clock for GPT2 in all modes"]
    #[inline(always)]
    pub fn gpt2_am_clk_en(&self) -> GPT2_AM_CLK_EN_R {
        GPT2_AM_CLK_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable clock for GPT1 in all modes"]
    #[inline(always)]
    pub fn gpt1_am_clk_en(&self) -> GPT1_AM_CLK_EN_R {
        GPT1_AM_CLK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable clock for GPT0 in all modes"]
    #[inline(always)]
    pub fn gpt0_am_clk_en(&self) -> GPT0_AM_CLK_EN_R {
        GPT0_AM_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable clock for GPT3"]
    #[inline(always)]
    pub fn gpt3_clk_en(&self) -> GPT3_CLK_EN_R {
        GPT3_CLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable clock for GPT2"]
    #[inline(always)]
    pub fn gpt2_clk_en(&self) -> GPT2_CLK_EN_R {
        GPT2_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable clock for GPT1"]
    #[inline(always)]
    pub fn gpt1_clk_en(&self) -> GPT1_CLK_EN_R {
        GPT1_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable clock for GPT0"]
    #[inline(always)]
    pub fn gpt0_clk_en(&self) -> GPT0_CLK_EN_R {
        GPT0_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Enable clock for GPT3 in all modes"]
    #[inline(always)]
    pub fn gpt3_am_clk_en(&mut self) -> GPT3_AM_CLK_EN_W {
        GPT3_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 10 - Enable clock for GPT2 in all modes"]
    #[inline(always)]
    pub fn gpt2_am_clk_en(&mut self) -> GPT2_AM_CLK_EN_W {
        GPT2_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 9 - Enable clock for GPT1 in all modes"]
    #[inline(always)]
    pub fn gpt1_am_clk_en(&mut self) -> GPT1_AM_CLK_EN_W {
        GPT1_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8 - Enable clock for GPT0 in all modes"]
    #[inline(always)]
    pub fn gpt0_am_clk_en(&mut self) -> GPT0_AM_CLK_EN_W {
        GPT0_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 3 - Enable clock for GPT3"]
    #[inline(always)]
    pub fn gpt3_clk_en(&mut self) -> GPT3_CLK_EN_W {
        GPT3_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable clock for GPT2"]
    #[inline(always)]
    pub fn gpt2_clk_en(&mut self) -> GPT2_CLK_EN_W {
        GPT2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable clock for GPT1"]
    #[inline(always)]
    pub fn gpt1_clk_en(&mut self) -> GPT1_CLK_EN_W {
        GPT1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable clock for GPT0"]
    #[inline(always)]
    pub fn gpt0_clk_en(&mut self) -> GPT0_CLK_EN_W {
        GPT0_CLK_EN_W { w: self }
    }
}
