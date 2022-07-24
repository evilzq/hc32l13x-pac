#[doc = "Register `TM` reader"]
pub struct R(crate::R<TM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TM` writer"]
pub struct W(crate::W<TM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TM_SPEC>;
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
impl From<crate::W<TM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TM` reader - desc TM"]
pub type TM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TM` writer - desc TM"]
pub type TM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TM_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc TM"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W<0> {
        TM_W::new(self)
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
#[doc = "desc TM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tm](index.html) module"]
pub struct TM_SPEC;
impl crate::RegisterSpec for TM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tm::R](R) reader structure"]
impl crate::Readable for TM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tm::W](W) writer structure"]
impl crate::Writable for TM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TM to value 0"]
impl crate::Resettable for TM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
