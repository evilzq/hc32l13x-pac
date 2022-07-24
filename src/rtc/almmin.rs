#[doc = "Register `ALMMIN` reader"]
pub struct R(crate::R<ALMMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALMMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALMMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALMMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALMMIN` writer"]
pub struct W(crate::W<ALMMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALMMIN_SPEC>;
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
impl From<crate::W<ALMMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALMMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALMMINL` reader - desc ALMMINL"]
pub type ALMMINL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALMMINL` writer - desc ALMMINL"]
pub type ALMMINL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALMMIN_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALMMINH` reader - desc ALMMINH"]
pub type ALMMINH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALMMINH` writer - desc ALMMINH"]
pub type ALMMINH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALMMIN_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALMMIN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc ALMMINL"]
    #[inline(always)]
    pub fn almminl(&self) -> ALMMINL_R {
        ALMMINL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - desc ALMMINH"]
    #[inline(always)]
    pub fn almminh(&self) -> ALMMINH_R {
        ALMMINH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc ALMMINL"]
    #[inline(always)]
    pub fn almminl(&mut self) -> ALMMINL_W<0> {
        ALMMINL_W::new(self)
    }
    #[doc = "Bits 4:6 - desc ALMMINH"]
    #[inline(always)]
    pub fn almminh(&mut self) -> ALMMINH_W<4> {
        ALMMINH_W::new(self)
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
#[doc = "desc ALMMIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [almmin](index.html) module"]
pub struct ALMMIN_SPEC;
impl crate::RegisterSpec for ALMMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [almmin::R](R) reader structure"]
impl crate::Readable for ALMMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [almmin::W](W) writer structure"]
impl crate::Writable for ALMMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALMMIN to value 0"]
impl crate::Resettable for ALMMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
