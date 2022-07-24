#[doc = "Register `FLTR` reader"]
pub struct R(crate::R<FLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTR` writer"]
pub struct W(crate::W<FLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTR_SPEC>;
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
impl From<crate::W<FLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCMA0_FLTA0` reader - desc OCMA0_FLTA0"]
pub type OCMA0_FLTA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCMA0_FLTA0` writer - desc OCMA0_FLTA0"]
pub type OCMA0_FLTA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CCPA0` reader - desc CCPA0"]
pub type CCPA0_R = crate::BitReader<bool>;
#[doc = "Field `CCPA0` writer - desc CCPA0"]
pub type CCPA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTR_SPEC, bool, O>;
#[doc = "Field `OCMB0_FLTB0` reader - desc OCMB0_FLTB0"]
pub type OCMB0_FLTB0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCMB0_FLTB0` writer - desc OCMB0_FLTB0"]
pub type OCMB0_FLTB0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CCPB0` reader - desc CCPB0"]
pub type CCPB0_R = crate::BitReader<bool>;
#[doc = "Field `CCPB0` writer - desc CCPB0"]
pub type CCPB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTR_SPEC, bool, O>;
#[doc = "Field `FLTBK` reader - desc FLTBK"]
pub type FLTBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLTBK` writer - desc FLTBK"]
pub type FLTBK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BKP` reader - desc BKP"]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - desc BKP"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTR_SPEC, bool, O>;
#[doc = "Field `FLTET` reader - desc FLTET"]
pub type FLTET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLTET` writer - desc FLTET"]
pub type FLTET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETP` reader - desc ETP"]
pub type ETP_R = crate::BitReader<bool>;
#[doc = "Field `ETP` writer - desc ETP"]
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc OCMA0_FLTA0"]
    #[inline(always)]
    pub fn ocma0_flta0(&self) -> OCMA0_FLTA0_R {
        OCMA0_FLTA0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc CCPA0"]
    #[inline(always)]
    pub fn ccpa0(&self) -> CCPA0_R {
        CCPA0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc OCMB0_FLTB0"]
    #[inline(always)]
    pub fn ocmb0_fltb0(&self) -> OCMB0_FLTB0_R {
        OCMB0_FLTB0_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc CCPB0"]
    #[inline(always)]
    pub fn ccpb0(&self) -> CCPB0_R {
        CCPB0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:26 - desc FLTBK"]
    #[inline(always)]
    pub fn fltbk(&self) -> FLTBK_R {
        FLTBK_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - desc BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - desc FLTET"]
    #[inline(always)]
    pub fn fltet(&self) -> FLTET_R {
        FLTET_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - desc ETP"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc OCMA0_FLTA0"]
    #[inline(always)]
    pub fn ocma0_flta0(&mut self) -> OCMA0_FLTA0_W<0> {
        OCMA0_FLTA0_W::new(self)
    }
    #[doc = "Bit 3 - desc CCPA0"]
    #[inline(always)]
    pub fn ccpa0(&mut self) -> CCPA0_W<3> {
        CCPA0_W::new(self)
    }
    #[doc = "Bits 4:6 - desc OCMB0_FLTB0"]
    #[inline(always)]
    pub fn ocmb0_fltb0(&mut self) -> OCMB0_FLTB0_W<4> {
        OCMB0_FLTB0_W::new(self)
    }
    #[doc = "Bit 7 - desc CCPB0"]
    #[inline(always)]
    pub fn ccpb0(&mut self) -> CCPB0_W<7> {
        CCPB0_W::new(self)
    }
    #[doc = "Bits 24:26 - desc FLTBK"]
    #[inline(always)]
    pub fn fltbk(&mut self) -> FLTBK_W<24> {
        FLTBK_W::new(self)
    }
    #[doc = "Bit 27 - desc BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<27> {
        BKP_W::new(self)
    }
    #[doc = "Bits 28:30 - desc FLTET"]
    #[inline(always)]
    pub fn fltet(&mut self) -> FLTET_W<28> {
        FLTET_W::new(self)
    }
    #[doc = "Bit 31 - desc ETP"]
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<31> {
        ETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc FLTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltr](index.html) module"]
pub struct FLTR_SPEC;
impl crate::RegisterSpec for FLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltr::R](R) reader structure"]
impl crate::Readable for FLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltr::W](W) writer structure"]
impl crate::Writable for FLTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTR to value 0"]
impl crate::Resettable for FLTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
