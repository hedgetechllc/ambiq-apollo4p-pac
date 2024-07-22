#[doc = "Register `SIMOBUCK13` reader"]
pub type R = crate::R<Simobuck13Spec>;
#[doc = "Register `SIMOBUCK13` writer"]
pub type W = crate::W<Simobuck13Spec>;
#[doc = "Field `ACTTRIMVDDS` reader - Active VDDS trim."]
pub type ActtrimvddsR = crate::FieldReader;
#[doc = "Field `ACTTRIMVDDS` writer - Active VDDS trim."]
pub type ActtrimvddsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LPTRIMVDDS` reader - Low power VDDS trim."]
pub type LptrimvddsR = crate::FieldReader;
#[doc = "Field `LPTRIMVDDS` writer - Low power VDDS trim."]
pub type LptrimvddsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 20:25 - Active VDDS trim."]
    #[inline(always)]
    pub fn acttrimvdds(&self) -> ActtrimvddsR {
        ActtrimvddsR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - Low power VDDS trim."]
    #[inline(always)]
    pub fn lptrimvdds(&self) -> LptrimvddsR {
        LptrimvddsR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:25 - Active VDDS trim."]
    #[inline(always)]
    #[must_use]
    pub fn acttrimvdds(&mut self) -> ActtrimvddsW<Simobuck13Spec> {
        ActtrimvddsW::new(self, 20)
    }
    #[doc = "Bits 26:31 - Low power VDDS trim."]
    #[inline(always)]
    #[must_use]
    pub fn lptrimvdds(&mut self) -> LptrimvddsW<Simobuck13Spec> {
        LptrimvddsW::new(self, 26)
    }
}
#[doc = "SIMO Buck Compare, Brown out, Active, Low power Trim Control\n\nYou can [`read`](crate::Reg::read) this register and get [`simobuck13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobuck13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simobuck13Spec;
impl crate::RegisterSpec for Simobuck13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobuck13::R`](R) reader structure"]
impl crate::Readable for Simobuck13Spec {}
#[doc = "`write(|w| ..)` method takes [`simobuck13::W`](W) writer structure"]
impl crate::Writable for Simobuck13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBUCK13 to value 0"]
impl crate::Resettable for Simobuck13Spec {
    const RESET_VALUE: u32 = 0;
}
