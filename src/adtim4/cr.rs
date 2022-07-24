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
#[doc = "Field `CMAE` reader - desc CMAE"]
pub type CMAE_R = crate::BitReader<bool>;
#[doc = "Field `CMAE` writer - desc CMAE"]
pub type CMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CMBE` reader - desc CMBE"]
pub type CMBE_R = crate::BitReader<bool>;
#[doc = "Field `CMBE` writer - desc CMBE"]
pub type CMBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CMCE` reader - desc CMCE"]
pub type CMCE_R = crate::BitReader<bool>;
#[doc = "Field `CMCE` writer - desc CMCE"]
pub type CMCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CMDE` reader - desc CMDE"]
pub type CMDE_R = crate::BitReader<bool>;
#[doc = "Field `CMDE` writer - desc CMDE"]
pub type CMDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OVFE` reader - desc OVFE"]
pub type OVFE_R = crate::BitReader<bool>;
#[doc = "Field `OVFE` writer - desc OVFE"]
pub type OVFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `UDFE` reader - desc UDFE"]
pub type UDFE_R = crate::BitReader<bool>;
#[doc = "Field `UDFE` writer - desc UDFE"]
pub type UDFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DITENA` reader - desc DITENA"]
pub type DITENA_R = crate::BitReader<bool>;
#[doc = "Field `DITENA` writer - desc DITENA"]
pub type DITENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DITENB` reader - desc DITENB"]
pub type DITENB_R = crate::BitReader<bool>;
#[doc = "Field `DITENB` writer - desc DITENB"]
pub type DITENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DITENS` reader - desc DITENS"]
pub type DITENS_R = crate::BitReader<bool>;
#[doc = "Field `DITENS` writer - desc DITENS"]
pub type DITENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CMSAE` reader - desc CMSAE"]
pub type CMSAE_R = crate::BitReader<bool>;
#[doc = "Field `CMSAE` writer - desc CMSAE"]
pub type CMSAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CMSBE` reader - desc CMSBE"]
pub type CMSBE_R = crate::BitReader<bool>;
#[doc = "Field `CMSBE` writer - desc CMSBE"]
pub type CMSBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMA_G_CMA` reader - desc DMA_G_CMA"]
pub type DMA_G_CMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA_G_CMA` writer - desc DMA_G_CMA"]
pub type DMA_G_CMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMA_G_CMB` reader - desc DMA_G_CMB"]
pub type DMA_G_CMB_R = crate::BitReader<bool>;
#[doc = "Field `DMA_G_CMB` writer - desc DMA_G_CMB"]
pub type DMA_G_CMB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMA_G_CMC` reader - desc DMA_G_CMC"]
pub type DMA_G_CMC_R = crate::BitReader<bool>;
#[doc = "Field `DMA_G_CMC` writer - desc DMA_G_CMC"]
pub type DMA_G_CMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMA_G_CMD` reader - desc DMA_G_CMD"]
pub type DMA_G_CMD_R = crate::BitReader<bool>;
#[doc = "Field `DMA_G_CMD` writer - desc DMA_G_CMD"]
pub type DMA_G_CMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMA_G_OVF` reader - desc DMA_G_OVF"]
pub type DMA_G_OVF_R = crate::BitReader<bool>;
#[doc = "Field `DMA_G_OVF` writer - desc DMA_G_OVF"]
pub type DMA_G_OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMA_G_UDF` reader - desc DMA_G_UDF"]
pub type DMA_G_UDF_R = crate::BitReader<bool>;
#[doc = "Field `DMA_G_UDF` writer - desc DMA_G_UDF"]
pub type DMA_G_UDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMA_S_CMA` reader - desc DMA_S_CMA"]
pub type DMA_S_CMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA_S_CMA` writer - desc DMA_S_CMA"]
pub type DMA_S_CMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMA_S_CMB` reader - desc DMA_S_CMB"]
pub type DMA_S_CMB_R = crate::BitReader<bool>;
#[doc = "Field `DMA_S_CMB` writer - desc DMA_S_CMB"]
pub type DMA_S_CMB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CMAE"]
    #[inline(always)]
    pub fn cmae(&self) -> CMAE_R {
        CMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CMBE"]
    #[inline(always)]
    pub fn cmbe(&self) -> CMBE_R {
        CMBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMCE"]
    #[inline(always)]
    pub fn cmce(&self) -> CMCE_R {
        CMCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMDE"]
    #[inline(always)]
    pub fn cmde(&self) -> CMDE_R {
        CMDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OVFE"]
    #[inline(always)]
    pub fn ovfe(&self) -> OVFE_R {
        OVFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc UDFE"]
    #[inline(always)]
    pub fn udfe(&self) -> UDFE_R {
        UDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc DITENA"]
    #[inline(always)]
    pub fn ditena(&self) -> DITENA_R {
        DITENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DITENB"]
    #[inline(always)]
    pub fn ditenb(&self) -> DITENB_R {
        DITENB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DITENS"]
    #[inline(always)]
    pub fn ditens(&self) -> DITENS_R {
        DITENS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CMSAE"]
    #[inline(always)]
    pub fn cmsae(&self) -> CMSAE_R {
        CMSAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CMSBE"]
    #[inline(always)]
    pub fn cmsbe(&self) -> CMSBE_R {
        CMSBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc DMA_G_CMA"]
    #[inline(always)]
    pub fn dma_g_cma(&self) -> DMA_G_CMA_R {
        DMA_G_CMA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc DMA_G_CMB"]
    #[inline(always)]
    pub fn dma_g_cmb(&self) -> DMA_G_CMB_R {
        DMA_G_CMB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DMA_G_CMC"]
    #[inline(always)]
    pub fn dma_g_cmc(&self) -> DMA_G_CMC_R {
        DMA_G_CMC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc DMA_G_CMD"]
    #[inline(always)]
    pub fn dma_g_cmd(&self) -> DMA_G_CMD_R {
        DMA_G_CMD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - desc DMA_G_OVF"]
    #[inline(always)]
    pub fn dma_g_ovf(&self) -> DMA_G_OVF_R {
        DMA_G_OVF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc DMA_G_UDF"]
    #[inline(always)]
    pub fn dma_g_udf(&self) -> DMA_G_UDF_R {
        DMA_G_UDF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc DMA_S_CMA"]
    #[inline(always)]
    pub fn dma_s_cma(&self) -> DMA_S_CMA_R {
        DMA_S_CMA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc DMA_S_CMB"]
    #[inline(always)]
    pub fn dma_s_cmb(&self) -> DMA_S_CMB_R {
        DMA_S_CMB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CMAE"]
    #[inline(always)]
    pub fn cmae(&mut self) -> CMAE_W<0> {
        CMAE_W::new(self)
    }
    #[doc = "Bit 1 - desc CMBE"]
    #[inline(always)]
    pub fn cmbe(&mut self) -> CMBE_W<1> {
        CMBE_W::new(self)
    }
    #[doc = "Bit 2 - desc CMCE"]
    #[inline(always)]
    pub fn cmce(&mut self) -> CMCE_W<2> {
        CMCE_W::new(self)
    }
    #[doc = "Bit 3 - desc CMDE"]
    #[inline(always)]
    pub fn cmde(&mut self) -> CMDE_W<3> {
        CMDE_W::new(self)
    }
    #[doc = "Bit 6 - desc OVFE"]
    #[inline(always)]
    pub fn ovfe(&mut self) -> OVFE_W<6> {
        OVFE_W::new(self)
    }
    #[doc = "Bit 7 - desc UDFE"]
    #[inline(always)]
    pub fn udfe(&mut self) -> UDFE_W<7> {
        UDFE_W::new(self)
    }
    #[doc = "Bit 8 - desc DITENA"]
    #[inline(always)]
    pub fn ditena(&mut self) -> DITENA_W<8> {
        DITENA_W::new(self)
    }
    #[doc = "Bit 9 - desc DITENB"]
    #[inline(always)]
    pub fn ditenb(&mut self) -> DITENB_W<9> {
        DITENB_W::new(self)
    }
    #[doc = "Bit 10 - desc DITENS"]
    #[inline(always)]
    pub fn ditens(&mut self) -> DITENS_W<10> {
        DITENS_W::new(self)
    }
    #[doc = "Bit 11 - desc CMSAE"]
    #[inline(always)]
    pub fn cmsae(&mut self) -> CMSAE_W<11> {
        CMSAE_W::new(self)
    }
    #[doc = "Bit 12 - desc CMSBE"]
    #[inline(always)]
    pub fn cmsbe(&mut self) -> CMSBE_W<12> {
        CMSBE_W::new(self)
    }
    #[doc = "Bit 13 - desc DMA_G_CMA"]
    #[inline(always)]
    pub fn dma_g_cma(&mut self) -> DMA_G_CMA_W<13> {
        DMA_G_CMA_W::new(self)
    }
    #[doc = "Bit 14 - desc DMA_G_CMB"]
    #[inline(always)]
    pub fn dma_g_cmb(&mut self) -> DMA_G_CMB_W<14> {
        DMA_G_CMB_W::new(self)
    }
    #[doc = "Bit 15 - desc DMA_G_CMC"]
    #[inline(always)]
    pub fn dma_g_cmc(&mut self) -> DMA_G_CMC_W<15> {
        DMA_G_CMC_W::new(self)
    }
    #[doc = "Bit 16 - desc DMA_G_CMD"]
    #[inline(always)]
    pub fn dma_g_cmd(&mut self) -> DMA_G_CMD_W<16> {
        DMA_G_CMD_W::new(self)
    }
    #[doc = "Bit 19 - desc DMA_G_OVF"]
    #[inline(always)]
    pub fn dma_g_ovf(&mut self) -> DMA_G_OVF_W<19> {
        DMA_G_OVF_W::new(self)
    }
    #[doc = "Bit 20 - desc DMA_G_UDF"]
    #[inline(always)]
    pub fn dma_g_udf(&mut self) -> DMA_G_UDF_W<20> {
        DMA_G_UDF_W::new(self)
    }
    #[doc = "Bit 21 - desc DMA_S_CMA"]
    #[inline(always)]
    pub fn dma_s_cma(&mut self) -> DMA_S_CMA_W<21> {
        DMA_S_CMA_W::new(self)
    }
    #[doc = "Bit 22 - desc DMA_S_CMB"]
    #[inline(always)]
    pub fn dma_s_cmb(&mut self) -> DMA_S_CMB_W<22> {
        DMA_S_CMB_W::new(self)
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
#[doc = "desc CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x0300"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300
    }
}
