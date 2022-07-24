#[doc = "Register `PBBSETCLR` reader"]
pub struct R(crate::R<PBBSETCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBBSETCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBBSETCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBBSETCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBBSETCLR` writer"]
pub struct W(crate::W<PBBSETCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBBSETCLR_SPEC>;
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
impl From<crate::W<PBBSETCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBBSETCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBBCLR` reader - desc PBBCLR"]
pub type PBBCLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PBBCLR` writer - desc PBBCLR"]
pub type PBBCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PBBSETCLR_SPEC, u16, u16, 16, O>;
#[doc = "Field `PBBSET` reader - desc PBBSET"]
pub type PBBSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PBBSET` writer - desc PBBSET"]
pub type PBBSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PBBSETCLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PBBCLR"]
    #[inline(always)]
    pub fn pbbclr(&self) -> PBBCLR_R {
        PBBCLR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc PBBSET"]
    #[inline(always)]
    pub fn pbbset(&self) -> PBBSET_R {
        PBBSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PBBCLR"]
    #[inline(always)]
    pub fn pbbclr(&mut self) -> PBBCLR_W<0> {
        PBBCLR_W::new(self)
    }
    #[doc = "Bits 16:31 - desc PBBSET"]
    #[inline(always)]
    pub fn pbbset(&mut self) -> PBBSET_W<16> {
        PBBSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PBBSETCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbbsetclr](index.html) module"]
pub struct PBBSETCLR_SPEC;
impl crate::RegisterSpec for PBBSETCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbbsetclr::R](R) reader structure"]
impl crate::Readable for PBBSETCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbbsetclr::W](W) writer structure"]
impl crate::Writable for PBBSETCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBBSETCLR to value 0"]
impl crate::Resettable for PBBSETCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
