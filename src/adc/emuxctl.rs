#[doc = "Register `EMUXCTL` reader"]
pub struct R(crate::R<EMUXCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMUXCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMUXCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMUXCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMUXCTL` writer"]
pub struct W(crate::W<EMUXCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMUXCTL_SPEC>;
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
impl From<crate::W<EMUXCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMUXCTL_SPEC>) -> Self {
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
#[doc = "EMUX Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emuxctl](index.html) module"]
pub struct EMUXCTL_SPEC;
impl crate::RegisterSpec for EMUXCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emuxctl::R](R) reader structure"]
impl crate::Readable for EMUXCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emuxctl::W](W) writer structure"]
impl crate::Writable for EMUXCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMUXCTL to value 0"]
impl crate::Resettable for EMUXCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
