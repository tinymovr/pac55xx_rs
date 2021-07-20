#[doc = "Register `ADCRES` reader"]
pub struct R(crate::R<ADCRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCRES` writer"]
pub struct W(crate::W<ADCRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCRES_SPEC>;
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
impl From<crate::W<ADCRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCRES_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcres](index.html) module"]
pub struct ADCRES_SPEC;
impl crate::RegisterSpec for ADCRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcres::R](R) reader structure"]
impl crate::Readable for ADCRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcres::W](W) writer structure"]
impl crate::Writable for ADCRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCRES to value 0"]
impl crate::Resettable for ADCRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
