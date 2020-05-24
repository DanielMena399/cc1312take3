#[doc = "Reader of register MISC_TRIM"]
pub type R = crate::R<u32, super::MISC_TRIM>;
#[doc = "Reader of field `TRIM_RECHARGE_COMP_OFFSET`"]
pub type TRIM_RECHARGE_COMP_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIM_RECHARGE_COMP_REFLEVEL`"]
pub type TRIM_RECHARGE_COMP_REFLEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TEMPVSLOPE`"]
pub type TEMPVSLOPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 12:16 - TRIM_RECHARGE_COMP_OFFSET"]
    #[inline(always)]
    pub fn trim_recharge_comp_offset(&self) -> TRIM_RECHARGE_COMP_OFFSET_R {
        TRIM_RECHARGE_COMP_OFFSET_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - TRIM_RECHARGE_COMP_REFLEVEL"]
    #[inline(always)]
    pub fn trim_recharge_comp_reflevel(&self) -> TRIM_RECHARGE_COMP_REFLEVEL_R {
        TRIM_RECHARGE_COMP_REFLEVEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - TEMPVSLOPE"]
    #[inline(always)]
    pub fn tempvslope(&self) -> TEMPVSLOPE_R {
        TEMPVSLOPE_R::new((self.bits & 0xff) as u8)
    }
}
