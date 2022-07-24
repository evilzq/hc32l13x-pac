#[doc = "Register `RESULT` reader"]
pub struct R(crate::R<RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESULT` writer"]
pub struct W(crate::W<RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESULT_SPEC>;
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
impl From<crate::W<RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type RESULT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESULT` writer - desc RESULT"]
pub type RESULT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESULT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc RESULT"]
    #[inline(always)]
    pub fn result(&mut self) -> RESULT_W<0> {
        RESULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RESULT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](index.html) module"]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result::R](R) reader structure"]
impl crate::Readable for RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [result::W](W) writer structure"]
impl crate::Writable for RESULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
