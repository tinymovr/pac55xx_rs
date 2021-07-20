#[doc = "Register `PFDS` reader"]
pub struct R(crate::R<PFDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFDS` writer"]
pub struct W(crate::W<PFDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFDS_SPEC>;
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
impl From<crate::W<PFDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFDS_SPEC>) -> Self {
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
#[doc = "PF Drive Strength/Schmitt Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfds](index.html) module"]
pub struct PFDS_SPEC;
impl crate::RegisterSpec for PFDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfds::R](R) reader structure"]
impl crate::Readable for PFDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfds::W](W) writer structure"]
impl crate::Writable for PFDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFDS to value 0"]
impl crate::Resettable for PFDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
