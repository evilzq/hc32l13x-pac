#[doc = "Register `ICLR` writer"]
pub struct W(crate::W<ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLR_SPEC>;
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
impl From<crate::W<ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIF` writer - desc UIF"]
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CA0F` writer - desc CA0F"]
pub type CA0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CA1F` writer - desc CA1F"]
pub type CA1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CA2F` writer - desc CA2F"]
pub type CA2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CB0F` writer - desc CB0F"]
pub type CB0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CB1F` writer - desc CB1F"]
pub type CB1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CB2F` writer - desc CB2F"]
pub type CB2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CA0E` writer - desc CA0E"]
pub type CA0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CA1E` writer - desc CA1E"]
pub type CA1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CA2E` writer - desc CA2E"]
pub type CA2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CB0E` writer - desc CB0E"]
pub type CB0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CB1E` writer - desc CB1E"]
pub type CB1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CB2E` writer - desc CB2E"]
pub type CB2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `BIF` writer - desc BIF"]
pub type BIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `TIF` writer - desc TIF"]
pub type TIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<0> {
        UIF_W::new(self)
    }
    #[doc = "Bit 2 - desc CA0F"]
    #[inline(always)]
    pub fn ca0f(&mut self) -> CA0F_W<2> {
        CA0F_W::new(self)
    }
    #[doc = "Bit 3 - desc CA1F"]
    #[inline(always)]
    pub fn ca1f(&mut self) -> CA1F_W<3> {
        CA1F_W::new(self)
    }
    #[doc = "Bit 4 - desc CA2F"]
    #[inline(always)]
    pub fn ca2f(&mut self) -> CA2F_W<4> {
        CA2F_W::new(self)
    }
    #[doc = "Bit 5 - desc CB0F"]
    #[inline(always)]
    pub fn cb0f(&mut self) -> CB0F_W<5> {
        CB0F_W::new(self)
    }
    #[doc = "Bit 6 - desc CB1F"]
    #[inline(always)]
    pub fn cb1f(&mut self) -> CB1F_W<6> {
        CB1F_W::new(self)
    }
    #[doc = "Bit 7 - desc CB2F"]
    #[inline(always)]
    pub fn cb2f(&mut self) -> CB2F_W<7> {
        CB2F_W::new(self)
    }
    #[doc = "Bit 8 - desc CA0E"]
    #[inline(always)]
    pub fn ca0e(&mut self) -> CA0E_W<8> {
        CA0E_W::new(self)
    }
    #[doc = "Bit 9 - desc CA1E"]
    #[inline(always)]
    pub fn ca1e(&mut self) -> CA1E_W<9> {
        CA1E_W::new(self)
    }
    #[doc = "Bit 10 - desc CA2E"]
    #[inline(always)]
    pub fn ca2e(&mut self) -> CA2E_W<10> {
        CA2E_W::new(self)
    }
    #[doc = "Bit 11 - desc CB0E"]
    #[inline(always)]
    pub fn cb0e(&mut self) -> CB0E_W<11> {
        CB0E_W::new(self)
    }
    #[doc = "Bit 12 - desc CB1E"]
    #[inline(always)]
    pub fn cb1e(&mut self) -> CB1E_W<12> {
        CB1E_W::new(self)
    }
    #[doc = "Bit 13 - desc CB2E"]
    #[inline(always)]
    pub fn cb2e(&mut self) -> CB2E_W<13> {
        CB2E_W::new(self)
    }
    #[doc = "Bit 14 - desc BIF"]
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W<14> {
        BIF_W::new(self)
    }
    #[doc = "Bit 15 - desc TIF"]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<15> {
        TIF_W::new(self)
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
#[doc = "desc ICLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](index.html) module"]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iclr::W](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICLR to value 0xffff"]
impl crate::Resettable for ICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
