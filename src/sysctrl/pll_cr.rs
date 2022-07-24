#[doc = "Register `PLL_CR` reader"]
pub struct R(crate::R<PLL_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_CR` writer"]
pub struct W(crate::W<PLL_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CR_SPEC>;
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
impl From<crate::W<PLL_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFSEL` reader - desc REFSEL"]
pub type REFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFSEL` writer - desc REFSEL"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FOSC` reader - desc FOSC"]
pub type FOSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FOSC` writer - desc FOSC"]
pub type FOSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DIVN` reader - desc DIVN"]
pub type DIVN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVN` writer - desc DIVN"]
pub type DIVN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `IBSEL` reader - desc IBSEL"]
pub type IBSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBSEL` writer - desc IBSEL"]
pub type IBSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LFSEL` reader - desc LFSEL"]
pub type LFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LFSEL` writer - desc LFSEL"]
pub type LFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FRSEL` reader - desc FRSEL"]
pub type FRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRSEL` writer - desc FRSEL"]
pub type FRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `STARTUP` reader - desc STARTUP"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - desc STARTUP"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::BitReader<bool>;
#[doc = "Field `STABLE` writer - desc STABLE"]
pub type STABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc REFSEL"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - desc FOSC"]
    #[inline(always)]
    pub fn fosc(&self) -> FOSC_R {
        FOSC_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - desc DIVN"]
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - desc IBSEL"]
    #[inline(always)]
    pub fn ibsel(&self) -> IBSEL_R {
        IBSEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - desc LFSEL"]
    #[inline(always)]
    pub fn lfsel(&self) -> LFSEL_R {
        LFSEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - desc FRSEL"]
    #[inline(always)]
    pub fn frsel(&self) -> FRSEL_R {
        FRSEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:17 - desc STARTUP"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc REFSEL"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W<0> {
        REFSEL_W::new(self)
    }
    #[doc = "Bits 2:4 - desc FOSC"]
    #[inline(always)]
    pub fn fosc(&mut self) -> FOSC_W<2> {
        FOSC_W::new(self)
    }
    #[doc = "Bits 5:8 - desc DIVN"]
    #[inline(always)]
    pub fn divn(&mut self) -> DIVN_W<5> {
        DIVN_W::new(self)
    }
    #[doc = "Bits 9:10 - desc IBSEL"]
    #[inline(always)]
    pub fn ibsel(&mut self) -> IBSEL_W<9> {
        IBSEL_W::new(self)
    }
    #[doc = "Bits 11:12 - desc LFSEL"]
    #[inline(always)]
    pub fn lfsel(&mut self) -> LFSEL_W<11> {
        LFSEL_W::new(self)
    }
    #[doc = "Bits 13:14 - desc FRSEL"]
    #[inline(always)]
    pub fn frsel(&mut self) -> FRSEL_W<13> {
        FRSEL_W::new(self)
    }
    #[doc = "Bits 15:17 - desc STARTUP"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W<15> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 18 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&mut self) -> STABLE_W<18> {
        STABLE_W::new(self)
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
#[doc = "desc PLL_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_cr](index.html) module"]
pub struct PLL_CR_SPEC;
impl crate::RegisterSpec for PLL_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_cr::R](R) reader structure"]
impl crate::Readable for PLL_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_cr::W](W) writer structure"]
impl crate::Writable for PLL_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_CR to value 0x0001_0b0f"]
impl crate::Resettable for PLL_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0b0f
    }
}
