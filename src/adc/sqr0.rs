#[doc = "Register `SQR0` reader"]
pub struct R(crate::R<SQR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR0` writer"]
pub struct W(crate::W<SQR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR0_SPEC>;
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
impl From<crate::W<SQR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0MUX` reader - desc CH0MUX"]
pub type CH0MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0MUX` writer - desc CH0MUX"]
pub type CH0MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH1MUX` reader - desc CH1MUX"]
pub type CH1MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1MUX` writer - desc CH1MUX"]
pub type CH1MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH2MUX` reader - desc CH2MUX"]
pub type CH2MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2MUX` writer - desc CH2MUX"]
pub type CH2MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH3MUX` reader - desc CH3MUX"]
pub type CH3MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3MUX` writer - desc CH3MUX"]
pub type CH3MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH4MUX` reader - desc CH4MUX"]
pub type CH4MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH4MUX` writer - desc CH4MUX"]
pub type CH4MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH5MUX` reader - desc CH5MUX"]
pub type CH5MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH5MUX` writer - desc CH5MUX"]
pub type CH5MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SQR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - desc CH0MUX"]
    #[inline(always)]
    pub fn ch0mux(&self) -> CH0MUX_R {
        CH0MUX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - desc CH1MUX"]
    #[inline(always)]
    pub fn ch1mux(&self) -> CH1MUX_R {
        CH1MUX_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - desc CH2MUX"]
    #[inline(always)]
    pub fn ch2mux(&self) -> CH2MUX_R {
        CH2MUX_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - desc CH3MUX"]
    #[inline(always)]
    pub fn ch3mux(&self) -> CH3MUX_R {
        CH3MUX_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - desc CH4MUX"]
    #[inline(always)]
    pub fn ch4mux(&self) -> CH4MUX_R {
        CH4MUX_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - desc CH5MUX"]
    #[inline(always)]
    pub fn ch5mux(&self) -> CH5MUX_R {
        CH5MUX_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc CH0MUX"]
    #[inline(always)]
    pub fn ch0mux(&mut self) -> CH0MUX_W<0> {
        CH0MUX_W::new(self)
    }
    #[doc = "Bits 5:9 - desc CH1MUX"]
    #[inline(always)]
    pub fn ch1mux(&mut self) -> CH1MUX_W<5> {
        CH1MUX_W::new(self)
    }
    #[doc = "Bits 10:14 - desc CH2MUX"]
    #[inline(always)]
    pub fn ch2mux(&mut self) -> CH2MUX_W<10> {
        CH2MUX_W::new(self)
    }
    #[doc = "Bits 15:19 - desc CH3MUX"]
    #[inline(always)]
    pub fn ch3mux(&mut self) -> CH3MUX_W<15> {
        CH3MUX_W::new(self)
    }
    #[doc = "Bits 20:24 - desc CH4MUX"]
    #[inline(always)]
    pub fn ch4mux(&mut self) -> CH4MUX_W<20> {
        CH4MUX_W::new(self)
    }
    #[doc = "Bits 25:29 - desc CH5MUX"]
    #[inline(always)]
    pub fn ch5mux(&mut self) -> CH5MUX_W<25> {
        CH5MUX_W::new(self)
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
#[doc = "desc SQR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr0](index.html) module"]
pub struct SQR0_SPEC;
impl crate::RegisterSpec for SQR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr0::R](R) reader structure"]
impl crate::Readable for SQR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr0::W](W) writer structure"]
impl crate::Writable for SQR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR0 to value 0"]
impl crate::Resettable for SQR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
