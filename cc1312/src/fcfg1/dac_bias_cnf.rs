#[doc = "Reader of register DAC_BIAS_CNF"]
pub type R = crate::R<u32, super::DAC_BIAS_CNF>;
#[doc = "Reader of field `LPM_TRIM_IOUT`"]
pub type LPM_TRIM_IOUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPM_BIAS_WIDTH_TRIM`"]
pub type LPM_BIAS_WIDTH_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPM_BIAS_BACKUP_EN`"]
pub type LPM_BIAS_BACKUP_EN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 12:17 - LPM_TRIM_IOUT"]
    #[inline(always)]
    pub fn lpm_trim_iout(&self) -> LPM_TRIM_IOUT_R {
        LPM_TRIM_IOUT_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 9:11 - LPM_BIAS_WIDTH_TRIM"]
    #[inline(always)]
    pub fn lpm_bias_width_trim(&self) -> LPM_BIAS_WIDTH_TRIM_R {
        LPM_BIAS_WIDTH_TRIM_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 8 - LPM_BIAS_BACKUP_EN"]
    #[inline(always)]
    pub fn lpm_bias_backup_en(&self) -> LPM_BIAS_BACKUP_EN_R {
        LPM_BIAS_BACKUP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
