#[doc = "Register `TIMCPS` reader"]
pub struct R(crate::R<TIMCPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMCPS` writer"]
pub struct W(crate::W<TIMCPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCPS_SPEC>;
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
impl From<crate::W<TIMCPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM0_CA` reader - desc TIM0_CA"]
pub type TIM0_CA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM0_CA` writer - desc TIM0_CA"]
pub type TIM0_CA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCPS_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM1_CA` reader - desc TIM1_CA"]
pub type TIM1_CA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_CA` writer - desc TIM1_CA"]
pub type TIM1_CA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCPS_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM2_CA` reader - desc TIM2_CA"]
pub type TIM2_CA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM2_CA` writer - desc TIM2_CA"]
pub type TIM2_CA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCPS_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM3_CA` reader - desc TIM3_CA"]
pub type TIM3_CA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3_CA` writer - desc TIM3_CA"]
pub type TIM3_CA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCPS_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM3_CB` reader - desc TIM3_CB"]
pub type TIM3_CB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3_CB` writer - desc TIM3_CB"]
pub type TIM3_CB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCPS_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCPS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc TIM0_CA"]
    #[inline(always)]
    pub fn tim0_ca(&self) -> TIM0_CA_R {
        TIM0_CA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc TIM1_CA"]
    #[inline(always)]
    pub fn tim1_ca(&self) -> TIM1_CA_R {
        TIM1_CA_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - desc TIM2_CA"]
    #[inline(always)]
    pub fn tim2_ca(&self) -> TIM2_CA_R {
        TIM2_CA_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - desc TIM3_CA"]
    #[inline(always)]
    pub fn tim3_ca(&self) -> TIM3_CA_R {
        TIM3_CA_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc TIM3_CB"]
    #[inline(always)]
    pub fn tim3_cb(&self) -> TIM3_CB_R {
        TIM3_CB_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc TIM0_CA"]
    #[inline(always)]
    pub fn tim0_ca(&mut self) -> TIM0_CA_W<0> {
        TIM0_CA_W::new(self)
    }
    #[doc = "Bits 3:5 - desc TIM1_CA"]
    #[inline(always)]
    pub fn tim1_ca(&mut self) -> TIM1_CA_W<3> {
        TIM1_CA_W::new(self)
    }
    #[doc = "Bits 6:8 - desc TIM2_CA"]
    #[inline(always)]
    pub fn tim2_ca(&mut self) -> TIM2_CA_W<6> {
        TIM2_CA_W::new(self)
    }
    #[doc = "Bits 9:11 - desc TIM3_CA"]
    #[inline(always)]
    pub fn tim3_ca(&mut self) -> TIM3_CA_W<9> {
        TIM3_CA_W::new(self)
    }
    #[doc = "Bits 12:14 - desc TIM3_CB"]
    #[inline(always)]
    pub fn tim3_cb(&mut self) -> TIM3_CB_W<12> {
        TIM3_CB_W::new(self)
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
#[doc = "desc TIMCPS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcps](index.html) module"]
pub struct TIMCPS_SPEC;
impl crate::RegisterSpec for TIMCPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timcps::R](R) reader structure"]
impl crate::Readable for TIMCPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timcps::W](W) writer structure"]
impl crate::Writable for TIMCPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMCPS to value 0"]
impl crate::Resettable for TIMCPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
