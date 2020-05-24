#[doc = "Reader of register MVFR1"]
pub type R = crate::R<u32, super::MVFR1>;
#[doc = "Reader of field `FP_FUSED_MAC`"]
pub type FP_FUSED_MAC_R = crate::R<u8, u8>;
#[doc = "Reader of field `FP_HPFP`"]
pub type FP_HPFP_R = crate::R<u8, u8>;
#[doc = "Reader of field `D_NAN_MODE`"]
pub type D_NAN_MODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `FTZ_MODE`"]
pub type FTZ_MODE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - FP_FUSED_MAC"]
    #[inline(always)]
    pub fn fp_fused_mac(&self) -> FP_FUSED_MAC_R {
        FP_FUSED_MAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - FP_HPFP"]
    #[inline(always)]
    pub fn fp_hpfp(&self) -> FP_HPFP_R {
        FP_HPFP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - D_NAN_MODE"]
    #[inline(always)]
    pub fn d_nan_mode(&self) -> D_NAN_MODE_R {
        D_NAN_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - FTZ_MODE"]
    #[inline(always)]
    pub fn ftz_mode(&self) -> FTZ_MODE_R {
        FTZ_MODE_R::new((self.bits & 0x0f) as u8)
    }
}
