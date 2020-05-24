#[doc = "Reader of register PER_CHK"]
pub type R = crate::R<u32, super::PER_CHK>;
#[doc = "Writer for register PER_CHK"]
pub type W = crate::W<u32, super::PER_CHK>;
#[doc = "Register PER_CHK `reset()`'s with value 0"]
impl crate::ResetValue for super::PER_CHK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PER_ADDR`"]
pub type PER_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PER_ADDR`"]
pub struct PER_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - PER_ADDR"]
    #[inline(always)]
    pub fn per_addr(&self) -> PER_ADDR_R {
        PER_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - PER_ADDR"]
    #[inline(always)]
    pub fn per_addr(&mut self) -> PER_ADDR_W {
        PER_ADDR_W { w: self }
    }
}
