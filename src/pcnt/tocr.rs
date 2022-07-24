#[doc = "Register `TOCR` reader"]
pub struct R(crate::R<TOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCR` writer"]
pub struct W(crate::W<TOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCR_SPEC>;
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
impl From<crate::W<TOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TH` reader - desc TH"]
pub type TH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH` writer - desc TH"]
pub type TH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOCR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - desc TH"]
    #[inline(always)]
    pub fn th(&self) -> TH_R {
        TH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc TH"]
    #[inline(always)]
    pub fn th(&mut self) -> TH_W<0> {
        TH_W::new(self)
    }
    #[doc = "Bit 16 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<16> {
        EN_W::new(self)
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
#[doc = "desc TOCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocr](index.html) module"]
pub struct TOCR_SPEC;
impl crate::RegisterSpec for TOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tocr::R](R) reader structure"]
impl crate::Readable for TOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocr::W](W) writer structure"]
impl crate::Writable for TOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOCR to value 0x0fff"]
impl crate::Resettable for TOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
