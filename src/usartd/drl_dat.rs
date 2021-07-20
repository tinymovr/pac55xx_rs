#[doc = "Register `DRL_DAT` reader"]
pub struct R(crate::R<DRL_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRL_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRL_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRL_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRL_DAT` writer"]
pub struct W(crate::W<DRL_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRL_DAT_SPEC>;
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
impl From<crate::W<DRL_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRL_DAT_SPEC>) -> Self {
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
#[doc = "UART D Divisor Latch / SSP D Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drl_dat](index.html) module"]
pub struct DRL_DAT_SPEC;
impl crate::RegisterSpec for DRL_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [drl_dat::R](R) reader structure"]
impl crate::Readable for DRL_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drl_dat::W](W) writer structure"]
impl crate::Writable for DRL_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRL_DAT to value 0"]
impl crate::Resettable for DRL_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
