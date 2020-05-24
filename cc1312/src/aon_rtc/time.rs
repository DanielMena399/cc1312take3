#[doc = "Reader of register TIME"]
pub type R = crate::R<u32, super::TIME>;
#[doc = "Reader of field `SEC_L`"]
pub type SEC_L_R = crate::R<u16, u16>;
#[doc = "Reader of field `SUBSEC_H`"]
pub type SUBSEC_H_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - SEC_L"]
    #[inline(always)]
    pub fn sec_l(&self) -> SEC_L_R {
        SEC_L_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - SUBSEC_H"]
    #[inline(always)]
    pub fn subsec_h(&self) -> SUBSEC_H_R {
        SUBSEC_H_R::new((self.bits & 0xffff) as u16)
    }
}
