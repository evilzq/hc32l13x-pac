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
#[doc = "Field `FLTA0` reader - desc FLTA0"]
pub type FLTA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLTA0` writer - desc FLTA0"]
pub type FLTA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FLTB0` reader - desc FLTB0"]
pub type FLTB0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLTB0` writer - desc FLTB0"]
pub type FLTB0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FLTET` reader - desc FLTET"]
pub type FLTET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLTET` writer - desc FLTET"]
pub type FLTET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETP` reader - desc ETP"]
pub type ETP_R = crate::BitReader<bool>;
#[doc = "Field `ETP` writer - desc ETP"]
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc FLTA0"]
    #[inline(always)]
    pub fn flta0(&self) -> FLTA0_R {
        FLTA0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc FLTB0"]
    #[inline(always)]
    pub fn fltb0(&self) -> FLTB0_R {
        FLTB0_R::new(((self.bits >> 4) & 7) as u8)
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
    #[doc = "Bits 0:2 - desc FLTA0"]
    #[inline(always)]
    pub fn flta0(&mut self) -> FLTA0_W<0> {
        FLTA0_W::new(self)
    }
    #[doc = "Bits 4:6 - desc FLTB0"]
    #[inline(always)]
    pub fn fltb0(&mut self) -> FLTB0_W<4> {
        FLTB0_W::new(self)
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
