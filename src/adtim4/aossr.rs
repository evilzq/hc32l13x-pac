#[doc = "Register `AOSSR` reader"]
pub struct R(crate::R<AOSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AOSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AOSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AOSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AOSSR` writer"]
pub struct W(crate::W<AOSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AOSSR_SPEC>;
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
impl From<crate::W<AOSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AOSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBRAKE` reader - desc FBRAKE"]
pub type FBRAKE_R = crate::BitReader<bool>;
#[doc = "Field `FSAME` reader - desc FSAME"]
pub type FSAME_R = crate::BitReader<bool>;
#[doc = "Field `BFILTS` reader - desc BFILTS"]
pub type BFILTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BFILTS` writer - desc BFILTS"]
pub type BFILTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AOSSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BFILTEN` reader - desc BFILTEN"]
pub type BFILTEN_R = crate::BitReader<bool>;
#[doc = "Field `BFILTEN` writer - desc BFILTEN"]
pub type BFILTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
#[doc = "Field `SOFTBK` reader - desc SOFTBK"]
pub type SOFTBK_R = crate::BitReader<bool>;
#[doc = "Field `SOFTBK` writer - desc SOFTBK"]
pub type SOFTBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
#[doc = "Field `SML0` reader - desc SML0"]
pub type SML0_R = crate::BitReader<bool>;
#[doc = "Field `SML0` writer - desc SML0"]
pub type SML0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
#[doc = "Field `SML1` reader - desc SML1"]
pub type SML1_R = crate::BitReader<bool>;
#[doc = "Field `SML1` writer - desc SML1"]
pub type SML1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
#[doc = "Field `SML2` reader - desc SML2"]
pub type SML2_R = crate::BitReader<bool>;
#[doc = "Field `SML2` writer - desc SML2"]
pub type SML2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
#[doc = "Field `SMH0` reader - desc SMH0"]
pub type SMH0_R = crate::BitReader<bool>;
#[doc = "Field `SMH0` writer - desc SMH0"]
pub type SMH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
#[doc = "Field `SMH1` reader - desc SMH1"]
pub type SMH1_R = crate::BitReader<bool>;
#[doc = "Field `SMH1` writer - desc SMH1"]
pub type SMH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
#[doc = "Field `SMH2` reader - desc SMH2"]
pub type SMH2_R = crate::BitReader<bool>;
#[doc = "Field `SMH2` writer - desc SMH2"]
pub type SMH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc FBRAKE"]
    #[inline(always)]
    pub fn fbrake(&self) -> FBRAKE_R {
        FBRAKE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FSAME"]
    #[inline(always)]
    pub fn fsame(&self) -> FSAME_R {
        FSAME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc BFILTS"]
    #[inline(always)]
    pub fn bfilts(&self) -> BFILTS_R {
        BFILTS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc BFILTEN"]
    #[inline(always)]
    pub fn bfilten(&self) -> BFILTEN_R {
        BFILTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SOFTBK"]
    #[inline(always)]
    pub fn softbk(&self) -> SOFTBK_R {
        SOFTBK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SML0"]
    #[inline(always)]
    pub fn sml0(&self) -> SML0_R {
        SML0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SML1"]
    #[inline(always)]
    pub fn sml1(&self) -> SML1_R {
        SML1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc SML2"]
    #[inline(always)]
    pub fn sml2(&self) -> SML2_R {
        SML2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc SMH0"]
    #[inline(always)]
    pub fn smh0(&self) -> SMH0_R {
        SMH0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc SMH1"]
    #[inline(always)]
    pub fn smh1(&self) -> SMH1_R {
        SMH1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SMH2"]
    #[inline(always)]
    pub fn smh2(&self) -> SMH2_R {
        SMH2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - desc BFILTS"]
    #[inline(always)]
    pub fn bfilts(&mut self) -> BFILTS_W<2> {
        BFILTS_W::new(self)
    }
    #[doc = "Bit 4 - desc BFILTEN"]
    #[inline(always)]
    pub fn bfilten(&mut self) -> BFILTEN_W<4> {
        BFILTEN_W::new(self)
    }
    #[doc = "Bit 7 - desc SOFTBK"]
    #[inline(always)]
    pub fn softbk(&mut self) -> SOFTBK_W<7> {
        SOFTBK_W::new(self)
    }
    #[doc = "Bit 8 - desc SML0"]
    #[inline(always)]
    pub fn sml0(&mut self) -> SML0_W<8> {
        SML0_W::new(self)
    }
    #[doc = "Bit 9 - desc SML1"]
    #[inline(always)]
    pub fn sml1(&mut self) -> SML1_W<9> {
        SML1_W::new(self)
    }
    #[doc = "Bit 10 - desc SML2"]
    #[inline(always)]
    pub fn sml2(&mut self) -> SML2_W<10> {
        SML2_W::new(self)
    }
    #[doc = "Bit 11 - desc SMH0"]
    #[inline(always)]
    pub fn smh0(&mut self) -> SMH0_W<11> {
        SMH0_W::new(self)
    }
    #[doc = "Bit 12 - desc SMH1"]
    #[inline(always)]
    pub fn smh1(&mut self) -> SMH1_W<12> {
        SMH1_W::new(self)
    }
    #[doc = "Bit 13 - desc SMH2"]
    #[inline(always)]
    pub fn smh2(&mut self) -> SMH2_W<13> {
        SMH2_W::new(self)
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
#[doc = "desc AOSSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aossr](index.html) module"]
pub struct AOSSR_SPEC;
impl crate::RegisterSpec for AOSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aossr::R](R) reader structure"]
impl crate::Readable for AOSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aossr::W](W) writer structure"]
impl crate::Writable for AOSSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AOSSR to value 0"]
impl crate::Resettable for AOSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
