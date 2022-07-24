#[doc = "Register `SYSCTRL0` reader"]
pub struct R(crate::R<SYSCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCTRL0` writer"]
pub struct W(crate::W<SYSCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTRL0_SPEC>;
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
impl From<crate::W<SYSCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCH_EN` reader - desc RCH_EN"]
pub type RCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `RCH_EN` writer - desc RCH_EN"]
pub type RCH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL0_SPEC, bool, O>;
#[doc = "Field `XTH_EN` reader - desc XTH_EN"]
pub type XTH_EN_R = crate::BitReader<bool>;
#[doc = "Field `XTH_EN` writer - desc XTH_EN"]
pub type XTH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL0_SPEC, bool, O>;
#[doc = "Field `RCL_EN` reader - desc RCL_EN"]
pub type RCL_EN_R = crate::BitReader<bool>;
#[doc = "Field `RCL_EN` writer - desc RCL_EN"]
pub type RCL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL0_SPEC, bool, O>;
#[doc = "Field `XTL_EN` reader - desc XTL_EN"]
pub type XTL_EN_R = crate::BitReader<bool>;
#[doc = "Field `XTL_EN` writer - desc XTL_EN"]
pub type XTL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL0_SPEC, bool, O>;
#[doc = "Field `PLL_EN` reader - desc PLL_EN"]
pub type PLL_EN_R = crate::BitReader<bool>;
#[doc = "Field `PLL_EN` writer - desc PLL_EN"]
pub type PLL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL0_SPEC, bool, O>;
#[doc = "Field `CLKSW` reader - desc CLKSW"]
pub type CLKSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKSW` writer - desc CLKSW"]
pub type CLKSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCTRL0_SPEC, u8, u8, 3, O>;
#[doc = "Field `HCLK_PRS` reader - desc HCLK_PRS"]
pub type HCLK_PRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HCLK_PRS` writer - desc HCLK_PRS"]
pub type HCLK_PRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCTRL0_SPEC, u8, u8, 3, O>;
#[doc = "Field `PCLK_PRS` reader - desc PCLK_PRS"]
pub type PCLK_PRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCLK_PRS` writer - desc PCLK_PRS"]
pub type PCLK_PRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAKEUP_BYRCH` reader - desc WAKEUP_BYRCH"]
pub type WAKEUP_BYRCH_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP_BYRCH` writer - desc WAKEUP_BYRCH"]
pub type WAKEUP_BYRCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL0_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc RCH_EN"]
    #[inline(always)]
    pub fn rch_en(&self) -> RCH_EN_R {
        RCH_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc XTH_EN"]
    #[inline(always)]
    pub fn xth_en(&self) -> XTH_EN_R {
        XTH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RCL_EN"]
    #[inline(always)]
    pub fn rcl_en(&self) -> RCL_EN_R {
        RCL_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc XTL_EN"]
    #[inline(always)]
    pub fn xtl_en(&self) -> XTL_EN_R {
        XTL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PLL_EN"]
    #[inline(always)]
    pub fn pll_en(&self) -> PLL_EN_R {
        PLL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - desc CLKSW"]
    #[inline(always)]
    pub fn clksw(&self) -> CLKSW_R {
        CLKSW_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc HCLK_PRS"]
    #[inline(always)]
    pub fn hclk_prs(&self) -> HCLK_PRS_R {
        HCLK_PRS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - desc PCLK_PRS"]
    #[inline(always)]
    pub fn pclk_prs(&self) -> PCLK_PRS_R {
        PCLK_PRS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 15 - desc WAKEUP_BYRCH"]
    #[inline(always)]
    pub fn wakeup_byrch(&self) -> WAKEUP_BYRCH_R {
        WAKEUP_BYRCH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RCH_EN"]
    #[inline(always)]
    pub fn rch_en(&mut self) -> RCH_EN_W<0> {
        RCH_EN_W::new(self)
    }
    #[doc = "Bit 1 - desc XTH_EN"]
    #[inline(always)]
    pub fn xth_en(&mut self) -> XTH_EN_W<1> {
        XTH_EN_W::new(self)
    }
    #[doc = "Bit 2 - desc RCL_EN"]
    #[inline(always)]
    pub fn rcl_en(&mut self) -> RCL_EN_W<2> {
        RCL_EN_W::new(self)
    }
    #[doc = "Bit 3 - desc XTL_EN"]
    #[inline(always)]
    pub fn xtl_en(&mut self) -> XTL_EN_W<3> {
        XTL_EN_W::new(self)
    }
    #[doc = "Bit 4 - desc PLL_EN"]
    #[inline(always)]
    pub fn pll_en(&mut self) -> PLL_EN_W<4> {
        PLL_EN_W::new(self)
    }
    #[doc = "Bits 5:7 - desc CLKSW"]
    #[inline(always)]
    pub fn clksw(&mut self) -> CLKSW_W<5> {
        CLKSW_W::new(self)
    }
    #[doc = "Bits 8:10 - desc HCLK_PRS"]
    #[inline(always)]
    pub fn hclk_prs(&mut self) -> HCLK_PRS_W<8> {
        HCLK_PRS_W::new(self)
    }
    #[doc = "Bits 11:12 - desc PCLK_PRS"]
    #[inline(always)]
    pub fn pclk_prs(&mut self) -> PCLK_PRS_W<11> {
        PCLK_PRS_W::new(self)
    }
    #[doc = "Bit 15 - desc WAKEUP_BYRCH"]
    #[inline(always)]
    pub fn wakeup_byrch(&mut self) -> WAKEUP_BYRCH_W<15> {
        WAKEUP_BYRCH_W::new(self)
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
#[doc = "desc SYSCTRL0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl0](index.html) module"]
pub struct SYSCTRL0_SPEC;
impl crate::RegisterSpec for SYSCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysctrl0::R](R) reader structure"]
impl crate::Readable for SYSCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysctrl0::W](W) writer structure"]
impl crate::Writable for SYSCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCTRL0 to value 0x01"]
impl crate::Resettable for SYSCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
