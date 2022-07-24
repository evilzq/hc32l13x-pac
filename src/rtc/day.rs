#[doc = "Register `DAY` reader"]
pub struct R(crate::R<DAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAY` writer"]
pub struct W(crate::W<DAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAY_SPEC>;
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
impl From<crate::W<DAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAYL` reader - desc DAYL"]
pub type DAYL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYL` writer - desc DAYL"]
pub type DAYL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAY_SPEC, u8, u8, 4, O>;
#[doc = "Field `DAYH` reader - desc DAYH"]
pub type DAYH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYH` writer - desc DAYH"]
pub type DAYH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAY_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAY_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc DAYL"]
    #[inline(always)]
    pub fn dayl(&self) -> DAYL_R {
        DAYL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - desc DAYH"]
    #[inline(always)]
    pub fn dayh(&self) -> DAYH_R {
        DAYH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc DAYL"]
    #[inline(always)]
    pub fn dayl(&mut self) -> DAYL_W<0> {
        DAYL_W::new(self)
    }
    #[doc = "Bits 4:5 - desc DAYH"]
    #[inline(always)]
    pub fn dayh(&mut self) -> DAYH_W<4> {
        DAYH_W::new(self)
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
#[doc = "desc DAY\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [day](index.html) module"]
pub struct DAY_SPEC;
impl crate::RegisterSpec for DAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [day::R](R) reader structure"]
impl crate::Readable for DAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [day::W](W) writer structure"]
impl crate::Writable for DAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAY to value 0"]
impl crate::Resettable for DAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
