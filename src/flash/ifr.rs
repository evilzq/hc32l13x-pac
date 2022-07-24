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
#[doc = "Field `IF0` reader - desc IF0"]
pub type IF0_R = crate::BitReader<bool>;
#[doc = "Field `IF1` reader - desc IF1"]
pub type IF1_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0 - desc IF0"]
    #[inline(always)]
    pub fn if0(&self) -> IF0_R {
        IF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc IF1"]
    #[inline(always)]
    pub fn if1(&self) -> IF1_R {
        IF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
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
