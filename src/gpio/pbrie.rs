#[doc = "Register `PBRIE` reader"]
pub struct R(crate::R<PBRIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBRIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBRIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBRIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBRIE` writer"]
pub struct W(crate::W<PBRIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBRIE_SPEC>;
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
impl From<crate::W<PBRIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBRIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB00` reader - desc PB00"]
pub type PB00_R = crate::BitReader<bool>;
#[doc = "Field `PB00` writer - desc PB00"]
pub type PB00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB01` reader - desc PB01"]
pub type PB01_R = crate::BitReader<bool>;
#[doc = "Field `PB01` writer - desc PB01"]
pub type PB01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB02` reader - desc PB02"]
pub type PB02_R = crate::BitReader<bool>;
#[doc = "Field `PB02` writer - desc PB02"]
pub type PB02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB03` reader - desc PB03"]
pub type PB03_R = crate::BitReader<bool>;
#[doc = "Field `PB03` writer - desc PB03"]
pub type PB03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB04` reader - desc PB04"]
pub type PB04_R = crate::BitReader<bool>;
#[doc = "Field `PB04` writer - desc PB04"]
pub type PB04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB05` reader - desc PB05"]
pub type PB05_R = crate::BitReader<bool>;
#[doc = "Field `PB05` writer - desc PB05"]
pub type PB05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB06` reader - desc PB06"]
pub type PB06_R = crate::BitReader<bool>;
#[doc = "Field `PB06` writer - desc PB06"]
pub type PB06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB07` reader - desc PB07"]
pub type PB07_R = crate::BitReader<bool>;
#[doc = "Field `PB07` writer - desc PB07"]
pub type PB07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB08` reader - desc PB08"]
pub type PB08_R = crate::BitReader<bool>;
#[doc = "Field `PB08` writer - desc PB08"]
pub type PB08_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB09` reader - desc PB09"]
pub type PB09_R = crate::BitReader<bool>;
#[doc = "Field `PB09` writer - desc PB09"]
pub type PB09_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB10` reader - desc PB10"]
pub type PB10_R = crate::BitReader<bool>;
#[doc = "Field `PB10` writer - desc PB10"]
pub type PB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB11` reader - desc PB11"]
pub type PB11_R = crate::BitReader<bool>;
#[doc = "Field `PB11` writer - desc PB11"]
pub type PB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB12` reader - desc PB12"]
pub type PB12_R = crate::BitReader<bool>;
#[doc = "Field `PB12` writer - desc PB12"]
pub type PB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB13` reader - desc PB13"]
pub type PB13_R = crate::BitReader<bool>;
#[doc = "Field `PB13` writer - desc PB13"]
pub type PB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB14` reader - desc PB14"]
pub type PB14_R = crate::BitReader<bool>;
#[doc = "Field `PB14` writer - desc PB14"]
pub type PB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `PB15` reader - desc PB15"]
pub type PB15_R = crate::BitReader<bool>;
#[doc = "Field `PB15` writer - desc PB15"]
pub type PB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBRIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PB00"]
    #[inline(always)]
    pub fn pb00(&self) -> PB00_R {
        PB00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PB01"]
    #[inline(always)]
    pub fn pb01(&self) -> PB01_R {
        PB01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PB02"]
    #[inline(always)]
    pub fn pb02(&self) -> PB02_R {
        PB02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PB03"]
    #[inline(always)]
    pub fn pb03(&self) -> PB03_R {
        PB03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PB04"]
    #[inline(always)]
    pub fn pb04(&self) -> PB04_R {
        PB04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PB05"]
    #[inline(always)]
    pub fn pb05(&self) -> PB05_R {
        PB05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PB06"]
    #[inline(always)]
    pub fn pb06(&self) -> PB06_R {
        PB06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PB07"]
    #[inline(always)]
    pub fn pb07(&self) -> PB07_R {
        PB07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PB08"]
    #[inline(always)]
    pub fn pb08(&self) -> PB08_R {
        PB08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PB09"]
    #[inline(always)]
    pub fn pb09(&self) -> PB09_R {
        PB09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PB10"]
    #[inline(always)]
    pub fn pb10(&self) -> PB10_R {
        PB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PB11"]
    #[inline(always)]
    pub fn pb11(&self) -> PB11_R {
        PB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PB12"]
    #[inline(always)]
    pub fn pb12(&self) -> PB12_R {
        PB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PB13"]
    #[inline(always)]
    pub fn pb13(&self) -> PB13_R {
        PB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PB14"]
    #[inline(always)]
    pub fn pb14(&self) -> PB14_R {
        PB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PB15"]
    #[inline(always)]
    pub fn pb15(&self) -> PB15_R {
        PB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PB00"]
    #[inline(always)]
    pub fn pb00(&mut self) -> PB00_W<0> {
        PB00_W::new(self)
    }
    #[doc = "Bit 1 - desc PB01"]
    #[inline(always)]
    pub fn pb01(&mut self) -> PB01_W<1> {
        PB01_W::new(self)
    }
    #[doc = "Bit 2 - desc PB02"]
    #[inline(always)]
    pub fn pb02(&mut self) -> PB02_W<2> {
        PB02_W::new(self)
    }
    #[doc = "Bit 3 - desc PB03"]
    #[inline(always)]
    pub fn pb03(&mut self) -> PB03_W<3> {
        PB03_W::new(self)
    }
    #[doc = "Bit 4 - desc PB04"]
    #[inline(always)]
    pub fn pb04(&mut self) -> PB04_W<4> {
        PB04_W::new(self)
    }
    #[doc = "Bit 5 - desc PB05"]
    #[inline(always)]
    pub fn pb05(&mut self) -> PB05_W<5> {
        PB05_W::new(self)
    }
    #[doc = "Bit 6 - desc PB06"]
    #[inline(always)]
    pub fn pb06(&mut self) -> PB06_W<6> {
        PB06_W::new(self)
    }
    #[doc = "Bit 7 - desc PB07"]
    #[inline(always)]
    pub fn pb07(&mut self) -> PB07_W<7> {
        PB07_W::new(self)
    }
    #[doc = "Bit 8 - desc PB08"]
    #[inline(always)]
    pub fn pb08(&mut self) -> PB08_W<8> {
        PB08_W::new(self)
    }
    #[doc = "Bit 9 - desc PB09"]
    #[inline(always)]
    pub fn pb09(&mut self) -> PB09_W<9> {
        PB09_W::new(self)
    }
    #[doc = "Bit 10 - desc PB10"]
    #[inline(always)]
    pub fn pb10(&mut self) -> PB10_W<10> {
        PB10_W::new(self)
    }
    #[doc = "Bit 11 - desc PB11"]
    #[inline(always)]
    pub fn pb11(&mut self) -> PB11_W<11> {
        PB11_W::new(self)
    }
    #[doc = "Bit 12 - desc PB12"]
    #[inline(always)]
    pub fn pb12(&mut self) -> PB12_W<12> {
        PB12_W::new(self)
    }
    #[doc = "Bit 13 - desc PB13"]
    #[inline(always)]
    pub fn pb13(&mut self) -> PB13_W<13> {
        PB13_W::new(self)
    }
    #[doc = "Bit 14 - desc PB14"]
    #[inline(always)]
    pub fn pb14(&mut self) -> PB14_W<14> {
        PB14_W::new(self)
    }
    #[doc = "Bit 15 - desc PB15"]
    #[inline(always)]
    pub fn pb15(&mut self) -> PB15_W<15> {
        PB15_W::new(self)
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
#[doc = "desc PBRIE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbrie](index.html) module"]
pub struct PBRIE_SPEC;
impl crate::RegisterSpec for PBRIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbrie::R](R) reader structure"]
impl crate::Readable for PBRIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbrie::W](W) writer structure"]
impl crate::Writable for PBRIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBRIE to value 0xffff_ffff"]
impl crate::Resettable for PBRIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
