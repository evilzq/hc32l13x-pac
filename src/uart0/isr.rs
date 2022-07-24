#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RC` reader - desc RC"]
pub type RC_R = crate::BitReader<bool>;
#[doc = "Field `TC` reader - desc TC"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `FE` reader - desc FE"]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `PE` reader - desc PE"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `CTSIF` reader - desc CTSIF"]
pub type CTSIF_R = crate::BitReader<bool>;
#[doc = "Field `CTS` reader - desc CTS"]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc RC"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc FE"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CTSIF"]
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc ISR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0x08"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
