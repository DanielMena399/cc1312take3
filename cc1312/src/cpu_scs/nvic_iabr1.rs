#[doc = "Reader of register NVIC_IABR1"]
pub type R = crate::R<u32, super::NVIC_IABR1>;
#[doc = "Reader of field `ACTIVE37`"]
pub type ACTIVE37_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE36`"]
pub type ACTIVE36_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE35`"]
pub type ACTIVE35_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE34`"]
pub type ACTIVE34_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE33`"]
pub type ACTIVE33_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE32`"]
pub type ACTIVE32_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - ACTIVE37"]
    #[inline(always)]
    pub fn active37(&self) -> ACTIVE37_R {
        ACTIVE37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACTIVE36"]
    #[inline(always)]
    pub fn active36(&self) -> ACTIVE36_R {
        ACTIVE36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACTIVE35"]
    #[inline(always)]
    pub fn active35(&self) -> ACTIVE35_R {
        ACTIVE35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACTIVE34"]
    #[inline(always)]
    pub fn active34(&self) -> ACTIVE34_R {
        ACTIVE34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACTIVE33"]
    #[inline(always)]
    pub fn active33(&self) -> ACTIVE33_R {
        ACTIVE33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ACTIVE32"]
    #[inline(always)]
    pub fn active32(&self) -> ACTIVE32_R {
        ACTIVE32_R::new((self.bits & 0x01) != 0)
    }
}
