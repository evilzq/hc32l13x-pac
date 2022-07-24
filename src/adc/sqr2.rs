#[doc = "Register `SQR2` reader"]
pub struct R(crate::R<SQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR2` writer"]
pub struct W(crate::W<SQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR2_SPEC>;
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
impl From<crate::W<SQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH12MUX` reader - desc CH12MUX"]
pub type CH12MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH12MUX` writer - desc CH12MUX"]
pub type CH12MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH13MUX` reader - desc CH13MUX"]
pub type CH13MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH13MUX` writer - desc CH13MUX"]
pub type CH13MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH14MUX` reader - desc CH14MUX"]
pub type CH14MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH14MUX` writer - desc CH14MUX"]
pub type CH14MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH15MUX` reader - desc CH15MUX"]
pub type CH15MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH15MUX` writer - desc CH15MUX"]
pub type CH15MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SQR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - desc CH12MUX"]
    #[inline(always)]
    pub fn ch12mux(&self) -> CH12MUX_R {
        CH12MUX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - desc CH13MUX"]
    #[inline(always)]
    pub fn ch13mux(&self) -> CH13MUX_R {
        CH13MUX_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - desc CH14MUX"]
    #[inline(always)]
    pub fn ch14mux(&self) -> CH14MUX_R {
        CH14MUX_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - desc CH15MUX"]
    #[inline(always)]
    pub fn ch15mux(&self) -> CH15MUX_R {
        CH15MUX_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc CH12MUX"]
    #[inline(always)]
    pub fn ch12mux(&mut self) -> CH12MUX_W<0> {
        CH12MUX_W::new(self)
    }
    #[doc = "Bits 5:9 - desc CH13MUX"]
    #[inline(always)]
    pub fn ch13mux(&mut self) -> CH13MUX_W<5> {
        CH13MUX_W::new(self)
    }
    #[doc = "Bits 10:14 - desc CH14MUX"]
    #[inline(always)]
    pub fn ch14mux(&mut self) -> CH14MUX_W<10> {
        CH14MUX_W::new(self)
    }
    #[doc = "Bits 15:19 - desc CH15MUX"]
    #[inline(always)]
    pub fn ch15mux(&mut self) -> CH15MUX_W<15> {
        CH15MUX_W::new(self)
    }
    #[doc = "Bits 20:23 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<20> {
        CNT_W::new(self)
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
#[doc = "desc SQR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr2](index.html) module"]
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr2::R](R) reader structure"]
impl crate::Readable for SQR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr2::W](W) writer structure"]
impl crate::Writable for SQR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
