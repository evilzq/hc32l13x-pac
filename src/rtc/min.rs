#[doc = "Register `MIN` reader"]
pub struct R(crate::R<MIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIN` writer"]
pub struct W(crate::W<MIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIN_SPEC>;
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
impl From<crate::W<MIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MINL` reader - desc MINL"]
pub type MINL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINL` writer - desc MINL"]
pub type MINL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIN_SPEC, u8, u8, 4, O>;
#[doc = "Field `MINH` reader - desc MINH"]
pub type MINH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINH` writer - desc MINH"]
pub type MINH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIN_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc MINL"]
    #[inline(always)]
    pub fn minl(&self) -> MINL_R {
        MINL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - desc MINH"]
    #[inline(always)]
    pub fn minh(&self) -> MINH_R {
        MINH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MINL"]
    #[inline(always)]
    pub fn minl(&mut self) -> MINL_W<0> {
        MINL_W::new(self)
    }
    #[doc = "Bits 4:6 - desc MINH"]
    #[inline(always)]
    pub fn minh(&mut self) -> MINH_W<4> {
        MINH_W::new(self)
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
#[doc = "desc MIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [min](index.html) module"]
pub struct MIN_SPEC;
impl crate::RegisterSpec for MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [min::R](R) reader structure"]
impl crate::Readable for MIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [min::W](W) writer structure"]
impl crate::Writable for MIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIN to value 0"]
impl crate::Resettable for MIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
