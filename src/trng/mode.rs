#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` reader - desc LOAD"]
pub type LOAD_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` writer - desc LOAD"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `FDBK` reader - desc FDBK"]
pub type FDBK_R = crate::BitReader<bool>;
#[doc = "Field `FDBK` writer - desc FDBK"]
pub type FDBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc LOAD"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FDBK"]
    #[inline(always)]
    pub fn fdbk(&self) -> FDBK_R {
        FDBK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc LOAD"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Bit 1 - desc FDBK"]
    #[inline(always)]
    pub fn fdbk(&mut self) -> FDBK_W<1> {
        FDBK_W::new(self)
    }
    #[doc = "Bits 2:4 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<2> {
        CNT_W::new(self)
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
#[doc = "desc MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0x10"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
