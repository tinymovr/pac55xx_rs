#[doc = "Register `SCR_SSCR` reader"]
pub struct R(crate::R<SCR_SSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR_SSCR` writer"]
pub struct W(crate::W<SCR_SSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SSCR_SPEC>;
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
impl From<crate::W<SCR_SSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SSCR_SPEC>) -> Self {
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
#[doc = "UART B Scratch Pad / SSP B Slave Select Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr_sscr](index.html) module"]
pub struct SCR_SSCR_SPEC;
impl crate::RegisterSpec for SCR_SSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr_sscr::R](R) reader structure"]
impl crate::Readable for SCR_SSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr_sscr::W](W) writer structure"]
impl crate::Writable for SCR_SSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR_SSCR to value 0"]
impl crate::Resettable for SCR_SSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
