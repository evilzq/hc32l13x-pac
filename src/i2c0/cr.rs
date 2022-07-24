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
#[doc = "Field `H1M` reader - desc H1M"]
pub type H1M_R = crate::BitReader<bool>;
#[doc = "Field `H1M` writer - desc H1M"]
pub type H1M_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `AA` reader - desc AA"]
pub type AA_R = crate::BitReader<bool>;
#[doc = "Field `AA` writer - desc AA"]
pub type AA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SI` reader - desc SI"]
pub type SI_R = crate::BitReader<bool>;
#[doc = "Field `SI` writer - desc SI"]
pub type SI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STO` reader - desc STO"]
pub type STO_R = crate::BitReader<bool>;
#[doc = "Field `STO` writer - desc STO"]
pub type STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STA` reader - desc STA"]
pub type STA_R = crate::BitReader<bool>;
#[doc = "Field `STA` writer - desc STA"]
pub type STA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ENS` reader - desc ENS"]
pub type ENS_R = crate::BitReader<bool>;
#[doc = "Field `ENS` writer - desc ENS"]
pub type ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc H1M"]
    #[inline(always)]
    pub fn h1m(&self) -> H1M_R {
        H1M_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc AA"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SI"]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STO"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc STA"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ENS"]
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc H1M"]
    #[inline(always)]
    pub fn h1m(&mut self) -> H1M_W<0> {
        H1M_W::new(self)
    }
    #[doc = "Bit 2 - desc AA"]
    #[inline(always)]
    pub fn aa(&mut self) -> AA_W<2> {
        AA_W::new(self)
    }
    #[doc = "Bit 3 - desc SI"]
    #[inline(always)]
    pub fn si(&mut self) -> SI_W<3> {
        SI_W::new(self)
    }
    #[doc = "Bit 4 - desc STO"]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W<4> {
        STO_W::new(self)
    }
    #[doc = "Bit 5 - desc STA"]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W<5> {
        STA_W::new(self)
    }
    #[doc = "Bit 6 - desc ENS"]
    #[inline(always)]
    pub fn ens(&mut self) -> ENS_W<6> {
        ENS_W::new(self)
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
