#[doc = "Reader of register SSPSR"]
pub type R = crate::R<u32, super::SSPSR>;
#[doc = "Reader of field `FOUR`"]
pub type FOUR_R = crate::R<bool, bool>;
#[doc = "Reader of field `THREE`"]
pub type THREE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TWO`"]
pub type TWO_R = crate::R<bool, bool>;
#[doc = "Reader of field `ONE`"]
pub type ONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - FOUR"]
    #[inline(always)]
    pub fn four(&self) -> FOUR_R {
        FOUR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - THREE"]
    #[inline(always)]
    pub fn three(&self) -> THREE_R {
        THREE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TWO"]
    #[inline(always)]
    pub fn two(&self) -> TWO_R {
        TWO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ONE"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new((self.bits & 0x01) != 0)
    }
}
