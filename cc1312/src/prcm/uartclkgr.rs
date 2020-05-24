#[doc = "Reader of register UARTCLKGR"]
pub type R = crate::R<u32, super::UARTCLKGR>;
#[doc = "Writer for register UARTCLKGR"]
pub type W = crate::W<u32, super::UARTCLKGR>;
#[doc = "Register UARTCLKGR `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTCLKGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART1_AM_CLK_EN`"]
pub type UART1_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1_AM_CLK_EN`"]
pub struct UART1_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `UART0_AM_CLK_EN`"]
pub type UART0_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0_AM_CLK_EN`"]
pub struct UART0_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `UART1_CLK_EN`"]
pub type UART1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1_CLK_EN`"]
pub struct UART1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `UART0_CLK_EN`"]
pub type UART0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0_CLK_EN`"]
pub struct UART0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_CLK_EN_W<'a> {
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
    #[doc = "Bit 9 - Enable clock for UART1 for all modes"]
    #[inline(always)]
    pub fn uart1_am_clk_en(&self) -> UART1_AM_CLK_EN_R {
        UART1_AM_CLK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable clock for UART0 for all modes"]
    #[inline(always)]
    pub fn uart0_am_clk_en(&self) -> UART0_AM_CLK_EN_R {
        UART0_AM_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable clock for UART1"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable clock for UART0"]
    #[inline(always)]
    pub fn uart0_clk_en(&self) -> UART0_CLK_EN_R {
        UART0_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Enable clock for UART1 for all modes"]
    #[inline(always)]
    pub fn uart1_am_clk_en(&mut self) -> UART1_AM_CLK_EN_W {
        UART1_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8 - Enable clock for UART0 for all modes"]
    #[inline(always)]
    pub fn uart0_am_clk_en(&mut self) -> UART0_AM_CLK_EN_W {
        UART0_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable clock for UART1"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W {
        UART1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable clock for UART0"]
    #[inline(always)]
    pub fn uart0_clk_en(&mut self) -> UART0_CLK_EN_W {
        UART0_CLK_EN_W { w: self }
    }
}
