#[doc = "Reader of register DMASTAT"]
pub type R = crate::R<u32, super::DMASTAT>;
#[doc = "Reader of field `PORT_ERR`"]
pub type PORT_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1_ACT`"]
pub type CH1_ACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0_ACT`"]
pub type CH0_ACT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 17 - PORT_ERR"]
    #[inline(always)]
    pub fn port_err(&self) -> PORT_ERR_R {
        PORT_ERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1_ACT"]
    #[inline(always)]
    pub fn ch1_act(&self) -> CH1_ACT_R {
        CH1_ACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CH0_ACT"]
    #[inline(always)]
    pub fn ch0_act(&self) -> CH0_ACT_R {
        CH0_ACT_R::new((self.bits & 0x01) != 0)
    }
}
