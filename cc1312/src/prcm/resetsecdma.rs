#[doc = "Reader of register RESETSECDMA"]
pub type R = crate::R<u32, super::RESETSECDMA>;
#[doc = "Writer for register RESETSECDMA"]
pub type W = crate::W<u32, super::RESETSECDMA>;
#[doc = "Register RESETSECDMA `reset()`'s with value 0"]
impl crate::ResetValue for super::RESETSECDMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
#[doc = "Reader of field `PKA`"]
pub type PKA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA`"]
pub struct PKA_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_W<'a> {
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
#[doc = "Reader of field `TRNG`"]
pub type TRNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG`"]
pub struct TRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_W<'a> {
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
#[doc = "Reader of field `CRYPTO`"]
pub type CRYPTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO`"]
pub struct CRYPTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_W<'a> {
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
    #[doc = "Bit 8 - DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PKA"]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TRNG"]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CRYPTO"]
    #[inline(always)]
    pub fn crypto(&self) -> CRYPTO_R {
        CRYPTO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 2 - PKA"]
    #[inline(always)]
    pub fn pka(&mut self) -> PKA_W {
        PKA_W { w: self }
    }
    #[doc = "Bit 1 - TRNG"]
    #[inline(always)]
    pub fn trng(&mut self) -> TRNG_W {
        TRNG_W { w: self }
    }
    #[doc = "Bit 0 - CRYPTO"]
    #[inline(always)]
    pub fn crypto(&mut self) -> CRYPTO_W {
        CRYPTO_W { w: self }
    }
}
