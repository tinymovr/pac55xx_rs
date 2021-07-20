#[doc = "Register `CONSET` reader"]
pub struct R(crate::R<CONSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSET` writer"]
pub struct W(crate::W<CONSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSET_SPEC>;
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
impl From<crate::W<CONSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSET_SPEC>) -> Self {
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
#[doc = "I2C Control Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conset](index.html) module"]
pub struct CONSET_SPEC;
impl crate::RegisterSpec for CONSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conset::R](R) reader structure"]
impl crate::Readable for CONSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conset::W](W) writer structure"]
impl crate::Writable for CONSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONSET to value 0"]
impl crate::Resettable for CONSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
