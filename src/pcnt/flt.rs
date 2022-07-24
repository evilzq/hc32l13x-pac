#[doc = "Register `FLT` reader"]
pub struct R(crate::R<FLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT` writer"]
pub struct W(crate::W<FLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT_SPEC>;
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
impl From<crate::W<FLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - desc CLKDIV"]
pub type CLKDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKDIV` writer - desc CLKDIV"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT_SPEC, u16, u16, 13, O>;
#[doc = "Field `DEBTOP` reader - desc DEBTOP"]
pub type DEBTOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBTOP` writer - desc DEBTOP"]
pub type DEBTOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT_SPEC, u8, u8, 3, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:12 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - desc DEBTOP"]
    #[inline(always)]
    pub fn debtop(&self) -> DEBTOP_R {
        DEBTOP_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 13:15 - desc DEBTOP"]
    #[inline(always)]
    pub fn debtop(&mut self) -> DEBTOP_W<13> {
        DEBTOP_W::new(self)
    }
    #[doc = "Bit 16 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<16> {
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
#[doc = "desc FLT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt](index.html) module"]
pub struct FLT_SPEC;
impl crate::RegisterSpec for FLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt::R](R) reader structure"]
impl crate::Readable for FLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt::W](W) writer structure"]
impl crate::Writable for FLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT to value 0x2000"]
impl crate::Resettable for FLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
