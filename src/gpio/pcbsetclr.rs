#[doc = "Register `PCBSETCLR` reader"]
pub struct R(crate::R<PCBSETCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCBSETCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCBSETCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCBSETCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCBSETCLR` writer"]
pub struct W(crate::W<PCBSETCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCBSETCLR_SPEC>;
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
impl From<crate::W<PCBSETCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCBSETCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCBCLR` reader - desc PCBCLR"]
pub type PCBCLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCBCLR` writer - desc PCBCLR"]
pub type PCBCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCBSETCLR_SPEC, u16, u16, 16, O>;
#[doc = "Field `PCBSET` reader - desc PCBSET"]
pub type PCBSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCBSET` writer - desc PCBSET"]
pub type PCBSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCBSETCLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PCBCLR"]
    #[inline(always)]
    pub fn pcbclr(&self) -> PCBCLR_R {
        PCBCLR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc PCBSET"]
    #[inline(always)]
    pub fn pcbset(&self) -> PCBSET_R {
        PCBSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PCBCLR"]
    #[inline(always)]
    pub fn pcbclr(&mut self) -> PCBCLR_W<0> {
        PCBCLR_W::new(self)
    }
    #[doc = "Bits 16:31 - desc PCBSET"]
    #[inline(always)]
    pub fn pcbset(&mut self) -> PCBSET_W<16> {
        PCBSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PCBSETCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcbsetclr](index.html) module"]
pub struct PCBSETCLR_SPEC;
impl crate::RegisterSpec for PCBSETCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcbsetclr::R](R) reader structure"]
impl crate::Readable for PCBSETCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcbsetclr::W](W) writer structure"]
impl crate::Writable for PCBSETCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCBSETCLR to value 0"]
impl crate::Resettable for PCBSETCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
