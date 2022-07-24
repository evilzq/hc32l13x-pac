#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_CLK_SEL` reader - desc EXT_CLK_SEL"]
pub type EXT_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXT_CLK_SEL` writer - desc EXT_CLK_SEL"]
pub type EXT_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SSN0_SEL` reader - desc SSN0_SEL"]
pub type SSN0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSN0_SEL` writer - desc SSN0_SEL"]
pub type SSN0_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PCLK_SEL` reader - desc PCLK_SEL"]
pub type PCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCLK_SEL` writer - desc PCLK_SEL"]
pub type PCLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `HCLK_SEL` reader - desc HCLK_SEL"]
pub type HCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HCLK_SEL` writer - desc HCLK_SEL"]
pub type HCLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL1_SPEC, u8, u8, 2, O>;
#[doc = "Field `PCLK_EN` reader - desc PCLK_EN"]
pub type PCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_EN` writer - desc PCLK_EN"]
pub type PCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, O>;
#[doc = "Field `HCLK_EN` reader - desc HCLK_EN"]
pub type HCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `HCLK_EN` writer - desc HCLK_EN"]
pub type HCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, O>;
#[doc = "Field `IR_POL` reader - desc IR_POL"]
pub type IR_POL_R = crate::BitReader<bool>;
#[doc = "Field `IR_POL` writer - desc IR_POL"]
pub type IR_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc EXT_CLK_SEL"]
    #[inline(always)]
    pub fn ext_clk_sel(&self) -> EXT_CLK_SEL_R {
        EXT_CLK_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc SSN0_SEL"]
    #[inline(always)]
    pub fn ssn0_sel(&self) -> SSN0_SEL_R {
        SSN0_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - desc PCLK_SEL"]
    #[inline(always)]
    pub fn pclk_sel(&self) -> PCLK_SEL_R {
        PCLK_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc HCLK_SEL"]
    #[inline(always)]
    pub fn hclk_sel(&self) -> HCLK_SEL_R {
        HCLK_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - desc PCLK_EN"]
    #[inline(always)]
    pub fn pclk_en(&self) -> PCLK_EN_R {
        PCLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HCLK_EN"]
    #[inline(always)]
    pub fn hclk_en(&self) -> HCLK_EN_R {
        HCLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc IR_POL"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc EXT_CLK_SEL"]
    #[inline(always)]
    pub fn ext_clk_sel(&mut self) -> EXT_CLK_SEL_W<0> {
        EXT_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - desc SSN0_SEL"]
    #[inline(always)]
    pub fn ssn0_sel(&mut self) -> SSN0_SEL_W<4> {
        SSN0_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - desc PCLK_SEL"]
    #[inline(always)]
    pub fn pclk_sel(&mut self) -> PCLK_SEL_W<8> {
        PCLK_SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - desc HCLK_SEL"]
    #[inline(always)]
    pub fn hclk_sel(&mut self) -> HCLK_SEL_W<10> {
        HCLK_SEL_W::new(self)
    }
    #[doc = "Bit 12 - desc PCLK_EN"]
    #[inline(always)]
    pub fn pclk_en(&mut self) -> PCLK_EN_W<12> {
        PCLK_EN_W::new(self)
    }
    #[doc = "Bit 13 - desc HCLK_EN"]
    #[inline(always)]
    pub fn hclk_en(&mut self) -> HCLK_EN_W<13> {
        HCLK_EN_W::new(self)
    }
    #[doc = "Bit 14 - desc IR_POL"]
    #[inline(always)]
    pub fn ir_pol(&mut self) -> IR_POL_W<14> {
        IR_POL_W::new(self)
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
#[doc = "desc CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
