#[doc = "Register `PAFIE` reader"]
pub struct R(crate::R<PAFIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAFIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAFIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAFIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAFIE` writer"]
pub struct W(crate::W<PAFIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAFIE_SPEC>;
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
impl From<crate::W<PAFIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAFIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA00` reader - desc PA00"]
pub type PA00_R = crate::BitReader<bool>;
#[doc = "Field `PA00` writer - desc PA00"]
pub type PA00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA01` reader - desc PA01"]
pub type PA01_R = crate::BitReader<bool>;
#[doc = "Field `PA01` writer - desc PA01"]
pub type PA01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA02` reader - desc PA02"]
pub type PA02_R = crate::BitReader<bool>;
#[doc = "Field `PA02` writer - desc PA02"]
pub type PA02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA03` reader - desc PA03"]
pub type PA03_R = crate::BitReader<bool>;
#[doc = "Field `PA03` writer - desc PA03"]
pub type PA03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA04` reader - desc PA04"]
pub type PA04_R = crate::BitReader<bool>;
#[doc = "Field `PA04` writer - desc PA04"]
pub type PA04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA05` reader - desc PA05"]
pub type PA05_R = crate::BitReader<bool>;
#[doc = "Field `PA05` writer - desc PA05"]
pub type PA05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA06` reader - desc PA06"]
pub type PA06_R = crate::BitReader<bool>;
#[doc = "Field `PA06` writer - desc PA06"]
pub type PA06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA07` reader - desc PA07"]
pub type PA07_R = crate::BitReader<bool>;
#[doc = "Field `PA07` writer - desc PA07"]
pub type PA07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA08` reader - desc PA08"]
pub type PA08_R = crate::BitReader<bool>;
#[doc = "Field `PA08` writer - desc PA08"]
pub type PA08_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA09` reader - desc PA09"]
pub type PA09_R = crate::BitReader<bool>;
#[doc = "Field `PA09` writer - desc PA09"]
pub type PA09_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA10` reader - desc PA10"]
pub type PA10_R = crate::BitReader<bool>;
#[doc = "Field `PA10` writer - desc PA10"]
pub type PA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA11` reader - desc PA11"]
pub type PA11_R = crate::BitReader<bool>;
#[doc = "Field `PA11` writer - desc PA11"]
pub type PA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA12` reader - desc PA12"]
pub type PA12_R = crate::BitReader<bool>;
#[doc = "Field `PA12` writer - desc PA12"]
pub type PA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA13` reader - desc PA13"]
pub type PA13_R = crate::BitReader<bool>;
#[doc = "Field `PA13` writer - desc PA13"]
pub type PA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA14` reader - desc PA14"]
pub type PA14_R = crate::BitReader<bool>;
#[doc = "Field `PA14` writer - desc PA14"]
pub type PA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `PA15` reader - desc PA15"]
pub type PA15_R = crate::BitReader<bool>;
#[doc = "Field `PA15` writer - desc PA15"]
pub type PA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAFIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PA00"]
    #[inline(always)]
    pub fn pa00(&self) -> PA00_R {
        PA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PA01"]
    #[inline(always)]
    pub fn pa01(&self) -> PA01_R {
        PA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PA02"]
    #[inline(always)]
    pub fn pa02(&self) -> PA02_R {
        PA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PA03"]
    #[inline(always)]
    pub fn pa03(&self) -> PA03_R {
        PA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PA04"]
    #[inline(always)]
    pub fn pa04(&self) -> PA04_R {
        PA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PA05"]
    #[inline(always)]
    pub fn pa05(&self) -> PA05_R {
        PA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PA06"]
    #[inline(always)]
    pub fn pa06(&self) -> PA06_R {
        PA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PA07"]
    #[inline(always)]
    pub fn pa07(&self) -> PA07_R {
        PA07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PA08"]
    #[inline(always)]
    pub fn pa08(&self) -> PA08_R {
        PA08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PA09"]
    #[inline(always)]
    pub fn pa09(&self) -> PA09_R {
        PA09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PA10"]
    #[inline(always)]
    pub fn pa10(&self) -> PA10_R {
        PA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PA11"]
    #[inline(always)]
    pub fn pa11(&self) -> PA11_R {
        PA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PA12"]
    #[inline(always)]
    pub fn pa12(&self) -> PA12_R {
        PA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PA13"]
    #[inline(always)]
    pub fn pa13(&self) -> PA13_R {
        PA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PA14"]
    #[inline(always)]
    pub fn pa14(&self) -> PA14_R {
        PA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PA15"]
    #[inline(always)]
    pub fn pa15(&self) -> PA15_R {
        PA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PA00"]
    #[inline(always)]
    pub fn pa00(&mut self) -> PA00_W<0> {
        PA00_W::new(self)
    }
    #[doc = "Bit 1 - desc PA01"]
    #[inline(always)]
    pub fn pa01(&mut self) -> PA01_W<1> {
        PA01_W::new(self)
    }
    #[doc = "Bit 2 - desc PA02"]
    #[inline(always)]
    pub fn pa02(&mut self) -> PA02_W<2> {
        PA02_W::new(self)
    }
    #[doc = "Bit 3 - desc PA03"]
    #[inline(always)]
    pub fn pa03(&mut self) -> PA03_W<3> {
        PA03_W::new(self)
    }
    #[doc = "Bit 4 - desc PA04"]
    #[inline(always)]
    pub fn pa04(&mut self) -> PA04_W<4> {
        PA04_W::new(self)
    }
    #[doc = "Bit 5 - desc PA05"]
    #[inline(always)]
    pub fn pa05(&mut self) -> PA05_W<5> {
        PA05_W::new(self)
    }
    #[doc = "Bit 6 - desc PA06"]
    #[inline(always)]
    pub fn pa06(&mut self) -> PA06_W<6> {
        PA06_W::new(self)
    }
    #[doc = "Bit 7 - desc PA07"]
    #[inline(always)]
    pub fn pa07(&mut self) -> PA07_W<7> {
        PA07_W::new(self)
    }
    #[doc = "Bit 8 - desc PA08"]
    #[inline(always)]
    pub fn pa08(&mut self) -> PA08_W<8> {
        PA08_W::new(self)
    }
    #[doc = "Bit 9 - desc PA09"]
    #[inline(always)]
    pub fn pa09(&mut self) -> PA09_W<9> {
        PA09_W::new(self)
    }
    #[doc = "Bit 10 - desc PA10"]
    #[inline(always)]
    pub fn pa10(&mut self) -> PA10_W<10> {
        PA10_W::new(self)
    }
    #[doc = "Bit 11 - desc PA11"]
    #[inline(always)]
    pub fn pa11(&mut self) -> PA11_W<11> {
        PA11_W::new(self)
    }
    #[doc = "Bit 12 - desc PA12"]
    #[inline(always)]
    pub fn pa12(&mut self) -> PA12_W<12> {
        PA12_W::new(self)
    }
    #[doc = "Bit 13 - desc PA13"]
    #[inline(always)]
    pub fn pa13(&mut self) -> PA13_W<13> {
        PA13_W::new(self)
    }
    #[doc = "Bit 14 - desc PA14"]
    #[inline(always)]
    pub fn pa14(&mut self) -> PA14_W<14> {
        PA14_W::new(self)
    }
    #[doc = "Bit 15 - desc PA15"]
    #[inline(always)]
    pub fn pa15(&mut self) -> PA15_W<15> {
        PA15_W::new(self)
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
#[doc = "desc PAFIE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pafie](index.html) module"]
pub struct PAFIE_SPEC;
impl crate::RegisterSpec for PAFIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pafie::R](R) reader structure"]
impl crate::Readable for PAFIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pafie::W](W) writer structure"]
impl crate::Writable for PAFIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAFIE to value 0xffff_ffff"]
impl crate::Resettable for PAFIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
