#[doc = "Register `ADRM1` reader"]
pub struct R(crate::R<ADRM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADRM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADRM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADRM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADRM1` writer"]
pub struct W(crate::W<ADRM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADRM1_SPEC>;
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
impl From<crate::W<ADRM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADRM1_SPEC>) -> Self {
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
#[doc = "I2C Slave Address Mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adrm1](index.html) module"]
pub struct ADRM1_SPEC;
impl crate::RegisterSpec for ADRM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adrm1::R](R) reader structure"]
impl crate::Readable for ADRM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adrm1::W](W) writer structure"]
impl crate::Writable for ADRM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADRM1 to value 0"]
impl crate::Resettable for ADRM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
