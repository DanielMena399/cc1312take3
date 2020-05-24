#[doc = "Writer for register HASHDATAIN25"]
pub type W = crate::W<u32, super::HASHDATAIN25>;
#[doc = "Register HASHDATAIN25 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASHDATAIN25 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `HASH_DATA_IN`"]
pub struct HASH_DATA_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_DATA_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - HASH_DATA_IN"]
    #[inline(always)]
    pub fn hash_data_in(&mut self) -> HASH_DATA_IN_W {
        HASH_DATA_IN_W { w: self }
    }
}
