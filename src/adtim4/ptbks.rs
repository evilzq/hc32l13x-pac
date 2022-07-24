#[doc = "Register `PTBKS` reader"]
pub struct R(crate::R<PTBKS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTBKS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTBKS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTBKS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTBKS` writer"]
pub struct W(crate::W<PTBKS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTBKS_SPEC>;
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
impl From<crate::W<PTBKS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTBKS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN0` reader - desc EN0"]
pub type EN0_R = crate::BitReader<bool>;
#[doc = "Field `EN0` writer - desc EN0"]
pub type EN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN1` reader - desc EN1"]
pub type EN1_R = crate::BitReader<bool>;
#[doc = "Field `EN1` writer - desc EN1"]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN2` reader - desc EN2"]
pub type EN2_R = crate::BitReader<bool>;
#[doc = "Field `EN2` writer - desc EN2"]
pub type EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN3` reader - desc EN3"]
pub type EN3_R = crate::BitReader<bool>;
#[doc = "Field `EN3` writer - desc EN3"]
pub type EN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN4` reader - desc EN4"]
pub type EN4_R = crate::BitReader<bool>;
#[doc = "Field `EN4` writer - desc EN4"]
pub type EN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN5` reader - desc EN5"]
pub type EN5_R = crate::BitReader<bool>;
#[doc = "Field `EN5` writer - desc EN5"]
pub type EN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN6` reader - desc EN6"]
pub type EN6_R = crate::BitReader<bool>;
#[doc = "Field `EN6` writer - desc EN6"]
pub type EN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN7` reader - desc EN7"]
pub type EN7_R = crate::BitReader<bool>;
#[doc = "Field `EN7` writer - desc EN7"]
pub type EN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN8` reader - desc EN8"]
pub type EN8_R = crate::BitReader<bool>;
#[doc = "Field `EN8` writer - desc EN8"]
pub type EN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN9` reader - desc EN9"]
pub type EN9_R = crate::BitReader<bool>;
#[doc = "Field `EN9` writer - desc EN9"]
pub type EN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN10` reader - desc EN10"]
pub type EN10_R = crate::BitReader<bool>;
#[doc = "Field `EN10` writer - desc EN10"]
pub type EN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN11` reader - desc EN11"]
pub type EN11_R = crate::BitReader<bool>;
#[doc = "Field `EN11` writer - desc EN11"]
pub type EN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN12` reader - desc EN12"]
pub type EN12_R = crate::BitReader<bool>;
#[doc = "Field `EN12` writer - desc EN12"]
pub type EN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN13` reader - desc EN13"]
pub type EN13_R = crate::BitReader<bool>;
#[doc = "Field `EN13` writer - desc EN13"]
pub type EN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN14` reader - desc EN14"]
pub type EN14_R = crate::BitReader<bool>;
#[doc = "Field `EN14` writer - desc EN14"]
pub type EN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `EN15` reader - desc EN15"]
pub type EN15_R = crate::BitReader<bool>;
#[doc = "Field `EN15` writer - desc EN15"]
pub type EN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc EN0"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EN1"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EN2"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc EN3"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc EN4"]
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EN5"]
    #[inline(always)]
    pub fn en5(&self) -> EN5_R {
        EN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EN6"]
    #[inline(always)]
    pub fn en6(&self) -> EN6_R {
        EN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc EN7"]
    #[inline(always)]
    pub fn en7(&self) -> EN7_R {
        EN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EN8"]
    #[inline(always)]
    pub fn en8(&self) -> EN8_R {
        EN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EN9"]
    #[inline(always)]
    pub fn en9(&self) -> EN9_R {
        EN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc EN10"]
    #[inline(always)]
    pub fn en10(&self) -> EN10_R {
        EN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc EN11"]
    #[inline(always)]
    pub fn en11(&self) -> EN11_R {
        EN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EN12"]
    #[inline(always)]
    pub fn en12(&self) -> EN12_R {
        EN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc EN13"]
    #[inline(always)]
    pub fn en13(&self) -> EN13_R {
        EN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc EN14"]
    #[inline(always)]
    pub fn en14(&self) -> EN14_R {
        EN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EN15"]
    #[inline(always)]
    pub fn en15(&self) -> EN15_R {
        EN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN0"]
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W<0> {
        EN0_W::new(self)
    }
    #[doc = "Bit 1 - desc EN1"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<1> {
        EN1_W::new(self)
    }
    #[doc = "Bit 2 - desc EN2"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W<2> {
        EN2_W::new(self)
    }
    #[doc = "Bit 3 - desc EN3"]
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W<3> {
        EN3_W::new(self)
    }
    #[doc = "Bit 4 - desc EN4"]
    #[inline(always)]
    pub fn en4(&mut self) -> EN4_W<4> {
        EN4_W::new(self)
    }
    #[doc = "Bit 5 - desc EN5"]
    #[inline(always)]
    pub fn en5(&mut self) -> EN5_W<5> {
        EN5_W::new(self)
    }
    #[doc = "Bit 6 - desc EN6"]
    #[inline(always)]
    pub fn en6(&mut self) -> EN6_W<6> {
        EN6_W::new(self)
    }
    #[doc = "Bit 7 - desc EN7"]
    #[inline(always)]
    pub fn en7(&mut self) -> EN7_W<7> {
        EN7_W::new(self)
    }
    #[doc = "Bit 8 - desc EN8"]
    #[inline(always)]
    pub fn en8(&mut self) -> EN8_W<8> {
        EN8_W::new(self)
    }
    #[doc = "Bit 9 - desc EN9"]
    #[inline(always)]
    pub fn en9(&mut self) -> EN9_W<9> {
        EN9_W::new(self)
    }
    #[doc = "Bit 10 - desc EN10"]
    #[inline(always)]
    pub fn en10(&mut self) -> EN10_W<10> {
        EN10_W::new(self)
    }
    #[doc = "Bit 11 - desc EN11"]
    #[inline(always)]
    pub fn en11(&mut self) -> EN11_W<11> {
        EN11_W::new(self)
    }
    #[doc = "Bit 12 - desc EN12"]
    #[inline(always)]
    pub fn en12(&mut self) -> EN12_W<12> {
        EN12_W::new(self)
    }
    #[doc = "Bit 13 - desc EN13"]
    #[inline(always)]
    pub fn en13(&mut self) -> EN13_W<13> {
        EN13_W::new(self)
    }
    #[doc = "Bit 14 - desc EN14"]
    #[inline(always)]
    pub fn en14(&mut self) -> EN14_W<14> {
        EN14_W::new(self)
    }
    #[doc = "Bit 15 - desc EN15"]
    #[inline(always)]
    pub fn en15(&mut self) -> EN15_W<15> {
        EN15_W::new(self)
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
#[doc = "desc PTBKS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptbks](index.html) module"]
pub struct PTBKS_SPEC;
impl crate::RegisterSpec for PTBKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptbks::R](R) reader structure"]
impl crate::Readable for PTBKS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptbks::W](W) writer structure"]
impl crate::Writable for PTBKS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTBKS to value 0"]
impl crate::Resettable for PTBKS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
