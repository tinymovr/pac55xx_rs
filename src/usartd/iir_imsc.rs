#[doc = "Register `IIR_IMSC` reader"]
pub struct R(crate::R<IIR_IMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_IMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_IMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_IMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIR_IMSC` writer"]
pub struct W(crate::W<IIR_IMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIR_IMSC_SPEC>;
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
impl From<crate::W<IIR_IMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIR_IMSC_SPEC>) -> Self {
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
#[doc = "UART D Interrupt Identification / SSP D Interrupt Mask Set and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir_imsc](index.html) module"]
pub struct IIR_IMSC_SPEC;
impl crate::RegisterSpec for IIR_IMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iir_imsc::R](R) reader structure"]
impl crate::Readable for IIR_IMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iir_imsc::W](W) writer structure"]
impl crate::Writable for IIR_IMSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IIR_IMSC to value 0"]
impl crate::Resettable for IIR_IMSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
