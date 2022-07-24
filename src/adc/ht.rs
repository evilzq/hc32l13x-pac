#[doc = "Register `HT` reader"]
pub struct R(crate::R<HT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HT` writer"]
pub struct W(crate::W<HT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HT_SPEC>;
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
impl From<crate::W<HT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT` reader - desc HT"]
pub type HT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HT` writer - desc HT"]
pub type HT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HT_SPEC, u16, u16, 12, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - desc HT"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc HT"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<0> {
        HT_W::new(self)
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
#[doc = "desc HT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ht](index.html) module"]
pub struct HT_SPEC;
impl crate::RegisterSpec for HT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ht::R](R) reader structure"]
impl crate::Readable for HT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ht::W](W) writer structure"]
impl crate::Writable for HT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HT to value 0x0fff"]
impl crate::Resettable for HT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
