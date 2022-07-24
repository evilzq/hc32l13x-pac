#[doc = "Register `PABSETCLR` reader"]
pub struct R(crate::R<PABSETCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PABSETCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PABSETCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PABSETCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PABSETCLR` writer"]
pub struct W(crate::W<PABSETCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PABSETCLR_SPEC>;
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
impl From<crate::W<PABSETCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PABSETCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PABCLR` reader - desc PABCLR"]
pub type PABCLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PABCLR` writer - desc PABCLR"]
pub type PABCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PABSETCLR_SPEC, u16, u16, 16, O>;
#[doc = "Field `PABSET` reader - desc PABSET"]
pub type PABSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PABSET` writer - desc PABSET"]
pub type PABSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PABSETCLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PABCLR"]
    #[inline(always)]
    pub fn pabclr(&self) -> PABCLR_R {
        PABCLR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc PABSET"]
    #[inline(always)]
    pub fn pabset(&self) -> PABSET_R {
        PABSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PABCLR"]
    #[inline(always)]
    pub fn pabclr(&mut self) -> PABCLR_W<0> {
        PABCLR_W::new(self)
    }
    #[doc = "Bits 16:31 - desc PABSET"]
    #[inline(always)]
    pub fn pabset(&mut self) -> PABSET_W<16> {
        PABSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PABSETCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pabsetclr](index.html) module"]
pub struct PABSETCLR_SPEC;
impl crate::RegisterSpec for PABSETCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pabsetclr::R](R) reader structure"]
impl crate::Readable for PABSETCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pabsetclr::W](W) writer structure"]
impl crate::Writable for PABSETCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PABSETCLR to value 0"]
impl crate::Resettable for PABSETCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
