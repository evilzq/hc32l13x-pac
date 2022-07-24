#[doc = "Register `GCMBR` reader"]
pub struct R(crate::R<GCMBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCMBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCMBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCMBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCMBR` writer"]
pub struct W(crate::W<GCMBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCMBR_SPEC>;
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
impl From<crate::W<GCMBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCMBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCMB` reader - desc GCMB"]
pub type GCMB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GCMB` writer - desc GCMB"]
pub type GCMB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCMBR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCMBR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc GCMB"]
    #[inline(always)]
    pub fn gcmb(&self) -> GCMB_R {
        GCMB_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMB"]
    #[inline(always)]
    pub fn gcmb(&mut self) -> GCMB_W<0> {
        GCMB_W::new(self)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<31> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc GCMBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcmbr](index.html) module"]
pub struct GCMBR_SPEC;
impl crate::RegisterSpec for GCMBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcmbr::R](R) reader structure"]
impl crate::Readable for GCMBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcmbr::W](W) writer structure"]
impl crate::Writable for GCMBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCMBR to value 0xffff"]
impl crate::Resettable for GCMBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
