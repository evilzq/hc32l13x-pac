#[doc = "Register `SYSCTRL1` reader"]
pub struct R(crate::R<SYSCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCTRL1` writer"]
pub struct W(crate::W<SYSCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTRL1_SPEC>;
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
impl From<crate::W<SYSCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTH_EN` reader - desc EXTH_EN"]
pub type EXTH_EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTH_EN` writer - desc EXTH_EN"]
pub type EXTH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL1_SPEC, bool, O>;
#[doc = "Field `EXTL_EN` reader - desc EXTL_EN"]
pub type EXTL_EN_R = crate::BitReader<bool>;
#[doc = "Field `EXTL_EN` writer - desc EXTL_EN"]
pub type EXTL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL1_SPEC, bool, O>;
#[doc = "Field `XTL_ALWAYS_ON` reader - desc XTL_ALWAYS_ON"]
pub type XTL_ALWAYS_ON_R = crate::BitReader<bool>;
#[doc = "Field `XTL_ALWAYS_ON` writer - desc XTL_ALWAYS_ON"]
pub type XTL_ALWAYS_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL1_SPEC, bool, O>;
#[doc = "Field `RTC_LPW` reader - desc RTC_LPW"]
pub type RTC_LPW_R = crate::BitReader<bool>;
#[doc = "Field `RTC_LPW` writer - desc RTC_LPW"]
pub type RTC_LPW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL1_SPEC, bool, O>;
#[doc = "Field `LOCKUP_EN` reader - desc LOCKUP_EN"]
pub type LOCKUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP_EN` writer - desc LOCKUP_EN"]
pub type LOCKUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL1_SPEC, bool, O>;
#[doc = "Field `SWD_USE_IO` reader - desc SWD_USE_IO"]
pub type SWD_USE_IO_R = crate::BitReader<bool>;
#[doc = "Field `SWD_USE_IO` writer - desc SWD_USE_IO"]
pub type SWD_USE_IO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL1_SPEC, bool, O>;
#[doc = "Field `RTC_FREQ_ADJUST` reader - desc RTC_FREQ_ADJUST"]
pub type RTC_FREQ_ADJUST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_FREQ_ADJUST` writer - desc RTC_FREQ_ADJUST"]
pub type RTC_FREQ_ADJUST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSCTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - desc EXTH_EN"]
    #[inline(always)]
    pub fn exth_en(&self) -> EXTH_EN_R {
        EXTH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EXTL_EN"]
    #[inline(always)]
    pub fn extl_en(&self) -> EXTL_EN_R {
        EXTL_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc XTL_ALWAYS_ON"]
    #[inline(always)]
    pub fn xtl_always_on(&self) -> XTL_ALWAYS_ON_R {
        XTL_ALWAYS_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RTC_LPW"]
    #[inline(always)]
    pub fn rtc_lpw(&self) -> RTC_LPW_R {
        RTC_LPW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LOCKUP_EN"]
    #[inline(always)]
    pub fn lockup_en(&self) -> LOCKUP_EN_R {
        LOCKUP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SWD_USE_IO"]
    #[inline(always)]
    pub fn swd_use_io(&self) -> SWD_USE_IO_R {
        SWD_USE_IO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - desc RTC_FREQ_ADJUST"]
    #[inline(always)]
    pub fn rtc_freq_adjust(&self) -> RTC_FREQ_ADJUST_R {
        RTC_FREQ_ADJUST_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc EXTH_EN"]
    #[inline(always)]
    pub fn exth_en(&mut self) -> EXTH_EN_W<1> {
        EXTH_EN_W::new(self)
    }
    #[doc = "Bit 2 - desc EXTL_EN"]
    #[inline(always)]
    pub fn extl_en(&mut self) -> EXTL_EN_W<2> {
        EXTL_EN_W::new(self)
    }
    #[doc = "Bit 3 - desc XTL_ALWAYS_ON"]
    #[inline(always)]
    pub fn xtl_always_on(&mut self) -> XTL_ALWAYS_ON_W<3> {
        XTL_ALWAYS_ON_W::new(self)
    }
    #[doc = "Bit 5 - desc RTC_LPW"]
    #[inline(always)]
    pub fn rtc_lpw(&mut self) -> RTC_LPW_W<5> {
        RTC_LPW_W::new(self)
    }
    #[doc = "Bit 6 - desc LOCKUP_EN"]
    #[inline(always)]
    pub fn lockup_en(&mut self) -> LOCKUP_EN_W<6> {
        LOCKUP_EN_W::new(self)
    }
    #[doc = "Bit 8 - desc SWD_USE_IO"]
    #[inline(always)]
    pub fn swd_use_io(&mut self) -> SWD_USE_IO_W<8> {
        SWD_USE_IO_W::new(self)
    }
    #[doc = "Bits 9:11 - desc RTC_FREQ_ADJUST"]
    #[inline(always)]
    pub fn rtc_freq_adjust(&mut self) -> RTC_FREQ_ADJUST_W<9> {
        RTC_FREQ_ADJUST_W::new(self)
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
#[doc = "desc SYSCTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl1](index.html) module"]
pub struct SYSCTRL1_SPEC;
impl crate::RegisterSpec for SYSCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysctrl1::R](R) reader structure"]
impl crate::Readable for SYSCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysctrl1::W](W) writer structure"]
impl crate::Writable for SYSCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCTRL1 to value 0x08"]
impl crate::Resettable for SYSCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
