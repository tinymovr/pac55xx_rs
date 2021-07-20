#[doc = "Register `PGPUEN` reader"]
pub struct R(crate::R<PGPUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGPUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGPUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGPUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGPUEN` writer"]
pub struct W(crate::W<PGPUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGPUEN_SPEC>;
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
impl From<crate::W<PGPUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGPUEN_SPEC>) -> Self {
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
#[doc = "PG Weak Pull-up Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgpuen](index.html) module"]
pub struct PGPUEN_SPEC;
impl crate::RegisterSpec for PGPUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgpuen::R](R) reader structure"]
impl crate::Readable for PGPUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgpuen::W](W) writer structure"]
impl crate::Writable for PGPUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGPUEN to value 0"]
impl crate::Resettable for PGPUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
