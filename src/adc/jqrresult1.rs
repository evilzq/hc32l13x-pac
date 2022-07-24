#[doc = "Register `JQRRESULT1` reader"]
pub struct R(crate::R<JQRRESULT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JQRRESULT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JQRRESULT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JQRRESULT1_SPEC>) -> Self {
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
#[doc = "desc JQRRESULT1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jqrresult1](index.html) module"]
pub struct JQRRESULT1_SPEC;
impl crate::RegisterSpec for JQRRESULT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jqrresult1::R](R) reader structure"]
impl crate::Readable for JQRRESULT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JQRRESULT1 to value 0"]
impl crate::Resettable for JQRRESULT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
