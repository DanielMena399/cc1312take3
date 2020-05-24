#[doc = "Reader of register EVSTAT0"]
pub type R = crate::R<u32, super::EVSTAT0>;
#[doc = "Reader of field `AUXIO15`"]
pub type AUXIO15_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO14`"]
pub type AUXIO14_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO13`"]
pub type AUXIO13_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO12`"]
pub type AUXIO12_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO11`"]
pub type AUXIO11_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO10`"]
pub type AUXIO10_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO9`"]
pub type AUXIO9_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO8`"]
pub type AUXIO8_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO7`"]
pub type AUXIO7_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO6`"]
pub type AUXIO6_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO5`"]
pub type AUXIO5_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO4`"]
pub type AUXIO4_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO3`"]
pub type AUXIO3_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO2`"]
pub type AUXIO2_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO1`"]
pub type AUXIO1_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO0`"]
pub type AUXIO0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - AUXIO15"]
    #[inline(always)]
    pub fn auxio15(&self) -> AUXIO15_R {
        AUXIO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AUXIO14"]
    #[inline(always)]
    pub fn auxio14(&self) -> AUXIO14_R {
        AUXIO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AUXIO13"]
    #[inline(always)]
    pub fn auxio13(&self) -> AUXIO13_R {
        AUXIO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AUXIO12"]
    #[inline(always)]
    pub fn auxio12(&self) -> AUXIO12_R {
        AUXIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AUXIO11"]
    #[inline(always)]
    pub fn auxio11(&self) -> AUXIO11_R {
        AUXIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AUXIO10"]
    #[inline(always)]
    pub fn auxio10(&self) -> AUXIO10_R {
        AUXIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AUXIO9"]
    #[inline(always)]
    pub fn auxio9(&self) -> AUXIO9_R {
        AUXIO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AUXIO8"]
    #[inline(always)]
    pub fn auxio8(&self) -> AUXIO8_R {
        AUXIO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AUXIO7"]
    #[inline(always)]
    pub fn auxio7(&self) -> AUXIO7_R {
        AUXIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AUXIO6"]
    #[inline(always)]
    pub fn auxio6(&self) -> AUXIO6_R {
        AUXIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AUXIO5"]
    #[inline(always)]
    pub fn auxio5(&self) -> AUXIO5_R {
        AUXIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXIO4"]
    #[inline(always)]
    pub fn auxio4(&self) -> AUXIO4_R {
        AUXIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AUXIO3"]
    #[inline(always)]
    pub fn auxio3(&self) -> AUXIO3_R {
        AUXIO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AUXIO2"]
    #[inline(always)]
    pub fn auxio2(&self) -> AUXIO2_R {
        AUXIO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AUXIO1"]
    #[inline(always)]
    pub fn auxio1(&self) -> AUXIO1_R {
        AUXIO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AUXIO0"]
    #[inline(always)]
    pub fn auxio0(&self) -> AUXIO0_R {
        AUXIO0_R::new((self.bits & 0x01) != 0)
    }
}
