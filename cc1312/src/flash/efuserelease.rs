#[doc = "Reader of register EFUSERELEASE"]
pub type R = crate::R<u32, super::EFUSERELEASE>;
#[doc = "Reader of field `ODPYEAR`"]
pub type ODPYEAR_R = crate::R<u8, u8>;
#[doc = "Reader of field `ODPMONTH`"]
pub type ODPMONTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `ODPDAY`"]
pub type ODPDAY_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSEYEAR`"]
pub type EFUSEYEAR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSEMONTH`"]
pub type EFUSEMONTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSEDAY`"]
pub type EFUSEDAY_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 25:31 - ODPYEAR"]
    #[inline(always)]
    pub fn odpyear(&self) -> ODPYEAR_R {
        ODPYEAR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 21:24 - ODPMONTH"]
    #[inline(always)]
    pub fn odpmonth(&self) -> ODPMONTH_R {
        ODPMONTH_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - ODPDAY"]
    #[inline(always)]
    pub fn odpday(&self) -> ODPDAY_R {
        ODPDAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 9:15 - EFUSEYEAR"]
    #[inline(always)]
    pub fn efuseyear(&self) -> EFUSEYEAR_R {
        EFUSEYEAR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 5:8 - EFUSEMONTH"]
    #[inline(always)]
    pub fn efusemonth(&self) -> EFUSEMONTH_R {
        EFUSEMONTH_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - EFUSEDAY"]
    #[inline(always)]
    pub fn efuseday(&self) -> EFUSEDAY_R {
        EFUSEDAY_R::new((self.bits & 0x1f) as u8)
    }
}
