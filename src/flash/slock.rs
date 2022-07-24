#[doc = "Register `SLOCK` reader"]
pub struct R(crate::R<SLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLOCK` writer"]
pub struct W(crate::W<SLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOCK_SPEC>;
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
impl From<crate::W<SLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOCK` reader - desc SLOCK"]
pub type SLOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLOCK` writer - desc SLOCK"]
pub type SLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOCK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc SLOCK"]
    #[inline(always)]
    pub fn slock(&self) -> SLOCK_R {
        SLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc SLOCK"]
    #[inline(always)]
    pub fn slock(&mut self) -> SLOCK_W<0> {
        SLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slock](index.html) module"]
pub struct SLOCK_SPEC;
impl crate::RegisterSpec for SLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slock::R](R) reader structure"]
impl crate::Readable for SLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slock::W](W) writer structure"]
impl crate::Writable for SLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLOCK to value 0"]
impl crate::Resettable for SLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
