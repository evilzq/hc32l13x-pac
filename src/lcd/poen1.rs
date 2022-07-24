#[doc = "Register `POEN1` reader"]
pub struct R(crate::R<POEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEN1` writer"]
pub struct W(crate::W<POEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEN1_SPEC>;
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
impl From<crate::W<POEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S32` reader - desc S32"]
pub type S32_R = crate::BitReader<bool>;
#[doc = "Field `S32` writer - desc S32"]
pub type S32_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S33` reader - desc S33"]
pub type S33_R = crate::BitReader<bool>;
#[doc = "Field `S33` writer - desc S33"]
pub type S33_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S34` reader - desc S34"]
pub type S34_R = crate::BitReader<bool>;
#[doc = "Field `S34` writer - desc S34"]
pub type S34_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S35` reader - desc S35"]
pub type S35_R = crate::BitReader<bool>;
#[doc = "Field `S35` writer - desc S35"]
pub type S35_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S36C7` reader - desc S36C7"]
pub type S36C7_R = crate::BitReader<bool>;
#[doc = "Field `S36C7` writer - desc S36C7"]
pub type S36C7_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S37C6` reader - desc S37C6"]
pub type S37C6_R = crate::BitReader<bool>;
#[doc = "Field `S37C6` writer - desc S37C6"]
pub type S37C6_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S38C5` reader - desc S38C5"]
pub type S38C5_R = crate::BitReader<bool>;
#[doc = "Field `S38C5` writer - desc S38C5"]
pub type S38C5_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S39C4` reader - desc S39C4"]
pub type S39C4_R = crate::BitReader<bool>;
#[doc = "Field `S39C4` writer - desc S39C4"]
pub type S39C4_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C0` reader - desc C0"]
pub type C0_R = crate::BitReader<bool>;
#[doc = "Field `C0` writer - desc C0"]
pub type C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C1` reader - desc C1"]
pub type C1_R = crate::BitReader<bool>;
#[doc = "Field `C1` writer - desc C1"]
pub type C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C2` reader - desc C2"]
pub type C2_R = crate::BitReader<bool>;
#[doc = "Field `C2` writer - desc C2"]
pub type C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C3` reader - desc C3"]
pub type C3_R = crate::BitReader<bool>;
#[doc = "Field `C3` writer - desc C3"]
pub type C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `MUX` reader - desc MUX"]
pub type MUX_R = crate::BitReader<bool>;
#[doc = "Field `MUX` writer - desc MUX"]
pub type MUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc S32"]
    #[inline(always)]
    pub fn s32(&self) -> S32_R {
        S32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc S33"]
    #[inline(always)]
    pub fn s33(&self) -> S33_R {
        S33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc S34"]
    #[inline(always)]
    pub fn s34(&self) -> S34_R {
        S34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc S35"]
    #[inline(always)]
    pub fn s35(&self) -> S35_R {
        S35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc S36C7"]
    #[inline(always)]
    pub fn s36c7(&self) -> S36C7_R {
        S36C7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc S37C6"]
    #[inline(always)]
    pub fn s37c6(&self) -> S37C6_R {
        S37C6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc S38C5"]
    #[inline(always)]
    pub fn s38c5(&self) -> S38C5_R {
        S38C5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc S39C4"]
    #[inline(always)]
    pub fn s39c4(&self) -> S39C4_R {
        S39C4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc C3"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc MUX"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc S32"]
    #[inline(always)]
    pub fn s32(&mut self) -> S32_W<0> {
        S32_W::new(self)
    }
    #[doc = "Bit 1 - desc S33"]
    #[inline(always)]
    pub fn s33(&mut self) -> S33_W<1> {
        S33_W::new(self)
    }
    #[doc = "Bit 2 - desc S34"]
    #[inline(always)]
    pub fn s34(&mut self) -> S34_W<2> {
        S34_W::new(self)
    }
    #[doc = "Bit 3 - desc S35"]
    #[inline(always)]
    pub fn s35(&mut self) -> S35_W<3> {
        S35_W::new(self)
    }
    #[doc = "Bit 4 - desc S36C7"]
    #[inline(always)]
    pub fn s36c7(&mut self) -> S36C7_W<4> {
        S36C7_W::new(self)
    }
    #[doc = "Bit 5 - desc S37C6"]
    #[inline(always)]
    pub fn s37c6(&mut self) -> S37C6_W<5> {
        S37C6_W::new(self)
    }
    #[doc = "Bit 6 - desc S38C5"]
    #[inline(always)]
    pub fn s38c5(&mut self) -> S38C5_W<6> {
        S38C5_W::new(self)
    }
    #[doc = "Bit 7 - desc S39C4"]
    #[inline(always)]
    pub fn s39c4(&mut self) -> S39C4_W<7> {
        S39C4_W::new(self)
    }
    #[doc = "Bit 8 - desc C0"]
    #[inline(always)]
    pub fn c0(&mut self) -> C0_W<8> {
        C0_W::new(self)
    }
    #[doc = "Bit 9 - desc C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W<9> {
        C1_W::new(self)
    }
    #[doc = "Bit 10 - desc C2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W<10> {
        C2_W::new(self)
    }
    #[doc = "Bit 11 - desc C3"]
    #[inline(always)]
    pub fn c3(&mut self) -> C3_W<11> {
        C3_W::new(self)
    }
    #[doc = "Bit 12 - desc MUX"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W<12> {
        MUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc POEN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poen1](index.html) module"]
pub struct POEN1_SPEC;
impl crate::RegisterSpec for POEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poen1::R](R) reader structure"]
impl crate::Readable for POEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poen1::W](W) writer structure"]
impl crate::Writable for POEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POEN1 to value 0x1fff"]
impl crate::Resettable for POEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1fff
    }
}
