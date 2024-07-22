#[doc = "Register `SIMOBUCK12` reader"]
pub type R = crate::R<Simobuck12Spec>;
#[doc = "Register `SIMOBUCK12` writer"]
pub type W = crate::W<Simobuck12Spec>;
#[doc = "Field `ACTTRIMVDDF` reader - Active VDDF trim."]
pub type ActtrimvddfR = crate::FieldReader;
#[doc = "Field `ACTTRIMVDDF` writer - Active VDDF trim."]
pub type ActtrimvddfW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LPTRIMVDDF` reader - Low power VDDF trim."]
pub type LptrimvddfR = crate::FieldReader;
#[doc = "Field `LPTRIMVDDF` writer - Low power VDDF trim."]
pub type LptrimvddfW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 20:25 - Active VDDF trim."]
    #[inline(always)]
    pub fn acttrimvddf(&self) -> ActtrimvddfR {
        ActtrimvddfR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - Low power VDDF trim."]
    #[inline(always)]
    pub fn lptrimvddf(&self) -> LptrimvddfR {
        LptrimvddfR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:25 - Active VDDF trim."]
    #[inline(always)]
    #[must_use]
    pub fn acttrimvddf(&mut self) -> ActtrimvddfW<Simobuck12Spec> {
        ActtrimvddfW::new(self, 20)
    }
    #[doc = "Bits 26:31 - Low power VDDF trim."]
    #[inline(always)]
    #[must_use]
    pub fn lptrimvddf(&mut self) -> LptrimvddfW<Simobuck12Spec> {
        LptrimvddfW::new(self, 26)
    }
}
#[doc = "SIMO Buck Compare, Brown out, Active, Low power Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck12Spec;
impl crate::RegisterSpec for Simobuck12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck12::R`](R) reader structure"]
impl crate::Readable for Simobuck12Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck12::W`](W) writer structure"]
impl crate::Writable for Simobuck12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK12 to value 0"]
impl crate::Resettable for Simobuck12Spec {
    const RESET_VALUE: u32 = 0;
}
