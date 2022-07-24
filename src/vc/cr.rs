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
#[doc = "Field `DIV` reader - desc DIV"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 6, O>;
#[doc = "Field `DIV_EN` reader - desc DIV_EN"]
pub type DIV_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIV_EN` writer - desc DIV_EN"]
pub type DIV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REF2P5_SEL` reader - desc REF2P5_SEL"]
pub type REF2P5_SEL_R = crate::BitReader<bool>;
#[doc = "Field `REF2P5_SEL` writer - desc REF2P5_SEL"]
pub type REF2P5_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `VC0_BIAS_SEL` reader - desc VC0_BIAS_SEL"]
pub type VC0_BIAS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VC0_BIAS_SEL` writer - desc VC0_BIAS_SEL"]
pub type VC0_BIAS_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VC0_HYS_SEL` reader - desc VC0_HYS_SEL"]
pub type VC0_HYS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VC0_HYS_SEL` writer - desc VC0_HYS_SEL"]
pub type VC0_HYS_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VC1_BIAS_SEL` reader - desc VC1_BIAS_SEL"]
pub type VC1_BIAS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VC1_BIAS_SEL` writer - desc VC1_BIAS_SEL"]
pub type VC1_BIAS_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VC1_HYS_SEL` reader - desc VC1_HYS_SEL"]
pub type VC1_HYS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VC1_HYS_SEL` writer - desc VC1_HYS_SEL"]
pub type VC1_HYS_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - desc DIV_EN"]
    #[inline(always)]
    pub fn div_en(&self) -> DIV_EN_R {
        DIV_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc REF2P5_SEL"]
    #[inline(always)]
    pub fn ref2p5_sel(&self) -> REF2P5_SEL_R {
        REF2P5_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc VC0_BIAS_SEL"]
    #[inline(always)]
    pub fn vc0_bias_sel(&self) -> VC0_BIAS_SEL_R {
        VC0_BIAS_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc VC0_HYS_SEL"]
    #[inline(always)]
    pub fn vc0_hys_sel(&self) -> VC0_HYS_SEL_R {
        VC0_HYS_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - desc VC1_BIAS_SEL"]
    #[inline(always)]
    pub fn vc1_bias_sel(&self) -> VC1_BIAS_SEL_R {
        VC1_BIAS_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - desc VC1_HYS_SEL"]
    #[inline(always)]
    pub fn vc1_hys_sel(&self) -> VC1_HYS_SEL_R {
        VC1_HYS_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 6 - desc DIV_EN"]
    #[inline(always)]
    pub fn div_en(&mut self) -> DIV_EN_W<6> {
        DIV_EN_W::new(self)
    }
    #[doc = "Bit 7 - desc REF2P5_SEL"]
    #[inline(always)]
    pub fn ref2p5_sel(&mut self) -> REF2P5_SEL_W<7> {
        REF2P5_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - desc VC0_BIAS_SEL"]
    #[inline(always)]
    pub fn vc0_bias_sel(&mut self) -> VC0_BIAS_SEL_W<8> {
        VC0_BIAS_SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - desc VC0_HYS_SEL"]
    #[inline(always)]
    pub fn vc0_hys_sel(&mut self) -> VC0_HYS_SEL_W<10> {
        VC0_HYS_SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - desc VC1_BIAS_SEL"]
    #[inline(always)]
    pub fn vc1_bias_sel(&mut self) -> VC1_BIAS_SEL_W<12> {
        VC1_BIAS_SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - desc VC1_HYS_SEL"]
    #[inline(always)]
    pub fn vc1_hys_sel(&mut self) -> VC1_HYS_SEL_W<14> {
        VC1_HYS_SEL_W::new(self)
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
#[doc = "`reset()` method sets CR to value 0x20"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
