#[doc = "Register `PDPUEN` reader"]
pub struct R(crate::R<PDPUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDPUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDPUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDPUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDPUEN` writer"]
pub struct W(crate::W<PDPUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDPUEN_SPEC>;
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
impl From<crate::W<PDPUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDPUEN_SPEC>) -> Self {
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
#[doc = "PD Weak Pull-up Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdpuen](index.html) module"]
pub struct PDPUEN_SPEC;
impl crate::RegisterSpec for PDPUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdpuen::R](R) reader structure"]
impl crate::Readable for PDPUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdpuen::W](W) writer structure"]
impl crate::Writable for PDPUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDPUEN to value 0"]
impl crate::Resettable for PDPUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
