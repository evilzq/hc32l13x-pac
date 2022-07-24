#[doc = "Register `REFCON` reader"]
pub struct R(crate::R<REFCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFCON` writer"]
pub struct W(crate::W<REFCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCON_SPEC>;
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
impl From<crate::W<REFCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCNTVAL` reader - desc RCNTVAL"]
pub type RCNTVAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RCNTVAL` writer - desc RCNTVAL"]
pub type RCNTVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REFCON_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc RCNTVAL"]
    #[inline(always)]
    pub fn rcntval(&self) -> RCNTVAL_R {
        RCNTVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc RCNTVAL"]
    #[inline(always)]
    pub fn rcntval(&mut self) -> RCNTVAL_W<0> {
        RCNTVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc REFCON\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refcon](index.html) module"]
pub struct REFCON_SPEC;
impl crate::RegisterSpec for REFCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refcon::R](R) reader structure"]
impl crate::Readable for REFCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refcon::W](W) writer structure"]
impl crate::Writable for REFCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFCON to value 0"]
impl crate::Resettable for REFCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
