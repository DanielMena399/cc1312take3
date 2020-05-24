#[doc = "Reader of register PER_CTL"]
pub type R = crate::R<u32, super::PER_CTL>;
#[doc = "Writer for register PER_CTL"]
pub type W = crate::W<u32, super::PER_CTL>;
#[doc = "Register PER_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PER_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PER_DISABLE`"]
pub type PER_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER_DISABLE`"]
pub struct PER_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_DISABLE_W<'a> {
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
#[doc = "Reader of field `PER_DEBUG_ENABLE`"]
pub type PER_DEBUG_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER_DEBUG_ENABLE`"]
pub struct PER_DEBUG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_DEBUG_ENABLE_W<'a> {
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
    #[doc = "Bit 8 - PER_DISABLE"]
    #[inline(always)]
    pub fn per_disable(&self) -> PER_DISABLE_R {
        PER_DISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PER_DEBUG_ENABLE"]
    #[inline(always)]
    pub fn per_debug_enable(&self) -> PER_DEBUG_ENABLE_R {
        PER_DEBUG_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PER_DISABLE"]
    #[inline(always)]
    pub fn per_disable(&mut self) -> PER_DISABLE_W {
        PER_DISABLE_W { w: self }
    }
    #[doc = "Bit 0 - PER_DEBUG_ENABLE"]
    #[inline(always)]
    pub fn per_debug_enable(&mut self) -> PER_DEBUG_ENABLE_W {
        PER_DEBUG_ENABLE_W { w: self }
    }
}
