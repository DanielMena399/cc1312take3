#[doc = "Reader of register AESIV%s"]
pub type R = crate::R<u32, super::AESIV>;
#[doc = "Writer for register AESIV%s"]
pub type W = crate::W<u32, super::AESIV>;
#[doc = "Register AESIV%s `reset()`'s with value 0"]
impl crate::ResetValue for super::AESIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_IV`"]
pub type AES_IV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_IV`"]
pub struct AES_IV_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_IV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES_IV"]
    #[inline(always)]
    pub fn aes_iv(&self) -> AES_IV_R {
        AES_IV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES_IV"]
    #[inline(always)]
    pub fn aes_iv(&mut self) -> AES_IV_W {
        AES_IV_W { w: self }
    }
}
