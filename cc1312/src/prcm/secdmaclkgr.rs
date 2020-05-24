#[doc = "Reader of register SECDMACLKGR"]
pub type R = crate::R<u32, super::SECDMACLKGR>;
#[doc = "Writer for register SECDMACLKGR"]
pub type W = crate::W<u32, super::SECDMACLKGR>;
#[doc = "Register SECDMACLKGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECDMACLKGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_AM_CLK_EN`"]
pub type DMA_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_AM_CLK_EN`"]
pub struct DMA_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_AM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `PKA_ZERIOZE_RESET_N`"]
pub type PKA_ZERIOZE_RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA_ZERIOZE_RESET_N`"]
pub struct PKA_ZERIOZE_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_ZERIOZE_RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PKA_AM_CLK_EN`"]
pub type PKA_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA_AM_CLK_EN`"]
pub struct PKA_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_AM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TRNG_AM_CLK_EN`"]
pub type TRNG_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG_AM_CLK_EN`"]
pub struct TRNG_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `CRYPTO_AM_CLK_EN`"]
pub type CRYPTO_AM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO_AM_CLK_EN`"]
pub struct CRYPTO_AM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_AM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `DMA_CLK_EN`"]
pub type DMA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_CLK_EN`"]
pub struct DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CLK_EN_W<'a> {
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
#[doc = "Reader of field `PKA_CLK_EN`"]
pub type PKA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA_CLK_EN`"]
pub struct PKA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_CLK_EN_W<'a> {
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
#[doc = "Reader of field `TRNG_CLK_EN`"]
pub type TRNG_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG_CLK_EN`"]
pub struct TRNG_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_CLK_EN_W<'a> {
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
#[doc = "Reader of field `CRYPTO_CLK_EN`"]
pub type CRYPTO_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO_CLK_EN`"]
pub struct CRYPTO_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_CLK_EN_W<'a> {
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
    #[doc = "Bit 24 - DMA_AM_CLK_EN"]
    #[inline(always)]
    pub fn dma_am_clk_en(&self) -> DMA_AM_CLK_EN_R {
        DMA_AM_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PKA_ZERIOZE_RESET_N"]
    #[inline(always)]
    pub fn pka_zerioze_reset_n(&self) -> PKA_ZERIOZE_RESET_N_R {
        PKA_ZERIOZE_RESET_N_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PKA_AM_CLK_EN"]
    #[inline(always)]
    pub fn pka_am_clk_en(&self) -> PKA_AM_CLK_EN_R {
        PKA_AM_CLK_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TRNG_AM_CLK_EN"]
    #[inline(always)]
    pub fn trng_am_clk_en(&self) -> TRNG_AM_CLK_EN_R {
        TRNG_AM_CLK_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRYPTO_AM_CLK_EN"]
    #[inline(always)]
    pub fn crypto_am_clk_en(&self) -> CRYPTO_AM_CLK_EN_R {
        CRYPTO_AM_CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA_CLK_EN"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PKA_CLK_EN"]
    #[inline(always)]
    pub fn pka_clk_en(&self) -> PKA_CLK_EN_R {
        PKA_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TRNG_CLK_EN"]
    #[inline(always)]
    pub fn trng_clk_en(&self) -> TRNG_CLK_EN_R {
        TRNG_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CRYPTO_CLK_EN"]
    #[inline(always)]
    pub fn crypto_clk_en(&self) -> CRYPTO_CLK_EN_R {
        CRYPTO_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DMA_AM_CLK_EN"]
    #[inline(always)]
    pub fn dma_am_clk_en(&mut self) -> DMA_AM_CLK_EN_W {
        DMA_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 19 - PKA_ZERIOZE_RESET_N"]
    #[inline(always)]
    pub fn pka_zerioze_reset_n(&mut self) -> PKA_ZERIOZE_RESET_N_W {
        PKA_ZERIOZE_RESET_N_W { w: self }
    }
    #[doc = "Bit 18 - PKA_AM_CLK_EN"]
    #[inline(always)]
    pub fn pka_am_clk_en(&mut self) -> PKA_AM_CLK_EN_W {
        PKA_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 17 - TRNG_AM_CLK_EN"]
    #[inline(always)]
    pub fn trng_am_clk_en(&mut self) -> TRNG_AM_CLK_EN_W {
        TRNG_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 16 - CRYPTO_AM_CLK_EN"]
    #[inline(always)]
    pub fn crypto_am_clk_en(&mut self) -> CRYPTO_AM_CLK_EN_W {
        CRYPTO_AM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8 - DMA_CLK_EN"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - PKA_CLK_EN"]
    #[inline(always)]
    pub fn pka_clk_en(&mut self) -> PKA_CLK_EN_W {
        PKA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - TRNG_CLK_EN"]
    #[inline(always)]
    pub fn trng_clk_en(&mut self) -> TRNG_CLK_EN_W {
        TRNG_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - CRYPTO_CLK_EN"]
    #[inline(always)]
    pub fn crypto_clk_en(&mut self) -> CRYPTO_CLK_EN_W {
        CRYPTO_CLK_EN_W { w: self }
    }
}
