#[doc = "Reader of register OSC_CONF"]
pub type R = crate::R<u32, super::OSC_CONF>;
#[doc = "Reader of field `ADC_SH_VBUF_EN`"]
pub type ADC_SH_VBUF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_SH_MODE_EN`"]
pub type ADC_SH_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ATESTLF_RCOSCLF_IBIAS_TRIM`"]
pub type ATESTLF_RCOSCLF_IBIAS_TRIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCLF_REGULATOR_TRIM`"]
pub type XOSCLF_REGULATOR_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSCLF_CMIRRWR_RATIO`"]
pub type XOSCLF_CMIRRWR_RATIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSC_HF_FAST_START`"]
pub type XOSC_HF_FAST_START_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSC_OPTION`"]
pub type XOSC_OPTION_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPOSC_OPTION`"]
pub type HPOSC_OPTION_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPOSC_BIAS_HOLD_MODE_EN`"]
pub type HPOSC_BIAS_HOLD_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPOSC_CURRMIRR_RATIO`"]
pub type HPOSC_CURRMIRR_RATIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPOSC_BIAS_RES_SET`"]
pub type HPOSC_BIAS_RES_SET_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPOSC_FILTER_EN`"]
pub type HPOSC_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPOSC_BIAS_RECHARGE_DELAY`"]
pub type HPOSC_BIAS_RECHARGE_DELAY_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPOSC_SERIES_CAP`"]
pub type HPOSC_SERIES_CAP_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPOSC_DIV3_BYPASS`"]
pub type HPOSC_DIV3_BYPASS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 29 - ADC_SH_VBUF_EN"]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_EN_R {
        ADC_SH_VBUF_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC_SH_MODE_EN"]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_EN_R {
        ADC_SH_MODE_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ATESTLF_RCOSCLF_IBIAS_TRIM"]
    #[inline(always)]
    pub fn atestlf_rcosclf_ibias_trim(&self) -> ATESTLF_RCOSCLF_IBIAS_TRIM_R {
        ATESTLF_RCOSCLF_IBIAS_TRIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - XOSCLF_REGULATOR_TRIM"]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIM_R {
        XOSCLF_REGULATOR_TRIM_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 21:24 - XOSCLF_CMIRRWR_RATIO"]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIO_R {
        XOSCLF_CMIRRWR_RATIO_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 19:20 - XOSC_HF_FAST_START"]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_START_R {
        XOSC_HF_FAST_START_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - XOSC_OPTION"]
    #[inline(always)]
    pub fn xosc_option(&self) -> XOSC_OPTION_R {
        XOSC_OPTION_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HPOSC_OPTION"]
    #[inline(always)]
    pub fn hposc_option(&self) -> HPOSC_OPTION_R {
        HPOSC_OPTION_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HPOSC_BIAS_HOLD_MODE_EN"]
    #[inline(always)]
    pub fn hposc_bias_hold_mode_en(&self) -> HPOSC_BIAS_HOLD_MODE_EN_R {
        HPOSC_BIAS_HOLD_MODE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - HPOSC_CURRMIRR_RATIO"]
    #[inline(always)]
    pub fn hposc_currmirr_ratio(&self) -> HPOSC_CURRMIRR_RATIO_R {
        HPOSC_CURRMIRR_RATIO_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - HPOSC_BIAS_RES_SET"]
    #[inline(always)]
    pub fn hposc_bias_res_set(&self) -> HPOSC_BIAS_RES_SET_R {
        HPOSC_BIAS_RES_SET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - HPOSC_FILTER_EN"]
    #[inline(always)]
    pub fn hposc_filter_en(&self) -> HPOSC_FILTER_EN_R {
        HPOSC_FILTER_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - HPOSC_BIAS_RECHARGE_DELAY"]
    #[inline(always)]
    pub fn hposc_bias_recharge_delay(&self) -> HPOSC_BIAS_RECHARGE_DELAY_R {
        HPOSC_BIAS_RECHARGE_DELAY_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - HPOSC_SERIES_CAP"]
    #[inline(always)]
    pub fn hposc_series_cap(&self) -> HPOSC_SERIES_CAP_R {
        HPOSC_SERIES_CAP_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - HPOSC_DIV3_BYPASS"]
    #[inline(always)]
    pub fn hposc_div3_bypass(&self) -> HPOSC_DIV3_BYPASS_R {
        HPOSC_DIV3_BYPASS_R::new((self.bits & 0x01) != 0)
    }
}
