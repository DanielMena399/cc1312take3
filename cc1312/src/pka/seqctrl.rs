#[doc = "Reader of register SEQCTRL"]
pub type R = crate::R<u32, super::SEQCTRL>;
#[doc = "Writer for register SEQCTRL"]
pub type W = crate::W<u32, super::SEQCTRL>;
#[doc = "Register SEQCTRL `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::SEQCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SEQUENCER_STAT`"]
pub type SEQUENCER_STAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQUENCER_STAT`"]
pub struct SEQUENCER_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQUENCER_STAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SW_CONTROL_STAT`"]
pub type SW_CONTROL_STAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_CONTROL_STAT`"]
pub struct SW_CONTROL_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CONTROL_STAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - SEQUENCER_STAT"]
    #[inline(always)]
    pub fn sequencer_stat(&self) -> SEQUENCER_STAT_R {
        SEQUENCER_STAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - SW_CONTROL_STAT"]
    #[inline(always)]
    pub fn sw_control_stat(&self) -> SW_CONTROL_STAT_R {
        SW_CONTROL_STAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bits 8:15 - SEQUENCER_STAT"]
    #[inline(always)]
    pub fn sequencer_stat(&mut self) -> SEQUENCER_STAT_W {
        SEQUENCER_STAT_W { w: self }
    }
    #[doc = "Bits 0:7 - SW_CONTROL_STAT"]
    #[inline(always)]
    pub fn sw_control_stat(&mut self) -> SW_CONTROL_STAT_W {
        SW_CONTROL_STAT_W { w: self }
    }
}
