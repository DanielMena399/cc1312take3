#[doc = "Reader of register DAC_CAL0"]
pub type R = crate::R<u32, super::DAC_CAL0>;
#[doc = "Reader of field `SOC_DAC_VOUT_CAL_DECOUPLE_C2`"]
pub type SOC_DAC_VOUT_CAL_DECOUPLE_C2_R = crate::R<u16, u16>;
#[doc = "Reader of field `SOC_DAC_VOUT_CAL_DECOUPLE_C1`"]
pub type SOC_DAC_VOUT_CAL_DECOUPLE_C1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - SOC_DAC_VOUT_CAL_DECOUPLE_C2"]
    #[inline(always)]
    pub fn soc_dac_vout_cal_decouple_c2(&self) -> SOC_DAC_VOUT_CAL_DECOUPLE_C2_R {
        SOC_DAC_VOUT_CAL_DECOUPLE_C2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - SOC_DAC_VOUT_CAL_DECOUPLE_C1"]
    #[inline(always)]
    pub fn soc_dac_vout_cal_decouple_c1(&self) -> SOC_DAC_VOUT_CAL_DECOUPLE_C1_R {
        SOC_DAC_VOUT_CAL_DECOUPLE_C1_R::new((self.bits & 0xffff) as u16)
    }
}
