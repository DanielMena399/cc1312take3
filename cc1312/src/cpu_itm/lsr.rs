#[doc = "Reader of register LSR"]
pub type R = crate::R<u32, super::LSR>;
#[doc = "Reader of field `BYTEACC`"]
pub type BYTEACC_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACCESS`"]
pub type ACCESS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRESENT`"]
pub type PRESENT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - BYTEACC"]
    #[inline(always)]
    pub fn byteacc(&self) -> BYTEACC_R {
        BYTEACC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACCESS"]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PRESENT"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new((self.bits & 0x01) != 0)
    }
}
