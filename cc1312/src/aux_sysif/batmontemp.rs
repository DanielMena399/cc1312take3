#[doc = "Reader of register BATMONTEMP"]
pub type R = crate::R<u32, super::BATMONTEMP>;
#[doc = "Reader of field `SIGN`"]
pub type SIGN_R = crate::R<u8, u8>;
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u16, u16>;
#[doc = "Reader of field `FRAC`"]
pub type FRAC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 11:15 - SIGN"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 2:10 - INT"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:1 - FRAC"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x03) as u8)
    }
}
