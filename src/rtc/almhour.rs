#[doc = "Register `ALMHOUR` reader"]
pub struct R(crate::R<ALMHOUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALMHOUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALMHOUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALMHOUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALMHOUR` writer"]
pub struct W(crate::W<ALMHOUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALMHOUR_SPEC>;
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
impl From<crate::W<ALMHOUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALMHOUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALMHOURL` reader - desc ALMHOURL"]
pub type ALMHOURL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALMHOURL` writer - desc ALMHOURL"]
pub type ALMHOURL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALMHOUR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALMHOURH` reader - desc ALMHOURH"]
pub type ALMHOURH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALMHOURH` writer - desc ALMHOURH"]
pub type ALMHOURH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALMHOUR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALMHOUR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc ALMHOURL"]
    #[inline(always)]
    pub fn almhourl(&self) -> ALMHOURL_R {
        ALMHOURL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - desc ALMHOURH"]
    #[inline(always)]
    pub fn almhourh(&self) -> ALMHOURH_R {
        ALMHOURH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc ALMHOURL"]
    #[inline(always)]
    pub fn almhourl(&mut self) -> ALMHOURL_W<0> {
        ALMHOURL_W::new(self)
    }
    #[doc = "Bits 4:5 - desc ALMHOURH"]
    #[inline(always)]
    pub fn almhourh(&mut self) -> ALMHOURH_W<4> {
        ALMHOURH_W::new(self)
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
#[doc = "desc ALMHOUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [almhour](index.html) module"]
pub struct ALMHOUR_SPEC;
impl crate::RegisterSpec for ALMHOUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [almhour::R](R) reader structure"]
impl crate::Readable for ALMHOUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [almhour::W](W) writer structure"]
impl crate::Writable for ALMHOUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALMHOUR to value 0"]
impl crate::Resettable for ALMHOUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
