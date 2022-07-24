#[doc = "Register `TMRCV` reader"]
pub struct R(crate::R<TMRCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMRCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMRCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMRCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMRCV` writer"]
pub struct W(crate::W<TMRCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMRCV_SPEC>;
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
impl From<crate::W<TMRCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMRCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMRCV` reader - desc TMRCV"]
pub type TMRCV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TMRCV` writer - desc TMRCV"]
pub type TMRCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMRCV_SPEC, u16, u16, 14, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMRCV_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:13 - desc TMRCV"]
    #[inline(always)]
    pub fn tmrcv(&self) -> TMRCV_R {
        TMRCV_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:13 - desc TMRCV"]
    #[inline(always)]
    pub fn tmrcv(&mut self) -> TMRCV_W<0> {
        TMRCV_W::new(self)
    }
    #[doc = "Bits 14:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<14> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TMRCV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmrcv](index.html) module"]
pub struct TMRCV_SPEC;
impl crate::RegisterSpec for TMRCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmrcv::R](R) reader structure"]
impl crate::Readable for TMRCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmrcv::W](W) writer structure"]
impl crate::Writable for TMRCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMRCV to value 0x03e8"]
impl crate::Resettable for TMRCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03e8
    }
}
