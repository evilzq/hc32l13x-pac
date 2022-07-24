#[doc = "Register `REFCNT` reader"]
pub struct R(crate::R<REFCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REFCNT` reader - desc REFCNT"]
pub type REFCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc REFCNT"]
    #[inline(always)]
    pub fn refcnt(&self) -> REFCNT_R {
        REFCNT_R::new(self.bits)
    }
}
#[doc = "desc REFCNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refcnt](index.html) module"]
pub struct REFCNT_SPEC;
impl crate::RegisterSpec for REFCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refcnt::R](R) reader structure"]
impl crate::Readable for REFCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REFCNT to value 0"]
impl crate::Resettable for REFCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
