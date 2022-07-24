#[doc = "Register `MSCR` reader"]
pub struct R(crate::R<MSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSCR` writer"]
pub struct W(crate::W<MSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSCR_SPEC>;
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
impl From<crate::W<MSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS` reader - desc TS"]
pub type TS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS` writer - desc TS"]
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IA0S` reader - desc IA0S"]
pub type IA0S_R = crate::BitReader<bool>;
#[doc = "Field `IA0S` writer - desc IA0S"]
pub type IA0S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSCR_SPEC, bool, O>;
#[doc = "Field `IB0S` reader - desc IB0S"]
pub type IB0S_R = crate::BitReader<bool>;
#[doc = "Field `IB0S` writer - desc IB0S"]
pub type IB0S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSCR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 5:7 - desc TS"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 11 - desc IA0S"]
    #[inline(always)]
    pub fn ia0s(&self) -> IA0S_R {
        IA0S_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc IB0S"]
    #[inline(always)]
    pub fn ib0s(&self) -> IB0S_R {
        IB0S_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - desc TS"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<5> {
        TS_W::new(self)
    }
    #[doc = "Bit 11 - desc IA0S"]
    #[inline(always)]
    pub fn ia0s(&mut self) -> IA0S_W<11> {
        IA0S_W::new(self)
    }
    #[doc = "Bit 12 - desc IB0S"]
    #[inline(always)]
    pub fn ib0s(&mut self) -> IB0S_W<12> {
        IB0S_W::new(self)
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
#[doc = "desc MSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mscr](index.html) module"]
pub struct MSCR_SPEC;
impl crate::RegisterSpec for MSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mscr::R](R) reader structure"]
impl crate::Readable for MSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mscr::W](W) writer structure"]
impl crate::Writable for MSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSCR to value 0"]
impl crate::Resettable for MSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
