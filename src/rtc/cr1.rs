#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT` reader - desc WAIT"]
pub type WAIT_R = crate::BitReader<bool>;
#[doc = "Field `WAIT` writer - desc WAIT"]
pub type WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `WAITF` reader - desc WAITF"]
pub type WAITF_R = crate::BitReader<bool>;
#[doc = "Field `WAITF` writer - desc WAITF"]
pub type WAITF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PRDF` reader - desc PRDF"]
pub type PRDF_R = crate::BitReader<bool>;
#[doc = "Field `PRDF` writer - desc PRDF"]
pub type PRDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ALMF` reader - desc ALMF"]
pub type ALMF_R = crate::BitReader<bool>;
#[doc = "Field `ALMF` writer - desc ALMF"]
pub type ALMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ALMIE` reader - desc ALMIE"]
pub type ALMIE_R = crate::BitReader<bool>;
#[doc = "Field `ALMIE` writer - desc ALMIE"]
pub type ALMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ALMEN` reader - desc ALMEN"]
pub type ALMEN_R = crate::BitReader<bool>;
#[doc = "Field `ALMEN` writer - desc ALMEN"]
pub type ALMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `CKSEL` reader - desc CKSEL"]
pub type CKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKSEL` writer - desc CKSEL"]
pub type CKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WAITF"]
    #[inline(always)]
    pub fn waitf(&self) -> WAITF_R {
        WAITF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PRDF"]
    #[inline(always)]
    pub fn prdf(&self) -> PRDF_R {
        PRDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ALMF"]
    #[inline(always)]
    pub fn almf(&self) -> ALMF_R {
        ALMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ALMIE"]
    #[inline(always)]
    pub fn almie(&self) -> ALMIE_R {
        ALMIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ALMEN"]
    #[inline(always)]
    pub fn almen(&self) -> ALMEN_R {
        ALMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<0> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 1 - desc WAITF"]
    #[inline(always)]
    pub fn waitf(&mut self) -> WAITF_W<1> {
        WAITF_W::new(self)
    }
    #[doc = "Bit 3 - desc PRDF"]
    #[inline(always)]
    pub fn prdf(&mut self) -> PRDF_W<3> {
        PRDF_W::new(self)
    }
    #[doc = "Bit 4 - desc ALMF"]
    #[inline(always)]
    pub fn almf(&mut self) -> ALMF_W<4> {
        ALMF_W::new(self)
    }
    #[doc = "Bit 6 - desc ALMIE"]
    #[inline(always)]
    pub fn almie(&mut self) -> ALMIE_W<6> {
        ALMIE_W::new(self)
    }
    #[doc = "Bit 7 - desc ALMEN"]
    #[inline(always)]
    pub fn almen(&mut self) -> ALMEN_W<7> {
        ALMEN_W::new(self)
    }
    #[doc = "Bits 8:10 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W<8> {
        CKSEL_W::new(self)
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
#[doc = "desc CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
