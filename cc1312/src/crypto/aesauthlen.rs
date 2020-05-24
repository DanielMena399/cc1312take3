#[doc = "Writer for register AESAUTHLEN"]
pub type W = crate::W<u32, super::AESAUTHLEN>;
#[doc = "Register AESAUTHLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::AESAUTHLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AUTH_LENGTH`"]
pub struct AUTH_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTH_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - AUTH_LENGTH"]
    #[inline(always)]
    pub fn auth_length(&mut self) -> AUTH_LENGTH_W {
        AUTH_LENGTH_W { w: self }
    }
}
