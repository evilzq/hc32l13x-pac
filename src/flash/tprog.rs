#[doc = "Register `TPROG` reader"]
pub struct R(crate::R<TPROG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPROG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPROG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPROG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPROG` writer"]
pub struct W(crate::W<TPROG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPROG_SPEC>;
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
impl From<crate::W<TPROG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPROG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPROG` reader - desc TPROG"]
pub type TPROG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPROG` writer - desc TPROG"]
pub type TPROG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPROG_SPEC, u16, u16, 9, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPROG_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 0:8 - desc TPROG"]
    #[inline(always)]
    pub fn tprog(&self) -> TPROG_R {
        TPROG_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TPROG"]
    #[inline(always)]
    pub fn tprog(&mut self) -> TPROG_W<0> {
        TPROG_W::new(self)
    }
    #[doc = "Bits 9:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<9> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TPROG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tprog](index.html) module"]
pub struct TPROG_SPEC;
impl crate::RegisterSpec for TPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tprog::R](R) reader structure"]
impl crate::Readable for TPROG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tprog::W](W) writer structure"]
impl crate::Writable for TPROG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPROG to value 0x1b"]
impl crate::Resettable for TPROG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1b
    }
}
