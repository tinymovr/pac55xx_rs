#[doc = "Register `PCPDEN` reader"]
pub struct R(crate::R<PCPDEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPDEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPDEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPDEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCPDEN` writer"]
pub struct W(crate::W<PCPDEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCPDEN_SPEC>;
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
impl From<crate::W<PCPDEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCPDEN_SPEC>) -> Self {
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
#[doc = "PC Weak Pull-down Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpden](index.html) module"]
pub struct PCPDEN_SPEC;
impl crate::RegisterSpec for PCPDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpden::R](R) reader structure"]
impl crate::Readable for PCPDEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcpden::W](W) writer structure"]
impl crate::Writable for PCPDEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCPDEN to value 0"]
impl crate::Resettable for PCPDEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
