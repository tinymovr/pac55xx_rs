#[doc = "Register `PCPUEN` reader"]
pub struct R(crate::R<PCPUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCPUEN` writer"]
pub struct W(crate::W<PCPUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCPUEN_SPEC>;
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
impl From<crate::W<PCPUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCPUEN_SPEC>) -> Self {
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
#[doc = "PC Weak Pull-up Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpuen](index.html) module"]
pub struct PCPUEN_SPEC;
impl crate::RegisterSpec for PCPUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpuen::R](R) reader structure"]
impl crate::Readable for PCPUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcpuen::W](W) writer structure"]
impl crate::Writable for PCPUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCPUEN to value 0"]
impl crate::Resettable for PCPUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
