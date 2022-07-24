#[doc = "Register `PB11_SEL` reader"]
pub struct R(crate::R<PB11_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB11_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB11_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB11_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB11_SEL` writer"]
pub struct W(crate::W<PB11_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB11_SEL_SPEC>;
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
impl From<crate::W<PB11_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB11_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - desc SEL"]
pub type SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL` writer - desc SEL"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PB11_SEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PB11_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc SEL"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc SEL"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
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
#[doc = "desc PB11_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb11_sel](index.html) module"]
pub struct PB11_SEL_SPEC;
impl crate::RegisterSpec for PB11_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb11_sel::R](R) reader structure"]
impl crate::Readable for PB11_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb11_sel::W](W) writer structure"]
impl crate::Writable for PB11_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB11_SEL to value 0"]
impl crate::Resettable for PB11_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
