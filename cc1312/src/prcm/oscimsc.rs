#[doc = "Reader of register OSCIMSC"]
pub type R = crate::R<u32, super::OSCIMSC>;
#[doc = "Writer for register OSCIMSC"]
pub type W = crate::W<u32, super::OSCIMSC>;
#[doc = "Register OSCIMSC `reset()`'s with value 0x36"]
impl crate::ResetValue for super::OSCIMSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x36
    }
}
#[doc = "Reader of field `HFSRCPENDIM`"]
pub type HFSRCPENDIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFSRCPENDIM`"]
pub struct HFSRCPENDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HFSRCPENDIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `LFSRCDONEIM`"]
pub type LFSRCDONEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFSRCDONEIM`"]
pub struct LFSRCDONEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSRCDONEIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `XOSCDLFIM`"]
pub type XOSCDLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCDLFIM`"]
pub struct XOSCDLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCDLFIM_W<'a> {
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
#[doc = "Reader of field `XOSCLFIM`"]
pub type XOSCLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCLFIM`"]
pub struct XOSCLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLFIM_W<'a> {
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
#[doc = "Reader of field `RCOSCDLFIM`"]
pub type RCOSCDLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCDLFIM`"]
pub struct RCOSCDLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCDLFIM_W<'a> {
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
#[doc = "Reader of field `RCOSCLFIM`"]
pub type RCOSCLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCLFIM`"]
pub struct RCOSCLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLFIM_W<'a> {
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
#[doc = "Reader of field `XOSCHFIM`"]
pub type XOSCHFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCHFIM`"]
pub struct XOSCHFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCHFIM_W<'a> {
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
#[doc = "Reader of field `RCOSCHFIM`"]
pub type RCOSCHFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCHFIM`"]
pub struct RCOSCHFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFIM_W<'a> {
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
    #[doc = "Bit 7 - HFSRCPENDIM"]
    #[inline(always)]
    pub fn hfsrcpendim(&self) -> HFSRCPENDIM_R {
        HFSRCPENDIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFSRCDONEIM"]
    #[inline(always)]
    pub fn lfsrcdoneim(&self) -> LFSRCDONEIM_R {
        LFSRCDONEIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XOSCDLFIM"]
    #[inline(always)]
    pub fn xoscdlfim(&self) -> XOSCDLFIM_R {
        XOSCDLFIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XOSCLFIM"]
    #[inline(always)]
    pub fn xosclfim(&self) -> XOSCLFIM_R {
        XOSCLFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RCOSCDLFIM"]
    #[inline(always)]
    pub fn rcoscdlfim(&self) -> RCOSCDLFIM_R {
        RCOSCDLFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RCOSCLFIM"]
    #[inline(always)]
    pub fn rcosclfim(&self) -> RCOSCLFIM_R {
        RCOSCLFIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSCHFIM"]
    #[inline(always)]
    pub fn xoschfim(&self) -> XOSCHFIM_R {
        XOSCHFIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RCOSCHFIM"]
    #[inline(always)]
    pub fn rcoschfim(&self) -> RCOSCHFIM_R {
        RCOSCHFIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - HFSRCPENDIM"]
    #[inline(always)]
    pub fn hfsrcpendim(&mut self) -> HFSRCPENDIM_W {
        HFSRCPENDIM_W { w: self }
    }
    #[doc = "Bit 6 - LFSRCDONEIM"]
    #[inline(always)]
    pub fn lfsrcdoneim(&mut self) -> LFSRCDONEIM_W {
        LFSRCDONEIM_W { w: self }
    }
    #[doc = "Bit 5 - XOSCDLFIM"]
    #[inline(always)]
    pub fn xoscdlfim(&mut self) -> XOSCDLFIM_W {
        XOSCDLFIM_W { w: self }
    }
    #[doc = "Bit 4 - XOSCLFIM"]
    #[inline(always)]
    pub fn xosclfim(&mut self) -> XOSCLFIM_W {
        XOSCLFIM_W { w: self }
    }
    #[doc = "Bit 3 - RCOSCDLFIM"]
    #[inline(always)]
    pub fn rcoscdlfim(&mut self) -> RCOSCDLFIM_W {
        RCOSCDLFIM_W { w: self }
    }
    #[doc = "Bit 2 - RCOSCLFIM"]
    #[inline(always)]
    pub fn rcosclfim(&mut self) -> RCOSCLFIM_W {
        RCOSCLFIM_W { w: self }
    }
    #[doc = "Bit 1 - XOSCHFIM"]
    #[inline(always)]
    pub fn xoschfim(&mut self) -> XOSCHFIM_W {
        XOSCHFIM_W { w: self }
    }
    #[doc = "Bit 0 - RCOSCHFIM"]
    #[inline(always)]
    pub fn rcoschfim(&mut self) -> RCOSCHFIM_W {
        RCOSCHFIM_W { w: self }
    }
}
