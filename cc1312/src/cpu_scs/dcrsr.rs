#[doc = "Writer for register DCRSR"]
pub type W = crate::W<u32, super::DCRSR>;
#[doc = "Register DCRSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCRSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `REGWNR`"]
pub struct REGWNR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGWNR_W<'a> {
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
#[doc = "Write proxy for field `REGSEL`"]
pub struct REGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl W {
    #[doc = "Bit 16 - REGWNR"]
    #[inline(always)]
    pub fn regwnr(&mut self) -> REGWNR_W {
        REGWNR_W { w: self }
    }
    #[doc = "Bits 0:4 - REGSEL"]
    #[inline(always)]
    pub fn regsel(&mut self) -> REGSEL_W {
        REGSEL_W { w: self }
    }
}
