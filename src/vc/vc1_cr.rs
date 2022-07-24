#[doc = "Register `VC1_CR` reader"]
pub struct R(crate::R<VC1_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VC1_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VC1_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VC1_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VC1_CR` writer"]
pub struct W(crate::W<VC1_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VC1_CR_SPEC>;
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
impl From<crate::W<VC1_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VC1_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P_SEL` reader - desc P_SEL"]
pub type P_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P_SEL` writer - desc P_SEL"]
pub type P_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VC1_CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `N_SEL` reader - desc N_SEL"]
pub type N_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `N_SEL` writer - desc N_SEL"]
pub type N_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VC1_CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLTEN` reader - desc FLTEN"]
pub type FLTEN_R = crate::BitReader<bool>;
#[doc = "Field `FLTEN` writer - desc FLTEN"]
pub type FLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC1_CR_SPEC, bool, O>;
#[doc = "Field `DEBOUNCE_TIME` reader - desc DEBOUNCE_TIME"]
pub type DEBOUNCE_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBOUNCE_TIME` writer - desc DEBOUNCE_TIME"]
pub type DEBOUNCE_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VC1_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FALLING` reader - desc FALLING"]
pub type FALLING_R = crate::BitReader<bool>;
#[doc = "Field `FALLING` writer - desc FALLING"]
pub type FALLING_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC1_CR_SPEC, bool, O>;
#[doc = "Field `RISING` reader - desc RISING"]
pub type RISING_R = crate::BitReader<bool>;
#[doc = "Field `RISING` writer - desc RISING"]
pub type RISING_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC1_CR_SPEC, bool, O>;
#[doc = "Field `LEVEL` reader - desc LEVEL"]
pub type LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `LEVEL` writer - desc LEVEL"]
pub type LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC1_CR_SPEC, bool, O>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC1_CR_SPEC, bool, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC1_CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, VC1_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc P_SEL"]
    #[inline(always)]
    pub fn p_sel(&self) -> P_SEL_R {
        P_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc N_SEL"]
    #[inline(always)]
    pub fn n_sel(&self) -> N_SEL_R {
        N_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - desc DEBOUNCE_TIME"]
    #[inline(always)]
    pub fn debounce_time(&self) -> DEBOUNCE_TIME_R {
        DEBOUNCE_TIME_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - desc FALLING"]
    #[inline(always)]
    pub fn falling(&self) -> FALLING_R {
        FALLING_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc RISING"]
    #[inline(always)]
    pub fn rising(&self) -> RISING_R {
        RISING_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc P_SEL"]
    #[inline(always)]
    pub fn p_sel(&mut self) -> P_SEL_W<0> {
        P_SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - desc N_SEL"]
    #[inline(always)]
    pub fn n_sel(&mut self) -> N_SEL_W<4> {
        N_SEL_W::new(self)
    }
    #[doc = "Bit 8 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W<8> {
        FLTEN_W::new(self)
    }
    #[doc = "Bits 9:11 - desc DEBOUNCE_TIME"]
    #[inline(always)]
    pub fn debounce_time(&mut self) -> DEBOUNCE_TIME_W<9> {
        DEBOUNCE_TIME_W::new(self)
    }
    #[doc = "Bit 12 - desc FALLING"]
    #[inline(always)]
    pub fn falling(&mut self) -> FALLING_W<12> {
        FALLING_W::new(self)
    }
    #[doc = "Bit 13 - desc RISING"]
    #[inline(always)]
    pub fn rising(&mut self) -> RISING_W<13> {
        RISING_W::new(self)
    }
    #[doc = "Bit 14 - desc LEVEL"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W<14> {
        LEVEL_W::new(self)
    }
    #[doc = "Bit 15 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<15> {
        IE_W::new(self)
    }
    #[doc = "Bit 16 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<16> {
        EN_W::new(self)
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
#[doc = "desc VC1_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vc1_cr](index.html) module"]
pub struct VC1_CR_SPEC;
impl crate::RegisterSpec for VC1_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vc1_cr::R](R) reader structure"]
impl crate::Readable for VC1_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vc1_cr::W](W) writer structure"]
impl crate::Writable for VC1_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VC1_CR to value 0"]
impl crate::Resettable for VC1_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
