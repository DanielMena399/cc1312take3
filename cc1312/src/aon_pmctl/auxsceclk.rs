#[doc = "Reader of register AUXSCECLK"]
pub type R = crate::R<u32, super::AUXSCECLK>;
#[doc = "Writer for register AUXSCECLK"]
pub type W = crate::W<u32, super::AUXSCECLK>;
#[doc = "Register AUXSCECLK `reset()`'s with value 0"]
impl crate::ResetValue for super::AUXSCECLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PD_SRC`"]
pub type PD_SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_SRC`"]
pub struct PD_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_SRC_W<'a> {
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
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
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
    #[doc = "Bit 8 - PD_SRC"]
    #[inline(always)]
    pub fn pd_src(&self) -> PD_SRC_R {
        PD_SRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SRC"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PD_SRC"]
    #[inline(always)]
    pub fn pd_src(&mut self) -> PD_SRC_W {
        PD_SRC_W { w: self }
    }
    #[doc = "Bit 0 - SRC"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
