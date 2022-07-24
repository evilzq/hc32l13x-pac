#[doc = "Register `ERRADDR` reader"]
pub struct R(crate::R<ERRADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERRADDR` reader - desc ERRADDR"]
pub type ERRADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:12 - desc ERRADDR"]
    #[inline(always)]
    pub fn erraddr(&self) -> ERRADDR_R {
        ERRADDR_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc ERRADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erraddr](index.html) module"]
pub struct ERRADDR_SPEC;
impl crate::RegisterSpec for ERRADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erraddr::R](R) reader structure"]
impl crate::Readable for ERRADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERRADDR to value 0"]
impl crate::Resettable for ERRADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
