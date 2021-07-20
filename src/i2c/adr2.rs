#[doc = "Register `ADR2` reader"]
pub struct R(crate::R<ADR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADR2` writer"]
pub struct W(crate::W<ADR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADR2_SPEC>;
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
impl From<crate::W<ADR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADR2_SPEC>) -> Self {
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
#[doc = "I2C Slave Address 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adr2](index.html) module"]
pub struct ADR2_SPEC;
impl crate::RegisterSpec for ADR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adr2::R](R) reader structure"]
impl crate::Readable for ADR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adr2::W](W) writer structure"]
impl crate::Writable for ADR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADR2 to value 0"]
impl crate::Resettable for ADR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
