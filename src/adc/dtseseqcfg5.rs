#[doc = "Register `DTSESEQCFG5` reader"]
pub struct R(crate::R<DTSESEQCFG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSESEQCFG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSESEQCFG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSESEQCFG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSESEQCFG5` writer"]
pub struct W(crate::W<DTSESEQCFG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSESEQCFG5_SPEC>;
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
impl From<crate::W<DTSESEQCFG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSESEQCFG5_SPEC>) -> Self {
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
#[doc = "DTSE Sequence Config 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtseseqcfg5](index.html) module"]
pub struct DTSESEQCFG5_SPEC;
impl crate::RegisterSpec for DTSESEQCFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtseseqcfg5::R](R) reader structure"]
impl crate::Readable for DTSESEQCFG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtseseqcfg5::W](W) writer structure"]
impl crate::Writable for DTSESEQCFG5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTSESEQCFG5 to value 0"]
impl crate::Resettable for DTSESEQCFG5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
