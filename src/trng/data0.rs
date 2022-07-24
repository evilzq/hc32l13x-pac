#[doc = "Register `DATA0` reader"]
pub struct R(crate::R<DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA0` reader - desc DATA0"]
pub type DATA0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(self.bits)
    }
}
#[doc = "desc DATA0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0](index.html) module"]
pub struct DATA0_SPEC;
impl crate::RegisterSpec for DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0::R](R) reader structure"]
impl crate::Readable for DATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for DATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
