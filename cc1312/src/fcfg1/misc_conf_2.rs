#[doc = "Reader of register MISC_CONF_2"]
pub type R = crate::R<u32, super::MISC_CONF_2>;
#[doc = "Reader of field `HPOSC_COMP_P3`"]
pub type HPOSC_COMP_P3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - HPOSC_COMP_P3"]
    #[inline(always)]
    pub fn hposc_comp_p3(&self) -> HPOSC_COMP_P3_R {
        HPOSC_COMP_P3_R::new((self.bits & 0xff) as u8)
    }
}
