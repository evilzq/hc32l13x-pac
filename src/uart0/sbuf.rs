#[doc = "Register `SBUF` reader"]
pub struct R(crate::R<SBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBUF` writer"]
pub struct W(crate::W<SBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBUF_SPEC>;
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
impl From<crate::W<SBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - desc DATA"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - desc DATA"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SBUF_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA8` reader - desc DATA8"]
pub type DATA8_R = crate::BitReader<bool>;
#[doc = "Field `DATA8` writer - desc DATA8"]
pub type DATA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBUF_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBUF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - desc DATA8"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bit 8 - desc DATA8"]
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W<8> {
        DATA8_W::new(self)
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
#[doc = "desc SBUF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbuf](index.html) module"]
pub struct SBUF_SPEC;
impl crate::RegisterSpec for SBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbuf::R](R) reader structure"]
impl crate::Readable for SBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbuf::W](W) writer structure"]
impl crate::Writable for SBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SBUF to value 0"]
impl crate::Resettable for SBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
