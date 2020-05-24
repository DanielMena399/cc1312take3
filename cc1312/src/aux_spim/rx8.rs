#[doc = "Reader of register RX8"]
pub type R = crate::R<u32, super::RX8>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
