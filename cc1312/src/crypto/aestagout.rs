#[doc = "Reader of register AESTAGOUT%s"]
pub type R = crate::R<u32, super::AESTAGOUT>;
#[doc = "Reader of field `AES_TAG`"]
pub type AES_TAG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES_TAG"]
    #[inline(always)]
    pub fn aes_tag(&self) -> AES_TAG_R {
        AES_TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
