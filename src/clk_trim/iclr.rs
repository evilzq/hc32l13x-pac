#[doc = "Register `ICLR` reader"]
pub struct R(crate::R<ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICLR` writer"]
pub struct W(crate::W<ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLR_SPEC>;
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
impl From<crate::W<ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTL_FAULT_CLR` reader - desc XTL_FAULT_CLR"]
pub type XTL_FAULT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `XTL_FAULT_CLR` writer - desc XTL_FAULT_CLR"]
pub type XTL_FAULT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `XTH_FAULT_CLR` reader - desc XTH_FAULT_CLR"]
pub type XTH_FAULT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `XTH_FAULT_CLR` writer - desc XTH_FAULT_CLR"]
pub type XTH_FAULT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `PLL_FAULT_CLR` reader - desc PLL_FAULT_CLR"]
pub type PLL_FAULT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `PLL_FAULT_CLR` writer - desc PLL_FAULT_CLR"]
pub type PLL_FAULT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - desc XTL_FAULT_CLR"]
    #[inline(always)]
    pub fn xtl_fault_clr(&self) -> XTL_FAULT_CLR_R {
        XTL_FAULT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc XTH_FAULT_CLR"]
    #[inline(always)]
    pub fn xth_fault_clr(&self) -> XTH_FAULT_CLR_R {
        XTH_FAULT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PLL_FAULT_CLR"]
    #[inline(always)]
    pub fn pll_fault_clr(&self) -> PLL_FAULT_CLR_R {
        PLL_FAULT_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - desc XTL_FAULT_CLR"]
    #[inline(always)]
    pub fn xtl_fault_clr(&mut self) -> XTL_FAULT_CLR_W<2> {
        XTL_FAULT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - desc XTH_FAULT_CLR"]
    #[inline(always)]
    pub fn xth_fault_clr(&mut self) -> XTH_FAULT_CLR_W<3> {
        XTH_FAULT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - desc PLL_FAULT_CLR"]
    #[inline(always)]
    pub fn pll_fault_clr(&mut self) -> PLL_FAULT_CLR_W<4> {
        PLL_FAULT_CLR_W::new(self)
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
#[doc = "desc ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](index.html) module"]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iclr::R](R) reader structure"]
impl crate::Readable for ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iclr::W](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICLR to value 0x0c"]
impl crate::Resettable for ICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
