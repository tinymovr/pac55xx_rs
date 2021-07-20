#[doc = "Register `CONCLR` reader"]
pub struct R(crate::R<CONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONCLR` writer"]
pub struct W(crate::W<CONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONCLR_SPEC>;
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
impl From<crate::W<CONCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONCLR_SPEC>) -> Self {
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
#[doc = "I2C Control Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conclr](index.html) module"]
pub struct CONCLR_SPEC;
impl crate::RegisterSpec for CONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conclr::R](R) reader structure"]
impl crate::Readable for CONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conclr::W](W) writer structure"]
impl crate::Writable for CONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONCLR to value 0"]
impl crate::Resettable for CONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
