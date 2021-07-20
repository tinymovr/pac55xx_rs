#[doc = "Register `DTSERES20` reader"]
pub struct R(crate::R<DTSERES20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSERES20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSERES20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSERES20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSERES20` writer"]
pub struct W(crate::W<DTSERES20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSERES20_SPEC>;
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
impl From<crate::W<DTSERES20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSERES20_SPEC>) -> Self {
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
#[doc = "DTSE Result 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtseres20](index.html) module"]
pub struct DTSERES20_SPEC;
impl crate::RegisterSpec for DTSERES20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtseres20::R](R) reader structure"]
impl crate::Readable for DTSERES20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtseres20::W](W) writer structure"]
impl crate::Writable for DTSERES20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTSERES20 to value 0"]
impl crate::Resettable for DTSERES20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
