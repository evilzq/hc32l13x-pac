#[doc = "Register `EXTTRIGGER0` reader"]
pub struct R(crate::R<EXTTRIGGER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTTRIGGER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTTRIGGER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTTRIGGER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTTRIGGER0` writer"]
pub struct W(crate::W<EXTTRIGGER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTTRIGGER0_SPEC>;
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
impl From<crate::W<EXTTRIGGER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTTRIGGER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM0` reader - desc TIM0"]
pub type TIM0_R = crate::BitReader<bool>;
#[doc = "Field `TIM0` writer - desc TIM0"]
pub type TIM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `TIM1` reader - desc TIM1"]
pub type TIM1_R = crate::BitReader<bool>;
#[doc = "Field `TIM1` writer - desc TIM1"]
pub type TIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `TIM2` reader - desc TIM2"]
pub type TIM2_R = crate::BitReader<bool>;
#[doc = "Field `TIM2` writer - desc TIM2"]
pub type TIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `TIM3` reader - desc TIM3"]
pub type TIM3_R = crate::BitReader<bool>;
#[doc = "Field `TIM3` writer - desc TIM3"]
pub type TIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `TIM4` reader - desc TIM4"]
pub type TIM4_R = crate::BitReader<bool>;
#[doc = "Field `TIM4` writer - desc TIM4"]
pub type TIM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `TIM5` reader - desc TIM5"]
pub type TIM5_R = crate::BitReader<bool>;
#[doc = "Field `TIM5` writer - desc TIM5"]
pub type TIM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `TIM6` reader - desc TIM6"]
pub type TIM6_R = crate::BitReader<bool>;
#[doc = "Field `TIM6` writer - desc TIM6"]
pub type TIM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `UART0` reader - desc UART0"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - desc UART0"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `UART1` reader - desc UART1"]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - desc UART1"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `LPUART0` reader - desc LPUART0"]
pub type LPUART0_R = crate::BitReader<bool>;
#[doc = "Field `LPUART0` writer - desc LPUART0"]
pub type LPUART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `LPUART1` reader - desc LPUART1"]
pub type LPUART1_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1` writer - desc LPUART1"]
pub type LPUART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `VC0` reader - desc VC0"]
pub type VC0_R = crate::BitReader<bool>;
#[doc = "Field `VC0` writer - desc VC0"]
pub type VC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `VC1` reader - desc VC1"]
pub type VC1_R = crate::BitReader<bool>;
#[doc = "Field `VC1` writer - desc VC1"]
pub type VC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PCA` reader - desc PCA"]
pub type PCA_R = crate::BitReader<bool>;
#[doc = "Field `PCA` writer - desc PCA"]
pub type PCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `SPI0` reader - desc SPI0"]
pub type SPI0_R = crate::BitReader<bool>;
#[doc = "Field `SPI0` writer - desc SPI0"]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type SPI1_R = crate::BitReader<bool>;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `DMA` reader - desc DMA"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - desc DMA"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PA03` reader - desc PA03"]
pub type PA03_R = crate::BitReader<bool>;
#[doc = "Field `PA03` writer - desc PA03"]
pub type PA03_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PB03` reader - desc PB03"]
pub type PB03_R = crate::BitReader<bool>;
#[doc = "Field `PB03` writer - desc PB03"]
pub type PB03_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PC03` reader - desc PC03"]
pub type PC03_R = crate::BitReader<bool>;
#[doc = "Field `PC03` writer - desc PC03"]
pub type PC03_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PD03` reader - desc PD03"]
pub type PD03_R = crate::BitReader<bool>;
#[doc = "Field `PD03` writer - desc PD03"]
pub type PD03_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PA07` reader - desc PA07"]
pub type PA07_R = crate::BitReader<bool>;
#[doc = "Field `PA07` writer - desc PA07"]
pub type PA07_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PB07` reader - desc PB07"]
pub type PB07_R = crate::BitReader<bool>;
#[doc = "Field `PB07` writer - desc PB07"]
pub type PB07_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PC07` reader - desc PC07"]
pub type PC07_R = crate::BitReader<bool>;
#[doc = "Field `PC07` writer - desc PC07"]
pub type PC07_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PD07` reader - desc PD07"]
pub type PD07_R = crate::BitReader<bool>;
#[doc = "Field `PD07` writer - desc PD07"]
pub type PD07_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PA11` reader - desc PA11"]
pub type PA11_R = crate::BitReader<bool>;
#[doc = "Field `PA11` writer - desc PA11"]
pub type PA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PB11` reader - desc PB11"]
pub type PB11_R = crate::BitReader<bool>;
#[doc = "Field `PB11` writer - desc PB11"]
pub type PB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PC11` reader - desc PC11"]
pub type PC11_R = crate::BitReader<bool>;
#[doc = "Field `PC11` writer - desc PC11"]
pub type PC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PA15` reader - desc PA15"]
pub type PA15_R = crate::BitReader<bool>;
#[doc = "Field `PA15` writer - desc PA15"]
pub type PA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PB15` reader - desc PB15"]
pub type PB15_R = crate::BitReader<bool>;
#[doc = "Field `PB15` writer - desc PB15"]
pub type PB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
#[doc = "Field `PC15` reader - desc PC15"]
pub type PC15_R = crate::BitReader<bool>;
#[doc = "Field `PC15` writer - desc PC15"]
pub type PC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTTRIGGER0_SPEC, bool, O>;
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
    #[doc = "Bit 3 - desc TIM3"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 7 - desc UART0"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc LPUART0"]
    #[inline(always)]
    pub fn lpuart0(&self) -> LPUART0_R {
        LPUART0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc LPUART1"]
    #[inline(always)]
    pub fn lpuart1(&self) -> LPUART1_R {
        LPUART1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc VC0"]
    #[inline(always)]
    pub fn vc0(&self) -> VC0_R {
        VC0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc VC1"]
    #[inline(always)]
    pub fn vc1(&self) -> VC1_R {
        VC1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PCA"]
    #[inline(always)]
    pub fn pca(&self) -> PCA_R {
        PCA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc PA03"]
    #[inline(always)]
    pub fn pa03(&self) -> PA03_R {
        PA03_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc PB03"]
    #[inline(always)]
    pub fn pb03(&self) -> PB03_R {
        PB03_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc PC03"]
    #[inline(always)]
    pub fn pc03(&self) -> PC03_R {
        PC03_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc PD03"]
    #[inline(always)]
    pub fn pd03(&self) -> PD03_R {
        PD03_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc PA07"]
    #[inline(always)]
    pub fn pa07(&self) -> PA07_R {
        PA07_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc PB07"]
    #[inline(always)]
    pub fn pb07(&self) -> PB07_R {
        PB07_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc PC07"]
    #[inline(always)]
    pub fn pc07(&self) -> PC07_R {
        PC07_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc PD07"]
    #[inline(always)]
    pub fn pd07(&self) -> PD07_R {
        PD07_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc PA11"]
    #[inline(always)]
    pub fn pa11(&self) -> PA11_R {
        PA11_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc PB11"]
    #[inline(always)]
    pub fn pb11(&self) -> PB11_R {
        PB11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc PC11"]
    #[inline(always)]
    pub fn pc11(&self) -> PC11_R {
        PC11_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc PA15"]
    #[inline(always)]
    pub fn pa15(&self) -> PA15_R {
        PA15_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc PB15"]
    #[inline(always)]
    pub fn pb15(&self) -> PB15_R {
        PB15_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc PC15"]
    #[inline(always)]
    pub fn pc15(&self) -> PC15_R {
        PC15_R::new(((self.bits >> 31) & 1) != 0)
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
    #[doc = "Bit 3 - desc TIM3"]
    #[inline(always)]
    pub fn tim3(&mut self) -> TIM3_W<3> {
        TIM3_W::new(self)
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
    #[doc = "Bit 7 - desc UART0"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W<7> {
        UART0_W::new(self)
    }
    #[doc = "Bit 8 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W<8> {
        UART1_W::new(self)
    }
    #[doc = "Bit 9 - desc LPUART0"]
    #[inline(always)]
    pub fn lpuart0(&mut self) -> LPUART0_W<9> {
        LPUART0_W::new(self)
    }
    #[doc = "Bit 10 - desc LPUART1"]
    #[inline(always)]
    pub fn lpuart1(&mut self) -> LPUART1_W<10> {
        LPUART1_W::new(self)
    }
    #[doc = "Bit 11 - desc VC0"]
    #[inline(always)]
    pub fn vc0(&mut self) -> VC0_W<11> {
        VC0_W::new(self)
    }
    #[doc = "Bit 12 - desc VC1"]
    #[inline(always)]
    pub fn vc1(&mut self) -> VC1_W<12> {
        VC1_W::new(self)
    }
    #[doc = "Bit 13 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<13> {
        RTC_W::new(self)
    }
    #[doc = "Bit 14 - desc PCA"]
    #[inline(always)]
    pub fn pca(&mut self) -> PCA_W<14> {
        PCA_W::new(self)
    }
    #[doc = "Bit 15 - desc SPI0"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W<15> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 16 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W<16> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 17 - desc DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<17> {
        DMA_W::new(self)
    }
    #[doc = "Bit 18 - desc PA03"]
    #[inline(always)]
    pub fn pa03(&mut self) -> PA03_W<18> {
        PA03_W::new(self)
    }
    #[doc = "Bit 19 - desc PB03"]
    #[inline(always)]
    pub fn pb03(&mut self) -> PB03_W<19> {
        PB03_W::new(self)
    }
    #[doc = "Bit 20 - desc PC03"]
    #[inline(always)]
    pub fn pc03(&mut self) -> PC03_W<20> {
        PC03_W::new(self)
    }
    #[doc = "Bit 21 - desc PD03"]
    #[inline(always)]
    pub fn pd03(&mut self) -> PD03_W<21> {
        PD03_W::new(self)
    }
    #[doc = "Bit 22 - desc PA07"]
    #[inline(always)]
    pub fn pa07(&mut self) -> PA07_W<22> {
        PA07_W::new(self)
    }
    #[doc = "Bit 23 - desc PB07"]
    #[inline(always)]
    pub fn pb07(&mut self) -> PB07_W<23> {
        PB07_W::new(self)
    }
    #[doc = "Bit 24 - desc PC07"]
    #[inline(always)]
    pub fn pc07(&mut self) -> PC07_W<24> {
        PC07_W::new(self)
    }
    #[doc = "Bit 25 - desc PD07"]
    #[inline(always)]
    pub fn pd07(&mut self) -> PD07_W<25> {
        PD07_W::new(self)
    }
    #[doc = "Bit 26 - desc PA11"]
    #[inline(always)]
    pub fn pa11(&mut self) -> PA11_W<26> {
        PA11_W::new(self)
    }
    #[doc = "Bit 27 - desc PB11"]
    #[inline(always)]
    pub fn pb11(&mut self) -> PB11_W<27> {
        PB11_W::new(self)
    }
    #[doc = "Bit 28 - desc PC11"]
    #[inline(always)]
    pub fn pc11(&mut self) -> PC11_W<28> {
        PC11_W::new(self)
    }
    #[doc = "Bit 29 - desc PA15"]
    #[inline(always)]
    pub fn pa15(&mut self) -> PA15_W<29> {
        PA15_W::new(self)
    }
    #[doc = "Bit 30 - desc PB15"]
    #[inline(always)]
    pub fn pb15(&mut self) -> PB15_W<30> {
        PB15_W::new(self)
    }
    #[doc = "Bit 31 - desc PC15"]
    #[inline(always)]
    pub fn pc15(&mut self) -> PC15_W<31> {
        PC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc EXTTRIGGER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exttrigger0](index.html) module"]
pub struct EXTTRIGGER0_SPEC;
impl crate::RegisterSpec for EXTTRIGGER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exttrigger0::R](R) reader structure"]
impl crate::Readable for EXTTRIGGER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exttrigger0::W](W) writer structure"]
impl crate::Writable for EXTTRIGGER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTTRIGGER0 to value 0"]
impl crate::Resettable for EXTTRIGGER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
