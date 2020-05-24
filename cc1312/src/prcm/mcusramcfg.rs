#[doc = "Reader of register MCUSRAMCFG"]
pub type R = crate::R<u32, super::MCUSRAMCFG>;
#[doc = "Writer for register MCUSRAMCFG"]
pub type W = crate::W<u32, super::MCUSRAMCFG>;
#[doc = "Register MCUSRAMCFG `reset()`'s with value 0x20"]
impl crate::ResetValue for super::MCUSRAMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `BM_OFF`"]
pub type BM_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BM_OFF`"]
pub struct BM_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BM_OFF_W<'a> {
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
#[doc = "Reader of field `PAGE`"]
pub type PAGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAGE`"]
pub struct PAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE_W<'a> {
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
#[doc = "Reader of field `PGS`"]
pub type PGS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGS`"]
pub struct PGS_W<'a> {
    w: &'a mut W,
}
impl<'a> PGS_W<'a> {
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
#[doc = "Reader of field `BM`"]
pub type BM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BM`"]
pub struct BM_W<'a> {
    w: &'a mut W,
}
impl<'a> BM_W<'a> {
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
#[doc = "Reader of field `PCH_F`"]
pub type PCH_F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCH_F`"]
pub struct PCH_F_W<'a> {
    w: &'a mut W,
}
impl<'a> PCH_F_W<'a> {
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
#[doc = "Reader of field `PCH_L`"]
pub type PCH_L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCH_L`"]
pub struct PCH_L_W<'a> {
    w: &'a mut W,
}
impl<'a> PCH_L_W<'a> {
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
    #[doc = "Bit 5 - BM_OFF"]
    #[inline(always)]
    pub fn bm_off(&self) -> BM_OFF_R {
        BM_OFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PAGE"]
    #[inline(always)]
    pub fn page(&self) -> PAGE_R {
        PAGE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PGS"]
    #[inline(always)]
    pub fn pgs(&self) -> PGS_R {
        PGS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BM"]
    #[inline(always)]
    pub fn bm(&self) -> BM_R {
        BM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PCH_F"]
    #[inline(always)]
    pub fn pch_f(&self) -> PCH_F_R {
        PCH_F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PCH_L"]
    #[inline(always)]
    pub fn pch_l(&self) -> PCH_L_R {
        PCH_L_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - BM_OFF"]
    #[inline(always)]
    pub fn bm_off(&mut self) -> BM_OFF_W {
        BM_OFF_W { w: self }
    }
    #[doc = "Bit 4 - PAGE"]
    #[inline(always)]
    pub fn page(&mut self) -> PAGE_W {
        PAGE_W { w: self }
    }
    #[doc = "Bit 3 - PGS"]
    #[inline(always)]
    pub fn pgs(&mut self) -> PGS_W {
        PGS_W { w: self }
    }
    #[doc = "Bit 2 - BM"]
    #[inline(always)]
    pub fn bm(&mut self) -> BM_W {
        BM_W { w: self }
    }
    #[doc = "Bit 1 - PCH_F"]
    #[inline(always)]
    pub fn pch_f(&mut self) -> PCH_F_W {
        PCH_F_W { w: self }
    }
    #[doc = "Bit 0 - PCH_L"]
    #[inline(always)]
    pub fn pch_l(&mut self) -> PCH_L_W {
        PCH_L_W { w: self }
    }
}
