#[doc = "Reader of register NVIC_ISER1"]
pub type R = crate::R<u32, super::NVIC_ISER1>;
#[doc = "Writer for register NVIC_ISER1"]
pub type W = crate::W<u32, super::NVIC_ISER1>;
#[doc = "Register NVIC_ISER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SETENA37`"]
pub type SETENA37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA37`"]
pub struct SETENA37_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA37_W<'a> {
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
#[doc = "Reader of field `SETENA36`"]
pub type SETENA36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA36`"]
pub struct SETENA36_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA36_W<'a> {
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
#[doc = "Reader of field `SETENA35`"]
pub type SETENA35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA35`"]
pub struct SETENA35_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA35_W<'a> {
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
#[doc = "Reader of field `SETENA34`"]
pub type SETENA34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA34`"]
pub struct SETENA34_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA34_W<'a> {
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
#[doc = "Reader of field `SETENA33`"]
pub type SETENA33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA33`"]
pub struct SETENA33_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA33_W<'a> {
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
#[doc = "Reader of field `SETENA32`"]
pub type SETENA32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETENA32`"]
pub struct SETENA32_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA32_W<'a> {
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
    #[doc = "Bit 5 - SETENA37"]
    #[inline(always)]
    pub fn setena37(&self) -> SETENA37_R {
        SETENA37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SETENA36"]
    #[inline(always)]
    pub fn setena36(&self) -> SETENA36_R {
        SETENA36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SETENA35"]
    #[inline(always)]
    pub fn setena35(&self) -> SETENA35_R {
        SETENA35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SETENA34"]
    #[inline(always)]
    pub fn setena34(&self) -> SETENA34_R {
        SETENA34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SETENA33"]
    #[inline(always)]
    pub fn setena33(&self) -> SETENA33_R {
        SETENA33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SETENA32"]
    #[inline(always)]
    pub fn setena32(&self) -> SETENA32_R {
        SETENA32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - SETENA37"]
    #[inline(always)]
    pub fn setena37(&mut self) -> SETENA37_W {
        SETENA37_W { w: self }
    }
    #[doc = "Bit 4 - SETENA36"]
    #[inline(always)]
    pub fn setena36(&mut self) -> SETENA36_W {
        SETENA36_W { w: self }
    }
    #[doc = "Bit 3 - SETENA35"]
    #[inline(always)]
    pub fn setena35(&mut self) -> SETENA35_W {
        SETENA35_W { w: self }
    }
    #[doc = "Bit 2 - SETENA34"]
    #[inline(always)]
    pub fn setena34(&mut self) -> SETENA34_W {
        SETENA34_W { w: self }
    }
    #[doc = "Bit 1 - SETENA33"]
    #[inline(always)]
    pub fn setena33(&mut self) -> SETENA33_W {
        SETENA33_W { w: self }
    }
    #[doc = "Bit 0 - SETENA32"]
    #[inline(always)]
    pub fn setena32(&mut self) -> SETENA32_W {
        SETENA32_W { w: self }
    }
}
