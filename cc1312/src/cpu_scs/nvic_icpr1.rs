#[doc = "Reader of register NVIC_ICPR1"]
pub type R = crate::R<u32, super::NVIC_ICPR1>;
#[doc = "Writer for register NVIC_ICPR1"]
pub type W = crate::W<u32, super::NVIC_ICPR1>;
#[doc = "Register NVIC_ICPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ICPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRPEND37`"]
pub type CLRPEND37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND37`"]
pub struct CLRPEND37_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND37_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CLRPEND36`"]
pub type CLRPEND36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND36`"]
pub struct CLRPEND36_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND36_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CLRPEND35`"]
pub type CLRPEND35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND35`"]
pub struct CLRPEND35_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND35_W<'a> {
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
#[doc = "Reader of field `CLRPEND34`"]
pub type CLRPEND34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND34`"]
pub struct CLRPEND34_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND34_W<'a> {
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
#[doc = "Reader of field `CLRPEND33`"]
pub type CLRPEND33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND33`"]
pub struct CLRPEND33_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND33_W<'a> {
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
#[doc = "Reader of field `CLRPEND32`"]
pub type CLRPEND32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRPEND32`"]
pub struct CLRPEND32_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND32_W<'a> {
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
    #[doc = "Bit 5 - CLRPEND37"]
    #[inline(always)]
    pub fn clrpend37(&self) -> CLRPEND37_R {
        CLRPEND37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CLRPEND36"]
    #[inline(always)]
    pub fn clrpend36(&self) -> CLRPEND36_R {
        CLRPEND36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLRPEND35"]
    #[inline(always)]
    pub fn clrpend35(&self) -> CLRPEND35_R {
        CLRPEND35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CLRPEND34"]
    #[inline(always)]
    pub fn clrpend34(&self) -> CLRPEND34_R {
        CLRPEND34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CLRPEND33"]
    #[inline(always)]
    pub fn clrpend33(&self) -> CLRPEND33_R {
        CLRPEND33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CLRPEND32"]
    #[inline(always)]
    pub fn clrpend32(&self) -> CLRPEND32_R {
        CLRPEND32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CLRPEND37"]
    #[inline(always)]
    pub fn clrpend37(&mut self) -> CLRPEND37_W {
        CLRPEND37_W { w: self }
    }
    #[doc = "Bit 4 - CLRPEND36"]
    #[inline(always)]
    pub fn clrpend36(&mut self) -> CLRPEND36_W {
        CLRPEND36_W { w: self }
    }
    #[doc = "Bit 3 - CLRPEND35"]
    #[inline(always)]
    pub fn clrpend35(&mut self) -> CLRPEND35_W {
        CLRPEND35_W { w: self }
    }
    #[doc = "Bit 2 - CLRPEND34"]
    #[inline(always)]
    pub fn clrpend34(&mut self) -> CLRPEND34_W {
        CLRPEND34_W { w: self }
    }
    #[doc = "Bit 1 - CLRPEND33"]
    #[inline(always)]
    pub fn clrpend33(&mut self) -> CLRPEND33_W {
        CLRPEND33_W { w: self }
    }
    #[doc = "Bit 0 - CLRPEND32"]
    #[inline(always)]
    pub fn clrpend32(&mut self) -> CLRPEND32_W {
        CLRPEND32_W { w: self }
    }
}
