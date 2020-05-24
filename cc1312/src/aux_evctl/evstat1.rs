#[doc = "Reader of register EVSTAT1"]
pub type R = crate::R<u32, super::EVSTAT1>;
#[doc = "Reader of field `AUXIO31`"]
pub type AUXIO31_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO30`"]
pub type AUXIO30_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO29`"]
pub type AUXIO29_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO28`"]
pub type AUXIO28_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO27`"]
pub type AUXIO27_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO26`"]
pub type AUXIO26_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO25`"]
pub type AUXIO25_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO24`"]
pub type AUXIO24_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO23`"]
pub type AUXIO23_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO22`"]
pub type AUXIO22_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO21`"]
pub type AUXIO21_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO20`"]
pub type AUXIO20_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO19`"]
pub type AUXIO19_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO18`"]
pub type AUXIO18_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO17`"]
pub type AUXIO17_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO16`"]
pub type AUXIO16_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - AUXIO31"]
    #[inline(always)]
    pub fn auxio31(&self) -> AUXIO31_R {
        AUXIO31_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AUXIO30"]
    #[inline(always)]
    pub fn auxio30(&self) -> AUXIO30_R {
        AUXIO30_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AUXIO29"]
    #[inline(always)]
    pub fn auxio29(&self) -> AUXIO29_R {
        AUXIO29_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AUXIO28"]
    #[inline(always)]
    pub fn auxio28(&self) -> AUXIO28_R {
        AUXIO28_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AUXIO27"]
    #[inline(always)]
    pub fn auxio27(&self) -> AUXIO27_R {
        AUXIO27_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AUXIO26"]
    #[inline(always)]
    pub fn auxio26(&self) -> AUXIO26_R {
        AUXIO26_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AUXIO25"]
    #[inline(always)]
    pub fn auxio25(&self) -> AUXIO25_R {
        AUXIO25_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AUXIO24"]
    #[inline(always)]
    pub fn auxio24(&self) -> AUXIO24_R {
        AUXIO24_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AUXIO23"]
    #[inline(always)]
    pub fn auxio23(&self) -> AUXIO23_R {
        AUXIO23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AUXIO22"]
    #[inline(always)]
    pub fn auxio22(&self) -> AUXIO22_R {
        AUXIO22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AUXIO21"]
    #[inline(always)]
    pub fn auxio21(&self) -> AUXIO21_R {
        AUXIO21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXIO20"]
    #[inline(always)]
    pub fn auxio20(&self) -> AUXIO20_R {
        AUXIO20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AUXIO19"]
    #[inline(always)]
    pub fn auxio19(&self) -> AUXIO19_R {
        AUXIO19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AUXIO18"]
    #[inline(always)]
    pub fn auxio18(&self) -> AUXIO18_R {
        AUXIO18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AUXIO17"]
    #[inline(always)]
    pub fn auxio17(&self) -> AUXIO17_R {
        AUXIO17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AUXIO16"]
    #[inline(always)]
    pub fn auxio16(&self) -> AUXIO16_R {
        AUXIO16_R::new((self.bits & 0x01) != 0)
    }
}
