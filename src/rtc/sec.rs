#[doc = "Register `SEC` reader"]
pub struct R(crate::R<SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC` writer"]
pub struct W(crate::W<SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_SPEC>;
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
impl From<crate::W<SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECL` reader - desc SECL"]
pub type SECL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECL` writer - desc SECL"]
pub type SECL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEC_SPEC, u8, u8, 4, O>;
#[doc = "Field `SECH` reader - desc SECH"]
pub type SECH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECH` writer - desc SECH"]
pub type SECH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEC_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc SECL"]
    #[inline(always)]
    pub fn secl(&self) -> SECL_R {
        SECL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - desc SECH"]
    #[inline(always)]
    pub fn sech(&self) -> SECH_R {
        SECH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SECL"]
    #[inline(always)]
    pub fn secl(&mut self) -> SECL_W<0> {
        SECL_W::new(self)
    }
    #[doc = "Bits 4:6 - desc SECH"]
    #[inline(always)]
    pub fn sech(&mut self) -> SECH_W<4> {
        SECH_W::new(self)
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
#[doc = "desc SEC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec](index.html) module"]
pub struct SEC_SPEC;
impl crate::RegisterSpec for SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec::R](R) reader structure"]
impl crate::Readable for SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec::W](W) writer structure"]
impl crate::Writable for SEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC to value 0"]
impl crate::Resettable for SEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
