#[doc = "Reader of register PDCTL1VIMS"]
pub type R = crate::R<u32, super::PDCTL1VIMS>;
#[doc = "Writer for register PDCTL1VIMS"]
pub type W = crate::W<u32, super::PDCTL1VIMS>;
#[doc = "Register PDCTL1VIMS `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PDCTL1VIMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "MODE"]
pub type MODE_A = super::pdctl1::VIMS_MODE_A;
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, super::pdctl1::VIMS_MODE_A>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VIMS power domain is only powered when CPU power domain is powered."]
    #[inline(always)]
    pub fn cpu_dep(self) -> &'a mut W {
        self.variant(super::pdctl1::VIMS_MODE_A::CPU_DEP)
    }
    #[doc = "VIMS power domain is powered whenever the BUS power domain is powered."]
    #[inline(always)]
    pub fn bus_dep(self) -> &'a mut W {
        self.variant(super::pdctl1::VIMS_MODE_A::BUS_DEP)
    }
    #[doc = "Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(super::pdctl1::VIMS_MODE_A::BLOCKED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
