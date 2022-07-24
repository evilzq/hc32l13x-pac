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
#[doc = "Field `SGLIF` reader - desc SGLIF"]
pub type SGLIF_R = crate::BitReader<bool>;
#[doc = "Field `LTIF` reader - desc LTIF"]
pub type LTIF_R = crate::BitReader<bool>;
#[doc = "Field `HTIF` reader - desc HTIF"]
pub type HTIF_R = crate::BitReader<bool>;
#[doc = "Field `REGIF` reader - desc REGIF"]
pub type REGIF_R = crate::BitReader<bool>;
#[doc = "Field `SQRIF` reader - desc SQRIF"]
pub type SQRIF_R = crate::BitReader<bool>;
#[doc = "Field `JQRIF` reader - desc JQRIF"]
pub type JQRIF_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc SGLIF"]
    #[inline(always)]
    pub fn sglif(&self) -> SGLIF_R {
        SGLIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc LTIF"]
    #[inline(always)]
    pub fn ltif(&self) -> LTIF_R {
        LTIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HTIF"]
    #[inline(always)]
    pub fn htif(&self) -> HTIF_R {
        HTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc REGIF"]
    #[inline(always)]
    pub fn regif(&self) -> REGIF_R {
        REGIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SQRIF"]
    #[inline(always)]
    pub fn sqrif(&self) -> SQRIF_R {
        SQRIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc JQRIF"]
    #[inline(always)]
    pub fn jqrif(&self) -> JQRIF_R {
        JQRIF_R::new(((self.bits >> 5) & 1) != 0)
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
