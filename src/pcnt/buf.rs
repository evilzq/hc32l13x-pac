#[doc = "Register `BUF` reader"]
pub struct R(crate::R<BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF` writer"]
pub struct W(crate::W<BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_SPEC>;
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
impl From<crate::W<BUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF` reader - desc BUF"]
pub type BUF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BUF` writer - desc BUF"]
pub type BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc BUF"]
    #[inline(always)]
    pub fn buf(&self) -> BUF_R {
        BUF_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc BUF"]
    #[inline(always)]
    pub fn buf(&mut self) -> BUF_W<0> {
        BUF_W::new(self)
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
#[doc = "desc BUF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf](index.html) module"]
pub struct BUF_SPEC;
impl crate::RegisterSpec for BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf::R](R) reader structure"]
impl crate::Readable for BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf::W](W) writer structure"]
impl crate::Writable for BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF to value 0xff"]
impl crate::Resettable for BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
