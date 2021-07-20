#[doc = "Register `AMR` reader"]
pub struct R(crate::R<AMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMR` writer"]
pub struct W(crate::W<AMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMR_SPEC>;
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
impl From<crate::W<AMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMR0` reader - CAN Acceptance Mask 0"]
pub struct AMR0_R(crate::FieldReader<u8, u8>);
impl AMR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        AMR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMR0` writer - CAN Acceptance Mask 0"]
pub struct AMR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AMR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `AMR1` reader - CAN Acceptance Mask 1"]
pub struct AMR1_R(crate::FieldReader<u8, u8>);
impl AMR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        AMR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMR1` writer - CAN Acceptance Mask 1"]
pub struct AMR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AMR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `AMR2` reader - CAN Acceptance Mask 2"]
pub struct AMR2_R(crate::FieldReader<u8, u8>);
impl AMR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        AMR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMR2` writer - CAN Acceptance Mask 2"]
pub struct AMR2_W<'a> {
    w: &'a mut W,
}
impl<'a> AMR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `AMR3` reader - CAN Acceptance Mask 3"]
pub struct AMR3_R(crate::FieldReader<u8, u8>);
impl AMR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        AMR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMR3` writer - CAN Acceptance Mask 3"]
pub struct AMR3_W<'a> {
    w: &'a mut W,
}
impl<'a> AMR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CAN Acceptance Mask 0"]
    #[inline(always)]
    pub fn amr0(&self) -> AMR0_R {
        AMR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CAN Acceptance Mask 1"]
    #[inline(always)]
    pub fn amr1(&self) -> AMR1_R {
        AMR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CAN Acceptance Mask 2"]
    #[inline(always)]
    pub fn amr2(&self) -> AMR2_R {
        AMR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CAN Acceptance Mask 3"]
    #[inline(always)]
    pub fn amr3(&self) -> AMR3_R {
        AMR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CAN Acceptance Mask 0"]
    #[inline(always)]
    pub fn amr0(&mut self) -> AMR0_W {
        AMR0_W { w: self }
    }
    #[doc = "Bits 8:15 - CAN Acceptance Mask 1"]
    #[inline(always)]
    pub fn amr1(&mut self) -> AMR1_W {
        AMR1_W { w: self }
    }
    #[doc = "Bits 16:23 - CAN Acceptance Mask 2"]
    #[inline(always)]
    pub fn amr2(&mut self) -> AMR2_W {
        AMR2_W { w: self }
    }
    #[doc = "Bits 24:31 - CAN Acceptance Mask 3"]
    #[inline(always)]
    pub fn amr3(&mut self) -> AMR3_W {
        AMR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined CAN Acceptance Mask registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amr](index.html) module"]
pub struct AMR_SPEC;
impl crate::RegisterSpec for AMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amr::R](R) reader structure"]
impl crate::Readable for AMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amr::W](W) writer structure"]
impl crate::Writable for AMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMR to value 0"]
impl crate::Resettable for AMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
