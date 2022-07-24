#[doc = "Register `PERI_RESET` reader"]
pub struct R(crate::R<PERI_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_RESET` writer"]
pub struct W(crate::W<PERI_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_RESET_SPEC>;
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
impl From<crate::W<PERI_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0` reader - desc UART0"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - desc UART0"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `UART1` reader - desc UART1"]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - desc UART1"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `LPUART0` reader - desc LPUART0"]
pub type LPUART0_R = crate::BitReader<bool>;
#[doc = "Field `LPUART0` writer - desc LPUART0"]
pub type LPUART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `LPUART1` reader - desc LPUART1"]
pub type LPUART1_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1` writer - desc LPUART1"]
pub type LPUART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `I2C0` reader - desc I2C0"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - desc I2C0"]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `I2C1` reader - desc I2C1"]
pub type I2C1_R = crate::BitReader<bool>;
#[doc = "Field `I2C1` writer - desc I2C1"]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `SPI0` reader - desc SPI0"]
pub type SPI0_R = crate::BitReader<bool>;
#[doc = "Field `SPI0` writer - desc SPI0"]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type SPI1_R = crate::BitReader<bool>;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `BASETIM` reader - desc BASETIM"]
pub type BASETIM_R = crate::BitReader<bool>;
#[doc = "Field `BASETIM` writer - desc BASETIM"]
pub type BASETIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `LPTIM` reader - desc LPTIM"]
pub type LPTIM_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM` writer - desc LPTIM"]
pub type LPTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `ADVTIM` reader - desc ADVTIM"]
pub type ADVTIM_R = crate::BitReader<bool>;
#[doc = "Field `ADVTIM` writer - desc ADVTIM"]
pub type ADVTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `TIM3` reader - desc TIM3"]
pub type TIM3_R = crate::BitReader<bool>;
#[doc = "Field `TIM3` writer - desc TIM3"]
pub type TIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `OPA` reader - desc OPA"]
pub type OPA_R = crate::BitReader<bool>;
#[doc = "Field `OPA` writer - desc OPA"]
pub type OPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `PCA` reader - desc PCA"]
pub type PCA_R = crate::BitReader<bool>;
#[doc = "Field `PCA` writer - desc PCA"]
pub type PCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `ADC` reader - desc ADC"]
pub type ADC_R = crate::BitReader<bool>;
#[doc = "Field `ADC` writer - desc ADC"]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `VC` reader - desc VC"]
pub type VC_R = crate::BitReader<bool>;
#[doc = "Field `VC` writer - desc VC"]
pub type VC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `RNG` reader - desc RNG"]
pub type RNG_R = crate::BitReader<bool>;
#[doc = "Field `RNG` writer - desc RNG"]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `PCNT` reader - desc PCNT"]
pub type PCNT_R = crate::BitReader<bool>;
#[doc = "Field `PCNT` writer - desc PCNT"]
pub type PCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TRIM_R = crate::BitReader<bool>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `LCD` reader - desc LCD"]
pub type LCD_R = crate::BitReader<bool>;
#[doc = "Field `LCD` writer - desc LCD"]
pub type LCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `TICK` reader - desc TICK"]
pub type TICK_R = crate::BitReader<bool>;
#[doc = "Field `TICK` writer - desc TICK"]
pub type TICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `SWD` reader - desc SWD"]
pub type SWD_R = crate::BitReader<bool>;
#[doc = "Field `SWD` writer - desc SWD"]
pub type SWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `CRC` reader - desc CRC"]
pub type CRC_R = crate::BitReader<bool>;
#[doc = "Field `CRC` writer - desc CRC"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `AES` reader - desc AES"]
pub type AES_R = crate::BitReader<bool>;
#[doc = "Field `AES` writer - desc AES"]
pub type AES_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `GPIO` reader - desc GPIO"]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - desc GPIO"]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `DMA` reader - desc DMA"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - desc DMA"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DIV_R = crate::BitReader<bool>;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_RESET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc UART0"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc LPUART0"]
    #[inline(always)]
    pub fn lpuart0(&self) -> LPUART0_R {
        LPUART0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc LPUART1"]
    #[inline(always)]
    pub fn lpuart1(&self) -> LPUART1_R {
        LPUART1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc I2C0"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BASETIM"]
    #[inline(always)]
    pub fn basetim(&self) -> BASETIM_R {
        BASETIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc LPTIM"]
    #[inline(always)]
    pub fn lptim(&self) -> LPTIM_R {
        LPTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ADVTIM"]
    #[inline(always)]
    pub fn advtim(&self) -> ADVTIM_R {
        ADVTIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc TIM3"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - desc OPA"]
    #[inline(always)]
    pub fn opa(&self) -> OPA_R {
        OPA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PCA"]
    #[inline(always)]
    pub fn pca(&self) -> PCA_R {
        PCA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - desc ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc VC"]
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc PCNT"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc LCD"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - desc TICK"]
    #[inline(always)]
    pub fn tick(&self) -> TICK_R {
        TICK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc SWD"]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc AES"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc GPIO"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UART0"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W<0> {
        UART0_W::new(self)
    }
    #[doc = "Bit 1 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W<1> {
        UART1_W::new(self)
    }
    #[doc = "Bit 2 - desc LPUART0"]
    #[inline(always)]
    pub fn lpuart0(&mut self) -> LPUART0_W<2> {
        LPUART0_W::new(self)
    }
    #[doc = "Bit 3 - desc LPUART1"]
    #[inline(always)]
    pub fn lpuart1(&mut self) -> LPUART1_W<3> {
        LPUART1_W::new(self)
    }
    #[doc = "Bit 4 - desc I2C0"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W<4> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 5 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W<5> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 6 - desc SPI0"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W<6> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 7 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W<7> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 8 - desc BASETIM"]
    #[inline(always)]
    pub fn basetim(&mut self) -> BASETIM_W<8> {
        BASETIM_W::new(self)
    }
    #[doc = "Bit 9 - desc LPTIM"]
    #[inline(always)]
    pub fn lptim(&mut self) -> LPTIM_W<9> {
        LPTIM_W::new(self)
    }
    #[doc = "Bit 10 - desc ADVTIM"]
    #[inline(always)]
    pub fn advtim(&mut self) -> ADVTIM_W<10> {
        ADVTIM_W::new(self)
    }
    #[doc = "Bit 11 - desc TIM3"]
    #[inline(always)]
    pub fn tim3(&mut self) -> TIM3_W<11> {
        TIM3_W::new(self)
    }
    #[doc = "Bit 13 - desc OPA"]
    #[inline(always)]
    pub fn opa(&mut self) -> OPA_W<13> {
        OPA_W::new(self)
    }
    #[doc = "Bit 14 - desc PCA"]
    #[inline(always)]
    pub fn pca(&mut self) -> PCA_W<14> {
        PCA_W::new(self)
    }
    #[doc = "Bit 16 - desc ADC"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W<16> {
        ADC_W::new(self)
    }
    #[doc = "Bit 17 - desc VC"]
    #[inline(always)]
    pub fn vc(&mut self) -> VC_W<17> {
        VC_W::new(self)
    }
    #[doc = "Bit 18 - desc RNG"]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W<18> {
        RNG_W::new(self)
    }
    #[doc = "Bit 19 - desc PCNT"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PCNT_W<19> {
        PCNT_W::new(self)
    }
    #[doc = "Bit 20 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<20> {
        RTC_W::new(self)
    }
    #[doc = "Bit 21 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<21> {
        TRIM_W::new(self)
    }
    #[doc = "Bit 22 - desc LCD"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W<22> {
        LCD_W::new(self)
    }
    #[doc = "Bit 24 - desc TICK"]
    #[inline(always)]
    pub fn tick(&mut self) -> TICK_W<24> {
        TICK_W::new(self)
    }
    #[doc = "Bit 25 - desc SWD"]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W<25> {
        SWD_W::new(self)
    }
    #[doc = "Bit 26 - desc CRC"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W<26> {
        CRC_W::new(self)
    }
    #[doc = "Bit 27 - desc AES"]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W<27> {
        AES_W::new(self)
    }
    #[doc = "Bit 28 - desc GPIO"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W<28> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 29 - desc DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<29> {
        DMA_W::new(self)
    }
    #[doc = "Bit 30 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<30> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PERI_RESET\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_reset](index.html) module"]
pub struct PERI_RESET_SPEC;
impl crate::RegisterSpec for PERI_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_reset::R](R) reader structure"]
impl crate::Readable for PERI_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_reset::W](W) writer structure"]
impl crate::Writable for PERI_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_RESET to value 0x7f7f_6fff"]
impl crate::Resettable for PERI_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f7f_6fff
    }
}
