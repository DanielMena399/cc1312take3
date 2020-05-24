#[doc = "Reader of register FUNCTION1"]
pub type R = crate::R<u32, super::FUNCTION1>;
#[doc = "Writer for register FUNCTION1"]
pub type W = crate::W<u32, super::FUNCTION1>;
#[doc = "Register FUNCTION1 `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::FUNCTION1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
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
#[doc = "Reader of field `DATAVADDR1`"]
pub type DATAVADDR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAVADDR1`"]
pub struct DATAVADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATAVADDR0`"]
pub type DATAVADDR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAVADDR0`"]
pub struct DATAVADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DATAVSIZE`"]
pub type DATAVSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAVSIZE`"]
pub struct DATAVSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `LNK1ENA`"]
pub type LNK1ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNK1ENA`"]
pub struct LNK1ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LNK1ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DATAVMATCH`"]
pub type DATAVMATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAVMATCH`"]
pub struct DATAVMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAVMATCH_W<'a> {
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
    #[doc = "Bits 16:19 - DATAVADDR1"]
    #[inline(always)]
    pub fn datavaddr1(&self) -> DATAVADDR1_R {
        DATAVADDR1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DATAVADDR0"]
    #[inline(always)]
    pub fn datavaddr0(&self) -> DATAVADDR0_R {
        DATAVADDR0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - DATAVSIZE"]
    #[inline(always)]
    pub fn datavsize(&self) -> DATAVSIZE_R {
        DATAVSIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - LNK1ENA"]
    #[inline(always)]
    pub fn lnk1ena(&self) -> LNK1ENA_R {
        LNK1ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DATAVMATCH"]
    #[inline(always)]
    pub fn datavmatch(&self) -> DATAVMATCH_R {
        DATAVMATCH_R::new(((self.bits >> 8) & 0x01) != 0)
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
    #[doc = "Bits 16:19 - DATAVADDR1"]
    #[inline(always)]
    pub fn datavaddr1(&mut self) -> DATAVADDR1_W {
        DATAVADDR1_W { w: self }
    }
    #[doc = "Bits 12:15 - DATAVADDR0"]
    #[inline(always)]
    pub fn datavaddr0(&mut self) -> DATAVADDR0_W {
        DATAVADDR0_W { w: self }
    }
    #[doc = "Bits 10:11 - DATAVSIZE"]
    #[inline(always)]
    pub fn datavsize(&mut self) -> DATAVSIZE_W {
        DATAVSIZE_W { w: self }
    }
    #[doc = "Bit 9 - LNK1ENA"]
    #[inline(always)]
    pub fn lnk1ena(&mut self) -> LNK1ENA_W {
        LNK1ENA_W { w: self }
    }
    #[doc = "Bit 8 - DATAVMATCH"]
    #[inline(always)]
    pub fn datavmatch(&mut self) -> DATAVMATCH_W {
        DATAVMATCH_W { w: self }
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
