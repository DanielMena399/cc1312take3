#[doc = "Reader of register STCR"]
pub type R = crate::R<u32, super::STCR>;
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TENMS`"]
pub type TENMS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 31 - NOREF"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SKEW"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - TENMS"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
