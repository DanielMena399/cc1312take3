#[doc = "Writer for register OSCICR"]
pub type W = crate::W<u32, super::OSCICR>;
#[doc = "Register OSCICR `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `HFSRCPENDC`"]
pub struct HFSRCPENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> HFSRCPENDC_W<'a> {
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
#[doc = "Write proxy for field `LFSRCDONEC`"]
pub struct LFSRCDONEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSRCDONEC_W<'a> {
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
#[doc = "Write proxy for field `XOSCDLFC`"]
pub struct XOSCDLFC_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCDLFC_W<'a> {
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
#[doc = "Write proxy for field `XOSCLFC`"]
pub struct XOSCLFC_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLFC_W<'a> {
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
#[doc = "Write proxy for field `RCOSCDLFC`"]
pub struct RCOSCDLFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCDLFC_W<'a> {
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
#[doc = "Write proxy for field `RCOSCLFC`"]
pub struct RCOSCLFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLFC_W<'a> {
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
#[doc = "Write proxy for field `XOSCHFC`"]
pub struct XOSCHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCHFC_W<'a> {
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
#[doc = "Write proxy for field `RCOSCHFC`"]
pub struct RCOSCHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFC_W<'a> {
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
impl W {
    #[doc = "Bit 7 - HFSRCPENDC"]
    #[inline(always)]
    pub fn hfsrcpendc(&mut self) -> HFSRCPENDC_W {
        HFSRCPENDC_W { w: self }
    }
    #[doc = "Bit 6 - LFSRCDONEC"]
    #[inline(always)]
    pub fn lfsrcdonec(&mut self) -> LFSRCDONEC_W {
        LFSRCDONEC_W { w: self }
    }
    #[doc = "Bit 5 - XOSCDLFC"]
    #[inline(always)]
    pub fn xoscdlfc(&mut self) -> XOSCDLFC_W {
        XOSCDLFC_W { w: self }
    }
    #[doc = "Bit 4 - XOSCLFC"]
    #[inline(always)]
    pub fn xosclfc(&mut self) -> XOSCLFC_W {
        XOSCLFC_W { w: self }
    }
    #[doc = "Bit 3 - RCOSCDLFC"]
    #[inline(always)]
    pub fn rcoscdlfc(&mut self) -> RCOSCDLFC_W {
        RCOSCDLFC_W { w: self }
    }
    #[doc = "Bit 2 - RCOSCLFC"]
    #[inline(always)]
    pub fn rcosclfc(&mut self) -> RCOSCLFC_W {
        RCOSCLFC_W { w: self }
    }
    #[doc = "Bit 1 - XOSCHFC"]
    #[inline(always)]
    pub fn xoschfc(&mut self) -> XOSCHFC_W {
        XOSCHFC_W { w: self }
    }
    #[doc = "Bit 0 - RCOSCHFC"]
    #[inline(always)]
    pub fn rcoschfc(&mut self) -> RCOSCHFC_W {
        RCOSCHFC_W { w: self }
    }
}
