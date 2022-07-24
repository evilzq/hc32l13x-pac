#[doc = "Register `HSTAR` reader"]
pub struct R(crate::R<HSTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTAR` writer"]
pub struct W(crate::W<HSTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTAR_SPEC>;
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
impl From<crate::W<HSTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTA0` reader - desc HSTA0"]
pub type HSTA0_R = crate::BitReader<bool>;
#[doc = "Field `HSTA0` writer - desc HSTA0"]
pub type HSTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA1` reader - desc HSTA1"]
pub type HSTA1_R = crate::BitReader<bool>;
#[doc = "Field `HSTA1` writer - desc HSTA1"]
pub type HSTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA2` reader - desc HSTA2"]
pub type HSTA2_R = crate::BitReader<bool>;
#[doc = "Field `HSTA2` writer - desc HSTA2"]
pub type HSTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA3` reader - desc HSTA3"]
pub type HSTA3_R = crate::BitReader<bool>;
#[doc = "Field `HSTA3` writer - desc HSTA3"]
pub type HSTA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA4` reader - desc HSTA4"]
pub type HSTA4_R = crate::BitReader<bool>;
#[doc = "Field `HSTA4` writer - desc HSTA4"]
pub type HSTA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA5` reader - desc HSTA5"]
pub type HSTA5_R = crate::BitReader<bool>;
#[doc = "Field `HSTA5` writer - desc HSTA5"]
pub type HSTA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA6` reader - desc HSTA6"]
pub type HSTA6_R = crate::BitReader<bool>;
#[doc = "Field `HSTA6` writer - desc HSTA6"]
pub type HSTA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA7` reader - desc HSTA7"]
pub type HSTA7_R = crate::BitReader<bool>;
#[doc = "Field `HSTA7` writer - desc HSTA7"]
pub type HSTA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA8` reader - desc HSTA8"]
pub type HSTA8_R = crate::BitReader<bool>;
#[doc = "Field `HSTA8` writer - desc HSTA8"]
pub type HSTA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA9` reader - desc HSTA9"]
pub type HSTA9_R = crate::BitReader<bool>;
#[doc = "Field `HSTA9` writer - desc HSTA9"]
pub type HSTA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA10` reader - desc HSTA10"]
pub type HSTA10_R = crate::BitReader<bool>;
#[doc = "Field `HSTA10` writer - desc HSTA10"]
pub type HSTA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA11` reader - desc HSTA11"]
pub type HSTA11_R = crate::BitReader<bool>;
#[doc = "Field `HSTA11` writer - desc HSTA11"]
pub type HSTA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA12` reader - desc HSTA12"]
pub type HSTA12_R = crate::BitReader<bool>;
#[doc = "Field `HSTA12` writer - desc HSTA12"]
pub type HSTA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA13` reader - desc HSTA13"]
pub type HSTA13_R = crate::BitReader<bool>;
#[doc = "Field `HSTA13` writer - desc HSTA13"]
pub type HSTA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA14` reader - desc HSTA14"]
pub type HSTA14_R = crate::BitReader<bool>;
#[doc = "Field `HSTA14` writer - desc HSTA14"]
pub type HSTA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `HSTA15` reader - desc HSTA15"]
pub type HSTA15_R = crate::BitReader<bool>;
#[doc = "Field `HSTA15` writer - desc HSTA15"]
pub type HSTA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
#[doc = "Field `STARTS` reader - desc STARTS"]
pub type STARTS_R = crate::BitReader<bool>;
#[doc = "Field `STARTS` writer - desc STARTS"]
pub type STARTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc HSTA0"]
    #[inline(always)]
    pub fn hsta0(&self) -> HSTA0_R {
        HSTA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSTA1"]
    #[inline(always)]
    pub fn hsta1(&self) -> HSTA1_R {
        HSTA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HSTA2"]
    #[inline(always)]
    pub fn hsta2(&self) -> HSTA2_R {
        HSTA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HSTA3"]
    #[inline(always)]
    pub fn hsta3(&self) -> HSTA3_R {
        HSTA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HSTA4"]
    #[inline(always)]
    pub fn hsta4(&self) -> HSTA4_R {
        HSTA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HSTA5"]
    #[inline(always)]
    pub fn hsta5(&self) -> HSTA5_R {
        HSTA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSTA6"]
    #[inline(always)]
    pub fn hsta6(&self) -> HSTA6_R {
        HSTA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HSTA7"]
    #[inline(always)]
    pub fn hsta7(&self) -> HSTA7_R {
        HSTA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSTA8"]
    #[inline(always)]
    pub fn hsta8(&self) -> HSTA8_R {
        HSTA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HSTA9"]
    #[inline(always)]
    pub fn hsta9(&self) -> HSTA9_R {
        HSTA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HSTA10"]
    #[inline(always)]
    pub fn hsta10(&self) -> HSTA10_R {
        HSTA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HSTA11"]
    #[inline(always)]
    pub fn hsta11(&self) -> HSTA11_R {
        HSTA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HSTA12"]
    #[inline(always)]
    pub fn hsta12(&self) -> HSTA12_R {
        HSTA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HSTA13"]
    #[inline(always)]
    pub fn hsta13(&self) -> HSTA13_R {
        HSTA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HSTA14"]
    #[inline(always)]
    pub fn hsta14(&self) -> HSTA14_R {
        HSTA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HSTA15"]
    #[inline(always)]
    pub fn hsta15(&self) -> HSTA15_R {
        HSTA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc STARTS"]
    #[inline(always)]
    pub fn starts(&self) -> STARTS_R {
        STARTS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSTA0"]
    #[inline(always)]
    pub fn hsta0(&mut self) -> HSTA0_W<0> {
        HSTA0_W::new(self)
    }
    #[doc = "Bit 1 - desc HSTA1"]
    #[inline(always)]
    pub fn hsta1(&mut self) -> HSTA1_W<1> {
        HSTA1_W::new(self)
    }
    #[doc = "Bit 2 - desc HSTA2"]
    #[inline(always)]
    pub fn hsta2(&mut self) -> HSTA2_W<2> {
        HSTA2_W::new(self)
    }
    #[doc = "Bit 3 - desc HSTA3"]
    #[inline(always)]
    pub fn hsta3(&mut self) -> HSTA3_W<3> {
        HSTA3_W::new(self)
    }
    #[doc = "Bit 4 - desc HSTA4"]
    #[inline(always)]
    pub fn hsta4(&mut self) -> HSTA4_W<4> {
        HSTA4_W::new(self)
    }
    #[doc = "Bit 5 - desc HSTA5"]
    #[inline(always)]
    pub fn hsta5(&mut self) -> HSTA5_W<5> {
        HSTA5_W::new(self)
    }
    #[doc = "Bit 6 - desc HSTA6"]
    #[inline(always)]
    pub fn hsta6(&mut self) -> HSTA6_W<6> {
        HSTA6_W::new(self)
    }
    #[doc = "Bit 7 - desc HSTA7"]
    #[inline(always)]
    pub fn hsta7(&mut self) -> HSTA7_W<7> {
        HSTA7_W::new(self)
    }
    #[doc = "Bit 8 - desc HSTA8"]
    #[inline(always)]
    pub fn hsta8(&mut self) -> HSTA8_W<8> {
        HSTA8_W::new(self)
    }
    #[doc = "Bit 9 - desc HSTA9"]
    #[inline(always)]
    pub fn hsta9(&mut self) -> HSTA9_W<9> {
        HSTA9_W::new(self)
    }
    #[doc = "Bit 10 - desc HSTA10"]
    #[inline(always)]
    pub fn hsta10(&mut self) -> HSTA10_W<10> {
        HSTA10_W::new(self)
    }
    #[doc = "Bit 11 - desc HSTA11"]
    #[inline(always)]
    pub fn hsta11(&mut self) -> HSTA11_W<11> {
        HSTA11_W::new(self)
    }
    #[doc = "Bit 12 - desc HSTA12"]
    #[inline(always)]
    pub fn hsta12(&mut self) -> HSTA12_W<12> {
        HSTA12_W::new(self)
    }
    #[doc = "Bit 13 - desc HSTA13"]
    #[inline(always)]
    pub fn hsta13(&mut self) -> HSTA13_W<13> {
        HSTA13_W::new(self)
    }
    #[doc = "Bit 14 - desc HSTA14"]
    #[inline(always)]
    pub fn hsta14(&mut self) -> HSTA14_W<14> {
        HSTA14_W::new(self)
    }
    #[doc = "Bit 15 - desc HSTA15"]
    #[inline(always)]
    pub fn hsta15(&mut self) -> HSTA15_W<15> {
        HSTA15_W::new(self)
    }
    #[doc = "Bit 31 - desc STARTS"]
    #[inline(always)]
    pub fn starts(&mut self) -> STARTS_W<31> {
        STARTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc HSTAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstar](index.html) module"]
pub struct HSTAR_SPEC;
impl crate::RegisterSpec for HSTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstar::R](R) reader structure"]
impl crate::Readable for HSTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstar::W](W) writer structure"]
impl crate::Writable for HSTAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTAR to value 0"]
impl crate::Resettable for HSTAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
