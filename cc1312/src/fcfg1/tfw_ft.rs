#[doc = "Reader of register TFW_FT"]
pub type R = crate::R<u32, super::TFW_FT>;
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - REV"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
