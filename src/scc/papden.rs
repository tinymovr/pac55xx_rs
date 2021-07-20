#[doc = "Register `PAPDEN` reader"]
pub struct R(crate::R<PAPDEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAPDEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAPDEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAPDEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAPDEN` writer"]
pub struct W(crate::W<PAPDEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAPDEN_SPEC>;
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
impl From<crate::W<PAPDEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAPDEN_SPEC>) -> Self {
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
#[doc = "PA Weak Pull-down Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [papden](index.html) module"]
pub struct PAPDEN_SPEC;
impl crate::RegisterSpec for PAPDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [papden::R](R) reader structure"]
impl crate::Readable for PAPDEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [papden::W](W) writer structure"]
impl crate::Writable for PAPDEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAPDEN to value 0"]
impl crate::Resettable for PAPDEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
