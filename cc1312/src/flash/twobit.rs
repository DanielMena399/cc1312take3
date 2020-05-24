#[doc = "Reader of register TWOBIT"]
pub type R = crate::R<u32, super::TWOBIT>;
#[doc = "Reader of field `FROMN`"]
pub type FROMN_R = crate::R<u32, u32>;
#[doc = "Reader of field `FROM0`"]
pub type FROM0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 1:31 - FROMN"]
    #[inline(always)]
    pub fn fromn(&self) -> FROMN_R {
        FROMN_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - FROM0"]
    #[inline(always)]
    pub fn from0(&self) -> FROM0_R {
        FROM0_R::new((self.bits & 0x01) != 0)
    }
}
