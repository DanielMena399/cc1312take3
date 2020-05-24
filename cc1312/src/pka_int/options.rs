#[doc = "Reader of register OPTIONS"]
pub type R = crate::R<u32, super::OPTIONS>;
#[doc = "Reader of field `AIC_PRESENT`"]
pub type AIC_PRESENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EIP76_PRESENT`"]
pub type EIP76_PRESENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EIP28_PRESENT`"]
pub type EIP28_PRESENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `AXI_INTERFACE`"]
pub type AXI_INTERFACE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB_IS_ASYNC`"]
pub type AHB_IS_ASYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB_INTERFACE`"]
pub type AHB_INTERFACE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLB_INTERFACE`"]
pub type PLB_INTERFACE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 10 - AIC_PRESENT"]
    #[inline(always)]
    pub fn aic_present(&self) -> AIC_PRESENT_R {
        AIC_PRESENT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EIP76_PRESENT"]
    #[inline(always)]
    pub fn eip76_present(&self) -> EIP76_PRESENT_R {
        EIP76_PRESENT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EIP28_PRESENT"]
    #[inline(always)]
    pub fn eip28_present(&self) -> EIP28_PRESENT_R {
        EIP28_PRESENT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXI_INTERFACE"]
    #[inline(always)]
    pub fn axi_interface(&self) -> AXI_INTERFACE_R {
        AXI_INTERFACE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB_IS_ASYNC"]
    #[inline(always)]
    pub fn ahb_is_async(&self) -> AHB_IS_ASYNC_R {
        AHB_IS_ASYNC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AHB_INTERFACE"]
    #[inline(always)]
    pub fn ahb_interface(&self) -> AHB_INTERFACE_R {
        AHB_INTERFACE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PLB_INTERFACE"]
    #[inline(always)]
    pub fn plb_interface(&self) -> PLB_INTERFACE_R {
        PLB_INTERFACE_R::new((self.bits & 0x01) != 0)
    }
}
