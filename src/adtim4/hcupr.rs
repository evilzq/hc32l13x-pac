#[doc = "Register `HCUPR` reader"]
pub struct R(crate::R<HCUPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCUPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCUPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCUPR` writer"]
pub struct W(crate::W<HCUPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCUPR_SPEC>;
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
impl From<crate::W<HCUPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCUPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCUP0` reader - desc HCUP0"]
pub type HCUP0_R = crate::BitReader<bool>;
#[doc = "Field `HCUP0` writer - desc HCUP0"]
pub type HCUP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP1` reader - desc HCUP1"]
pub type HCUP1_R = crate::BitReader<bool>;
#[doc = "Field `HCUP1` writer - desc HCUP1"]
pub type HCUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP2` reader - desc HCUP2"]
pub type HCUP2_R = crate::BitReader<bool>;
#[doc = "Field `HCUP2` writer - desc HCUP2"]
pub type HCUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP3` reader - desc HCUP3"]
pub type HCUP3_R = crate::BitReader<bool>;
#[doc = "Field `HCUP3` writer - desc HCUP3"]
pub type HCUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP4` reader - desc HCUP4"]
pub type HCUP4_R = crate::BitReader<bool>;
#[doc = "Field `HCUP4` writer - desc HCUP4"]
pub type HCUP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP5` reader - desc HCUP5"]
pub type HCUP5_R = crate::BitReader<bool>;
#[doc = "Field `HCUP5` writer - desc HCUP5"]
pub type HCUP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP6` reader - desc HCUP6"]
pub type HCUP6_R = crate::BitReader<bool>;
#[doc = "Field `HCUP6` writer - desc HCUP6"]
pub type HCUP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP7` reader - desc HCUP7"]
pub type HCUP7_R = crate::BitReader<bool>;
#[doc = "Field `HCUP7` writer - desc HCUP7"]
pub type HCUP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP8` reader - desc HCUP8"]
pub type HCUP8_R = crate::BitReader<bool>;
#[doc = "Field `HCUP8` writer - desc HCUP8"]
pub type HCUP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP9` reader - desc HCUP9"]
pub type HCUP9_R = crate::BitReader<bool>;
#[doc = "Field `HCUP9` writer - desc HCUP9"]
pub type HCUP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP10` reader - desc HCUP10"]
pub type HCUP10_R = crate::BitReader<bool>;
#[doc = "Field `HCUP10` writer - desc HCUP10"]
pub type HCUP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP11` reader - desc HCUP11"]
pub type HCUP11_R = crate::BitReader<bool>;
#[doc = "Field `HCUP11` writer - desc HCUP11"]
pub type HCUP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP12` reader - desc HCUP12"]
pub type HCUP12_R = crate::BitReader<bool>;
#[doc = "Field `HCUP12` writer - desc HCUP12"]
pub type HCUP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP13` reader - desc HCUP13"]
pub type HCUP13_R = crate::BitReader<bool>;
#[doc = "Field `HCUP13` writer - desc HCUP13"]
pub type HCUP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP14` reader - desc HCUP14"]
pub type HCUP14_R = crate::BitReader<bool>;
#[doc = "Field `HCUP14` writer - desc HCUP14"]
pub type HCUP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP15` reader - desc HCUP15"]
pub type HCUP15_R = crate::BitReader<bool>;
#[doc = "Field `HCUP15` writer - desc HCUP15"]
pub type HCUP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP16` reader - desc HCUP16"]
pub type HCUP16_R = crate::BitReader<bool>;
#[doc = "Field `HCUP16` writer - desc HCUP16"]
pub type HCUP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP17` reader - desc HCUP17"]
pub type HCUP17_R = crate::BitReader<bool>;
#[doc = "Field `HCUP17` writer - desc HCUP17"]
pub type HCUP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP18` reader - desc HCUP18"]
pub type HCUP18_R = crate::BitReader<bool>;
#[doc = "Field `HCUP18` writer - desc HCUP18"]
pub type HCUP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `HCUP19` reader - desc HCUP19"]
pub type HCUP19_R = crate::BitReader<bool>;
#[doc = "Field `HCUP19` writer - desc HCUP19"]
pub type HCUP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCUPR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc HCUP0"]
    #[inline(always)]
    pub fn hcup0(&self) -> HCUP0_R {
        HCUP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCUP1"]
    #[inline(always)]
    pub fn hcup1(&self) -> HCUP1_R {
        HCUP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCUP2"]
    #[inline(always)]
    pub fn hcup2(&self) -> HCUP2_R {
        HCUP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCUP3"]
    #[inline(always)]
    pub fn hcup3(&self) -> HCUP3_R {
        HCUP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCUP4"]
    #[inline(always)]
    pub fn hcup4(&self) -> HCUP4_R {
        HCUP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCUP5"]
    #[inline(always)]
    pub fn hcup5(&self) -> HCUP5_R {
        HCUP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCUP6"]
    #[inline(always)]
    pub fn hcup6(&self) -> HCUP6_R {
        HCUP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCUP7"]
    #[inline(always)]
    pub fn hcup7(&self) -> HCUP7_R {
        HCUP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCUP8"]
    #[inline(always)]
    pub fn hcup8(&self) -> HCUP8_R {
        HCUP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCUP9"]
    #[inline(always)]
    pub fn hcup9(&self) -> HCUP9_R {
        HCUP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCUP10"]
    #[inline(always)]
    pub fn hcup10(&self) -> HCUP10_R {
        HCUP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCUP11"]
    #[inline(always)]
    pub fn hcup11(&self) -> HCUP11_R {
        HCUP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HCUP12"]
    #[inline(always)]
    pub fn hcup12(&self) -> HCUP12_R {
        HCUP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HCUP13"]
    #[inline(always)]
    pub fn hcup13(&self) -> HCUP13_R {
        HCUP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HCUP14"]
    #[inline(always)]
    pub fn hcup14(&self) -> HCUP14_R {
        HCUP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HCUP15"]
    #[inline(always)]
    pub fn hcup15(&self) -> HCUP15_R {
        HCUP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HCUP16"]
    #[inline(always)]
    pub fn hcup16(&self) -> HCUP16_R {
        HCUP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HCUP17"]
    #[inline(always)]
    pub fn hcup17(&self) -> HCUP17_R {
        HCUP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc HCUP18"]
    #[inline(always)]
    pub fn hcup18(&self) -> HCUP18_R {
        HCUP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc HCUP19"]
    #[inline(always)]
    pub fn hcup19(&self) -> HCUP19_R {
        HCUP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCUP0"]
    #[inline(always)]
    pub fn hcup0(&mut self) -> HCUP0_W<0> {
        HCUP0_W::new(self)
    }
    #[doc = "Bit 1 - desc HCUP1"]
    #[inline(always)]
    pub fn hcup1(&mut self) -> HCUP1_W<1> {
        HCUP1_W::new(self)
    }
    #[doc = "Bit 2 - desc HCUP2"]
    #[inline(always)]
    pub fn hcup2(&mut self) -> HCUP2_W<2> {
        HCUP2_W::new(self)
    }
    #[doc = "Bit 3 - desc HCUP3"]
    #[inline(always)]
    pub fn hcup3(&mut self) -> HCUP3_W<3> {
        HCUP3_W::new(self)
    }
    #[doc = "Bit 4 - desc HCUP4"]
    #[inline(always)]
    pub fn hcup4(&mut self) -> HCUP4_W<4> {
        HCUP4_W::new(self)
    }
    #[doc = "Bit 5 - desc HCUP5"]
    #[inline(always)]
    pub fn hcup5(&mut self) -> HCUP5_W<5> {
        HCUP5_W::new(self)
    }
    #[doc = "Bit 6 - desc HCUP6"]
    #[inline(always)]
    pub fn hcup6(&mut self) -> HCUP6_W<6> {
        HCUP6_W::new(self)
    }
    #[doc = "Bit 7 - desc HCUP7"]
    #[inline(always)]
    pub fn hcup7(&mut self) -> HCUP7_W<7> {
        HCUP7_W::new(self)
    }
    #[doc = "Bit 8 - desc HCUP8"]
    #[inline(always)]
    pub fn hcup8(&mut self) -> HCUP8_W<8> {
        HCUP8_W::new(self)
    }
    #[doc = "Bit 9 - desc HCUP9"]
    #[inline(always)]
    pub fn hcup9(&mut self) -> HCUP9_W<9> {
        HCUP9_W::new(self)
    }
    #[doc = "Bit 10 - desc HCUP10"]
    #[inline(always)]
    pub fn hcup10(&mut self) -> HCUP10_W<10> {
        HCUP10_W::new(self)
    }
    #[doc = "Bit 11 - desc HCUP11"]
    #[inline(always)]
    pub fn hcup11(&mut self) -> HCUP11_W<11> {
        HCUP11_W::new(self)
    }
    #[doc = "Bit 12 - desc HCUP12"]
    #[inline(always)]
    pub fn hcup12(&mut self) -> HCUP12_W<12> {
        HCUP12_W::new(self)
    }
    #[doc = "Bit 13 - desc HCUP13"]
    #[inline(always)]
    pub fn hcup13(&mut self) -> HCUP13_W<13> {
        HCUP13_W::new(self)
    }
    #[doc = "Bit 14 - desc HCUP14"]
    #[inline(always)]
    pub fn hcup14(&mut self) -> HCUP14_W<14> {
        HCUP14_W::new(self)
    }
    #[doc = "Bit 15 - desc HCUP15"]
    #[inline(always)]
    pub fn hcup15(&mut self) -> HCUP15_W<15> {
        HCUP15_W::new(self)
    }
    #[doc = "Bit 16 - desc HCUP16"]
    #[inline(always)]
    pub fn hcup16(&mut self) -> HCUP16_W<16> {
        HCUP16_W::new(self)
    }
    #[doc = "Bit 17 - desc HCUP17"]
    #[inline(always)]
    pub fn hcup17(&mut self) -> HCUP17_W<17> {
        HCUP17_W::new(self)
    }
    #[doc = "Bit 18 - desc HCUP18"]
    #[inline(always)]
    pub fn hcup18(&mut self) -> HCUP18_W<18> {
        HCUP18_W::new(self)
    }
    #[doc = "Bit 19 - desc HCUP19"]
    #[inline(always)]
    pub fn hcup19(&mut self) -> HCUP19_W<19> {
        HCUP19_W::new(self)
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
#[doc = "desc HCUPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcupr](index.html) module"]
pub struct HCUPR_SPEC;
impl crate::RegisterSpec for HCUPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcupr::R](R) reader structure"]
impl crate::Readable for HCUPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcupr::W](W) writer structure"]
impl crate::Writable for HCUPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCUPR to value 0"]
impl crate::Resettable for HCUPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
