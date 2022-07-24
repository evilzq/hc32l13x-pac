#[doc = "Register `HCDOR` reader"]
pub struct R(crate::R<HCDOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDOR` writer"]
pub struct W(crate::W<HCDOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDOR_SPEC>;
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
impl From<crate::W<HCDOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCDO0` reader - desc HCDO0"]
pub type HCDO0_R = crate::BitReader<bool>;
#[doc = "Field `HCDO0` writer - desc HCDO0"]
pub type HCDO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO1` reader - desc HCDO1"]
pub type HCDO1_R = crate::BitReader<bool>;
#[doc = "Field `HCDO1` writer - desc HCDO1"]
pub type HCDO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO2` reader - desc HCDO2"]
pub type HCDO2_R = crate::BitReader<bool>;
#[doc = "Field `HCDO2` writer - desc HCDO2"]
pub type HCDO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO3` reader - desc HCDO3"]
pub type HCDO3_R = crate::BitReader<bool>;
#[doc = "Field `HCDO3` writer - desc HCDO3"]
pub type HCDO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO4` reader - desc HCDO4"]
pub type HCDO4_R = crate::BitReader<bool>;
#[doc = "Field `HCDO4` writer - desc HCDO4"]
pub type HCDO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO5` reader - desc HCDO5"]
pub type HCDO5_R = crate::BitReader<bool>;
#[doc = "Field `HCDO5` writer - desc HCDO5"]
pub type HCDO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO6` reader - desc HCDO6"]
pub type HCDO6_R = crate::BitReader<bool>;
#[doc = "Field `HCDO6` writer - desc HCDO6"]
pub type HCDO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO7` reader - desc HCDO7"]
pub type HCDO7_R = crate::BitReader<bool>;
#[doc = "Field `HCDO7` writer - desc HCDO7"]
pub type HCDO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO8` reader - desc HCDO8"]
pub type HCDO8_R = crate::BitReader<bool>;
#[doc = "Field `HCDO8` writer - desc HCDO8"]
pub type HCDO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO9` reader - desc HCDO9"]
pub type HCDO9_R = crate::BitReader<bool>;
#[doc = "Field `HCDO9` writer - desc HCDO9"]
pub type HCDO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO10` reader - desc HCDO10"]
pub type HCDO10_R = crate::BitReader<bool>;
#[doc = "Field `HCDO10` writer - desc HCDO10"]
pub type HCDO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO11` reader - desc HCDO11"]
pub type HCDO11_R = crate::BitReader<bool>;
#[doc = "Field `HCDO11` writer - desc HCDO11"]
pub type HCDO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO12` reader - desc HCDO12"]
pub type HCDO12_R = crate::BitReader<bool>;
#[doc = "Field `HCDO12` writer - desc HCDO12"]
pub type HCDO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO13` reader - desc HCDO13"]
pub type HCDO13_R = crate::BitReader<bool>;
#[doc = "Field `HCDO13` writer - desc HCDO13"]
pub type HCDO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO14` reader - desc HCDO14"]
pub type HCDO14_R = crate::BitReader<bool>;
#[doc = "Field `HCDO14` writer - desc HCDO14"]
pub type HCDO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO15` reader - desc HCDO15"]
pub type HCDO15_R = crate::BitReader<bool>;
#[doc = "Field `HCDO15` writer - desc HCDO15"]
pub type HCDO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO16` reader - desc HCDO16"]
pub type HCDO16_R = crate::BitReader<bool>;
#[doc = "Field `HCDO16` writer - desc HCDO16"]
pub type HCDO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO17` reader - desc HCDO17"]
pub type HCDO17_R = crate::BitReader<bool>;
#[doc = "Field `HCDO17` writer - desc HCDO17"]
pub type HCDO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO18` reader - desc HCDO18"]
pub type HCDO18_R = crate::BitReader<bool>;
#[doc = "Field `HCDO18` writer - desc HCDO18"]
pub type HCDO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `HCDO19` reader - desc HCDO19"]
pub type HCDO19_R = crate::BitReader<bool>;
#[doc = "Field `HCDO19` writer - desc HCDO19"]
pub type HCDO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCDOR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc HCDO0"]
    #[inline(always)]
    pub fn hcdo0(&self) -> HCDO0_R {
        HCDO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCDO1"]
    #[inline(always)]
    pub fn hcdo1(&self) -> HCDO1_R {
        HCDO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCDO2"]
    #[inline(always)]
    pub fn hcdo2(&self) -> HCDO2_R {
        HCDO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCDO3"]
    #[inline(always)]
    pub fn hcdo3(&self) -> HCDO3_R {
        HCDO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCDO4"]
    #[inline(always)]
    pub fn hcdo4(&self) -> HCDO4_R {
        HCDO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCDO5"]
    #[inline(always)]
    pub fn hcdo5(&self) -> HCDO5_R {
        HCDO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCDO6"]
    #[inline(always)]
    pub fn hcdo6(&self) -> HCDO6_R {
        HCDO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCDO7"]
    #[inline(always)]
    pub fn hcdo7(&self) -> HCDO7_R {
        HCDO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCDO8"]
    #[inline(always)]
    pub fn hcdo8(&self) -> HCDO8_R {
        HCDO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCDO9"]
    #[inline(always)]
    pub fn hcdo9(&self) -> HCDO9_R {
        HCDO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCDO10"]
    #[inline(always)]
    pub fn hcdo10(&self) -> HCDO10_R {
        HCDO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCDO11"]
    #[inline(always)]
    pub fn hcdo11(&self) -> HCDO11_R {
        HCDO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HCDO12"]
    #[inline(always)]
    pub fn hcdo12(&self) -> HCDO12_R {
        HCDO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HCDO13"]
    #[inline(always)]
    pub fn hcdo13(&self) -> HCDO13_R {
        HCDO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HCDO14"]
    #[inline(always)]
    pub fn hcdo14(&self) -> HCDO14_R {
        HCDO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HCDO15"]
    #[inline(always)]
    pub fn hcdo15(&self) -> HCDO15_R {
        HCDO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HCDO16"]
    #[inline(always)]
    pub fn hcdo16(&self) -> HCDO16_R {
        HCDO16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HCDO17"]
    #[inline(always)]
    pub fn hcdo17(&self) -> HCDO17_R {
        HCDO17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HCDO18"]
    #[inline(always)]
    pub fn hcdo18(&self) -> HCDO18_R {
        HCDO18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HCDO19"]
    #[inline(always)]
    pub fn hcdo19(&self) -> HCDO19_R {
        HCDO19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCDO0"]
    #[inline(always)]
    pub fn hcdo0(&mut self) -> HCDO0_W<0> {
        HCDO0_W::new(self)
    }
    #[doc = "Bit 1 - desc HCDO1"]
    #[inline(always)]
    pub fn hcdo1(&mut self) -> HCDO1_W<1> {
        HCDO1_W::new(self)
    }
    #[doc = "Bit 2 - desc HCDO2"]
    #[inline(always)]
    pub fn hcdo2(&mut self) -> HCDO2_W<2> {
        HCDO2_W::new(self)
    }
    #[doc = "Bit 3 - desc HCDO3"]
    #[inline(always)]
    pub fn hcdo3(&mut self) -> HCDO3_W<3> {
        HCDO3_W::new(self)
    }
    #[doc = "Bit 4 - desc HCDO4"]
    #[inline(always)]
    pub fn hcdo4(&mut self) -> HCDO4_W<4> {
        HCDO4_W::new(self)
    }
    #[doc = "Bit 5 - desc HCDO5"]
    #[inline(always)]
    pub fn hcdo5(&mut self) -> HCDO5_W<5> {
        HCDO5_W::new(self)
    }
    #[doc = "Bit 6 - desc HCDO6"]
    #[inline(always)]
    pub fn hcdo6(&mut self) -> HCDO6_W<6> {
        HCDO6_W::new(self)
    }
    #[doc = "Bit 7 - desc HCDO7"]
    #[inline(always)]
    pub fn hcdo7(&mut self) -> HCDO7_W<7> {
        HCDO7_W::new(self)
    }
    #[doc = "Bit 8 - desc HCDO8"]
    #[inline(always)]
    pub fn hcdo8(&mut self) -> HCDO8_W<8> {
        HCDO8_W::new(self)
    }
    #[doc = "Bit 9 - desc HCDO9"]
    #[inline(always)]
    pub fn hcdo9(&mut self) -> HCDO9_W<9> {
        HCDO9_W::new(self)
    }
    #[doc = "Bit 10 - desc HCDO10"]
    #[inline(always)]
    pub fn hcdo10(&mut self) -> HCDO10_W<10> {
        HCDO10_W::new(self)
    }
    #[doc = "Bit 11 - desc HCDO11"]
    #[inline(always)]
    pub fn hcdo11(&mut self) -> HCDO11_W<11> {
        HCDO11_W::new(self)
    }
    #[doc = "Bit 12 - desc HCDO12"]
    #[inline(always)]
    pub fn hcdo12(&mut self) -> HCDO12_W<12> {
        HCDO12_W::new(self)
    }
    #[doc = "Bit 13 - desc HCDO13"]
    #[inline(always)]
    pub fn hcdo13(&mut self) -> HCDO13_W<13> {
        HCDO13_W::new(self)
    }
    #[doc = "Bit 14 - desc HCDO14"]
    #[inline(always)]
    pub fn hcdo14(&mut self) -> HCDO14_W<14> {
        HCDO14_W::new(self)
    }
    #[doc = "Bit 15 - desc HCDO15"]
    #[inline(always)]
    pub fn hcdo15(&mut self) -> HCDO15_W<15> {
        HCDO15_W::new(self)
    }
    #[doc = "Bit 16 - desc HCDO16"]
    #[inline(always)]
    pub fn hcdo16(&mut self) -> HCDO16_W<16> {
        HCDO16_W::new(self)
    }
    #[doc = "Bit 17 - desc HCDO17"]
    #[inline(always)]
    pub fn hcdo17(&mut self) -> HCDO17_W<17> {
        HCDO17_W::new(self)
    }
    #[doc = "Bit 18 - desc HCDO18"]
    #[inline(always)]
    pub fn hcdo18(&mut self) -> HCDO18_W<18> {
        HCDO18_W::new(self)
    }
    #[doc = "Bit 19 - desc HCDO19"]
    #[inline(always)]
    pub fn hcdo19(&mut self) -> HCDO19_W<19> {
        HCDO19_W::new(self)
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
#[doc = "desc HCDOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdor](index.html) module"]
pub struct HCDOR_SPEC;
impl crate::RegisterSpec for HCDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdor::R](R) reader structure"]
impl crate::Readable for HCDOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdor::W](W) writer structure"]
impl crate::Writable for HCDOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCDOR to value 0"]
impl crate::Resettable for HCDOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
