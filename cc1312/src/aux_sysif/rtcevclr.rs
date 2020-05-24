#[doc = "Reader of register RTCEVCLR"]
pub type R = crate::R<u32, super::RTCEVCLR>;
#[doc = "Writer for register RTCEVCLR"]
pub type W = crate::W<u32, super::RTCEVCLR>;
#[doc = "Register RTCEVCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCEVCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CH2_EV_CLR`"]
pub type RTC_CH2_EV_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CH2_EV_CLR`"]
pub struct RTC_CH2_EV_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CH2_EV_CLR_W<'a> {
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
    #[doc = "Bit 0 - RTC_CH2_EV_CLR"]
    #[inline(always)]
    pub fn rtc_ch2_ev_clr(&self) -> RTC_CH2_EV_CLR_R {
        RTC_CH2_EV_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_CH2_EV_CLR"]
    #[inline(always)]
    pub fn rtc_ch2_ev_clr(&mut self) -> RTC_CH2_EV_CLR_W {
        RTC_CH2_EV_CLR_W { w: self }
    }
}
