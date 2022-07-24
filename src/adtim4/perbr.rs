#[doc = "Register `PERBR` reader"]
pub struct R(crate::R<PERBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERBR` writer"]
pub struct W(crate::W<PERBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERBR_SPEC>;
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
impl From<crate::W<PERBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERB` reader - desc PERB"]
pub type PERB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERB` writer - desc PERB"]
pub type PERB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERBR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERBR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc PERB"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PERB"]
    #[inline(always)]
    pub fn perb(&mut self) -> PERB_W<0> {
        PERB_W::new(self)
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
#[doc = "desc PERBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbr](index.html) module"]
pub struct PERBR_SPEC;
impl crate::RegisterSpec for PERBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perbr::R](R) reader structure"]
impl crate::Readable for PERBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perbr::W](W) writer structure"]
impl crate::Writable for PERBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERBR to value 0xffff"]
impl crate::Resettable for PERBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
