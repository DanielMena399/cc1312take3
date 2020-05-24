#[doc = "Reader of register COMPARE"]
pub type R = crate::R<u32, super::COMPARE>;
#[doc = "Reader of field `A_GREATER_THAN_B`"]
pub type A_GREATER_THAN_B_R = crate::R<bool, bool>;
#[doc = "Reader of field `A_LESS_THAN_B`"]
pub type A_LESS_THAN_B_R = crate::R<bool, bool>;
#[doc = "Reader of field `A_EQUALS_B`"]
pub type A_EQUALS_B_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - A_GREATER_THAN_B"]
    #[inline(always)]
    pub fn a_greater_than_b(&self) -> A_GREATER_THAN_B_R {
        A_GREATER_THAN_B_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - A_LESS_THAN_B"]
    #[inline(always)]
    pub fn a_less_than_b(&self) -> A_LESS_THAN_B_R {
        A_LESS_THAN_B_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - A_EQUALS_B"]
    #[inline(always)]
    pub fn a_equals_b(&self) -> A_EQUALS_B_R {
        A_EQUALS_B_R::new((self.bits & 0x01) != 0)
    }
}
