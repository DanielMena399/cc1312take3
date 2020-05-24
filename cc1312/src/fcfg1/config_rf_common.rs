#[doc = "Reader of register CONFIG_RF_COMMON"]
pub type R = crate::R<u32, super::CONFIG_RF_COMMON>;
#[doc = "Reader of field `DISABLE_CORNER_CAP`"]
pub type DISABLE_CORNER_CAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLDO_TRIM_OUTPUT`"]
pub type SLDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `PA20DBMTRIMCOMPLETE_N`"]
pub type PA20DBMTRIMCOMPLETE_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTL_PA_20DBM_TRIM`"]
pub type CTL_PA_20DBM_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `RFLDO_TRIM_OUTPUT`"]
pub type RFLDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `QUANTCTLTHRES`"]
pub type QUANTCTLTHRES_R = crate::R<u8, u8>;
#[doc = "Reader of field `DACTRIM`"]
pub type DACTRIM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31 - DISABLE_CORNER_CAP"]
    #[inline(always)]
    pub fn disable_corner_cap(&self) -> DISABLE_CORNER_CAP_R {
        DISABLE_CORNER_CAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 25:30 - SLDO_TRIM_OUTPUT"]
    #[inline(always)]
    pub fn sldo_trim_output(&self) -> SLDO_TRIM_OUTPUT_R {
        SLDO_TRIM_OUTPUT_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 21 - PA20DBMTRIMCOMPLETE_N"]
    #[inline(always)]
    pub fn pa20dbmtrimcomplete_n(&self) -> PA20DBMTRIMCOMPLETE_N_R {
        PA20DBMTRIMCOMPLETE_N_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - CTL_PA_20DBM_TRIM"]
    #[inline(always)]
    pub fn ctl_pa_20dbm_trim(&self) -> CTL_PA_20DBM_TRIM_R {
        CTL_PA_20DBM_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 9:15 - RFLDO_TRIM_OUTPUT"]
    #[inline(always)]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUT_R {
        RFLDO_TRIM_OUTPUT_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 6:8 - QUANTCTLTHRES"]
    #[inline(always)]
    pub fn quantctlthres(&self) -> QUANTCTLTHRES_R {
        QUANTCTLTHRES_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 0:5 - DACTRIM"]
    #[inline(always)]
    pub fn dactrim(&self) -> DACTRIM_R {
        DACTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
