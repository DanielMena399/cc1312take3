#[doc = "Reader of register TIMER2DBGCTL"]
pub type R = crate::R<u32, super::TIMER2DBGCTL>;
#[doc = "Writer for register TIMER2DBGCTL"]
pub type W = crate::W<u32, super::TIMER2DBGCTL>;
#[doc = "Register TIMER2DBGCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER2DBGCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBG_FREEZE_EN`"]
pub type DBG_FREEZE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_FREEZE_EN`"]
pub struct DBG_FREEZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_FREEZE_EN_W<'a> {
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
    #[doc = "Bit 0 - DBG_FREEZE_EN"]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DBG_FREEZE_EN_R {
        DBG_FREEZE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBG_FREEZE_EN"]
    #[inline(always)]
    pub fn dbg_freeze_en(&mut self) -> DBG_FREEZE_EN_W {
        DBG_FREEZE_EN_W { w: self }
    }
}
