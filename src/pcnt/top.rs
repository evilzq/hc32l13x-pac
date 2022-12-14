#[doc = "Register `TOP` reader"]
pub struct R(crate::R<TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOP` reader - desc TOP"]
pub type TOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - desc TOP"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc TOP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [top](index.html) module"]
pub struct TOP_SPEC;
impl crate::RegisterSpec for TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [top::R](R) reader structure"]
impl crate::Readable for TOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TOP to value 0xff"]
impl crate::Resettable for TOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
