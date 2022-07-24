#[doc = "Register `DEBUG_ACTIVE` reader"]
pub struct R(crate::R<DEBUG_ACTIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_ACTIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_ACTIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_ACTIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_ACTIVE` writer"]
pub struct W(crate::W<DEBUG_ACTIVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_ACTIVE_SPEC>;
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
impl From<crate::W<DEBUG_ACTIVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_ACTIVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM0` reader - desc TIM0"]
pub type TIM0_R = crate::BitReader<bool>;
#[doc = "Field `TIM0` writer - desc TIM0"]
pub type TIM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `TIM1` reader - desc TIM1"]
pub type TIM1_R = crate::BitReader<bool>;
#[doc = "Field `TIM1` writer - desc TIM1"]
pub type TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `TIM2` reader - desc TIM2"]
pub type TIM2_R = crate::BitReader<bool>;
#[doc = "Field `TIM2` writer - desc TIM2"]
pub type TIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `LPTIM` reader - desc LPTIM"]
pub type LPTIM_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM` writer - desc LPTIM"]
pub type LPTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `TIM4` reader - desc TIM4"]
pub type TIM4_R = crate::BitReader<bool>;
#[doc = "Field `TIM4` writer - desc TIM4"]
pub type TIM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `TIM5` reader - desc TIM5"]
pub type TIM5_R = crate::BitReader<bool>;
#[doc = "Field `TIM5` writer - desc TIM5"]
pub type TIM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `TIM6` reader - desc TIM6"]
pub type TIM6_R = crate::BitReader<bool>;
#[doc = "Field `TIM6` writer - desc TIM6"]
pub type TIM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `PCA` reader - desc PCA"]
pub type PCA_R = crate::BitReader<bool>;
#[doc = "Field `PCA` writer - desc PCA"]
pub type PCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `WDT` reader - desc WDT"]
pub type WDT_R = crate::BitReader<bool>;
#[doc = "Field `WDT` writer - desc WDT"]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `TIM3` reader - desc TIM3"]
pub type TIM3_R = crate::BitReader<bool>;
#[doc = "Field `TIM3` writer - desc TIM3"]
pub type TIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_ACTIVE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc TIM0"]
    #[inline(always)]
    pub fn tim0(&self) -> TIM0_R {
        TIM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TIM1"]
    #[inline(always)]
    pub fn tim1(&self) -> TIM1_R {
        TIM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TIM2"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc LPTIM"]
    #[inline(always)]
    pub fn lptim(&self) -> LPTIM_R {
        LPTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TIM4"]
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TIM5"]
    #[inline(always)]
    pub fn tim5(&self) -> TIM5_R {
        TIM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIM6"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PCA"]
    #[inline(always)]
    pub fn pca(&self) -> PCA_R {
        PCA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc WDT"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TIM3"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TIM0"]
    #[inline(always)]
    pub fn tim0(&mut self) -> TIM0_W<0> {
        TIM0_W::new(self)
    }
    #[doc = "Bit 1 - desc TIM1"]
    #[inline(always)]
    pub fn tim1(&mut self) -> TIM1_W<1> {
        TIM1_W::new(self)
    }
    #[doc = "Bit 2 - desc TIM2"]
    #[inline(always)]
    pub fn tim2(&mut self) -> TIM2_W<2> {
        TIM2_W::new(self)
    }
    #[doc = "Bit 3 - desc LPTIM"]
    #[inline(always)]
    pub fn lptim(&mut self) -> LPTIM_W<3> {
        LPTIM_W::new(self)
    }
    #[doc = "Bit 4 - desc TIM4"]
    #[inline(always)]
    pub fn tim4(&mut self) -> TIM4_W<4> {
        TIM4_W::new(self)
    }
    #[doc = "Bit 5 - desc TIM5"]
    #[inline(always)]
    pub fn tim5(&mut self) -> TIM5_W<5> {
        TIM5_W::new(self)
    }
    #[doc = "Bit 6 - desc TIM6"]
    #[inline(always)]
    pub fn tim6(&mut self) -> TIM6_W<6> {
        TIM6_W::new(self)
    }
    #[doc = "Bit 7 - desc PCA"]
    #[inline(always)]
    pub fn pca(&mut self) -> PCA_W<7> {
        PCA_W::new(self)
    }
    #[doc = "Bit 8 - desc WDT"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<8> {
        WDT_W::new(self)
    }
    #[doc = "Bit 9 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<9> {
        RTC_W::new(self)
    }
    #[doc = "Bit 11 - desc TIM3"]
    #[inline(always)]
    pub fn tim3(&mut self) -> TIM3_W<11> {
        TIM3_W::new(self)
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
#[doc = "desc DEBUG_ACTIVE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_active](index.html) module"]
pub struct DEBUG_ACTIVE_SPEC;
impl crate::RegisterSpec for DEBUG_ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_active::R](R) reader structure"]
impl crate::Readable for DEBUG_ACTIVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_active::W](W) writer structure"]
impl crate::Writable for DEBUG_ACTIVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_ACTIVE to value 0x0fff"]
impl crate::Resettable for DEBUG_ACTIVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
