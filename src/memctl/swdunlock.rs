#[doc = "Register `SWDUNLOCK` reader"]
pub struct R(crate::R<SWDUNLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWDUNLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWDUNLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWDUNLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWDUNLOCK` writer"]
pub struct W(crate::W<SWDUNLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWDUNLOCK_SPEC>;
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
impl From<crate::W<SWDUNLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWDUNLOCK_SPEC>) -> Self {
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
#[doc = "SWD Unlock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swdunlock](index.html) module"]
pub struct SWDUNLOCK_SPEC;
impl crate::RegisterSpec for SWDUNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swdunlock::R](R) reader structure"]
impl crate::Readable for SWDUNLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swdunlock::W](W) writer structure"]
impl crate::Writable for SWDUNLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWDUNLOCK to value 0"]
impl crate::Resettable for SWDUNLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
