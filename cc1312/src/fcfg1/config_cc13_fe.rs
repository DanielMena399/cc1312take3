#[doc = "Reader of register CONFIG_CC13_FE"]
pub type R = crate::R<u32, super::CONFIG_CC13_FE>;
#[doc = "Reader of field `IFAMP_IB`"]
pub type IFAMP_IB_R = crate::R<u8, u8>;
#[doc = "Reader of field `LNA_IB`"]
pub type LNA_IB_R = crate::R<u8, u8>;
#[doc = "Reader of field `IFAMP_TRIM`"]
pub type IFAMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `CTL_PA0_TRIM`"]
pub type CTL_PA0_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `PATRIMCOMPLETE_N`"]
pub type PATRIMCOMPLETE_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSSITRIMCOMPLETE_N`"]
pub type RSSITRIMCOMPLETE_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSSI_OFFSET`"]
pub type RSSI_OFFSET_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - IFAMP_IB"]
    #[inline(always)]
    pub fn ifamp_ib(&self) -> IFAMP_IB_R {
        IFAMP_IB_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - LNA_IB"]
    #[inline(always)]
    pub fn lna_ib(&self) -> LNA_IB_R {
        LNA_IB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 19:23 - IFAMP_TRIM"]
    #[inline(always)]
    pub fn ifamp_trim(&self) -> IFAMP_TRIM_R {
        IFAMP_TRIM_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - CTL_PA0_TRIM"]
    #[inline(always)]
    pub fn ctl_pa0_trim(&self) -> CTL_PA0_TRIM_R {
        CTL_PA0_TRIM_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - PATRIMCOMPLETE_N"]
    #[inline(always)]
    pub fn patrimcomplete_n(&self) -> PATRIMCOMPLETE_N_R {
        PATRIMCOMPLETE_N_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RSSITRIMCOMPLETE_N"]
    #[inline(always)]
    pub fn rssitrimcomplete_n(&self) -> RSSITRIMCOMPLETE_N_R {
        RSSITRIMCOMPLETE_N_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - RSSI_OFFSET"]
    #[inline(always)]
    pub fn rssi_offset(&self) -> RSSI_OFFSET_R {
        RSSI_OFFSET_R::new((self.bits & 0xff) as u8)
    }
}
