#[doc = "Reader of register EVSTAT2"]
pub type R = crate::R<u32, super::EVSTAT2>;
#[doc = "Reader of field `AUX_COMPB`"]
pub type AUX_COMPB_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_COMPA`"]
pub type AUX_COMPA_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCU_OBSMUX1`"]
pub type MCU_OBSMUX1_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCU_OBSMUX0`"]
pub type MCU_OBSMUX0_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCU_EV`"]
pub type MCU_EV_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACLK_REF`"]
pub type ACLK_REF_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDR_RECHARGE`"]
pub type VDDR_RECHARGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCU_ACTIVE`"]
pub type MCU_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWR_DWN`"]
pub type PWR_DWN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_LF`"]
pub type SCLK_LF_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_BATMON_TEMP_UPD`"]
pub type AON_BATMON_TEMP_UPD_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_BATMON_BAT_UPD`"]
pub type AON_BATMON_BAT_UPD_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_RTC_4KHZ`"]
pub type AON_RTC_4KHZ_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_RTC_CH2_DLY`"]
pub type AON_RTC_CH2_DLY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_RTC_CH2`"]
pub type AON_RTC_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `MANUAL_EV`"]
pub type MANUAL_EV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MCU_OBSMUX1"]
    #[inline(always)]
    pub fn mcu_obsmux1(&self) -> MCU_OBSMUX1_R {
        MCU_OBSMUX1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MCU_OBSMUX0"]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> MCU_OBSMUX0_R {
        MCU_OBSMUX0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MCU_EV"]
    #[inline(always)]
    pub fn mcu_ev(&self) -> MCU_EV_R {
        MCU_EV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ACLK_REF"]
    #[inline(always)]
    pub fn aclk_ref(&self) -> ACLK_REF_R {
        ACLK_REF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VDDR_RECHARGE"]
    #[inline(always)]
    pub fn vddr_recharge(&self) -> VDDR_RECHARGE_R {
        VDDR_RECHARGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MCU_ACTIVE"]
    #[inline(always)]
    pub fn mcu_active(&self) -> MCU_ACTIVE_R {
        MCU_ACTIVE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PWR_DWN"]
    #[inline(always)]
    pub fn pwr_dwn(&self) -> PWR_DWN_R {
        PWR_DWN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(&self) -> SCLK_LF_R {
        SCLK_LF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AON_BATMON_TEMP_UPD"]
    #[inline(always)]
    pub fn aon_batmon_temp_upd(&self) -> AON_BATMON_TEMP_UPD_R {
        AON_BATMON_TEMP_UPD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AON_BATMON_BAT_UPD"]
    #[inline(always)]
    pub fn aon_batmon_bat_upd(&self) -> AON_BATMON_BAT_UPD_R {
        AON_BATMON_BAT_UPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AON_RTC_4KHZ"]
    #[inline(always)]
    pub fn aon_rtc_4khz(&self) -> AON_RTC_4KHZ_R {
        AON_RTC_4KHZ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(&self) -> AON_RTC_CH2_DLY_R {
        AON_RTC_CH2_DLY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MANUAL_EV"]
    #[inline(always)]
    pub fn manual_ev(&self) -> MANUAL_EV_R {
        MANUAL_EV_R::new((self.bits & 0x01) != 0)
    }
}
