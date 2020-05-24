#[doc = "Reader of register WUFLAGS"]
pub type R = crate::R<u32, super::WUFLAGS>;
#[doc = "Reader of field `SW_WU3`"]
pub type SW_WU3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_WU2`"]
pub type SW_WU2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_WU1`"]
pub type SW_WU1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SW_WU0`"]
pub type SW_WU0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROG_WU3`"]
pub type PROG_WU3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROG_WU2`"]
pub type PROG_WU2_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROG_WU1`"]
pub type PROG_WU1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROG_WU0`"]
pub type PROG_WU0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - SW_WU3"]
    #[inline(always)]
    pub fn sw_wu3(&self) -> SW_WU3_R {
        SW_WU3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SW_WU2"]
    #[inline(always)]
    pub fn sw_wu2(&self) -> SW_WU2_R {
        SW_WU2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SW_WU1"]
    #[inline(always)]
    pub fn sw_wu1(&self) -> SW_WU1_R {
        SW_WU1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SW_WU0"]
    #[inline(always)]
    pub fn sw_wu0(&self) -> SW_WU0_R {
        SW_WU0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PROG_WU3"]
    #[inline(always)]
    pub fn prog_wu3(&self) -> PROG_WU3_R {
        PROG_WU3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PROG_WU2"]
    #[inline(always)]
    pub fn prog_wu2(&self) -> PROG_WU2_R {
        PROG_WU2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PROG_WU1"]
    #[inline(always)]
    pub fn prog_wu1(&self) -> PROG_WU1_R {
        PROG_WU1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PROG_WU0"]
    #[inline(always)]
    pub fn prog_wu0(&self) -> PROG_WU0_R {
        PROG_WU0_R::new((self.bits & 0x01) != 0)
    }
}
