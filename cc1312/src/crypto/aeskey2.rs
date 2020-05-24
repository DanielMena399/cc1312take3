#[doc = "Writer for register AESKEY2%s"]
pub type W = crate::W<u32, super::AESKEY2>;
#[doc = "Register AESKEY2%s `reset()`'s with value 0"]
impl crate::ResetValue for super::AESKEY2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AES_KEY2`"]
pub struct AES_KEY2_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - AES_KEY2"]
    #[inline(always)]
    pub fn aes_key2(&mut self) -> AES_KEY2_W {
        AES_KEY2_W { w: self }
    }
}
