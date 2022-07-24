#[doc = "Register `HCELR` reader"]
pub struct R(crate::R<HCELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCELR` writer"]
pub struct W(crate::W<HCELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCELR_SPEC>;
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
impl From<crate::W<HCELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCEL0` reader - desc HCEL0"]
pub type HCEL0_R = crate::BitReader<bool>;
#[doc = "Field `HCEL0` writer - desc HCEL0"]
pub type HCEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL1` reader - desc HCEL1"]
pub type HCEL1_R = crate::BitReader<bool>;
#[doc = "Field `HCEL1` writer - desc HCEL1"]
pub type HCEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL2` reader - desc HCEL2"]
pub type HCEL2_R = crate::BitReader<bool>;
#[doc = "Field `HCEL2` writer - desc HCEL2"]
pub type HCEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL3` reader - desc HCEL3"]
pub type HCEL3_R = crate::BitReader<bool>;
#[doc = "Field `HCEL3` writer - desc HCEL3"]
pub type HCEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL4` reader - desc HCEL4"]
pub type HCEL4_R = crate::BitReader<bool>;
#[doc = "Field `HCEL4` writer - desc HCEL4"]
pub type HCEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL5` reader - desc HCEL5"]
pub type HCEL5_R = crate::BitReader<bool>;
#[doc = "Field `HCEL5` writer - desc HCEL5"]
pub type HCEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL6` reader - desc HCEL6"]
pub type HCEL6_R = crate::BitReader<bool>;
#[doc = "Field `HCEL6` writer - desc HCEL6"]
pub type HCEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL7` reader - desc HCEL7"]
pub type HCEL7_R = crate::BitReader<bool>;
#[doc = "Field `HCEL7` writer - desc HCEL7"]
pub type HCEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL8` reader - desc HCEL8"]
pub type HCEL8_R = crate::BitReader<bool>;
#[doc = "Field `HCEL8` writer - desc HCEL8"]
pub type HCEL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL9` reader - desc HCEL9"]
pub type HCEL9_R = crate::BitReader<bool>;
#[doc = "Field `HCEL9` writer - desc HCEL9"]
pub type HCEL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL10` reader - desc HCEL10"]
pub type HCEL10_R = crate::BitReader<bool>;
#[doc = "Field `HCEL10` writer - desc HCEL10"]
pub type HCEL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL11` reader - desc HCEL11"]
pub type HCEL11_R = crate::BitReader<bool>;
#[doc = "Field `HCEL11` writer - desc HCEL11"]
pub type HCEL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL12` reader - desc HCEL12"]
pub type HCEL12_R = crate::BitReader<bool>;
#[doc = "Field `HCEL12` writer - desc HCEL12"]
pub type HCEL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL13` reader - desc HCEL13"]
pub type HCEL13_R = crate::BitReader<bool>;
#[doc = "Field `HCEL13` writer - desc HCEL13"]
pub type HCEL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL14` reader - desc HCEL14"]
pub type HCEL14_R = crate::BitReader<bool>;
#[doc = "Field `HCEL14` writer - desc HCEL14"]
pub type HCEL14_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `HCEL15` reader - desc HCEL15"]
pub type HCEL15_R = crate::BitReader<bool>;
#[doc = "Field `HCEL15` writer - desc HCEL15"]
pub type HCEL15_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
#[doc = "Field `CLEARS` reader - desc CLEARS"]
pub type CLEARS_R = crate::BitReader<bool>;
#[doc = "Field `CLEARS` writer - desc CLEARS"]
pub type CLEARS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCELR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc HCEL0"]
    #[inline(always)]
    pub fn hcel0(&self) -> HCEL0_R {
        HCEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCEL1"]
    #[inline(always)]
    pub fn hcel1(&self) -> HCEL1_R {
        HCEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCEL2"]
    #[inline(always)]
    pub fn hcel2(&self) -> HCEL2_R {
        HCEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCEL3"]
    #[inline(always)]
    pub fn hcel3(&self) -> HCEL3_R {
        HCEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCEL4"]
    #[inline(always)]
    pub fn hcel4(&self) -> HCEL4_R {
        HCEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCEL5"]
    #[inline(always)]
    pub fn hcel5(&self) -> HCEL5_R {
        HCEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCEL6"]
    #[inline(always)]
    pub fn hcel6(&self) -> HCEL6_R {
        HCEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCEL7"]
    #[inline(always)]
    pub fn hcel7(&self) -> HCEL7_R {
        HCEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCEL8"]
    #[inline(always)]
    pub fn hcel8(&self) -> HCEL8_R {
        HCEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCEL9"]
    #[inline(always)]
    pub fn hcel9(&self) -> HCEL9_R {
        HCEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCEL10"]
    #[inline(always)]
    pub fn hcel10(&self) -> HCEL10_R {
        HCEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCEL11"]
    #[inline(always)]
    pub fn hcel11(&self) -> HCEL11_R {
        HCEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HCEL12"]
    #[inline(always)]
    pub fn hcel12(&self) -> HCEL12_R {
        HCEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HCEL13"]
    #[inline(always)]
    pub fn hcel13(&self) -> HCEL13_R {
        HCEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HCEL14"]
    #[inline(always)]
    pub fn hcel14(&self) -> HCEL14_R {
        HCEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HCEL15"]
    #[inline(always)]
    pub fn hcel15(&self) -> HCEL15_R {
        HCEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc CLEARS"]
    #[inline(always)]
    pub fn clears(&self) -> CLEARS_R {
        CLEARS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCEL0"]
    #[inline(always)]
    pub fn hcel0(&mut self) -> HCEL0_W<0> {
        HCEL0_W::new(self)
    }
    #[doc = "Bit 1 - desc HCEL1"]
    #[inline(always)]
    pub fn hcel1(&mut self) -> HCEL1_W<1> {
        HCEL1_W::new(self)
    }
    #[doc = "Bit 2 - desc HCEL2"]
    #[inline(always)]
    pub fn hcel2(&mut self) -> HCEL2_W<2> {
        HCEL2_W::new(self)
    }
    #[doc = "Bit 3 - desc HCEL3"]
    #[inline(always)]
    pub fn hcel3(&mut self) -> HCEL3_W<3> {
        HCEL3_W::new(self)
    }
    #[doc = "Bit 4 - desc HCEL4"]
    #[inline(always)]
    pub fn hcel4(&mut self) -> HCEL4_W<4> {
        HCEL4_W::new(self)
    }
    #[doc = "Bit 5 - desc HCEL5"]
    #[inline(always)]
    pub fn hcel5(&mut self) -> HCEL5_W<5> {
        HCEL5_W::new(self)
    }
    #[doc = "Bit 6 - desc HCEL6"]
    #[inline(always)]
    pub fn hcel6(&mut self) -> HCEL6_W<6> {
        HCEL6_W::new(self)
    }
    #[doc = "Bit 7 - desc HCEL7"]
    #[inline(always)]
    pub fn hcel7(&mut self) -> HCEL7_W<7> {
        HCEL7_W::new(self)
    }
    #[doc = "Bit 8 - desc HCEL8"]
    #[inline(always)]
    pub fn hcel8(&mut self) -> HCEL8_W<8> {
        HCEL8_W::new(self)
    }
    #[doc = "Bit 9 - desc HCEL9"]
    #[inline(always)]
    pub fn hcel9(&mut self) -> HCEL9_W<9> {
        HCEL9_W::new(self)
    }
    #[doc = "Bit 10 - desc HCEL10"]
    #[inline(always)]
    pub fn hcel10(&mut self) -> HCEL10_W<10> {
        HCEL10_W::new(self)
    }
    #[doc = "Bit 11 - desc HCEL11"]
    #[inline(always)]
    pub fn hcel11(&mut self) -> HCEL11_W<11> {
        HCEL11_W::new(self)
    }
    #[doc = "Bit 12 - desc HCEL12"]
    #[inline(always)]
    pub fn hcel12(&mut self) -> HCEL12_W<12> {
        HCEL12_W::new(self)
    }
    #[doc = "Bit 13 - desc HCEL13"]
    #[inline(always)]
    pub fn hcel13(&mut self) -> HCEL13_W<13> {
        HCEL13_W::new(self)
    }
    #[doc = "Bit 14 - desc HCEL14"]
    #[inline(always)]
    pub fn hcel14(&mut self) -> HCEL14_W<14> {
        HCEL14_W::new(self)
    }
    #[doc = "Bit 15 - desc HCEL15"]
    #[inline(always)]
    pub fn hcel15(&mut self) -> HCEL15_W<15> {
        HCEL15_W::new(self)
    }
    #[doc = "Bit 31 - desc CLEARS"]
    #[inline(always)]
    pub fn clears(&mut self) -> CLEARS_W<31> {
        CLEARS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc HCELR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcelr](index.html) module"]
pub struct HCELR_SPEC;
impl crate::RegisterSpec for HCELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcelr::R](R) reader structure"]
impl crate::Readable for HCELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcelr::W](W) writer structure"]
impl crate::Writable for HCELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCELR to value 0"]
impl crate::Resettable for HCELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
