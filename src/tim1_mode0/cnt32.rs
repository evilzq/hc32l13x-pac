#[doc = "Register `CNT32` reader"]
pub struct R(crate::R<CNT32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT32` writer"]
pub struct W(crate::W<CNT32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT32_SPEC>;
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
impl From<crate::W<CNT32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT32` reader - desc CNT32"]
pub type CNT32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT32` writer - desc CNT32"]
pub type CNT32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT32_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc CNT32"]
    #[inline(always)]
    pub fn cnt32(&self) -> CNT32_R {
        CNT32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc CNT32"]
    #[inline(always)]
    pub fn cnt32(&mut self) -> CNT32_W<0> {
        CNT32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CNT32\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt32](index.html) module"]
pub struct CNT32_SPEC;
impl crate::RegisterSpec for CNT32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt32::R](R) reader structure"]
impl crate::Readable for CNT32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt32::W](W) writer structure"]
impl crate::Writable for CNT32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT32 to value 0"]
impl crate::Resettable for CNT32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
