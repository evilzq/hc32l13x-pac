#[doc = "Register `JQR` reader"]
pub struct R(crate::R<JQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JQR` writer"]
pub struct W(crate::W<JQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JQR_SPEC>;
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
impl From<crate::W<JQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0MUX` reader - desc CH0MUX"]
pub type CH0MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0MUX` writer - desc CH0MUX"]
pub type CH0MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH1MUX` reader - desc CH1MUX"]
pub type CH1MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1MUX` writer - desc CH1MUX"]
pub type CH1MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH2MUX` reader - desc CH2MUX"]
pub type CH2MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2MUX` writer - desc CH2MUX"]
pub type CH2MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CH3MUX` reader - desc CH3MUX"]
pub type CH3MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3MUX` writer - desc CH3MUX"]
pub type CH3MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JQR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, JQR_SPEC, bool, O>;
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
    #[doc = "Bits 20:21 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 20) & 3) as u8)
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
    #[doc = "Bits 20:21 - desc CNT"]
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
#[doc = "desc JQR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jqr](index.html) module"]
pub struct JQR_SPEC;
impl crate::RegisterSpec for JQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jqr::R](R) reader structure"]
impl crate::Readable for JQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jqr::W](W) writer structure"]
impl crate::Writable for JQR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JQR to value 0"]
impl crate::Resettable for JQR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
