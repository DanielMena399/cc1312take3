#[doc = "Reader of register T1CNTR"]
pub type R = crate::R<u32, super::T1CNTR>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
