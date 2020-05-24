#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0260"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0260
    }
}
#[doc = "Reader of field `NUM_CODE2`"]
pub type NUM_CODE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_CODE2`"]
pub struct NUM_CODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_CODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `NUM_LIT`"]
pub type NUM_LIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_LIT`"]
pub struct NUM_LIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_LIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NUM_CODE1`"]
pub type NUM_CODE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUM_CODE1`"]
pub struct NUM_CODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_CODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 12:13 - NUM_CODE2"]
    #[inline(always)]
    pub fn num_code2(&self) -> NUM_CODE2_R {
        NUM_CODE2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - NUM_LIT"]
    #[inline(always)]
    pub fn num_lit(&self) -> NUM_LIT_R {
        NUM_LIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - NUM_CODE1"]
    #[inline(always)]
    pub fn num_code1(&self) -> NUM_CODE1_R {
        NUM_CODE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - KEY"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:13 - NUM_CODE2"]
    #[inline(always)]
    pub fn num_code2(&mut self) -> NUM_CODE2_W {
        NUM_CODE2_W { w: self }
    }
    #[doc = "Bits 8:11 - NUM_LIT"]
    #[inline(always)]
    pub fn num_lit(&mut self) -> NUM_LIT_W {
        NUM_LIT_W { w: self }
    }
    #[doc = "Bits 4:7 - NUM_CODE1"]
    #[inline(always)]
    pub fn num_code1(&mut self) -> NUM_CODE1_W {
        NUM_CODE1_W { w: self }
    }
    #[doc = "Bit 1 - KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
