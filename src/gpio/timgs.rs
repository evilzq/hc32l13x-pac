#[doc = "Register `TIMGS` reader"]
pub struct R(crate::R<TIMGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMGS` writer"]
pub struct W(crate::W<TIMGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMGS_SPEC>;
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
impl From<crate::W<TIMGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM0_G` reader - desc TIM0_G"]
pub type TIM0_G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM0_G` writer - desc TIM0_G"]
pub type TIM0_G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM1_G` reader - desc TIM1_G"]
pub type TIM1_G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_G` writer - desc TIM1_G"]
pub type TIM1_G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM2_G` reader - desc TIM2_G"]
pub type TIM2_G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM2_G` writer - desc TIM2_G"]
pub type TIM2_G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM3_G` reader - desc TIM3_G"]
pub type TIM3_G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3_G` writer - desc TIM3_G"]
pub type TIM3_G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM_G` reader - desc LPTIM_G"]
pub type LPTIM_G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM_G` writer - desc LPTIM_G"]
pub type LPTIM_G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMGS_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMGS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc TIM0_G"]
    #[inline(always)]
    pub fn tim0_g(&self) -> TIM0_G_R {
        TIM0_G_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc TIM1_G"]
    #[inline(always)]
    pub fn tim1_g(&self) -> TIM1_G_R {
        TIM1_G_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - desc TIM2_G"]
    #[inline(always)]
    pub fn tim2_g(&self) -> TIM2_G_R {
        TIM2_G_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - desc TIM3_G"]
    #[inline(always)]
    pub fn tim3_g(&self) -> TIM3_G_R {
        TIM3_G_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc LPTIM_G"]
    #[inline(always)]
    pub fn lptim_g(&self) -> LPTIM_G_R {
        LPTIM_G_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc TIM0_G"]
    #[inline(always)]
    pub fn tim0_g(&mut self) -> TIM0_G_W<0> {
        TIM0_G_W::new(self)
    }
    #[doc = "Bits 3:5 - desc TIM1_G"]
    #[inline(always)]
    pub fn tim1_g(&mut self) -> TIM1_G_W<3> {
        TIM1_G_W::new(self)
    }
    #[doc = "Bits 6:8 - desc TIM2_G"]
    #[inline(always)]
    pub fn tim2_g(&mut self) -> TIM2_G_W<6> {
        TIM2_G_W::new(self)
    }
    #[doc = "Bits 9:11 - desc TIM3_G"]
    #[inline(always)]
    pub fn tim3_g(&mut self) -> TIM3_G_W<9> {
        TIM3_G_W::new(self)
    }
    #[doc = "Bits 12:14 - desc LPTIM_G"]
    #[inline(always)]
    pub fn lptim_g(&mut self) -> LPTIM_G_W<12> {
        LPTIM_G_W::new(self)
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
#[doc = "desc TIMGS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timgs](index.html) module"]
pub struct TIMGS_SPEC;
impl crate::RegisterSpec for TIMGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timgs::R](R) reader structure"]
impl crate::Readable for TIMGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timgs::W](W) writer structure"]
impl crate::Writable for TIMGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMGS to value 0"]
impl crate::Resettable for TIMGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
