#[doc = "Register `GCMCR` reader"]
pub struct R(crate::R<GCMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCMCR` writer"]
pub struct W(crate::W<GCMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GCMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCMC` reader - desc GCMC"]
pub type GCMC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GCMC` writer - desc GCMC"]
pub type GCMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCMCR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc GCMC"]
    #[inline(always)]
    pub fn gcmc(&self) -> GCMC_R {
        GCMC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMC"]
    #[inline(always)]
    pub fn gcmc(&mut self) -> GCMC_W<0> {
        GCMC_W::new(self)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<31> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc GCMCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcmcr](index.html) module"]
pub struct GCMCR_SPEC;
impl crate::RegisterSpec for GCMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcmcr::R](R) reader structure"]
impl crate::Readable for GCMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcmcr::W](W) writer structure"]
impl crate::Writable for GCMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCMCR to value 0xffff"]
impl crate::Resettable for GCMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
