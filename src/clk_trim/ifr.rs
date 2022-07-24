#[doc = "Register `IFR` reader"]
pub struct R(crate::R<IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STOP` reader - desc STOP"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `CALCNT_OF` reader - desc CALCNT_OF"]
pub type CALCNT_OF_R = crate::BitReader<bool>;
#[doc = "Field `XTL_FAULT` reader - desc XTL_FAULT"]
pub type XTL_FAULT_R = crate::BitReader<bool>;
#[doc = "Field `XTH_FAULT` reader - desc XTH_FAULT"]
pub type XTH_FAULT_R = crate::BitReader<bool>;
#[doc = "Field `PLL_FAULT` reader - desc PLL_FAULT"]
pub type PLL_FAULT_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CALCNT_OF"]
    #[inline(always)]
    pub fn calcnt_of(&self) -> CALCNT_OF_R {
        CALCNT_OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc XTL_FAULT"]
    #[inline(always)]
    pub fn xtl_fault(&self) -> XTL_FAULT_R {
        XTL_FAULT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc XTH_FAULT"]
    #[inline(always)]
    pub fn xth_fault(&self) -> XTH_FAULT_R {
        XTH_FAULT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PLL_FAULT"]
    #[inline(always)]
    pub fn pll_fault(&self) -> PLL_FAULT_R {
        PLL_FAULT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc IFR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](index.html) module"]
pub struct IFR_SPEC;
impl crate::RegisterSpec for IFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifr::R](R) reader structure"]
impl crate::Readable for IFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IFR to value 0"]
impl crate::Resettable for IFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
