#[doc = "Register `DSTADR0` reader"]
pub struct R(crate::R<DSTADR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTADR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTADR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTADR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSTADR0` writer"]
pub struct W(crate::W<DSTADR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSTADR0_SPEC>;
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
impl From<crate::W<DSTADR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSTADR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTADR` reader - desc DSTADR"]
pub type DSTADR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSTADR` writer - desc DSTADR"]
pub type DSTADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSTADR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc DSTADR"]
    #[inline(always)]
    pub fn dstadr(&self) -> DSTADR_R {
        DSTADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc DSTADR"]
    #[inline(always)]
    pub fn dstadr(&mut self) -> DSTADR_W<0> {
        DSTADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DSTADR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstadr0](index.html) module"]
pub struct DSTADR0_SPEC;
impl crate::RegisterSpec for DSTADR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dstadr0::R](R) reader structure"]
impl crate::Readable for DSTADR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dstadr0::W](W) writer structure"]
impl crate::Writable for DSTADR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSTADR0 to value 0"]
impl crate::Resettable for DSTADR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
