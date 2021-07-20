#[doc = "Register `ADRM3` reader"]
pub struct R(crate::R<ADRM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADRM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADRM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADRM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADRM3` writer"]
pub struct W(crate::W<ADRM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADRM3_SPEC>;
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
impl From<crate::W<ADRM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADRM3_SPEC>) -> Self {
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
#[doc = "I2C Slave Address Mask 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adrm3](index.html) module"]
pub struct ADRM3_SPEC;
impl crate::RegisterSpec for ADRM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adrm3::R](R) reader structure"]
impl crate::Readable for ADRM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adrm3::W](W) writer structure"]
impl crate::Writable for ADRM3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADRM3 to value 0"]
impl crate::Resettable for ADRM3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
