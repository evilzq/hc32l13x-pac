#[doc = "Register `GCMAR` reader"]
pub struct R(crate::R<GCMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCMAR` writer"]
pub struct W(crate::W<GCMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCMAR_SPEC>;
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
impl From<crate::W<GCMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCMA` reader - desc GCMA"]
pub type GCMA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GCMA` writer - desc GCMA"]
pub type GCMA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCMAR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCMAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc GCMA"]
    #[inline(always)]
    pub fn gcma(&self) -> GCMA_R {
        GCMA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMA"]
    #[inline(always)]
    pub fn gcma(&mut self) -> GCMA_W<0> {
        GCMA_W::new(self)
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
#[doc = "desc GCMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcmar](index.html) module"]
pub struct GCMAR_SPEC;
impl crate::RegisterSpec for GCMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcmar::R](R) reader structure"]
impl crate::Readable for GCMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcmar::W](W) writer structure"]
impl crate::Writable for GCMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCMAR to value 0xffff"]
impl crate::Resettable for GCMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
