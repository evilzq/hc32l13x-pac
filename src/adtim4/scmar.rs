#[doc = "Register `SCMAR` reader"]
pub struct R(crate::R<SCMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMAR` writer"]
pub struct W(crate::W<SCMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMAR_SPEC>;
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
impl From<crate::W<SCMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCMA` reader - desc SCMA"]
pub type SCMA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCMA` writer - desc SCMA"]
pub type SCMA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCMAR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCMAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc SCMA"]
    #[inline(always)]
    pub fn scma(&self) -> SCMA_R {
        SCMA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc SCMA"]
    #[inline(always)]
    pub fn scma(&mut self) -> SCMA_W<0> {
        SCMA_W::new(self)
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
#[doc = "desc SCMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmar](index.html) module"]
pub struct SCMAR_SPEC;
impl crate::RegisterSpec for SCMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scmar::R](R) reader structure"]
impl crate::Readable for SCMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmar::W](W) writer structure"]
impl crate::Writable for SCMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCMAR to value 0xffff"]
impl crate::Resettable for SCMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
