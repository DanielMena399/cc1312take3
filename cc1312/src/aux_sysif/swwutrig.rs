#[doc = "Writer for register SWWUTRIG"]
pub type W = crate::W<u32, super::SWWUTRIG>;
#[doc = "Register SWWUTRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::SWWUTRIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 3 - SW_WU3"]
    #[inline(always)]
    pub fn sw_wu3(&mut self) -> SW_WU3_W {
        SW_WU3_W { w: self }
    }
    #[doc = "Bit 2 - SW_WU2"]
    #[inline(always)]
    pub fn sw_wu2(&mut self) -> SW_WU2_W {
        SW_WU2_W { w: self }
    }
    #[doc = "Bit 1 - SW_WU1"]
    #[inline(always)]
    pub fn sw_wu1(&mut self) -> SW_WU1_W {
        SW_WU1_W { w: self }
    }
    #[doc = "Bit 0 - SW_WU0"]
    #[inline(always)]
    pub fn sw_wu0(&mut self) -> SW_WU0_W {
        SW_WU0_W { w: self }
    }
}
