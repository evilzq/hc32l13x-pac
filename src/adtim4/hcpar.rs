#[doc = "Register `HCPAR` reader"]
pub struct R(crate::R<HCPAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCPAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCPAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCPAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCPAR` writer"]
pub struct W(crate::W<HCPAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCPAR_SPEC>;
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
impl From<crate::W<HCPAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCPAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCPA0` reader - desc HCPA0"]
pub type HCPA0_R = crate::BitReader<bool>;
#[doc = "Field `HCPA0` writer - desc HCPA0"]
pub type HCPA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA1` reader - desc HCPA1"]
pub type HCPA1_R = crate::BitReader<bool>;
#[doc = "Field `HCPA1` writer - desc HCPA1"]
pub type HCPA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA2` reader - desc HCPA2"]
pub type HCPA2_R = crate::BitReader<bool>;
#[doc = "Field `HCPA2` writer - desc HCPA2"]
pub type HCPA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA3` reader - desc HCPA3"]
pub type HCPA3_R = crate::BitReader<bool>;
#[doc = "Field `HCPA3` writer - desc HCPA3"]
pub type HCPA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA4` reader - desc HCPA4"]
pub type HCPA4_R = crate::BitReader<bool>;
#[doc = "Field `HCPA4` writer - desc HCPA4"]
pub type HCPA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA5` reader - desc HCPA5"]
pub type HCPA5_R = crate::BitReader<bool>;
#[doc = "Field `HCPA5` writer - desc HCPA5"]
pub type HCPA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA6` reader - desc HCPA6"]
pub type HCPA6_R = crate::BitReader<bool>;
#[doc = "Field `HCPA6` writer - desc HCPA6"]
pub type HCPA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA7` reader - desc HCPA7"]
pub type HCPA7_R = crate::BitReader<bool>;
#[doc = "Field `HCPA7` writer - desc HCPA7"]
pub type HCPA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA8` reader - desc HCPA8"]
pub type HCPA8_R = crate::BitReader<bool>;
#[doc = "Field `HCPA8` writer - desc HCPA8"]
pub type HCPA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA9` reader - desc HCPA9"]
pub type HCPA9_R = crate::BitReader<bool>;
#[doc = "Field `HCPA9` writer - desc HCPA9"]
pub type HCPA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA10` reader - desc HCPA10"]
pub type HCPA10_R = crate::BitReader<bool>;
#[doc = "Field `HCPA10` writer - desc HCPA10"]
pub type HCPA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA11` reader - desc HCPA11"]
pub type HCPA11_R = crate::BitReader<bool>;
#[doc = "Field `HCPA11` writer - desc HCPA11"]
pub type HCPA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA12` reader - desc HCPA12"]
pub type HCPA12_R = crate::BitReader<bool>;
#[doc = "Field `HCPA12` writer - desc HCPA12"]
pub type HCPA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA13` reader - desc HCPA13"]
pub type HCPA13_R = crate::BitReader<bool>;
#[doc = "Field `HCPA13` writer - desc HCPA13"]
pub type HCPA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA14` reader - desc HCPA14"]
pub type HCPA14_R = crate::BitReader<bool>;
#[doc = "Field `HCPA14` writer - desc HCPA14"]
pub type HCPA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `HCPA15` reader - desc HCPA15"]
pub type HCPA15_R = crate::BitReader<bool>;
#[doc = "Field `HCPA15` writer - desc HCPA15"]
pub type HCPA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc HCPA0"]
    #[inline(always)]
    pub fn hcpa0(&self) -> HCPA0_R {
        HCPA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCPA1"]
    #[inline(always)]
    pub fn hcpa1(&self) -> HCPA1_R {
        HCPA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCPA2"]
    #[inline(always)]
    pub fn hcpa2(&self) -> HCPA2_R {
        HCPA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCPA3"]
    #[inline(always)]
    pub fn hcpa3(&self) -> HCPA3_R {
        HCPA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCPA4"]
    #[inline(always)]
    pub fn hcpa4(&self) -> HCPA4_R {
        HCPA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCPA5"]
    #[inline(always)]
    pub fn hcpa5(&self) -> HCPA5_R {
        HCPA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCPA6"]
    #[inline(always)]
    pub fn hcpa6(&self) -> HCPA6_R {
        HCPA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCPA7"]
    #[inline(always)]
    pub fn hcpa7(&self) -> HCPA7_R {
        HCPA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCPA8"]
    #[inline(always)]
    pub fn hcpa8(&self) -> HCPA8_R {
        HCPA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCPA9"]
    #[inline(always)]
    pub fn hcpa9(&self) -> HCPA9_R {
        HCPA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCPA10"]
    #[inline(always)]
    pub fn hcpa10(&self) -> HCPA10_R {
        HCPA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCPA11"]
    #[inline(always)]
    pub fn hcpa11(&self) -> HCPA11_R {
        HCPA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HCPA12"]
    #[inline(always)]
    pub fn hcpa12(&self) -> HCPA12_R {
        HCPA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HCPA13"]
    #[inline(always)]
    pub fn hcpa13(&self) -> HCPA13_R {
        HCPA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HCPA14"]
    #[inline(always)]
    pub fn hcpa14(&self) -> HCPA14_R {
        HCPA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HCPA15"]
    #[inline(always)]
    pub fn hcpa15(&self) -> HCPA15_R {
        HCPA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCPA0"]
    #[inline(always)]
    pub fn hcpa0(&mut self) -> HCPA0_W<0> {
        HCPA0_W::new(self)
    }
    #[doc = "Bit 1 - desc HCPA1"]
    #[inline(always)]
    pub fn hcpa1(&mut self) -> HCPA1_W<1> {
        HCPA1_W::new(self)
    }
    #[doc = "Bit 2 - desc HCPA2"]
    #[inline(always)]
    pub fn hcpa2(&mut self) -> HCPA2_W<2> {
        HCPA2_W::new(self)
    }
    #[doc = "Bit 3 - desc HCPA3"]
    #[inline(always)]
    pub fn hcpa3(&mut self) -> HCPA3_W<3> {
        HCPA3_W::new(self)
    }
    #[doc = "Bit 4 - desc HCPA4"]
    #[inline(always)]
    pub fn hcpa4(&mut self) -> HCPA4_W<4> {
        HCPA4_W::new(self)
    }
    #[doc = "Bit 5 - desc HCPA5"]
    #[inline(always)]
    pub fn hcpa5(&mut self) -> HCPA5_W<5> {
        HCPA5_W::new(self)
    }
    #[doc = "Bit 6 - desc HCPA6"]
    #[inline(always)]
    pub fn hcpa6(&mut self) -> HCPA6_W<6> {
        HCPA6_W::new(self)
    }
    #[doc = "Bit 7 - desc HCPA7"]
    #[inline(always)]
    pub fn hcpa7(&mut self) -> HCPA7_W<7> {
        HCPA7_W::new(self)
    }
    #[doc = "Bit 8 - desc HCPA8"]
    #[inline(always)]
    pub fn hcpa8(&mut self) -> HCPA8_W<8> {
        HCPA8_W::new(self)
    }
    #[doc = "Bit 9 - desc HCPA9"]
    #[inline(always)]
    pub fn hcpa9(&mut self) -> HCPA9_W<9> {
        HCPA9_W::new(self)
    }
    #[doc = "Bit 10 - desc HCPA10"]
    #[inline(always)]
    pub fn hcpa10(&mut self) -> HCPA10_W<10> {
        HCPA10_W::new(self)
    }
    #[doc = "Bit 11 - desc HCPA11"]
    #[inline(always)]
    pub fn hcpa11(&mut self) -> HCPA11_W<11> {
        HCPA11_W::new(self)
    }
    #[doc = "Bit 12 - desc HCPA12"]
    #[inline(always)]
    pub fn hcpa12(&mut self) -> HCPA12_W<12> {
        HCPA12_W::new(self)
    }
    #[doc = "Bit 13 - desc HCPA13"]
    #[inline(always)]
    pub fn hcpa13(&mut self) -> HCPA13_W<13> {
        HCPA13_W::new(self)
    }
    #[doc = "Bit 14 - desc HCPA14"]
    #[inline(always)]
    pub fn hcpa14(&mut self) -> HCPA14_W<14> {
        HCPA14_W::new(self)
    }
    #[doc = "Bit 15 - desc HCPA15"]
    #[inline(always)]
    pub fn hcpa15(&mut self) -> HCPA15_W<15> {
        HCPA15_W::new(self)
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
#[doc = "desc HCPAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcpar](index.html) module"]
pub struct HCPAR_SPEC;
impl crate::RegisterSpec for HCPAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcpar::R](R) reader structure"]
impl crate::Readable for HCPAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcpar::W](W) writer structure"]
impl crate::Writable for HCPAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCPAR to value 0"]
impl crate::Resettable for HCPAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
