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
#[doc = "Field `CB0F` writer - desc CB0F"]
pub type CB0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CA0E` writer - desc CA0E"]
pub type CA0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CB0E` writer - desc CB0E"]
pub type CB0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
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
    #[doc = "Bit 5 - desc CB0F"]
    #[inline(always)]
    pub fn cb0f(&mut self) -> CB0F_W<5> {
        CB0F_W::new(self)
    }
    #[doc = "Bit 8 - desc CA0E"]
    #[inline(always)]
    pub fn ca0e(&mut self) -> CA0E_W<8> {
        CA0E_W::new(self)
    }
    #[doc = "Bit 11 - desc CB0E"]
    #[inline(always)]
    pub fn cb0e(&mut self) -> CB0E_W<11> {
        CB0E_W::new(self)
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
