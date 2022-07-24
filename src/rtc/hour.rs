#[doc = "Register `HOUR` reader"]
pub struct R(crate::R<HOUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOUR` writer"]
pub struct W(crate::W<HOUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOUR_SPEC>;
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
impl From<crate::W<HOUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOURL` reader - desc HOURL"]
pub type HOURL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOURL` writer - desc HOURL"]
pub type HOURL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOUR_SPEC, u8, u8, 4, O>;
#[doc = "Field `HOURH` reader - desc HOURH"]
pub type HOURH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOURH` writer - desc HOURH"]
pub type HOURH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOUR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOUR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc HOURL"]
    #[inline(always)]
    pub fn hourl(&self) -> HOURL_R {
        HOURL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - desc HOURH"]
    #[inline(always)]
    pub fn hourh(&self) -> HOURH_R {
        HOURH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc HOURL"]
    #[inline(always)]
    pub fn hourl(&mut self) -> HOURL_W<0> {
        HOURL_W::new(self)
    }
    #[doc = "Bits 4:5 - desc HOURH"]
    #[inline(always)]
    pub fn hourh(&mut self) -> HOURH_W<4> {
        HOURH_W::new(self)
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
#[doc = "desc HOUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hour](index.html) module"]
pub struct HOUR_SPEC;
impl crate::RegisterSpec for HOUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hour::R](R) reader structure"]
impl crate::Readable for HOUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hour::W](W) writer structure"]
impl crate::Writable for HOUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOUR to value 0"]
impl crate::Resettable for HOUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
