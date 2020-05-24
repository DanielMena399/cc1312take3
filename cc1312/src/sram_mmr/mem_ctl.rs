#[doc = "Reader of register MEM_CTL"]
pub type R = crate::R<u32, super::MEM_CTL>;
#[doc = "Writer for register MEM_CTL"]
pub type W = crate::W<u32, super::MEM_CTL>;
#[doc = "Register MEM_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_BUSY`"]
pub type MEM_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_BUSY`"]
pub struct MEM_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BUSY_W<'a> {
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
#[doc = "Reader of field `MEM_CLR_EN`"]
pub type MEM_CLR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_CLR_EN`"]
pub struct MEM_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CLR_EN_W<'a> {
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
    #[doc = "Bit 1 - MEM_BUSY"]
    #[inline(always)]
    pub fn mem_busy(&self) -> MEM_BUSY_R {
        MEM_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MEM_CLR_EN"]
    #[inline(always)]
    pub fn mem_clr_en(&self) -> MEM_CLR_EN_R {
        MEM_CLR_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MEM_BUSY"]
    #[inline(always)]
    pub fn mem_busy(&mut self) -> MEM_BUSY_W {
        MEM_BUSY_W { w: self }
    }
    #[doc = "Bit 0 - MEM_CLR_EN"]
    #[inline(always)]
    pub fn mem_clr_en(&mut self) -> MEM_CLR_EN_W {
        MEM_CLR_EN_W { w: self }
    }
}
