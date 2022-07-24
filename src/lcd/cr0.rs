#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `LCDCLK` reader - desc LCDCLK"]
pub type LCDCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCDCLK` writer - desc LCDCLK"]
pub type LCDCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CPCLK` reader - desc CPCLK"]
pub type CPCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPCLK` writer - desc CPCLK"]
pub type CPCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `BIAS` reader - desc BIAS"]
pub type BIAS_R = crate::BitReader<bool>;
#[doc = "Field `BIAS` writer - desc BIAS"]
pub type BIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `DUTY` reader - desc DUTY"]
pub type DUTY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUTY` writer - desc DUTY"]
pub type DUTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `BSEL` reader - desc BSEL"]
pub type BSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSEL` writer - desc BSEL"]
pub type BSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `CONTRAST` reader - desc CONTRAST"]
pub type CONTRAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONTRAST` writer - desc CONTRAST"]
pub type CONTRAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc LCDCLK"]
    #[inline(always)]
    pub fn lcdclk(&self) -> LCDCLK_R {
        LCDCLK_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - desc CPCLK"]
    #[inline(always)]
    pub fn cpclk(&self) -> CPCLK_R {
        CPCLK_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - desc BIAS"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - desc DUTY"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - desc BSEL"]
    #[inline(always)]
    pub fn bsel(&self) -> BSEL_R {
        BSEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - desc CONTRAST"]
    #[inline(always)]
    pub fn contrast(&self) -> CONTRAST_R {
        CONTRAST_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - desc LCDCLK"]
    #[inline(always)]
    pub fn lcdclk(&mut self) -> LCDCLK_W<1> {
        LCDCLK_W::new(self)
    }
    #[doc = "Bits 3:4 - desc CPCLK"]
    #[inline(always)]
    pub fn cpclk(&mut self) -> CPCLK_W<3> {
        CPCLK_W::new(self)
    }
    #[doc = "Bit 5 - desc BIAS"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W<5> {
        BIAS_W::new(self)
    }
    #[doc = "Bits 6:8 - desc DUTY"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W<6> {
        DUTY_W::new(self)
    }
    #[doc = "Bits 9:11 - desc BSEL"]
    #[inline(always)]
    pub fn bsel(&mut self) -> BSEL_W<9> {
        BSEL_W::new(self)
    }
    #[doc = "Bits 12:15 - desc CONTRAST"]
    #[inline(always)]
    pub fn contrast(&mut self) -> CONTRAST_W<12> {
        CONTRAST_W::new(self)
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
#[doc = "desc CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0xda"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xda
    }
}
