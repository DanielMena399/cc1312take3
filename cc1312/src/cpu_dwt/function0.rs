#[doc = "Reader of register FUNCTION0"]
pub type R = crate::R<u32, super::FUNCTION0>;
#[doc = "Writer for register FUNCTION0"]
pub type W = crate::W<u32, super::FUNCTION0>;
#[doc = "Register FUNCTION0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FUNCTION0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCHED`"]
pub type MATCHED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MATCHED`"]
pub struct MATCHED_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHED_W<'a> {
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
#[doc = "Reader of field `CYCMATCH`"]
pub type CYCMATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CYCMATCH`"]
pub struct CYCMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCMATCH_W<'a> {
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
#[doc = "Reader of field `EMITRANGE`"]
pub type EMITRANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMITRANGE`"]
pub struct EMITRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EMITRANGE_W<'a> {
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
#[doc = "Reader of field `FUNCTION`"]
pub type FUNCTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FUNCTION`"]
pub struct FUNCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - MATCHED"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CYCMATCH"]
    #[inline(always)]
    pub fn cycmatch(&self) -> CYCMATCH_R {
        CYCMATCH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EMITRANGE"]
    #[inline(always)]
    pub fn emitrange(&self) -> EMITRANGE_R {
        EMITRANGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - FUNCTION"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - MATCHED"]
    #[inline(always)]
    pub fn matched(&mut self) -> MATCHED_W {
        MATCHED_W { w: self }
    }
    #[doc = "Bit 7 - CYCMATCH"]
    #[inline(always)]
    pub fn cycmatch(&mut self) -> CYCMATCH_W {
        CYCMATCH_W { w: self }
    }
    #[doc = "Bit 5 - EMITRANGE"]
    #[inline(always)]
    pub fn emitrange(&mut self) -> EMITRANGE_W {
        EMITRANGE_W { w: self }
    }
    #[doc = "Bits 0:3 - FUNCTION"]
    #[inline(always)]
    pub fn function(&mut self) -> FUNCTION_W {
        FUNCTION_W { w: self }
    }
}
