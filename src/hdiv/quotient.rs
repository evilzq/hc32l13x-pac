#[doc = "Register `QUOTIENT` reader"]
pub struct R(crate::R<QUOTIENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUOTIENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUOTIENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUOTIENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUOTIENT` writer"]
pub struct W(crate::W<QUOTIENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUOTIENT_SPEC>;
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
impl From<crate::W<QUOTIENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUOTIENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUOTIENT` reader - desc QUOTIENT"]
pub type QUOTIENT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QUOTIENT` writer - desc QUOTIENT"]
pub type QUOTIENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUOTIENT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc QUOTIENT"]
    #[inline(always)]
    pub fn quotient(&self) -> QUOTIENT_R {
        QUOTIENT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc QUOTIENT"]
    #[inline(always)]
    pub fn quotient(&mut self) -> QUOTIENT_W<0> {
        QUOTIENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc QUOTIENT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quotient](index.html) module"]
pub struct QUOTIENT_SPEC;
impl crate::RegisterSpec for QUOTIENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quotient::R](R) reader structure"]
impl crate::Readable for QUOTIENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quotient::W](W) writer structure"]
impl crate::Writable for QUOTIENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QUOTIENT to value 0"]
impl crate::Resettable for QUOTIENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
