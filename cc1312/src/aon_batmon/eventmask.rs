#[doc = "Reader of register EVENTMASK"]
pub type R = crate::R<u32, super::EVENTMASK>;
#[doc = "Writer for register EVENTMASK"]
pub type W = crate::W<u32, super::EVENTMASK>;
#[doc = "Register EVENTMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEMP_UPDATE_MASK`"]
pub type TEMP_UPDATE_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMP_UPDATE_MASK`"]
pub struct TEMP_UPDATE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_UPDATE_MASK_W<'a> {
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
#[doc = "Reader of field `BATT_UPDATE_MASK`"]
pub type BATT_UPDATE_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BATT_UPDATE_MASK`"]
pub struct BATT_UPDATE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> BATT_UPDATE_MASK_W<'a> {
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
#[doc = "Reader of field `TEMP_BELOW_LL_MASK`"]
pub type TEMP_BELOW_LL_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMP_BELOW_LL_MASK`"]
pub struct TEMP_BELOW_LL_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_BELOW_LL_MASK_W<'a> {
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
#[doc = "Reader of field `TEMP_OVER_UL_MASK`"]
pub type TEMP_OVER_UL_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEMP_OVER_UL_MASK`"]
pub struct TEMP_OVER_UL_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_OVER_UL_MASK_W<'a> {
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
#[doc = "Reader of field `BATT_BELOW_LL_MASK`"]
pub type BATT_BELOW_LL_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BATT_BELOW_LL_MASK`"]
pub struct BATT_BELOW_LL_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> BATT_BELOW_LL_MASK_W<'a> {
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
#[doc = "Reader of field `BATT_OVER_UL_MASK`"]
pub type BATT_OVER_UL_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BATT_OVER_UL_MASK`"]
pub struct BATT_OVER_UL_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> BATT_OVER_UL_MASK_W<'a> {
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
    #[doc = "Bit 5 - TEMP_UPDATE_MASK"]
    #[inline(always)]
    pub fn temp_update_mask(&self) -> TEMP_UPDATE_MASK_R {
        TEMP_UPDATE_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BATT_UPDATE_MASK"]
    #[inline(always)]
    pub fn batt_update_mask(&self) -> BATT_UPDATE_MASK_R {
        BATT_UPDATE_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TEMP_BELOW_LL_MASK"]
    #[inline(always)]
    pub fn temp_below_ll_mask(&self) -> TEMP_BELOW_LL_MASK_R {
        TEMP_BELOW_LL_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TEMP_OVER_UL_MASK"]
    #[inline(always)]
    pub fn temp_over_ul_mask(&self) -> TEMP_OVER_UL_MASK_R {
        TEMP_OVER_UL_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - BATT_BELOW_LL_MASK"]
    #[inline(always)]
    pub fn batt_below_ll_mask(&self) -> BATT_BELOW_LL_MASK_R {
        BATT_BELOW_LL_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BATT_OVER_UL_MASK"]
    #[inline(always)]
    pub fn batt_over_ul_mask(&self) -> BATT_OVER_UL_MASK_R {
        BATT_OVER_UL_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - TEMP_UPDATE_MASK"]
    #[inline(always)]
    pub fn temp_update_mask(&mut self) -> TEMP_UPDATE_MASK_W {
        TEMP_UPDATE_MASK_W { w: self }
    }
    #[doc = "Bit 4 - BATT_UPDATE_MASK"]
    #[inline(always)]
    pub fn batt_update_mask(&mut self) -> BATT_UPDATE_MASK_W {
        BATT_UPDATE_MASK_W { w: self }
    }
    #[doc = "Bit 3 - TEMP_BELOW_LL_MASK"]
    #[inline(always)]
    pub fn temp_below_ll_mask(&mut self) -> TEMP_BELOW_LL_MASK_W {
        TEMP_BELOW_LL_MASK_W { w: self }
    }
    #[doc = "Bit 2 - TEMP_OVER_UL_MASK"]
    #[inline(always)]
    pub fn temp_over_ul_mask(&mut self) -> TEMP_OVER_UL_MASK_W {
        TEMP_OVER_UL_MASK_W { w: self }
    }
    #[doc = "Bit 1 - BATT_BELOW_LL_MASK"]
    #[inline(always)]
    pub fn batt_below_ll_mask(&mut self) -> BATT_BELOW_LL_MASK_W {
        BATT_BELOW_LL_MASK_W { w: self }
    }
    #[doc = "Bit 0 - BATT_OVER_UL_MASK"]
    #[inline(always)]
    pub fn batt_over_ul_mask(&mut self) -> BATT_OVER_UL_MASK_W {
        BATT_OVER_UL_MASK_W { w: self }
    }
}
