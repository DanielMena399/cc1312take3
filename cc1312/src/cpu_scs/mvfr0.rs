#[doc = "Reader of register MVFR0"]
pub type R = crate::R<u32, super::MVFR0>;
#[doc = "Reader of field `FP_ROUNDING_MODES`"]
pub type FP_ROUNDING_MODES_R = crate::R<u8, u8>;
#[doc = "Reader of field `SHORT_VECTORS`"]
pub type SHORT_VECTORS_R = crate::R<u8, u8>;
#[doc = "Reader of field `SQUARE_ROOT`"]
pub type SQUARE_ROOT_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIVIDE`"]
pub type DIVIDE_R = crate::R<u8, u8>;
#[doc = "Reader of field `FP_EXCEPTION_TRAPPING`"]
pub type FP_EXCEPTION_TRAPPING_R = crate::R<u8, u8>;
#[doc = "Reader of field `DOUBLE_PRECISION`"]
pub type DOUBLE_PRECISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `SINGLE_PRECISION`"]
pub type SINGLE_PRECISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `A_SIMD`"]
pub type A_SIMD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - FP_ROUNDING_MODES"]
    #[inline(always)]
    pub fn fp_rounding_modes(&self) -> FP_ROUNDING_MODES_R {
        FP_ROUNDING_MODES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SHORT_VECTORS"]
    #[inline(always)]
    pub fn short_vectors(&self) -> SHORT_VECTORS_R {
        SHORT_VECTORS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SQUARE_ROOT"]
    #[inline(always)]
    pub fn square_root(&self) -> SQUARE_ROOT_R {
        SQUARE_ROOT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DIVIDE"]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - FP_EXCEPTION_TRAPPING"]
    #[inline(always)]
    pub fn fp_exception_trapping(&self) -> FP_EXCEPTION_TRAPPING_R {
        FP_EXCEPTION_TRAPPING_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DOUBLE_PRECISION"]
    #[inline(always)]
    pub fn double_precision(&self) -> DOUBLE_PRECISION_R {
        DOUBLE_PRECISION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SINGLE_PRECISION"]
    #[inline(always)]
    pub fn single_precision(&self) -> SINGLE_PRECISION_R {
        SINGLE_PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - A_SIMD"]
    #[inline(always)]
    pub fn a_simd(&self) -> A_SIMD_R {
        A_SIMD_R::new((self.bits & 0x0f) as u8)
    }
}
