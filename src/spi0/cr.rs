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
#[doc = "Field `SPR0` reader - desc SPR0"]
pub type SPR0_R = crate::BitReader<bool>;
#[doc = "Field `SPR0` writer - desc SPR0"]
pub type SPR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SPR1` reader - desc SPR1"]
pub type SPR1_R = crate::BitReader<bool>;
#[doc = "Field `SPR1` writer - desc SPR1"]
pub type SPR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CPHA` reader - desc CPHA"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - desc CPHA"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - desc CPOL"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - desc CPOL"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MSTR` reader - desc MSTR"]
pub type MSTR_R = crate::BitReader<bool>;
#[doc = "Field `MSTR` writer - desc MSTR"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SPEN` reader - desc SPEN"]
pub type SPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPEN` writer - desc SPEN"]
pub type SPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SPR2` reader - desc SPR2"]
pub type SPR2_R = crate::BitReader<bool>;
#[doc = "Field `SPR2` writer - desc SPR2"]
pub type SPR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SPR0"]
    #[inline(always)]
    pub fn spr0(&self) -> SPR0_R {
        SPR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SPR1"]
    #[inline(always)]
    pub fn spr1(&self) -> SPR1_R {
        SPR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SPEN"]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SPR2"]
    #[inline(always)]
    pub fn spr2(&self) -> SPR2_R {
        SPR2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SPR0"]
    #[inline(always)]
    pub fn spr0(&mut self) -> SPR0_W<0> {
        SPR0_W::new(self)
    }
    #[doc = "Bit 1 - desc SPR1"]
    #[inline(always)]
    pub fn spr1(&mut self) -> SPR1_W<1> {
        SPR1_W::new(self)
    }
    #[doc = "Bit 2 - desc CPHA"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<2> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 3 - desc CPOL"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<3> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 4 - desc MSTR"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<4> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 6 - desc SPEN"]
    #[inline(always)]
    pub fn spen(&mut self) -> SPEN_W<6> {
        SPEN_W::new(self)
    }
    #[doc = "Bit 7 - desc SPR2"]
    #[inline(always)]
    pub fn spr2(&mut self) -> SPR2_W<7> {
        SPR2_W::new(self)
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
