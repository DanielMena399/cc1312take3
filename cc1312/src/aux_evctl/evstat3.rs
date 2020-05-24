#[doc = "Reader of register EVSTAT3"]
pub type R = crate::R<u32, super::EVSTAT3>;
#[doc = "Reader of field `AUX_TIMER2_CLKSWITCH_RDY`"]
pub type AUX_TIMER2_CLKSWITCH_RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_DAC_HOLD_ACTIVE`"]
pub type AUX_DAC_HOLD_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_SMPH_AUTOTAKE_DONE`"]
pub type AUX_SMPH_AUTOTAKE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_ADC_FIFO_NOT_EMPTY`"]
pub type AUX_ADC_FIFO_NOT_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_ADC_FIFO_ALMOST_FULL`"]
pub type AUX_ADC_FIFO_ALMOST_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_ADC_IRQ`"]
pub type AUX_ADC_IRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_ADC_DONE`"]
pub type AUX_ADC_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_ISRC_RESET_N`"]
pub type AUX_ISRC_RESET_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_TDC_DONE`"]
pub type AUX_TDC_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_TIMER0_EV`"]
pub type AUX_TIMER0_EV_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_TIMER1_EV`"]
pub type AUX_TIMER1_EV_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_TIMER2_PULSE`"]
pub type AUX_TIMER2_PULSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_TIMER2_EV3`"]
pub type AUX_TIMER2_EV3_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_TIMER2_EV2`"]
pub type AUX_TIMER2_EV2_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_TIMER2_EV1`"]
pub type AUX_TIMER2_EV1_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_TIMER2_EV0`"]
pub type AUX_TIMER2_EV0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - AUX_TIMER2_CLKSWITCH_RDY"]
    #[inline(always)]
    pub fn aux_timer2_clkswitch_rdy(&self) -> AUX_TIMER2_CLKSWITCH_RDY_R {
        AUX_TIMER2_CLKSWITCH_RDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AUX_DAC_HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(&self) -> AUX_DAC_HOLD_ACTIVE_R {
        AUX_DAC_HOLD_ACTIVE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AUX_SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AUX_SMPH_AUTOTAKE_DONE_R {
        AUX_SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AUX_ADC_FIFO_NOT_EMPTY"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(&self) -> AUX_ADC_FIFO_NOT_EMPTY_R {
        AUX_ADC_FIFO_NOT_EMPTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AUX_ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AUX_ADC_FIFO_ALMOST_FULL_R {
        AUX_ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AUX_ADC_IRQ"]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AUX_ADC_IRQ_R {
        AUX_ADC_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AUX_ADC_DONE"]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONE_R {
        AUX_ADC_DONE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AUX_ISRC_RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(&self) -> AUX_ISRC_RESET_N_R {
        AUX_ISRC_RESET_N_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AUX_TDC_DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONE_R {
        AUX_TDC_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AUX_TIMER0_EV"]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EV_R {
        AUX_TIMER0_EV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AUX_TIMER1_EV"]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EV_R {
        AUX_TIMER1_EV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_pulse(&self) -> AUX_TIMER2_PULSE_R {
        AUX_TIMER2_PULSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AUX_TIMER2_EV3"]
    #[inline(always)]
    pub fn aux_timer2_ev3(&self) -> AUX_TIMER2_EV3_R {
        AUX_TIMER2_EV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AUX_TIMER2_EV2"]
    #[inline(always)]
    pub fn aux_timer2_ev2(&self) -> AUX_TIMER2_EV2_R {
        AUX_TIMER2_EV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AUX_TIMER2_EV1"]
    #[inline(always)]
    pub fn aux_timer2_ev1(&self) -> AUX_TIMER2_EV1_R {
        AUX_TIMER2_EV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AUX_TIMER2_EV0"]
    #[inline(always)]
    pub fn aux_timer2_ev0(&self) -> AUX_TIMER2_EV0_R {
        AUX_TIMER2_EV0_R::new((self.bits & 0x01) != 0)
    }
}
