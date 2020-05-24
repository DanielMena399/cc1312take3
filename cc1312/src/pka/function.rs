#[doc = "Reader of register FUNCTION"]
pub type R = crate::R<u32, super::FUNCTION>;
#[doc = "Writer for register FUNCTION"]
pub type W = crate::W<u32, super::FUNCTION>;
#[doc = "Register FUNCTION `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::FUNCTION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `STALL_RESULT`"]
pub type STALL_RESULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL_RESULT`"]
pub struct STALL_RESULT_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_RESULT_W<'a> {
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
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUN`"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SEQUENCER_OPERATIONS`"]
pub type SEQUENCER_OPERATIONS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQUENCER_OPERATIONS`"]
pub struct SEQUENCER_OPERATIONS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQUENCER_OPERATIONS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `COPY`"]
pub type COPY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COPY`"]
pub struct COPY_W<'a> {
    w: &'a mut W,
}
impl<'a> COPY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `COMPARE`"]
pub type COMPARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPARE`"]
pub struct COMPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MODULO`"]
pub type MODULO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODULO`"]
pub struct MODULO_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULO_W<'a> {
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
#[doc = "Reader of field `DIVIDE`"]
pub type DIVIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVIDE`"]
pub struct DIVIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDE_W<'a> {
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
#[doc = "Reader of field `LSHIFT`"]
pub type LSHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSHIFT`"]
pub struct LSHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSHIFT_W<'a> {
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
#[doc = "Reader of field `RSHIFT`"]
pub type RSHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSHIFT`"]
pub struct RSHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT_W<'a> {
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
#[doc = "Reader of field `SUBTRACT`"]
pub type SUBTRACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUBTRACT`"]
pub struct SUBTRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBTRACT_W<'a> {
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
#[doc = "Reader of field `ADD`"]
pub type ADD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD`"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
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
#[doc = "Reader of field `MS_ONE`"]
pub type MS_ONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MS_ONE`"]
pub struct MS_ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_ONE_W<'a> {
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
#[doc = "Reader of field `ADDSUB`"]
pub type ADDSUB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDSUB`"]
pub struct ADDSUB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSUB_W<'a> {
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
#[doc = "Reader of field `MULTIPLY`"]
pub type MULTIPLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MULTIPLY`"]
pub struct MULTIPLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTIPLY_W<'a> {
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
    #[doc = "Bit 24 - STALL_RESULT"]
    #[inline(always)]
    pub fn stall_result(&self) -> STALL_RESULT_R {
        STALL_RESULT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RUN"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - SEQUENCER_OPERATIONS"]
    #[inline(always)]
    pub fn sequencer_operations(&self) -> SEQUENCER_OPERATIONS_R {
        SEQUENCER_OPERATIONS_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - COPY"]
    #[inline(always)]
    pub fn copy(&self) -> COPY_R {
        COPY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - COMPARE"]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MODULO"]
    #[inline(always)]
    pub fn modulo(&self) -> MODULO_R {
        MODULO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIVIDE"]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LSHIFT"]
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RSHIFT"]
    #[inline(always)]
    pub fn rshift(&self) -> RSHIFT_R {
        RSHIFT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SUBTRACT"]
    #[inline(always)]
    pub fn subtract(&self) -> SUBTRACT_R {
        SUBTRACT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADD"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MS_ONE"]
    #[inline(always)]
    pub fn ms_one(&self) -> MS_ONE_R {
        MS_ONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADDSUB"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MULTIPLY"]
    #[inline(always)]
    pub fn multiply(&self) -> MULTIPLY_R {
        MULTIPLY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - STALL_RESULT"]
    #[inline(always)]
    pub fn stall_result(&mut self) -> STALL_RESULT_W {
        STALL_RESULT_W { w: self }
    }
    #[doc = "Bit 15 - RUN"]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
    #[doc = "Bits 12:14 - SEQUENCER_OPERATIONS"]
    #[inline(always)]
    pub fn sequencer_operations(&mut self) -> SEQUENCER_OPERATIONS_W {
        SEQUENCER_OPERATIONS_W { w: self }
    }
    #[doc = "Bit 11 - COPY"]
    #[inline(always)]
    pub fn copy(&mut self) -> COPY_W {
        COPY_W { w: self }
    }
    #[doc = "Bit 10 - COMPARE"]
    #[inline(always)]
    pub fn compare(&mut self) -> COMPARE_W {
        COMPARE_W { w: self }
    }
    #[doc = "Bit 9 - MODULO"]
    #[inline(always)]
    pub fn modulo(&mut self) -> MODULO_W {
        MODULO_W { w: self }
    }
    #[doc = "Bit 8 - DIVIDE"]
    #[inline(always)]
    pub fn divide(&mut self) -> DIVIDE_W {
        DIVIDE_W { w: self }
    }
    #[doc = "Bit 7 - LSHIFT"]
    #[inline(always)]
    pub fn lshift(&mut self) -> LSHIFT_W {
        LSHIFT_W { w: self }
    }
    #[doc = "Bit 6 - RSHIFT"]
    #[inline(always)]
    pub fn rshift(&mut self) -> RSHIFT_W {
        RSHIFT_W { w: self }
    }
    #[doc = "Bit 5 - SUBTRACT"]
    #[inline(always)]
    pub fn subtract(&mut self) -> SUBTRACT_W {
        SUBTRACT_W { w: self }
    }
    #[doc = "Bit 4 - ADD"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    #[doc = "Bit 3 - MS_ONE"]
    #[inline(always)]
    pub fn ms_one(&mut self) -> MS_ONE_W {
        MS_ONE_W { w: self }
    }
    #[doc = "Bit 1 - ADDSUB"]
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W {
        ADDSUB_W { w: self }
    }
    #[doc = "Bit 0 - MULTIPLY"]
    #[inline(always)]
    pub fn multiply(&mut self) -> MULTIPLY_W {
        MULTIPLY_W { w: self }
    }
}
