#[doc = "Register `SQR1` reader"]
pub struct R(crate::R<SQR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR1` writer"]
pub struct W(crate::W<SQR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR1_SPEC>;
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
impl From<crate::W<SQR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH6MUX` reader - desc CH6MUX"]
pub type CH6MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH6MUX` writer - desc CH6MUX"]
pub type CH6MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH7MUX` reader - desc CH7MUX"]
pub type CH7MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH7MUX` writer - desc CH7MUX"]
pub type CH7MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH8MUX` reader - desc CH8MUX"]
pub type CH8MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH8MUX` writer - desc CH8MUX"]
pub type CH8MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH9MUX` reader - desc CH9MUX"]
pub type CH9MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH9MUX` writer - desc CH9MUX"]
pub type CH9MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH10MUX` reader - desc CH10MUX"]
pub type CH10MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH10MUX` writer - desc CH10MUX"]
pub type CH10MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH11MUX` reader - desc CH11MUX"]
pub type CH11MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH11MUX` writer - desc CH11MUX"]
pub type CH11MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SQR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - desc CH6MUX"]
    #[inline(always)]
    pub fn ch6mux(&self) -> CH6MUX_R {
        CH6MUX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - desc CH7MUX"]
    #[inline(always)]
    pub fn ch7mux(&self) -> CH7MUX_R {
        CH7MUX_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - desc CH8MUX"]
    #[inline(always)]
    pub fn ch8mux(&self) -> CH8MUX_R {
        CH8MUX_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - desc CH9MUX"]
    #[inline(always)]
    pub fn ch9mux(&self) -> CH9MUX_R {
        CH9MUX_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - desc CH10MUX"]
    #[inline(always)]
    pub fn ch10mux(&self) -> CH10MUX_R {
        CH10MUX_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - desc CH11MUX"]
    #[inline(always)]
    pub fn ch11mux(&self) -> CH11MUX_R {
        CH11MUX_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc CH6MUX"]
    #[inline(always)]
    pub fn ch6mux(&mut self) -> CH6MUX_W<0> {
        CH6MUX_W::new(self)
    }
    #[doc = "Bits 5:9 - desc CH7MUX"]
    #[inline(always)]
    pub fn ch7mux(&mut self) -> CH7MUX_W<5> {
        CH7MUX_W::new(self)
    }
    #[doc = "Bits 10:14 - desc CH8MUX"]
    #[inline(always)]
    pub fn ch8mux(&mut self) -> CH8MUX_W<10> {
        CH8MUX_W::new(self)
    }
    #[doc = "Bits 15:19 - desc CH9MUX"]
    #[inline(always)]
    pub fn ch9mux(&mut self) -> CH9MUX_W<15> {
        CH9MUX_W::new(self)
    }
    #[doc = "Bits 20:24 - desc CH10MUX"]
    #[inline(always)]
    pub fn ch10mux(&mut self) -> CH10MUX_W<20> {
        CH10MUX_W::new(self)
    }
    #[doc = "Bits 25:29 - desc CH11MUX"]
    #[inline(always)]
    pub fn ch11mux(&mut self) -> CH11MUX_W<25> {
        CH11MUX_W::new(self)
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
#[doc = "desc SQR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr1](index.html) module"]
pub struct SQR1_SPEC;
impl crate::RegisterSpec for SQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr1::R](R) reader structure"]
impl crate::Readable for SQR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr1::W](W) writer structure"]
impl crate::Writable for SQR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
