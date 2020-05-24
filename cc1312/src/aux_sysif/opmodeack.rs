#[doc = "Reader of register OPMODEACK"]
pub type R = crate::R<u32, super::OPMODEACK>;
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 0x03) as u8)
    }
}
