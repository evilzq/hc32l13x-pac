#[doc = "Register `DATA1` reader"]
pub struct R(crate::R<DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA1` reader - desc DATA1"]
pub type DATA1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(self.bits)
    }
}
#[doc = "desc DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data1](index.html) module"]
pub struct DATA1_SPEC;
impl crate::RegisterSpec for DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data1::R](R) reader structure"]
impl crate::Readable for DATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA1 to value 0"]
impl crate::Resettable for DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
