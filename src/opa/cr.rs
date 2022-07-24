#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCTR_EN` reader - desc ADCTR_EN"]
pub type ADCTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADCTR_EN` writer - desc ADCTR_EN"]
pub type ADCTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `TRIGGER` reader - desc TRIGGER"]
pub type TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TRIGGER` writer - desc TRIGGER"]
pub type TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `AZ_PULSE` reader - desc AZ_PULSE"]
pub type AZ_PULSE_R = crate::BitReader<bool>;
#[doc = "Field `AZ_PULSE` writer - desc AZ_PULSE"]
pub type AZ_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `CLK_SW_SET` reader - desc CLK_SW_SET"]
pub type CLK_SW_SET_R = crate::BitReader<bool>;
#[doc = "Field `CLK_SW_SET` writer - desc CLK_SW_SET"]
pub type CLK_SW_SET_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, bool, O>;
#[doc = "Field `CLK_SEL` reader - desc CLK_SEL"]
pub type CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_SEL` writer - desc CLK_SEL"]
pub type CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - desc ADCTR_EN"]
    #[inline(always)]
    pub fn adctr_en(&self) -> ADCTR_EN_R {
        ADCTR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TRIGGER"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc AZ_PULSE"]
    #[inline(always)]
    pub fn az_pulse(&self) -> AZ_PULSE_R {
        AZ_PULSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CLK_SW_SET"]
    #[inline(always)]
    pub fn clk_sw_set(&self) -> CLK_SW_SET_R {
        CLK_SW_SET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - desc CLK_SEL"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADCTR_EN"]
    #[inline(always)]
    pub fn adctr_en(&mut self) -> ADCTR_EN_W<0> {
        ADCTR_EN_W::new(self)
    }
    #[doc = "Bit 1 - desc TRIGGER"]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W<1> {
        TRIGGER_W::new(self)
    }
    #[doc = "Bit 2 - desc AZ_PULSE"]
    #[inline(always)]
    pub fn az_pulse(&mut self) -> AZ_PULSE_W<2> {
        AZ_PULSE_W::new(self)
    }
    #[doc = "Bit 3 - desc CLK_SW_SET"]
    #[inline(always)]
    pub fn clk_sw_set(&mut self) -> CLK_SW_SET_W<3> {
        CLK_SW_SET_W::new(self)
    }
    #[doc = "Bits 4:7 - desc CLK_SEL"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<4> {
        CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
