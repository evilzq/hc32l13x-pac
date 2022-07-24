#[doc = "Register `PCOD` reader"]
pub struct R(crate::R<PCOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCOD` writer"]
pub struct W(crate::W<PCOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCOD_SPEC>;
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
impl From<crate::W<PCOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC00` reader - desc PC00"]
pub type PC00_R = crate::BitReader<bool>;
#[doc = "Field `PC00` writer - desc PC00"]
pub type PC00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC01` reader - desc PC01"]
pub type PC01_R = crate::BitReader<bool>;
#[doc = "Field `PC01` writer - desc PC01"]
pub type PC01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC02` reader - desc PC02"]
pub type PC02_R = crate::BitReader<bool>;
#[doc = "Field `PC02` writer - desc PC02"]
pub type PC02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC03` reader - desc PC03"]
pub type PC03_R = crate::BitReader<bool>;
#[doc = "Field `PC03` writer - desc PC03"]
pub type PC03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC04` reader - desc PC04"]
pub type PC04_R = crate::BitReader<bool>;
#[doc = "Field `PC04` writer - desc PC04"]
pub type PC04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC05` reader - desc PC05"]
pub type PC05_R = crate::BitReader<bool>;
#[doc = "Field `PC05` writer - desc PC05"]
pub type PC05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC06` reader - desc PC06"]
pub type PC06_R = crate::BitReader<bool>;
#[doc = "Field `PC06` writer - desc PC06"]
pub type PC06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC07` reader - desc PC07"]
pub type PC07_R = crate::BitReader<bool>;
#[doc = "Field `PC07` writer - desc PC07"]
pub type PC07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC08` reader - desc PC08"]
pub type PC08_R = crate::BitReader<bool>;
#[doc = "Field `PC08` writer - desc PC08"]
pub type PC08_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC09` reader - desc PC09"]
pub type PC09_R = crate::BitReader<bool>;
#[doc = "Field `PC09` writer - desc PC09"]
pub type PC09_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC10` reader - desc PC10"]
pub type PC10_R = crate::BitReader<bool>;
#[doc = "Field `PC10` writer - desc PC10"]
pub type PC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC11` reader - desc PC11"]
pub type PC11_R = crate::BitReader<bool>;
#[doc = "Field `PC11` writer - desc PC11"]
pub type PC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC12` reader - desc PC12"]
pub type PC12_R = crate::BitReader<bool>;
#[doc = "Field `PC12` writer - desc PC12"]
pub type PC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC13` reader - desc PC13"]
pub type PC13_R = crate::BitReader<bool>;
#[doc = "Field `PC13` writer - desc PC13"]
pub type PC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC14` reader - desc PC14"]
pub type PC14_R = crate::BitReader<bool>;
#[doc = "Field `PC14` writer - desc PC14"]
pub type PC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `PC15` reader - desc PC15"]
pub type PC15_R = crate::BitReader<bool>;
#[doc = "Field `PC15` writer - desc PC15"]
pub type PC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PC00"]
    #[inline(always)]
    pub fn pc00(&self) -> PC00_R {
        PC00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PC01"]
    #[inline(always)]
    pub fn pc01(&self) -> PC01_R {
        PC01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PC02"]
    #[inline(always)]
    pub fn pc02(&self) -> PC02_R {
        PC02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PC03"]
    #[inline(always)]
    pub fn pc03(&self) -> PC03_R {
        PC03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PC04"]
    #[inline(always)]
    pub fn pc04(&self) -> PC04_R {
        PC04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PC05"]
    #[inline(always)]
    pub fn pc05(&self) -> PC05_R {
        PC05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PC06"]
    #[inline(always)]
    pub fn pc06(&self) -> PC06_R {
        PC06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PC07"]
    #[inline(always)]
    pub fn pc07(&self) -> PC07_R {
        PC07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PC08"]
    #[inline(always)]
    pub fn pc08(&self) -> PC08_R {
        PC08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PC09"]
    #[inline(always)]
    pub fn pc09(&self) -> PC09_R {
        PC09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PC10"]
    #[inline(always)]
    pub fn pc10(&self) -> PC10_R {
        PC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PC11"]
    #[inline(always)]
    pub fn pc11(&self) -> PC11_R {
        PC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PC12"]
    #[inline(always)]
    pub fn pc12(&self) -> PC12_R {
        PC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PC13"]
    #[inline(always)]
    pub fn pc13(&self) -> PC13_R {
        PC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PC14"]
    #[inline(always)]
    pub fn pc14(&self) -> PC14_R {
        PC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PC15"]
    #[inline(always)]
    pub fn pc15(&self) -> PC15_R {
        PC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PC00"]
    #[inline(always)]
    pub fn pc00(&mut self) -> PC00_W<0> {
        PC00_W::new(self)
    }
    #[doc = "Bit 1 - desc PC01"]
    #[inline(always)]
    pub fn pc01(&mut self) -> PC01_W<1> {
        PC01_W::new(self)
    }
    #[doc = "Bit 2 - desc PC02"]
    #[inline(always)]
    pub fn pc02(&mut self) -> PC02_W<2> {
        PC02_W::new(self)
    }
    #[doc = "Bit 3 - desc PC03"]
    #[inline(always)]
    pub fn pc03(&mut self) -> PC03_W<3> {
        PC03_W::new(self)
    }
    #[doc = "Bit 4 - desc PC04"]
    #[inline(always)]
    pub fn pc04(&mut self) -> PC04_W<4> {
        PC04_W::new(self)
    }
    #[doc = "Bit 5 - desc PC05"]
    #[inline(always)]
    pub fn pc05(&mut self) -> PC05_W<5> {
        PC05_W::new(self)
    }
    #[doc = "Bit 6 - desc PC06"]
    #[inline(always)]
    pub fn pc06(&mut self) -> PC06_W<6> {
        PC06_W::new(self)
    }
    #[doc = "Bit 7 - desc PC07"]
    #[inline(always)]
    pub fn pc07(&mut self) -> PC07_W<7> {
        PC07_W::new(self)
    }
    #[doc = "Bit 8 - desc PC08"]
    #[inline(always)]
    pub fn pc08(&mut self) -> PC08_W<8> {
        PC08_W::new(self)
    }
    #[doc = "Bit 9 - desc PC09"]
    #[inline(always)]
    pub fn pc09(&mut self) -> PC09_W<9> {
        PC09_W::new(self)
    }
    #[doc = "Bit 10 - desc PC10"]
    #[inline(always)]
    pub fn pc10(&mut self) -> PC10_W<10> {
        PC10_W::new(self)
    }
    #[doc = "Bit 11 - desc PC11"]
    #[inline(always)]
    pub fn pc11(&mut self) -> PC11_W<11> {
        PC11_W::new(self)
    }
    #[doc = "Bit 12 - desc PC12"]
    #[inline(always)]
    pub fn pc12(&mut self) -> PC12_W<12> {
        PC12_W::new(self)
    }
    #[doc = "Bit 13 - desc PC13"]
    #[inline(always)]
    pub fn pc13(&mut self) -> PC13_W<13> {
        PC13_W::new(self)
    }
    #[doc = "Bit 14 - desc PC14"]
    #[inline(always)]
    pub fn pc14(&mut self) -> PC14_W<14> {
        PC14_W::new(self)
    }
    #[doc = "Bit 15 - desc PC15"]
    #[inline(always)]
    pub fn pc15(&mut self) -> PC15_W<15> {
        PC15_W::new(self)
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
#[doc = "desc PCOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcod](index.html) module"]
pub struct PCOD_SPEC;
impl crate::RegisterSpec for PCOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcod::R](R) reader structure"]
impl crate::Readable for PCOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcod::W](W) writer structure"]
impl crate::Writable for PCOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCOD to value 0xffff_ffff"]
impl crate::Resettable for PCOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
