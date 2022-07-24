#[doc = "Register `REMAINDER` reader"]
pub struct R(crate::R<REMAINDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMAINDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMAINDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMAINDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMAINDER` writer"]
pub struct W(crate::W<REMAINDER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMAINDER_SPEC>;
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
impl From<crate::W<REMAINDER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMAINDER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REMAINDER` reader - desc REMAINDER"]
pub type REMAINDER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REMAINDER` writer - desc REMAINDER"]
pub type REMAINDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REMAINDER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc REMAINDER"]
    #[inline(always)]
    pub fn remainder(&self) -> REMAINDER_R {
        REMAINDER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc REMAINDER"]
    #[inline(always)]
    pub fn remainder(&mut self) -> REMAINDER_W<0> {
        REMAINDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc REMAINDER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remainder](index.html) module"]
pub struct REMAINDER_SPEC;
impl crate::RegisterSpec for REMAINDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remainder::R](R) reader structure"]
impl crate::Readable for REMAINDER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remainder::W](W) writer structure"]
impl crate::Writable for REMAINDER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REMAINDER to value 0"]
impl crate::Resettable for REMAINDER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
