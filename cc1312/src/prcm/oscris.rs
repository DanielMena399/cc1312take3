#[doc = "Reader of register OSCRIS"]
pub type R = crate::R<u32, super::OSCRIS>;
#[doc = "Reader of field `HFSRCPENDRIS`"]
pub type HFSRCPENDRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFSRCDONERIS`"]
pub type LFSRCDONERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCDLFRIS`"]
pub type XOSCDLFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCLFRIS`"]
pub type XOSCLFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCOSCDLFRIS`"]
pub type RCOSCDLFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCOSCLFRIS`"]
pub type RCOSCLFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCHFRIS`"]
pub type XOSCHFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCOSCHFRIS`"]
pub type RCOSCHFRIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - HFSRCPENDRIS"]
    #[inline(always)]
    pub fn hfsrcpendris(&self) -> HFSRCPENDRIS_R {
        HFSRCPENDRIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFSRCDONERIS"]
    #[inline(always)]
    pub fn lfsrcdoneris(&self) -> LFSRCDONERIS_R {
        LFSRCDONERIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XOSCDLFRIS"]
    #[inline(always)]
    pub fn xoscdlfris(&self) -> XOSCDLFRIS_R {
        XOSCDLFRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XOSCLFRIS"]
    #[inline(always)]
    pub fn xosclfris(&self) -> XOSCLFRIS_R {
        XOSCLFRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RCOSCDLFRIS"]
    #[inline(always)]
    pub fn rcoscdlfris(&self) -> RCOSCDLFRIS_R {
        RCOSCDLFRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RCOSCLFRIS"]
    #[inline(always)]
    pub fn rcosclfris(&self) -> RCOSCLFRIS_R {
        RCOSCLFRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSCHFRIS"]
    #[inline(always)]
    pub fn xoschfris(&self) -> XOSCHFRIS_R {
        XOSCHFRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RCOSCHFRIS"]
    #[inline(always)]
    pub fn rcoschfris(&self) -> RCOSCHFRIS_R {
        RCOSCHFRIS_R::new((self.bits & 0x01) != 0)
    }
}
