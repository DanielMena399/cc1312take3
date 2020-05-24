#[doc = "Reader of register CH1CCFG"]
pub type R = crate::R<u32, super::CH1CCFG>;
#[doc = "Writer for register CH1CCFG"]
pub type W = crate::W<u32, super::CH1CCFG>;
#[doc = "Register CH1CCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1CCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPT_SRC`"]
pub type CAPT_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPT_SRC`"]
pub struct CAPT_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | (((value as u32) & 0x3f) << 1);
        self.w
    }
}
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE`"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
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
    #[doc = "Bits 1:6 - CAPT_SRC"]
    #[inline(always)]
    pub fn capt_src(&self) -> CAPT_SRC_R {
        CAPT_SRC_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 0 - EDGE"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:6 - CAPT_SRC"]
    #[inline(always)]
    pub fn capt_src(&mut self) -> CAPT_SRC_W {
        CAPT_SRC_W { w: self }
    }
    #[doc = "Bit 0 - EDGE"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W { w: self }
    }
}
