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
#[doc = "Field `TRIM_START` reader - desc TRIM_START"]
pub type TRIM_START_R = crate::BitReader<bool>;
#[doc = "Field `TRIM_START` writer - desc TRIM_START"]
pub type TRIM_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REFCLK_SEL` reader - desc REFCLK_SEL"]
pub type REFCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFCLK_SEL` writer - desc REFCLK_SEL"]
pub type REFCLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CALCLK_SEL` reader - desc CALCLK_SEL"]
pub type CALCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALCLK_SEL` writer - desc CALCLK_SEL"]
pub type CALCLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MON_EN` reader - desc MON_EN"]
pub type MON_EN_R = crate::BitReader<bool>;
#[doc = "Field `MON_EN` writer - desc MON_EN"]
pub type MON_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CALCLK_SEL2` reader - desc CALCLK_SEL2"]
pub type CALCLK_SEL2_R = crate::BitReader<bool>;
#[doc = "Field `CALCLK_SEL2` writer - desc CALCLK_SEL2"]
pub type CALCLK_SEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc TRIM_START"]
    #[inline(always)]
    pub fn trim_start(&self) -> TRIM_START_R {
        TRIM_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc REFCLK_SEL"]
    #[inline(always)]
    pub fn refclk_sel(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - desc CALCLK_SEL"]
    #[inline(always)]
    pub fn calclk_sel(&self) -> CALCLK_SEL_R {
        CALCLK_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc MON_EN"]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CALCLK_SEL2"]
    #[inline(always)]
    pub fn calclk_sel2(&self) -> CALCLK_SEL2_R {
        CALCLK_SEL2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TRIM_START"]
    #[inline(always)]
    pub fn trim_start(&mut self) -> TRIM_START_W<0> {
        TRIM_START_W::new(self)
    }
    #[doc = "Bits 1:3 - desc REFCLK_SEL"]
    #[inline(always)]
    pub fn refclk_sel(&mut self) -> REFCLK_SEL_W<1> {
        REFCLK_SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - desc CALCLK_SEL"]
    #[inline(always)]
    pub fn calclk_sel(&mut self) -> CALCLK_SEL_W<4> {
        CALCLK_SEL_W::new(self)
    }
    #[doc = "Bit 6 - desc MON_EN"]
    #[inline(always)]
    pub fn mon_en(&mut self) -> MON_EN_W<6> {
        MON_EN_W::new(self)
    }
    #[doc = "Bit 7 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<7> {
        IE_W::new(self)
    }
    #[doc = "Bit 8 - desc CALCLK_SEL2"]
    #[inline(always)]
    pub fn calclk_sel2(&mut self) -> CALCLK_SEL2_W<8> {
        CALCLK_SEL2_W::new(self)
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
