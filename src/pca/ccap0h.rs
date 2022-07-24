#[doc = "Register `CCAP0H` reader"]
pub struct R(crate::R<CCAP0H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCAP0H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCAP0H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCAP0H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCAP0H` writer"]
pub struct W(crate::W<CCAP0H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCAP0H_SPEC>;
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
impl From<crate::W<CCAP0H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCAP0H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCAP0` reader - desc CCAP0"]
pub type CCAP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCAP0` writer - desc CCAP0"]
pub type CCAP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCAP0H_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAP0H_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc CCAP0"]
    #[inline(always)]
    pub fn ccap0(&self) -> CCAP0_R {
        CCAP0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc CCAP0"]
    #[inline(always)]
    pub fn ccap0(&mut self) -> CCAP0_W<0> {
        CCAP0_W::new(self)
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
#[doc = "desc CCAP0H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccap0h](index.html) module"]
pub struct CCAP0H_SPEC;
impl crate::RegisterSpec for CCAP0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccap0h::R](R) reader structure"]
impl crate::Readable for CCAP0H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccap0h::W](W) writer structure"]
impl crate::Writable for CCAP0H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCAP0H to value 0"]
impl crate::Resettable for CCAP0H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
