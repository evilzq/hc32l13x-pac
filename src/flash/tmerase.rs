#[doc = "Register `TMERASE` reader"]
pub struct R(crate::R<TMERASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMERASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMERASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMERASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMERASE` writer"]
pub struct W(crate::W<TMERASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMERASE_SPEC>;
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
impl From<crate::W<TMERASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMERASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMERASE` reader - desc TMERASE"]
pub type TMERASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMERASE` writer - desc TMERASE"]
pub type TMERASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMERASE_SPEC, u32, u32, 21, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMERASE_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:20 - desc TMERASE"]
    #[inline(always)]
    pub fn tmerase(&self) -> TMERASE_R {
        TMERASE_R::new((self.bits & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 21:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:20 - desc TMERASE"]
    #[inline(always)]
    pub fn tmerase(&mut self) -> TMERASE_W<0> {
        TMERASE_W::new(self)
    }
    #[doc = "Bits 21:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<21> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TMERASE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmerase](index.html) module"]
pub struct TMERASE_SPEC;
impl crate::RegisterSpec for TMERASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmerase::R](R) reader structure"]
impl crate::Readable for TMERASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmerase::W](W) writer structure"]
impl crate::Writable for TMERASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMERASE to value 0x0002_22e0"]
impl crate::Resettable for TMERASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_22e0
    }
}
