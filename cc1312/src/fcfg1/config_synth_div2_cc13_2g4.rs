#[doc = "Reader of register CONFIG_SYNTH_DIV2_CC13_2G4"]
pub type R = crate::R<u32, super::CONFIG_SYNTH_DIV2_CC13_2G4>;
#[doc = "Reader of field `MIN_ALLOWED_RTRIM`"]
pub type MIN_ALLOWED_RTRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `RFC_MDM_DEMIQMC0`"]
pub type RFC_MDM_DEMIQMC0_R = crate::R<u16, u16>;
#[doc = "Reader of field `LDOVCO_TRIM_OUTPUT`"]
pub type LDOVCO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N`"]
pub type RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:31 - MIN_ALLOWED_RTRIM"]
    #[inline(always)]
    pub fn min_allowed_rtrim(&self) -> MIN_ALLOWED_RTRIM_R {
        MIN_ALLOWED_RTRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27 - RFC_MDM_DEMIQMC0"]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0(&self) -> RFC_MDM_DEMIQMC0_R {
        RFC_MDM_DEMIQMC0_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 6:11 - LDOVCO_TRIM_OUTPUT"]
    #[inline(always)]
    pub fn ldovco_trim_output(&self) -> LDOVCO_TRIM_OUTPUT_R {
        LDOVCO_TRIM_OUTPUT_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 5 - RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N"]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0_trimcomplete_n(&self) -> RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R {
        RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
