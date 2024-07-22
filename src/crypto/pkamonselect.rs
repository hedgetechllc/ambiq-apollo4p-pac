#[doc = "Register `PKAMONSELECT` reader"]
pub type R = crate::R<PkamonselectSpec>;
#[doc = "Register `PKAMONSELECT` writer"]
pub type W = crate::W<PkamonselectSpec>;
#[doc = "Field `PKAMONSELECT` reader - Defines which PKA FSM monitor is being output."]
pub type PkamonselectR = crate::FieldReader;
#[doc = "Field `PKAMONSELECT` writer - Defines which PKA FSM monitor is being output."]
pub type PkamonselectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Defines which PKA FSM monitor is being output."]
    #[inline(always)]
    pub fn pkamonselect(&self) -> PkamonselectR {
        PkamonselectR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines which PKA FSM monitor is being output."]
    #[inline(always)]
    #[must_use]
    pub fn pkamonselect(&mut self) -> PkamonselectW<PkamonselectSpec> {
        PkamonselectW::new(self, 0)
    }
}
#[doc = "This register defines which PKA FSM monitor is being output.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkamonselect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkamonselect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkamonselectSpec;
impl crate::RegisterSpec for PkamonselectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkamonselect::R`](R) reader structure"]
impl crate::Readable for PkamonselectSpec {}
#[doc = "`write(|w| ..)` method takes [`pkamonselect::W`](W) writer structure"]
impl crate::Writable for PkamonselectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAMONSELECT to value 0"]
impl crate::Resettable for PkamonselectSpec {
    const RESET_VALUE: u32 = 0;
}
