#[doc = "Reader of register DMAPORTERR"]
pub type R = crate::R<u32, super::DMAPORTERR>;
#[doc = "Reader of field `PORT1_AHB_ERROR`"]
pub type PORT1_AHB_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PORT1_CHANNEL`"]
pub type PORT1_CHANNEL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 12 - PORT1_AHB_ERROR"]
    #[inline(always)]
    pub fn port1_ahb_error(&self) -> PORT1_AHB_ERROR_R {
        PORT1_AHB_ERROR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PORT1_CHANNEL"]
    #[inline(always)]
    pub fn port1_channel(&self) -> PORT1_CHANNEL_R {
        PORT1_CHANNEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
