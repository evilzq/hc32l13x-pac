#[doc = "Register `SQRRESULT15` reader"]
pub struct R(crate::R<SQRRESULT15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQRRESULT15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQRRESULT15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQRRESULT15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:11 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc SQRRESULT15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqrresult15](index.html) module"]
pub struct SQRRESULT15_SPEC;
impl crate::RegisterSpec for SQRRESULT15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqrresult15::R](R) reader structure"]
impl crate::Readable for SQRRESULT15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SQRRESULT15 to value 0"]
impl crate::Resettable for SQRRESULT15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
