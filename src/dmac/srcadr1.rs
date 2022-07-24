#[doc = "Register `SRCADR1` reader"]
pub struct R(crate::R<SRCADR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCADR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCADR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCADR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCADR1` writer"]
pub struct W(crate::W<SRCADR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCADR1_SPEC>;
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
impl From<crate::W<SRCADR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCADR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCADR` reader - desc SRCADR"]
pub type SRCADR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRCADR` writer - desc SRCADR"]
pub type SRCADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRCADR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc SRCADR"]
    #[inline(always)]
    pub fn srcadr(&self) -> SRCADR_R {
        SRCADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc SRCADR"]
    #[inline(always)]
    pub fn srcadr(&mut self) -> SRCADR_W<0> {
        SRCADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SRCADR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcadr1](index.html) module"]
pub struct SRCADR1_SPEC;
impl crate::RegisterSpec for SRCADR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcadr1::R](R) reader structure"]
impl crate::Readable for SRCADR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcadr1::W](W) writer structure"]
impl crate::Writable for SRCADR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCADR1 to value 0"]
impl crate::Resettable for SRCADR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
