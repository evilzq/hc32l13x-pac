#[doc = "Register `VC0_OUT_CFG` reader"]
pub struct R(crate::R<VC0_OUT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VC0_OUT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VC0_OUT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VC0_OUT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VC0_OUT_CFG` writer"]
pub struct W(crate::W<VC0_OUT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VC0_OUT_CFG_SPEC>;
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
impl From<crate::W<VC0_OUT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VC0_OUT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV_TIMER` reader - desc INV_TIMER"]
pub type INV_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `INV_TIMER` writer - desc INV_TIMER"]
pub type INV_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `TIM0RCLR` reader - desc TIM0RCLR"]
pub type TIM0RCLR_R = crate::BitReader<bool>;
#[doc = "Field `TIM0RCLR` writer - desc TIM0RCLR"]
pub type TIM0RCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `TIM1RCLR` reader - desc TIM1RCLR"]
pub type TIM1RCLR_R = crate::BitReader<bool>;
#[doc = "Field `TIM1RCLR` writer - desc TIM1RCLR"]
pub type TIM1RCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `TIM2RCLR` reader - desc TIM2RCLR"]
pub type TIM2RCLR_R = crate::BitReader<bool>;
#[doc = "Field `TIM2RCLR` writer - desc TIM2RCLR"]
pub type TIM2RCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `TIM3RCLR` reader - desc TIM3RCLR"]
pub type TIM3RCLR_R = crate::BitReader<bool>;
#[doc = "Field `TIM3RCLR` writer - desc TIM3RCLR"]
pub type TIM3RCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `TIMBK` reader - desc TIMBK"]
pub type TIMBK_R = crate::BitReader<bool>;
#[doc = "Field `TIMBK` writer - desc TIMBK"]
pub type TIMBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `INV_TIM4` reader - desc INV_TIM4"]
pub type INV_TIM4_R = crate::BitReader<bool>;
#[doc = "Field `INV_TIM4` writer - desc INV_TIM4"]
pub type INV_TIM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `TIM4` reader - desc TIM4"]
pub type TIM4_R = crate::BitReader<bool>;
#[doc = "Field `TIM4` writer - desc TIM4"]
pub type TIM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `INV_TIM5` reader - desc INV_TIM5"]
pub type INV_TIM5_R = crate::BitReader<bool>;
#[doc = "Field `INV_TIM5` writer - desc INV_TIM5"]
pub type INV_TIM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `TIM5` reader - desc TIM5"]
pub type TIM5_R = crate::BitReader<bool>;
#[doc = "Field `TIM5` writer - desc TIM5"]
pub type TIM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `INV_TIM6` reader - desc INV_TIM6"]
pub type INV_TIM6_R = crate::BitReader<bool>;
#[doc = "Field `INV_TIM6` writer - desc INV_TIM6"]
pub type INV_TIM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `TIM6` reader - desc TIM6"]
pub type TIM6_R = crate::BitReader<bool>;
#[doc = "Field `TIM6` writer - desc TIM6"]
pub type TIM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `BRAKE` reader - desc BRAKE"]
pub type BRAKE_R = crate::BitReader<bool>;
#[doc = "Field `BRAKE` writer - desc BRAKE"]
pub type BRAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC0_OUT_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc INV_TIMER"]
    #[inline(always)]
    pub fn inv_timer(&self) -> INV_TIMER_R {
        INV_TIMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TIM0RCLR"]
    #[inline(always)]
    pub fn tim0rclr(&self) -> TIM0RCLR_R {
        TIM0RCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TIM1RCLR"]
    #[inline(always)]
    pub fn tim1rclr(&self) -> TIM1RCLR_R {
        TIM1RCLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TIM2RCLR"]
    #[inline(always)]
    pub fn tim2rclr(&self) -> TIM2RCLR_R {
        TIM2RCLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TIM3RCLR"]
    #[inline(always)]
    pub fn tim3rclr(&self) -> TIM3RCLR_R {
        TIM3RCLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TIMBK"]
    #[inline(always)]
    pub fn timbk(&self) -> TIMBK_R {
        TIMBK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - desc INV_TIM4"]
    #[inline(always)]
    pub fn inv_tim4(&self) -> INV_TIM4_R {
        INV_TIM4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TIM4"]
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc INV_TIM5"]
    #[inline(always)]
    pub fn inv_tim5(&self) -> INV_TIM5_R {
        INV_TIM5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TIM5"]
    #[inline(always)]
    pub fn tim5(&self) -> TIM5_R {
        TIM5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc INV_TIM6"]
    #[inline(always)]
    pub fn inv_tim6(&self) -> INV_TIM6_R {
        INV_TIM6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TIM6"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc BRAKE"]
    #[inline(always)]
    pub fn brake(&self) -> BRAKE_R {
        BRAKE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc INV_TIMER"]
    #[inline(always)]
    pub fn inv_timer(&mut self) -> INV_TIMER_W<0> {
        INV_TIMER_W::new(self)
    }
    #[doc = "Bit 1 - desc TIM0RCLR"]
    #[inline(always)]
    pub fn tim0rclr(&mut self) -> TIM0RCLR_W<1> {
        TIM0RCLR_W::new(self)
    }
    #[doc = "Bit 2 - desc TIM1RCLR"]
    #[inline(always)]
    pub fn tim1rclr(&mut self) -> TIM1RCLR_W<2> {
        TIM1RCLR_W::new(self)
    }
    #[doc = "Bit 3 - desc TIM2RCLR"]
    #[inline(always)]
    pub fn tim2rclr(&mut self) -> TIM2RCLR_W<3> {
        TIM2RCLR_W::new(self)
    }
    #[doc = "Bit 4 - desc TIM3RCLR"]
    #[inline(always)]
    pub fn tim3rclr(&mut self) -> TIM3RCLR_W<4> {
        TIM3RCLR_W::new(self)
    }
    #[doc = "Bit 5 - desc TIMBK"]
    #[inline(always)]
    pub fn timbk(&mut self) -> TIMBK_W<5> {
        TIMBK_W::new(self)
    }
    #[doc = "Bit 9 - desc INV_TIM4"]
    #[inline(always)]
    pub fn inv_tim4(&mut self) -> INV_TIM4_W<9> {
        INV_TIM4_W::new(self)
    }
    #[doc = "Bit 10 - desc TIM4"]
    #[inline(always)]
    pub fn tim4(&mut self) -> TIM4_W<10> {
        TIM4_W::new(self)
    }
    #[doc = "Bit 11 - desc INV_TIM5"]
    #[inline(always)]
    pub fn inv_tim5(&mut self) -> INV_TIM5_W<11> {
        INV_TIM5_W::new(self)
    }
    #[doc = "Bit 12 - desc TIM5"]
    #[inline(always)]
    pub fn tim5(&mut self) -> TIM5_W<12> {
        TIM5_W::new(self)
    }
    #[doc = "Bit 13 - desc INV_TIM6"]
    #[inline(always)]
    pub fn inv_tim6(&mut self) -> INV_TIM6_W<13> {
        INV_TIM6_W::new(self)
    }
    #[doc = "Bit 14 - desc TIM6"]
    #[inline(always)]
    pub fn tim6(&mut self) -> TIM6_W<14> {
        TIM6_W::new(self)
    }
    #[doc = "Bit 15 - desc BRAKE"]
    #[inline(always)]
    pub fn brake(&mut self) -> BRAKE_W<15> {
        BRAKE_W::new(self)
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
#[doc = "desc VC0_OUT_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vc0_out_cfg](index.html) module"]
pub struct VC0_OUT_CFG_SPEC;
impl crate::RegisterSpec for VC0_OUT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vc0_out_cfg::R](R) reader structure"]
impl crate::Readable for VC0_OUT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vc0_out_cfg::W](W) writer structure"]
impl crate::Writable for VC0_OUT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VC0_OUT_CFG to value 0"]
impl crate::Resettable for VC0_OUT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
