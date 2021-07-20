#[doc = "Register `INTEDGEBOTH` reader"]
pub struct R(crate::R<INTEDGEBOTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEDGEBOTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEDGEBOTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEDGEBOTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEDGEBOTH` writer"]
pub struct W(crate::W<INTEDGEBOTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEDGEBOTH_SPEC>;
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
impl From<crate::W<INTEDGEBOTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEDGEBOTH_SPEC>) -> Self {
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
#[doc = "GPIO G Interrupt Edge Both\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intedgeboth](index.html) module"]
pub struct INTEDGEBOTH_SPEC;
impl crate::RegisterSpec for INTEDGEBOTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intedgeboth::R](R) reader structure"]
impl crate::Readable for INTEDGEBOTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intedgeboth::W](W) writer structure"]
impl crate::Writable for INTEDGEBOTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEDGEBOTH to value 0"]
impl crate::Resettable for INTEDGEBOTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
