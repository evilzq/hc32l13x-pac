#[doc = "Register `RAM6` reader"]
pub struct R(crate::R<RAM6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM6` writer"]
pub struct W(crate::W<RAM6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM6_SPEC>;
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
impl From<crate::W<RAM6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D0` reader - desc D0"]
pub type D0_R = crate::BitReader<bool>;
#[doc = "Field `D0` writer - desc D0"]
pub type D0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D1` reader - desc D1"]
pub type D1_R = crate::BitReader<bool>;
#[doc = "Field `D1` writer - desc D1"]
pub type D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D2` reader - desc D2"]
pub type D2_R = crate::BitReader<bool>;
#[doc = "Field `D2` writer - desc D2"]
pub type D2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D3` reader - desc D3"]
pub type D3_R = crate::BitReader<bool>;
#[doc = "Field `D3` writer - desc D3"]
pub type D3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D4` reader - desc D4"]
pub type D4_R = crate::BitReader<bool>;
#[doc = "Field `D4` writer - desc D4"]
pub type D4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D5` reader - desc D5"]
pub type D5_R = crate::BitReader<bool>;
#[doc = "Field `D5` writer - desc D5"]
pub type D5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D6` reader - desc D6"]
pub type D6_R = crate::BitReader<bool>;
#[doc = "Field `D6` writer - desc D6"]
pub type D6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D7` reader - desc D7"]
pub type D7_R = crate::BitReader<bool>;
#[doc = "Field `D7` writer - desc D7"]
pub type D7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D8` reader - desc D8"]
pub type D8_R = crate::BitReader<bool>;
#[doc = "Field `D8` writer - desc D8"]
pub type D8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D9` reader - desc D9"]
pub type D9_R = crate::BitReader<bool>;
#[doc = "Field `D9` writer - desc D9"]
pub type D9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D10` reader - desc D10"]
pub type D10_R = crate::BitReader<bool>;
#[doc = "Field `D10` writer - desc D10"]
pub type D10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D11` reader - desc D11"]
pub type D11_R = crate::BitReader<bool>;
#[doc = "Field `D11` writer - desc D11"]
pub type D11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D12` reader - desc D12"]
pub type D12_R = crate::BitReader<bool>;
#[doc = "Field `D12` writer - desc D12"]
pub type D12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D13` reader - desc D13"]
pub type D13_R = crate::BitReader<bool>;
#[doc = "Field `D13` writer - desc D13"]
pub type D13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D14` reader - desc D14"]
pub type D14_R = crate::BitReader<bool>;
#[doc = "Field `D14` writer - desc D14"]
pub type D14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D15` reader - desc D15"]
pub type D15_R = crate::BitReader<bool>;
#[doc = "Field `D15` writer - desc D15"]
pub type D15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D16` reader - desc D16"]
pub type D16_R = crate::BitReader<bool>;
#[doc = "Field `D16` writer - desc D16"]
pub type D16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D17` reader - desc D17"]
pub type D17_R = crate::BitReader<bool>;
#[doc = "Field `D17` writer - desc D17"]
pub type D17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D18` reader - desc D18"]
pub type D18_R = crate::BitReader<bool>;
#[doc = "Field `D18` writer - desc D18"]
pub type D18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D19` reader - desc D19"]
pub type D19_R = crate::BitReader<bool>;
#[doc = "Field `D19` writer - desc D19"]
pub type D19_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D20` reader - desc D20"]
pub type D20_R = crate::BitReader<bool>;
#[doc = "Field `D20` writer - desc D20"]
pub type D20_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D21` reader - desc D21"]
pub type D21_R = crate::BitReader<bool>;
#[doc = "Field `D21` writer - desc D21"]
pub type D21_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D22` reader - desc D22"]
pub type D22_R = crate::BitReader<bool>;
#[doc = "Field `D22` writer - desc D22"]
pub type D22_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D23` reader - desc D23"]
pub type D23_R = crate::BitReader<bool>;
#[doc = "Field `D23` writer - desc D23"]
pub type D23_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D24` reader - desc D24"]
pub type D24_R = crate::BitReader<bool>;
#[doc = "Field `D24` writer - desc D24"]
pub type D24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D25` reader - desc D25"]
pub type D25_R = crate::BitReader<bool>;
#[doc = "Field `D25` writer - desc D25"]
pub type D25_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D26` reader - desc D26"]
pub type D26_R = crate::BitReader<bool>;
#[doc = "Field `D26` writer - desc D26"]
pub type D26_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D27` reader - desc D27"]
pub type D27_R = crate::BitReader<bool>;
#[doc = "Field `D27` writer - desc D27"]
pub type D27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D28` reader - desc D28"]
pub type D28_R = crate::BitReader<bool>;
#[doc = "Field `D28` writer - desc D28"]
pub type D28_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D29` reader - desc D29"]
pub type D29_R = crate::BitReader<bool>;
#[doc = "Field `D29` writer - desc D29"]
pub type D29_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D30` reader - desc D30"]
pub type D30_R = crate::BitReader<bool>;
#[doc = "Field `D30` writer - desc D30"]
pub type D30_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
#[doc = "Field `D31` reader - desc D31"]
pub type D31_R = crate::BitReader<bool>;
#[doc = "Field `D31` writer - desc D31"]
pub type D31_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc D0"]
    #[inline(always)]
    pub fn d0(&self) -> D0_R {
        D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc D1"]
    #[inline(always)]
    pub fn d1(&self) -> D1_R {
        D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc D2"]
    #[inline(always)]
    pub fn d2(&self) -> D2_R {
        D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc D3"]
    #[inline(always)]
    pub fn d3(&self) -> D3_R {
        D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc D4"]
    #[inline(always)]
    pub fn d4(&self) -> D4_R {
        D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc D5"]
    #[inline(always)]
    pub fn d5(&self) -> D5_R {
        D5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc D6"]
    #[inline(always)]
    pub fn d6(&self) -> D6_R {
        D6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc D7"]
    #[inline(always)]
    pub fn d7(&self) -> D7_R {
        D7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc D8"]
    #[inline(always)]
    pub fn d8(&self) -> D8_R {
        D8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc D9"]
    #[inline(always)]
    pub fn d9(&self) -> D9_R {
        D9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc D10"]
    #[inline(always)]
    pub fn d10(&self) -> D10_R {
        D10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc D11"]
    #[inline(always)]
    pub fn d11(&self) -> D11_R {
        D11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc D12"]
    #[inline(always)]
    pub fn d12(&self) -> D12_R {
        D12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc D13"]
    #[inline(always)]
    pub fn d13(&self) -> D13_R {
        D13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc D14"]
    #[inline(always)]
    pub fn d14(&self) -> D14_R {
        D14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc D15"]
    #[inline(always)]
    pub fn d15(&self) -> D15_R {
        D15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc D16"]
    #[inline(always)]
    pub fn d16(&self) -> D16_R {
        D16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc D17"]
    #[inline(always)]
    pub fn d17(&self) -> D17_R {
        D17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc D18"]
    #[inline(always)]
    pub fn d18(&self) -> D18_R {
        D18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc D19"]
    #[inline(always)]
    pub fn d19(&self) -> D19_R {
        D19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc D20"]
    #[inline(always)]
    pub fn d20(&self) -> D20_R {
        D20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc D21"]
    #[inline(always)]
    pub fn d21(&self) -> D21_R {
        D21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc D22"]
    #[inline(always)]
    pub fn d22(&self) -> D22_R {
        D22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc D23"]
    #[inline(always)]
    pub fn d23(&self) -> D23_R {
        D23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc D24"]
    #[inline(always)]
    pub fn d24(&self) -> D24_R {
        D24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc D25"]
    #[inline(always)]
    pub fn d25(&self) -> D25_R {
        D25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc D26"]
    #[inline(always)]
    pub fn d26(&self) -> D26_R {
        D26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc D27"]
    #[inline(always)]
    pub fn d27(&self) -> D27_R {
        D27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc D28"]
    #[inline(always)]
    pub fn d28(&self) -> D28_R {
        D28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc D29"]
    #[inline(always)]
    pub fn d29(&self) -> D29_R {
        D29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc D30"]
    #[inline(always)]
    pub fn d30(&self) -> D30_R {
        D30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc D31"]
    #[inline(always)]
    pub fn d31(&self) -> D31_R {
        D31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc D0"]
    #[inline(always)]
    pub fn d0(&mut self) -> D0_W<0> {
        D0_W::new(self)
    }
    #[doc = "Bit 1 - desc D1"]
    #[inline(always)]
    pub fn d1(&mut self) -> D1_W<1> {
        D1_W::new(self)
    }
    #[doc = "Bit 2 - desc D2"]
    #[inline(always)]
    pub fn d2(&mut self) -> D2_W<2> {
        D2_W::new(self)
    }
    #[doc = "Bit 3 - desc D3"]
    #[inline(always)]
    pub fn d3(&mut self) -> D3_W<3> {
        D3_W::new(self)
    }
    #[doc = "Bit 4 - desc D4"]
    #[inline(always)]
    pub fn d4(&mut self) -> D4_W<4> {
        D4_W::new(self)
    }
    #[doc = "Bit 5 - desc D5"]
    #[inline(always)]
    pub fn d5(&mut self) -> D5_W<5> {
        D5_W::new(self)
    }
    #[doc = "Bit 6 - desc D6"]
    #[inline(always)]
    pub fn d6(&mut self) -> D6_W<6> {
        D6_W::new(self)
    }
    #[doc = "Bit 7 - desc D7"]
    #[inline(always)]
    pub fn d7(&mut self) -> D7_W<7> {
        D7_W::new(self)
    }
    #[doc = "Bit 8 - desc D8"]
    #[inline(always)]
    pub fn d8(&mut self) -> D8_W<8> {
        D8_W::new(self)
    }
    #[doc = "Bit 9 - desc D9"]
    #[inline(always)]
    pub fn d9(&mut self) -> D9_W<9> {
        D9_W::new(self)
    }
    #[doc = "Bit 10 - desc D10"]
    #[inline(always)]
    pub fn d10(&mut self) -> D10_W<10> {
        D10_W::new(self)
    }
    #[doc = "Bit 11 - desc D11"]
    #[inline(always)]
    pub fn d11(&mut self) -> D11_W<11> {
        D11_W::new(self)
    }
    #[doc = "Bit 12 - desc D12"]
    #[inline(always)]
    pub fn d12(&mut self) -> D12_W<12> {
        D12_W::new(self)
    }
    #[doc = "Bit 13 - desc D13"]
    #[inline(always)]
    pub fn d13(&mut self) -> D13_W<13> {
        D13_W::new(self)
    }
    #[doc = "Bit 14 - desc D14"]
    #[inline(always)]
    pub fn d14(&mut self) -> D14_W<14> {
        D14_W::new(self)
    }
    #[doc = "Bit 15 - desc D15"]
    #[inline(always)]
    pub fn d15(&mut self) -> D15_W<15> {
        D15_W::new(self)
    }
    #[doc = "Bit 16 - desc D16"]
    #[inline(always)]
    pub fn d16(&mut self) -> D16_W<16> {
        D16_W::new(self)
    }
    #[doc = "Bit 17 - desc D17"]
    #[inline(always)]
    pub fn d17(&mut self) -> D17_W<17> {
        D17_W::new(self)
    }
    #[doc = "Bit 18 - desc D18"]
    #[inline(always)]
    pub fn d18(&mut self) -> D18_W<18> {
        D18_W::new(self)
    }
    #[doc = "Bit 19 - desc D19"]
    #[inline(always)]
    pub fn d19(&mut self) -> D19_W<19> {
        D19_W::new(self)
    }
    #[doc = "Bit 20 - desc D20"]
    #[inline(always)]
    pub fn d20(&mut self) -> D20_W<20> {
        D20_W::new(self)
    }
    #[doc = "Bit 21 - desc D21"]
    #[inline(always)]
    pub fn d21(&mut self) -> D21_W<21> {
        D21_W::new(self)
    }
    #[doc = "Bit 22 - desc D22"]
    #[inline(always)]
    pub fn d22(&mut self) -> D22_W<22> {
        D22_W::new(self)
    }
    #[doc = "Bit 23 - desc D23"]
    #[inline(always)]
    pub fn d23(&mut self) -> D23_W<23> {
        D23_W::new(self)
    }
    #[doc = "Bit 24 - desc D24"]
    #[inline(always)]
    pub fn d24(&mut self) -> D24_W<24> {
        D24_W::new(self)
    }
    #[doc = "Bit 25 - desc D25"]
    #[inline(always)]
    pub fn d25(&mut self) -> D25_W<25> {
        D25_W::new(self)
    }
    #[doc = "Bit 26 - desc D26"]
    #[inline(always)]
    pub fn d26(&mut self) -> D26_W<26> {
        D26_W::new(self)
    }
    #[doc = "Bit 27 - desc D27"]
    #[inline(always)]
    pub fn d27(&mut self) -> D27_W<27> {
        D27_W::new(self)
    }
    #[doc = "Bit 28 - desc D28"]
    #[inline(always)]
    pub fn d28(&mut self) -> D28_W<28> {
        D28_W::new(self)
    }
    #[doc = "Bit 29 - desc D29"]
    #[inline(always)]
    pub fn d29(&mut self) -> D29_W<29> {
        D29_W::new(self)
    }
    #[doc = "Bit 30 - desc D30"]
    #[inline(always)]
    pub fn d30(&mut self) -> D30_W<30> {
        D30_W::new(self)
    }
    #[doc = "Bit 31 - desc D31"]
    #[inline(always)]
    pub fn d31(&mut self) -> D31_W<31> {
        D31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RAM6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram6](index.html) module"]
pub struct RAM6_SPEC;
impl crate::RegisterSpec for RAM6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram6::R](R) reader structure"]
impl crate::Readable for RAM6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram6::W](W) writer structure"]
impl crate::Writable for RAM6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM6 to value 0"]
impl crate::Resettable for RAM6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
