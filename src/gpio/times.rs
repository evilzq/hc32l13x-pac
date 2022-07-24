#[doc = "Register `TIMES` reader"]
pub struct R(crate::R<TIMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMES` writer"]
pub struct W(crate::W<TIMES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMES_SPEC>;
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
impl From<crate::W<TIMES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM0_E` reader - desc TIM0_E"]
pub type TIM0_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM0_E` writer - desc TIM0_E"]
pub type TIM0_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMES_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM1_E` reader - desc TIM1_E"]
pub type TIM1_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_E` writer - desc TIM1_E"]
pub type TIM1_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMES_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM2_E` reader - desc TIM2_E"]
pub type TIM2_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM2_E` writer - desc TIM2_E"]
pub type TIM2_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMES_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIM3_E` reader - desc TIM3_E"]
pub type TIM3_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3_E` writer - desc TIM3_E"]
pub type TIM3_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMES_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPTIM_E` reader - desc LPTIM_E"]
pub type LPTIM_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPTIM_E` writer - desc LPTIM_E"]
pub type LPTIM_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMES_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMES_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc TIM0_E"]
    #[inline(always)]
    pub fn tim0_e(&self) -> TIM0_E_R {
        TIM0_E_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc TIM1_E"]
    #[inline(always)]
    pub fn tim1_e(&self) -> TIM1_E_R {
        TIM1_E_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - desc TIM2_E"]
    #[inline(always)]
    pub fn tim2_e(&self) -> TIM2_E_R {
        TIM2_E_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - desc TIM3_E"]
    #[inline(always)]
    pub fn tim3_e(&self) -> TIM3_E_R {
        TIM3_E_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc LPTIM_E"]
    #[inline(always)]
    pub fn lptim_e(&self) -> LPTIM_E_R {
        LPTIM_E_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc TIM0_E"]
    #[inline(always)]
    pub fn tim0_e(&mut self) -> TIM0_E_W<0> {
        TIM0_E_W::new(self)
    }
    #[doc = "Bits 3:5 - desc TIM1_E"]
    #[inline(always)]
    pub fn tim1_e(&mut self) -> TIM1_E_W<3> {
        TIM1_E_W::new(self)
    }
    #[doc = "Bits 6:8 - desc TIM2_E"]
    #[inline(always)]
    pub fn tim2_e(&mut self) -> TIM2_E_W<6> {
        TIM2_E_W::new(self)
    }
    #[doc = "Bits 9:11 - desc TIM3_E"]
    #[inline(always)]
    pub fn tim3_e(&mut self) -> TIM3_E_W<9> {
        TIM3_E_W::new(self)
    }
    #[doc = "Bits 12:14 - desc LPTIM_E"]
    #[inline(always)]
    pub fn lptim_e(&mut self) -> LPTIM_E_W<12> {
        LPTIM_E_W::new(self)
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
#[doc = "desc TIMES\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [times](index.html) module"]
pub struct TIMES_SPEC;
impl crate::RegisterSpec for TIMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [times::R](R) reader structure"]
impl crate::Readable for TIMES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [times::W](W) writer structure"]
impl crate::Writable for TIMES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMES to value 0"]
impl crate::Resettable for TIMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
