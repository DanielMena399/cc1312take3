#[doc = "Reader of register CLS"]
pub type R = crate::R<u32, super::CLS>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x3f) as u8)
    }
}
