#[doc = "Register `RESULTACC` reader"]
pub struct R(crate::R<RESULTACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULTACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULTACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULTACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULTACC` reader - desc RESULTACC"]
pub type RESULTACC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19 - desc RESULTACC"]
    #[inline(always)]
    pub fn resultacc(&self) -> RESULTACC_R {
        RESULTACC_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc RESULTACC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resultacc](index.html) module"]
pub struct RESULTACC_SPEC;
impl crate::RegisterSpec for RESULTACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resultacc::R](R) reader structure"]
impl crate::Readable for RESULTACC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULTACC to value 0"]
impl crate::Resettable for RESULTACC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
