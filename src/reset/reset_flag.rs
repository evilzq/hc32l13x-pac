#[doc = "Register `RESET_FLAG` reader"]
pub struct R(crate::R<RESET_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_FLAG` writer"]
pub struct W(crate::W<RESET_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_FLAG_SPEC>;
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
impl From<crate::W<RESET_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR5V` reader - desc POR5V"]
pub type POR5V_R = crate::BitReader<bool>;
#[doc = "Field `POR5V` writer - desc POR5V"]
pub type POR5V_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
#[doc = "Field `POR15V` reader - desc POR15V"]
pub type POR15V_R = crate::BitReader<bool>;
#[doc = "Field `POR15V` writer - desc POR15V"]
pub type POR15V_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
#[doc = "Field `LVD` reader - desc LVD"]
pub type LVD_R = crate::BitReader<bool>;
#[doc = "Field `LVD` writer - desc LVD"]
pub type LVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
#[doc = "Field `WDT` reader - desc WDT"]
pub type WDT_R = crate::BitReader<bool>;
#[doc = "Field `WDT` writer - desc WDT"]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
#[doc = "Field `PCA` reader - desc PCA"]
pub type PCA_R = crate::BitReader<bool>;
#[doc = "Field `PCA` writer - desc PCA"]
pub type PCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
#[doc = "Field `LOCKUP` reader - desc LOCKUP"]
pub type LOCKUP_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP` writer - desc LOCKUP"]
pub type LOCKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
#[doc = "Field `SYSREQ` reader - desc SYSREQ"]
pub type SYSREQ_R = crate::BitReader<bool>;
#[doc = "Field `SYSREQ` writer - desc SYSREQ"]
pub type SYSREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
#[doc = "Field `RSTB` reader - desc RSTB"]
pub type RSTB_R = crate::BitReader<bool>;
#[doc = "Field `RSTB` writer - desc RSTB"]
pub type RSTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_FLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc POR5V"]
    #[inline(always)]
    pub fn por5v(&self) -> POR5V_R {
        POR5V_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POR15V"]
    #[inline(always)]
    pub fn por15v(&self) -> POR15V_R {
        POR15V_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc LVD"]
    #[inline(always)]
    pub fn lvd(&self) -> LVD_R {
        LVD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WDT"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PCA"]
    #[inline(always)]
    pub fn pca(&self) -> PCA_R {
        PCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SYSREQ"]
    #[inline(always)]
    pub fn sysreq(&self) -> SYSREQ_R {
        SYSREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RSTB"]
    #[inline(always)]
    pub fn rstb(&self) -> RSTB_R {
        RSTB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POR5V"]
    #[inline(always)]
    pub fn por5v(&mut self) -> POR5V_W<0> {
        POR5V_W::new(self)
    }
    #[doc = "Bit 1 - desc POR15V"]
    #[inline(always)]
    pub fn por15v(&mut self) -> POR15V_W<1> {
        POR15V_W::new(self)
    }
    #[doc = "Bit 2 - desc LVD"]
    #[inline(always)]
    pub fn lvd(&mut self) -> LVD_W<2> {
        LVD_W::new(self)
    }
    #[doc = "Bit 3 - desc WDT"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W<3> {
        WDT_W::new(self)
    }
    #[doc = "Bit 4 - desc PCA"]
    #[inline(always)]
    pub fn pca(&mut self) -> PCA_W<4> {
        PCA_W::new(self)
    }
    #[doc = "Bit 5 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LOCKUP_W<5> {
        LOCKUP_W::new(self)
    }
    #[doc = "Bit 6 - desc SYSREQ"]
    #[inline(always)]
    pub fn sysreq(&mut self) -> SYSREQ_W<6> {
        SYSREQ_W::new(self)
    }
    #[doc = "Bit 7 - desc RSTB"]
    #[inline(always)]
    pub fn rstb(&mut self) -> RSTB_W<7> {
        RSTB_W::new(self)
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
#[doc = "desc RESET_FLAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_flag](index.html) module"]
pub struct RESET_FLAG_SPEC;
impl crate::RegisterSpec for RESET_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_flag::R](R) reader structure"]
impl crate::Readable for RESET_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_flag::W](W) writer structure"]
impl crate::Writable for RESET_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_FLAG to value 0x11"]
impl crate::Resettable for RESET_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
