#[doc = "Register `COMPEN` reader"]
pub struct R(crate::R<COMPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPEN` writer"]
pub struct W(crate::W<COMPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPEN_SPEC>;
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
impl From<crate::W<COMPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - desc CR"]
pub type CR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CR` writer - desc CR"]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMPEN_SPEC, u16, u16, 9, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPEN_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPEN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - desc CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    #[doc = "Bit 15 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<15> {
        EN_W::new(self)
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
#[doc = "desc COMPEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compen](index.html) module"]
pub struct COMPEN_SPEC;
impl crate::RegisterSpec for COMPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compen::R](R) reader structure"]
impl crate::Readable for COMPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compen::W](W) writer structure"]
impl crate::Writable for COMPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMPEN to value 0x20"]
impl crate::Resettable for COMPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
