#[doc = "Reader of register DMAPROTCTL"]
pub type R = crate::R<u32, super::DMAPROTCTL>;
#[doc = "Writer for register DMAPROTCTL"]
pub type W = crate::W<u32, super::DMAPROTCTL>;
#[doc = "Register DMAPROTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAPROTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROT_EN`"]
pub type PROT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT_EN`"]
pub struct PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_EN_W<'a> {
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
    #[doc = "Bit 0 - PROT_EN"]
    #[inline(always)]
    pub fn prot_en(&self) -> PROT_EN_R {
        PROT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PROT_EN"]
    #[inline(always)]
    pub fn prot_en(&mut self) -> PROT_EN_W {
        PROT_EN_W { w: self }
    }
}
