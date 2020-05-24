#[doc = "Reader of register NVIC_IABR0"]
pub type R = crate::R<u32, super::NVIC_IABR0>;
#[doc = "Reader of field `ACTIVE31`"]
pub type ACTIVE31_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE30`"]
pub type ACTIVE30_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE29`"]
pub type ACTIVE29_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE28`"]
pub type ACTIVE28_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE27`"]
pub type ACTIVE27_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE26`"]
pub type ACTIVE26_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE25`"]
pub type ACTIVE25_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE24`"]
pub type ACTIVE24_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE23`"]
pub type ACTIVE23_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE22`"]
pub type ACTIVE22_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE21`"]
pub type ACTIVE21_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE20`"]
pub type ACTIVE20_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE19`"]
pub type ACTIVE19_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE18`"]
pub type ACTIVE18_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE17`"]
pub type ACTIVE17_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE16`"]
pub type ACTIVE16_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE15`"]
pub type ACTIVE15_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE14`"]
pub type ACTIVE14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE13`"]
pub type ACTIVE13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE12`"]
pub type ACTIVE12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE11`"]
pub type ACTIVE11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE10`"]
pub type ACTIVE10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE9`"]
pub type ACTIVE9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE8`"]
pub type ACTIVE8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE7`"]
pub type ACTIVE7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE6`"]
pub type ACTIVE6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE5`"]
pub type ACTIVE5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE4`"]
pub type ACTIVE4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE3`"]
pub type ACTIVE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE2`"]
pub type ACTIVE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE1`"]
pub type ACTIVE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTIVE0`"]
pub type ACTIVE0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - ACTIVE31"]
    #[inline(always)]
    pub fn active31(&self) -> ACTIVE31_R {
        ACTIVE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ACTIVE30"]
    #[inline(always)]
    pub fn active30(&self) -> ACTIVE30_R {
        ACTIVE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ACTIVE29"]
    #[inline(always)]
    pub fn active29(&self) -> ACTIVE29_R {
        ACTIVE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ACTIVE28"]
    #[inline(always)]
    pub fn active28(&self) -> ACTIVE28_R {
        ACTIVE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ACTIVE27"]
    #[inline(always)]
    pub fn active27(&self) -> ACTIVE27_R {
        ACTIVE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ACTIVE26"]
    #[inline(always)]
    pub fn active26(&self) -> ACTIVE26_R {
        ACTIVE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ACTIVE25"]
    #[inline(always)]
    pub fn active25(&self) -> ACTIVE25_R {
        ACTIVE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ACTIVE24"]
    #[inline(always)]
    pub fn active24(&self) -> ACTIVE24_R {
        ACTIVE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ACTIVE23"]
    #[inline(always)]
    pub fn active23(&self) -> ACTIVE23_R {
        ACTIVE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ACTIVE22"]
    #[inline(always)]
    pub fn active22(&self) -> ACTIVE22_R {
        ACTIVE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ACTIVE21"]
    #[inline(always)]
    pub fn active21(&self) -> ACTIVE21_R {
        ACTIVE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ACTIVE20"]
    #[inline(always)]
    pub fn active20(&self) -> ACTIVE20_R {
        ACTIVE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ACTIVE19"]
    #[inline(always)]
    pub fn active19(&self) -> ACTIVE19_R {
        ACTIVE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ACTIVE18"]
    #[inline(always)]
    pub fn active18(&self) -> ACTIVE18_R {
        ACTIVE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ACTIVE17"]
    #[inline(always)]
    pub fn active17(&self) -> ACTIVE17_R {
        ACTIVE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ACTIVE16"]
    #[inline(always)]
    pub fn active16(&self) -> ACTIVE16_R {
        ACTIVE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ACTIVE15"]
    #[inline(always)]
    pub fn active15(&self) -> ACTIVE15_R {
        ACTIVE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ACTIVE14"]
    #[inline(always)]
    pub fn active14(&self) -> ACTIVE14_R {
        ACTIVE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ACTIVE13"]
    #[inline(always)]
    pub fn active13(&self) -> ACTIVE13_R {
        ACTIVE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ACTIVE12"]
    #[inline(always)]
    pub fn active12(&self) -> ACTIVE12_R {
        ACTIVE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ACTIVE11"]
    #[inline(always)]
    pub fn active11(&self) -> ACTIVE11_R {
        ACTIVE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ACTIVE10"]
    #[inline(always)]
    pub fn active10(&self) -> ACTIVE10_R {
        ACTIVE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ACTIVE9"]
    #[inline(always)]
    pub fn active9(&self) -> ACTIVE9_R {
        ACTIVE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ACTIVE8"]
    #[inline(always)]
    pub fn active8(&self) -> ACTIVE8_R {
        ACTIVE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ACTIVE7"]
    #[inline(always)]
    pub fn active7(&self) -> ACTIVE7_R {
        ACTIVE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ACTIVE6"]
    #[inline(always)]
    pub fn active6(&self) -> ACTIVE6_R {
        ACTIVE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACTIVE5"]
    #[inline(always)]
    pub fn active5(&self) -> ACTIVE5_R {
        ACTIVE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACTIVE4"]
    #[inline(always)]
    pub fn active4(&self) -> ACTIVE4_R {
        ACTIVE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACTIVE3"]
    #[inline(always)]
    pub fn active3(&self) -> ACTIVE3_R {
        ACTIVE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACTIVE2"]
    #[inline(always)]
    pub fn active2(&self) -> ACTIVE2_R {
        ACTIVE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACTIVE1"]
    #[inline(always)]
    pub fn active1(&self) -> ACTIVE1_R {
        ACTIVE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ACTIVE0"]
    #[inline(always)]
    pub fn active0(&self) -> ACTIVE0_R {
        ACTIVE0_R::new((self.bits & 0x01) != 0)
    }
}
