#[doc = "Reader of register MSW"]
pub type R = crate::R<u32, super::MSW>;
#[doc = "Reader of field `RESULT_IS_ZERO`"]
pub type RESULT_IS_ZERO_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSW_ADDRESS`"]
pub type MSW_ADDRESS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 15 - RESULT_IS_ZERO"]
    #[inline(always)]
    pub fn result_is_zero(&self) -> RESULT_IS_ZERO_R {
        RESULT_IS_ZERO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - MSW_ADDRESS"]
    #[inline(always)]
    pub fn msw_address(&self) -> MSW_ADDRESS_R {
        MSW_ADDRESS_R::new((self.bits & 0x07ff) as u16)
    }
}
