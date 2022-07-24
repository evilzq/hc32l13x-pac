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
#[doc = "Field `TF` reader - desc TF"]
pub type TF_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc TF"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 1) != 0)
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
