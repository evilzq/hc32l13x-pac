#[doc = "Register `CNTER` reader"]
pub struct R(crate::R<CNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTER` writer"]
pub struct W(crate::W<CNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTER_SPEC>;
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
impl From<crate::W<CNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - desc CNT"]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTER_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<0> {
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
#[doc = "desc CNTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnter](index.html) module"]
pub struct CNTER_SPEC;
impl crate::RegisterSpec for CNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnter::R](R) reader structure"]
impl crate::Readable for CNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnter::W](W) writer structure"]
impl crate::Writable for CNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTER to value 0"]
impl crate::Resettable for CNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
