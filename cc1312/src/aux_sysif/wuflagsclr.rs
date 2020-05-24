#[doc = "Reader of register WUFLAGSCLR"]
pub type R = crate::R<u32, super::WUFLAGSCLR>;
#[doc = "Writer for register WUFLAGSCLR"]
pub type W = crate::W<u32, super::WUFLAGSCLR>;
#[doc = "Register WUFLAGSCLR `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::WUFLAGSCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `SW_WU3`"]
pub type SW_WU3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_WU3`"]
pub struct SW_WU3_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WU3_W<'a> {
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
#[doc = "Reader of field `SW_WU2`"]
pub type SW_WU2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_WU2`"]
pub struct SW_WU2_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WU2_W<'a> {
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
#[doc = "Reader of field `SW_WU1`"]
pub type SW_WU1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_WU1`"]
pub struct SW_WU1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WU1_W<'a> {
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
#[doc = "Reader of field `SW_WU0`"]
pub type SW_WU0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_WU0`"]
pub struct SW_WU0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_WU0_W<'a> {
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
#[doc = "Reader of field `PROG_WU3`"]
pub type PROG_WU3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_WU3`"]
pub struct PROG_WU3_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_WU3_W<'a> {
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
#[doc = "Reader of field `PROG_WU2`"]
pub type PROG_WU2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_WU2`"]
pub struct PROG_WU2_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_WU2_W<'a> {
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
#[doc = "Reader of field `PROG_WU1`"]
pub type PROG_WU1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_WU1`"]
pub struct PROG_WU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_WU1_W<'a> {
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
#[doc = "Reader of field `PROG_WU0`"]
pub type PROG_WU0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG_WU0`"]
pub struct PROG_WU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_WU0_W<'a> {
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
    #[doc = "Bit 7 - SW_WU3"]
    #[inline(always)]
    pub fn sw_wu3(&self) -> SW_WU3_R {
        SW_WU3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SW_WU2"]
    #[inline(always)]
    pub fn sw_wu2(&self) -> SW_WU2_R {
        SW_WU2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SW_WU1"]
    #[inline(always)]
    pub fn sw_wu1(&self) -> SW_WU1_R {
        SW_WU1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SW_WU0"]
    #[inline(always)]
    pub fn sw_wu0(&self) -> SW_WU0_R {
        SW_WU0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PROG_WU3"]
    #[inline(always)]
    pub fn prog_wu3(&self) -> PROG_WU3_R {
        PROG_WU3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PROG_WU2"]
    #[inline(always)]
    pub fn prog_wu2(&self) -> PROG_WU2_R {
        PROG_WU2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PROG_WU1"]
    #[inline(always)]
    pub fn prog_wu1(&self) -> PROG_WU1_R {
        PROG_WU1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PROG_WU0"]
    #[inline(always)]
    pub fn prog_wu0(&self) -> PROG_WU0_R {
        PROG_WU0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - SW_WU3"]
    #[inline(always)]
    pub fn sw_wu3(&mut self) -> SW_WU3_W {
        SW_WU3_W { w: self }
    }
    #[doc = "Bit 6 - SW_WU2"]
    #[inline(always)]
    pub fn sw_wu2(&mut self) -> SW_WU2_W {
        SW_WU2_W { w: self }
    }
    #[doc = "Bit 5 - SW_WU1"]
    #[inline(always)]
    pub fn sw_wu1(&mut self) -> SW_WU1_W {
        SW_WU1_W { w: self }
    }
    #[doc = "Bit 4 - SW_WU0"]
    #[inline(always)]
    pub fn sw_wu0(&mut self) -> SW_WU0_W {
        SW_WU0_W { w: self }
    }
    #[doc = "Bit 3 - PROG_WU3"]
    #[inline(always)]
    pub fn prog_wu3(&mut self) -> PROG_WU3_W {
        PROG_WU3_W { w: self }
    }
    #[doc = "Bit 2 - PROG_WU2"]
    #[inline(always)]
    pub fn prog_wu2(&mut self) -> PROG_WU2_W {
        PROG_WU2_W { w: self }
    }
    #[doc = "Bit 1 - PROG_WU1"]
    #[inline(always)]
    pub fn prog_wu1(&mut self) -> PROG_WU1_W {
        PROG_WU1_W { w: self }
    }
    #[doc = "Bit 0 - PROG_WU0"]
    #[inline(always)]
    pub fn prog_wu0(&mut self) -> PROG_WU0_W {
        PROG_WU0_W { w: self }
    }
}
