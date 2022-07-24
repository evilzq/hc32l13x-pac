#[doc = "Register `HCPBR` reader"]
pub struct R(crate::R<HCPBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCPBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCPBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCPBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCPBR` writer"]
pub struct W(crate::W<HCPBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCPBR_SPEC>;
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
impl From<crate::W<HCPBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCPBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCPB0` reader - desc HCPB0"]
pub type HCPB0_R = crate::BitReader<bool>;
#[doc = "Field `HCPB0` writer - desc HCPB0"]
pub type HCPB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB1` reader - desc HCPB1"]
pub type HCPB1_R = crate::BitReader<bool>;
#[doc = "Field `HCPB1` writer - desc HCPB1"]
pub type HCPB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB2` reader - desc HCPB2"]
pub type HCPB2_R = crate::BitReader<bool>;
#[doc = "Field `HCPB2` writer - desc HCPB2"]
pub type HCPB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB3` reader - desc HCPB3"]
pub type HCPB3_R = crate::BitReader<bool>;
#[doc = "Field `HCPB3` writer - desc HCPB3"]
pub type HCPB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB4` reader - desc HCPB4"]
pub type HCPB4_R = crate::BitReader<bool>;
#[doc = "Field `HCPB4` writer - desc HCPB4"]
pub type HCPB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB5` reader - desc HCPB5"]
pub type HCPB5_R = crate::BitReader<bool>;
#[doc = "Field `HCPB5` writer - desc HCPB5"]
pub type HCPB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB6` reader - desc HCPB6"]
pub type HCPB6_R = crate::BitReader<bool>;
#[doc = "Field `HCPB6` writer - desc HCPB6"]
pub type HCPB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB7` reader - desc HCPB7"]
pub type HCPB7_R = crate::BitReader<bool>;
#[doc = "Field `HCPB7` writer - desc HCPB7"]
pub type HCPB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB8` reader - desc HCPB8"]
pub type HCPB8_R = crate::BitReader<bool>;
#[doc = "Field `HCPB8` writer - desc HCPB8"]
pub type HCPB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB9` reader - desc HCPB9"]
pub type HCPB9_R = crate::BitReader<bool>;
#[doc = "Field `HCPB9` writer - desc HCPB9"]
pub type HCPB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB10` reader - desc HCPB10"]
pub type HCPB10_R = crate::BitReader<bool>;
#[doc = "Field `HCPB10` writer - desc HCPB10"]
pub type HCPB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB11` reader - desc HCPB11"]
pub type HCPB11_R = crate::BitReader<bool>;
#[doc = "Field `HCPB11` writer - desc HCPB11"]
pub type HCPB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB12` reader - desc HCPB12"]
pub type HCPB12_R = crate::BitReader<bool>;
#[doc = "Field `HCPB12` writer - desc HCPB12"]
pub type HCPB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB13` reader - desc HCPB13"]
pub type HCPB13_R = crate::BitReader<bool>;
#[doc = "Field `HCPB13` writer - desc HCPB13"]
pub type HCPB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB14` reader - desc HCPB14"]
pub type HCPB14_R = crate::BitReader<bool>;
#[doc = "Field `HCPB14` writer - desc HCPB14"]
pub type HCPB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `HCPB15` reader - desc HCPB15"]
pub type HCPB15_R = crate::BitReader<bool>;
#[doc = "Field `HCPB15` writer - desc HCPB15"]
pub type HCPB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCPBR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc HCPB0"]
    #[inline(always)]
    pub fn hcpb0(&self) -> HCPB0_R {
        HCPB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCPB1"]
    #[inline(always)]
    pub fn hcpb1(&self) -> HCPB1_R {
        HCPB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCPB2"]
    #[inline(always)]
    pub fn hcpb2(&self) -> HCPB2_R {
        HCPB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCPB3"]
    #[inline(always)]
    pub fn hcpb3(&self) -> HCPB3_R {
        HCPB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCPB4"]
    #[inline(always)]
    pub fn hcpb4(&self) -> HCPB4_R {
        HCPB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCPB5"]
    #[inline(always)]
    pub fn hcpb5(&self) -> HCPB5_R {
        HCPB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCPB6"]
    #[inline(always)]
    pub fn hcpb6(&self) -> HCPB6_R {
        HCPB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCPB7"]
    #[inline(always)]
    pub fn hcpb7(&self) -> HCPB7_R {
        HCPB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCPB8"]
    #[inline(always)]
    pub fn hcpb8(&self) -> HCPB8_R {
        HCPB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCPB9"]
    #[inline(always)]
    pub fn hcpb9(&self) -> HCPB9_R {
        HCPB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCPB10"]
    #[inline(always)]
    pub fn hcpb10(&self) -> HCPB10_R {
        HCPB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCPB11"]
    #[inline(always)]
    pub fn hcpb11(&self) -> HCPB11_R {
        HCPB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HCPB12"]
    #[inline(always)]
    pub fn hcpb12(&self) -> HCPB12_R {
        HCPB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HCPB13"]
    #[inline(always)]
    pub fn hcpb13(&self) -> HCPB13_R {
        HCPB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HCPB14"]
    #[inline(always)]
    pub fn hcpb14(&self) -> HCPB14_R {
        HCPB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HCPB15"]
    #[inline(always)]
    pub fn hcpb15(&self) -> HCPB15_R {
        HCPB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCPB0"]
    #[inline(always)]
    pub fn hcpb0(&mut self) -> HCPB0_W<0> {
        HCPB0_W::new(self)
    }
    #[doc = "Bit 1 - desc HCPB1"]
    #[inline(always)]
    pub fn hcpb1(&mut self) -> HCPB1_W<1> {
        HCPB1_W::new(self)
    }
    #[doc = "Bit 2 - desc HCPB2"]
    #[inline(always)]
    pub fn hcpb2(&mut self) -> HCPB2_W<2> {
        HCPB2_W::new(self)
    }
    #[doc = "Bit 3 - desc HCPB3"]
    #[inline(always)]
    pub fn hcpb3(&mut self) -> HCPB3_W<3> {
        HCPB3_W::new(self)
    }
    #[doc = "Bit 4 - desc HCPB4"]
    #[inline(always)]
    pub fn hcpb4(&mut self) -> HCPB4_W<4> {
        HCPB4_W::new(self)
    }
    #[doc = "Bit 5 - desc HCPB5"]
    #[inline(always)]
    pub fn hcpb5(&mut self) -> HCPB5_W<5> {
        HCPB5_W::new(self)
    }
    #[doc = "Bit 6 - desc HCPB6"]
    #[inline(always)]
    pub fn hcpb6(&mut self) -> HCPB6_W<6> {
        HCPB6_W::new(self)
    }
    #[doc = "Bit 7 - desc HCPB7"]
    #[inline(always)]
    pub fn hcpb7(&mut self) -> HCPB7_W<7> {
        HCPB7_W::new(self)
    }
    #[doc = "Bit 8 - desc HCPB8"]
    #[inline(always)]
    pub fn hcpb8(&mut self) -> HCPB8_W<8> {
        HCPB8_W::new(self)
    }
    #[doc = "Bit 9 - desc HCPB9"]
    #[inline(always)]
    pub fn hcpb9(&mut self) -> HCPB9_W<9> {
        HCPB9_W::new(self)
    }
    #[doc = "Bit 10 - desc HCPB10"]
    #[inline(always)]
    pub fn hcpb10(&mut self) -> HCPB10_W<10> {
        HCPB10_W::new(self)
    }
    #[doc = "Bit 11 - desc HCPB11"]
    #[inline(always)]
    pub fn hcpb11(&mut self) -> HCPB11_W<11> {
        HCPB11_W::new(self)
    }
    #[doc = "Bit 12 - desc HCPB12"]
    #[inline(always)]
    pub fn hcpb12(&mut self) -> HCPB12_W<12> {
        HCPB12_W::new(self)
    }
    #[doc = "Bit 13 - desc HCPB13"]
    #[inline(always)]
    pub fn hcpb13(&mut self) -> HCPB13_W<13> {
        HCPB13_W::new(self)
    }
    #[doc = "Bit 14 - desc HCPB14"]
    #[inline(always)]
    pub fn hcpb14(&mut self) -> HCPB14_W<14> {
        HCPB14_W::new(self)
    }
    #[doc = "Bit 15 - desc HCPB15"]
    #[inline(always)]
    pub fn hcpb15(&mut self) -> HCPB15_W<15> {
        HCPB15_W::new(self)
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
#[doc = "desc HCPBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcpbr](index.html) module"]
pub struct HCPBR_SPEC;
impl crate::RegisterSpec for HCPBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcpbr::R](R) reader structure"]
impl crate::Readable for HCPBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcpbr::W](W) writer structure"]
impl crate::Writable for HCPBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCPBR to value 0"]
impl crate::Resettable for HCPBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
