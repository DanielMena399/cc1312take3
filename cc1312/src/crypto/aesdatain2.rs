#[doc = "Writer for register AESDATAIN2"]
pub type W = crate::W<u32, super::AESDATAIN2>;
#[doc = "Register AESDATAIN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AESDATAIN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AES_DATA_IN_OUT`"]
pub struct AES_DATA_IN_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_DATA_IN_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - AES_DATA_IN_OUT"]
    #[inline(always)]
    pub fn aes_data_in_out(&mut self) -> AES_DATA_IN_OUT_W {
        AES_DATA_IN_OUT_W { w: self }
    }
}
