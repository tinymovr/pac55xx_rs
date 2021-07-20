#[doc = "Register `INTTYPE` reader"]
pub struct R(crate::R<INTTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTTYPE` writer"]
pub struct W(crate::W<INTTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTTYPE_SPEC>;
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
impl From<crate::W<INTTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTTYPE_SPEC>) -> Self {
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
#[doc = "GPIO B Interrupt Type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttype](index.html) module"]
pub struct INTTYPE_SPEC;
impl crate::RegisterSpec for INTTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inttype::R](R) reader structure"]
impl crate::Readable for INTTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inttype::W](W) writer structure"]
impl crate::Writable for INTTYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTTYPE to value 0"]
impl crate::Resettable for INTTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
