#[doc = "Register `DTR` reader"]
pub struct R(crate::R<DTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTR` writer"]
pub struct W(crate::W<DTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTR_SPEC>;
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
impl From<crate::W<DTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTR` reader - desc DTR"]
pub type DTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTR` writer - desc DTR"]
pub type DTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BKSEL` reader - desc BKSEL"]
pub type BKSEL_R = crate::BitReader<bool>;
#[doc = "Field `BKSEL` writer - desc BKSEL"]
pub type BKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `DTEN` reader - desc DTEN"]
pub type DTEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEN` writer - desc DTEN"]
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `BKE` reader - desc BKE"]
pub type BKE_R = crate::BitReader<bool>;
#[doc = "Field `BKE` writer - desc BKE"]
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `AOE` reader - desc AOE"]
pub type AOE_R = crate::BitReader<bool>;
#[doc = "Field `AOE` writer - desc AOE"]
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `MOE` reader - desc MOE"]
pub type MOE_R = crate::BitReader<bool>;
#[doc = "Field `MOE` writer - desc MOE"]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `SAFEEN` reader - desc SAFEEN"]
pub type SAFEEN_R = crate::BitReader<bool>;
#[doc = "Field `SAFEEN` writer - desc SAFEEN"]
pub type SAFEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `VC0E` reader - desc VC0E"]
pub type VC0E_R = crate::BitReader<bool>;
#[doc = "Field `VC0E` writer - desc VC0E"]
pub type VC0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `VC1E` reader - desc VC1E"]
pub type VC1E_R = crate::BitReader<bool>;
#[doc = "Field `VC1E` writer - desc VC1E"]
pub type VC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - desc BKSEL"]
    #[inline(always)]
    pub fn bksel(&self) -> BKSEL_R {
        BKSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BKE"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SAFEEN"]
    #[inline(always)]
    pub fn safeen(&self) -> SAFEEN_R {
        SAFEEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc VC0E"]
    #[inline(always)]
    pub fn vc0e(&self) -> VC0E_R {
        VC0E_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc VC1E"]
    #[inline(always)]
    pub fn vc1e(&self) -> VC1E_R {
        VC1E_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DTR"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W<0> {
        DTR_W::new(self)
    }
    #[doc = "Bit 8 - desc BKSEL"]
    #[inline(always)]
    pub fn bksel(&mut self) -> BKSEL_W<8> {
        BKSEL_W::new(self)
    }
    #[doc = "Bit 9 - desc DTEN"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W<9> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 10 - desc BKE"]
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<10> {
        BKE_W::new(self)
    }
    #[doc = "Bit 11 - desc AOE"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<11> {
        AOE_W::new(self)
    }
    #[doc = "Bit 12 - desc MOE"]
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<12> {
        MOE_W::new(self)
    }
    #[doc = "Bit 13 - desc SAFEEN"]
    #[inline(always)]
    pub fn safeen(&mut self) -> SAFEEN_W<13> {
        SAFEEN_W::new(self)
    }
    #[doc = "Bit 14 - desc VC0E"]
    #[inline(always)]
    pub fn vc0e(&mut self) -> VC0E_W<14> {
        VC0E_W::new(self)
    }
    #[doc = "Bit 15 - desc VC1E"]
    #[inline(always)]
    pub fn vc1e(&mut self) -> VC1E_W<15> {
        VC1E_W::new(self)
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
#[doc = "desc DTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtr](index.html) module"]
pub struct DTR_SPEC;
impl crate::RegisterSpec for DTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtr::R](R) reader structure"]
impl crate::Readable for DTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtr::W](W) writer structure"]
impl crate::Writable for DTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTR to value 0"]
impl crate::Resettable for DTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
