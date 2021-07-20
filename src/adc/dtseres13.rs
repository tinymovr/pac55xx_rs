#[doc = "Register `DTSERES13` reader"]
pub struct R(crate::R<DTSERES13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTSERES13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTSERES13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTSERES13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTSERES13` writer"]
pub struct W(crate::W<DTSERES13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTSERES13_SPEC>;
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
impl From<crate::W<DTSERES13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTSERES13_SPEC>) -> Self {
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
#[doc = "DTSE Result 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtseres13](index.html) module"]
pub struct DTSERES13_SPEC;
impl crate::RegisterSpec for DTSERES13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtseres13::R](R) reader structure"]
impl crate::Readable for DTSERES13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtseres13::W](W) writer structure"]
impl crate::Writable for DTSERES13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTSERES13 to value 0"]
impl crate::Resettable for DTSERES13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
