#[doc = "Register `PTBKP` reader"]
pub struct R(crate::R<PTBKP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTBKP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTBKP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTBKP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTBKP` writer"]
pub struct W(crate::W<PTBKP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTBKP_SPEC>;
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
impl From<crate::W<PTBKP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTBKP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL0` reader - desc POL0"]
pub type POL0_R = crate::BitReader<bool>;
#[doc = "Field `POL0` writer - desc POL0"]
pub type POL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL1` reader - desc POL1"]
pub type POL1_R = crate::BitReader<bool>;
#[doc = "Field `POL1` writer - desc POL1"]
pub type POL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL2` reader - desc POL2"]
pub type POL2_R = crate::BitReader<bool>;
#[doc = "Field `POL2` writer - desc POL2"]
pub type POL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL3` reader - desc POL3"]
pub type POL3_R = crate::BitReader<bool>;
#[doc = "Field `POL3` writer - desc POL3"]
pub type POL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL4` reader - desc POL4"]
pub type POL4_R = crate::BitReader<bool>;
#[doc = "Field `POL4` writer - desc POL4"]
pub type POL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL5` reader - desc POL5"]
pub type POL5_R = crate::BitReader<bool>;
#[doc = "Field `POL5` writer - desc POL5"]
pub type POL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL6` reader - desc POL6"]
pub type POL6_R = crate::BitReader<bool>;
#[doc = "Field `POL6` writer - desc POL6"]
pub type POL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL7` reader - desc POL7"]
pub type POL7_R = crate::BitReader<bool>;
#[doc = "Field `POL7` writer - desc POL7"]
pub type POL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL8` reader - desc POL8"]
pub type POL8_R = crate::BitReader<bool>;
#[doc = "Field `POL8` writer - desc POL8"]
pub type POL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL9` reader - desc POL9"]
pub type POL9_R = crate::BitReader<bool>;
#[doc = "Field `POL9` writer - desc POL9"]
pub type POL9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL10` reader - desc POL10"]
pub type POL10_R = crate::BitReader<bool>;
#[doc = "Field `POL10` writer - desc POL10"]
pub type POL10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL11` reader - desc POL11"]
pub type POL11_R = crate::BitReader<bool>;
#[doc = "Field `POL11` writer - desc POL11"]
pub type POL11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL12` reader - desc POL12"]
pub type POL12_R = crate::BitReader<bool>;
#[doc = "Field `POL12` writer - desc POL12"]
pub type POL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL13` reader - desc POL13"]
pub type POL13_R = crate::BitReader<bool>;
#[doc = "Field `POL13` writer - desc POL13"]
pub type POL13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL14` reader - desc POL14"]
pub type POL14_R = crate::BitReader<bool>;
#[doc = "Field `POL14` writer - desc POL14"]
pub type POL14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `POL15` reader - desc POL15"]
pub type POL15_R = crate::BitReader<bool>;
#[doc = "Field `POL15` writer - desc POL15"]
pub type POL15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTBKP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc POL0"]
    #[inline(always)]
    pub fn pol0(&self) -> POL0_R {
        POL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POL1"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POL2"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc POL3"]
    #[inline(always)]
    pub fn pol3(&self) -> POL3_R {
        POL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POL4"]
    #[inline(always)]
    pub fn pol4(&self) -> POL4_R {
        POL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc POL5"]
    #[inline(always)]
    pub fn pol5(&self) -> POL5_R {
        POL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc POL6"]
    #[inline(always)]
    pub fn pol6(&self) -> POL6_R {
        POL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc POL7"]
    #[inline(always)]
    pub fn pol7(&self) -> POL7_R {
        POL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc POL8"]
    #[inline(always)]
    pub fn pol8(&self) -> POL8_R {
        POL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc POL9"]
    #[inline(always)]
    pub fn pol9(&self) -> POL9_R {
        POL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc POL10"]
    #[inline(always)]
    pub fn pol10(&self) -> POL10_R {
        POL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POL11"]
    #[inline(always)]
    pub fn pol11(&self) -> POL11_R {
        POL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc POL12"]
    #[inline(always)]
    pub fn pol12(&self) -> POL12_R {
        POL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc POL13"]
    #[inline(always)]
    pub fn pol13(&self) -> POL13_R {
        POL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POL14"]
    #[inline(always)]
    pub fn pol14(&self) -> POL14_R {
        POL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POL15"]
    #[inline(always)]
    pub fn pol15(&self) -> POL15_R {
        POL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POL0"]
    #[inline(always)]
    pub fn pol0(&mut self) -> POL0_W<0> {
        POL0_W::new(self)
    }
    #[doc = "Bit 1 - desc POL1"]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W<1> {
        POL1_W::new(self)
    }
    #[doc = "Bit 2 - desc POL2"]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W<2> {
        POL2_W::new(self)
    }
    #[doc = "Bit 3 - desc POL3"]
    #[inline(always)]
    pub fn pol3(&mut self) -> POL3_W<3> {
        POL3_W::new(self)
    }
    #[doc = "Bit 4 - desc POL4"]
    #[inline(always)]
    pub fn pol4(&mut self) -> POL4_W<4> {
        POL4_W::new(self)
    }
    #[doc = "Bit 5 - desc POL5"]
    #[inline(always)]
    pub fn pol5(&mut self) -> POL5_W<5> {
        POL5_W::new(self)
    }
    #[doc = "Bit 6 - desc POL6"]
    #[inline(always)]
    pub fn pol6(&mut self) -> POL6_W<6> {
        POL6_W::new(self)
    }
    #[doc = "Bit 7 - desc POL7"]
    #[inline(always)]
    pub fn pol7(&mut self) -> POL7_W<7> {
        POL7_W::new(self)
    }
    #[doc = "Bit 8 - desc POL8"]
    #[inline(always)]
    pub fn pol8(&mut self) -> POL8_W<8> {
        POL8_W::new(self)
    }
    #[doc = "Bit 9 - desc POL9"]
    #[inline(always)]
    pub fn pol9(&mut self) -> POL9_W<9> {
        POL9_W::new(self)
    }
    #[doc = "Bit 10 - desc POL10"]
    #[inline(always)]
    pub fn pol10(&mut self) -> POL10_W<10> {
        POL10_W::new(self)
    }
    #[doc = "Bit 11 - desc POL11"]
    #[inline(always)]
    pub fn pol11(&mut self) -> POL11_W<11> {
        POL11_W::new(self)
    }
    #[doc = "Bit 12 - desc POL12"]
    #[inline(always)]
    pub fn pol12(&mut self) -> POL12_W<12> {
        POL12_W::new(self)
    }
    #[doc = "Bit 13 - desc POL13"]
    #[inline(always)]
    pub fn pol13(&mut self) -> POL13_W<13> {
        POL13_W::new(self)
    }
    #[doc = "Bit 14 - desc POL14"]
    #[inline(always)]
    pub fn pol14(&mut self) -> POL14_W<14> {
        POL14_W::new(self)
    }
    #[doc = "Bit 15 - desc POL15"]
    #[inline(always)]
    pub fn pol15(&mut self) -> POL15_W<15> {
        POL15_W::new(self)
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
#[doc = "desc PTBKP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptbkp](index.html) module"]
pub struct PTBKP_SPEC;
impl crate::RegisterSpec for PTBKP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptbkp::R](R) reader structure"]
impl crate::Readable for PTBKP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptbkp::W](W) writer structure"]
impl crate::Writable for PTBKP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTBKP to value 0"]
impl crate::Resettable for PTBKP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
