#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ATBID`"]
pub type ATBID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATBID`"]
pub struct ATBID_W<'a> {
    w: &'a mut W,
}
impl<'a> ATBID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TSPRESCALE`"]
pub type TSPRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSPRESCALE`"]
pub struct TSPRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SWOENA`"]
pub type SWOENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWOENA`"]
pub struct SWOENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SWOENA_W<'a> {
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
#[doc = "Reader of field `DWTENA`"]
pub type DWTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DWTENA`"]
pub struct DWTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DWTENA_W<'a> {
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
#[doc = "Reader of field `SYNCENA`"]
pub type SYNCENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCENA`"]
pub struct SYNCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCENA_W<'a> {
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
#[doc = "Reader of field `TSENA`"]
pub type TSENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENA`"]
pub struct TSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENA_W<'a> {
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
#[doc = "Reader of field `ITMENA`"]
pub type ITMENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITMENA`"]
pub struct ITMENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ITMENA_W<'a> {
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
    #[doc = "Bit 23 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - ATBID"]
    #[inline(always)]
    pub fn atbid(&self) -> ATBID_R {
        ATBID_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - TSPRESCALE"]
    #[inline(always)]
    pub fn tsprescale(&self) -> TSPRESCALE_R {
        TSPRESCALE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4 - SWOENA"]
    #[inline(always)]
    pub fn swoena(&self) -> SWOENA_R {
        SWOENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DWTENA"]
    #[inline(always)]
    pub fn dwtena(&self) -> DWTENA_R {
        DWTENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYNCENA"]
    #[inline(always)]
    pub fn syncena(&self) -> SYNCENA_R {
        SYNCENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TSENA"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ITMENA"]
    #[inline(always)]
    pub fn itmena(&self) -> ITMENA_R {
        ITMENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bits 16:22 - ATBID"]
    #[inline(always)]
    pub fn atbid(&mut self) -> ATBID_W {
        ATBID_W { w: self }
    }
    #[doc = "Bits 8:9 - TSPRESCALE"]
    #[inline(always)]
    pub fn tsprescale(&mut self) -> TSPRESCALE_W {
        TSPRESCALE_W { w: self }
    }
    #[doc = "Bit 4 - SWOENA"]
    #[inline(always)]
    pub fn swoena(&mut self) -> SWOENA_W {
        SWOENA_W { w: self }
    }
    #[doc = "Bit 3 - DWTENA"]
    #[inline(always)]
    pub fn dwtena(&mut self) -> DWTENA_W {
        DWTENA_W { w: self }
    }
    #[doc = "Bit 2 - SYNCENA"]
    #[inline(always)]
    pub fn syncena(&mut self) -> SYNCENA_W {
        SYNCENA_W { w: self }
    }
    #[doc = "Bit 1 - TSENA"]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W {
        TSENA_W { w: self }
    }
    #[doc = "Bit 0 - ITMENA"]
    #[inline(always)]
    pub fn itmena(&mut self) -> ITMENA_W {
        ITMENA_W { w: self }
    }
}
