#[doc = "Register `CORECTRL` reader"]
pub type R = crate::R<CorectrlSpec>;
#[doc = "Register `CORECTRL` writer"]
pub type W = crate::W<CorectrlSpec>;
#[doc = "Field `CORECTRL` reader - Overall control of PDM core. Internal use only"]
pub type CorectrlR = crate::FieldReader<u32>;
#[doc = "Field `CORECTRL` writer - Overall control of PDM core. Internal use only"]
pub type CorectrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Overall control of PDM core. Internal use only"]
    #[inline(always)]
    pub fn corectrl(&self) -> CorectrlR {
        CorectrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Overall control of PDM core. Internal use only"]
    #[inline(always)]
    #[must_use]
    pub fn corectrl(&mut self) -> CorectrlW<CorectrlSpec> {
        CorectrlW::new(self, 0)
    }
}
#[doc = "PDM to PCM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`corectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorectrlSpec;
impl crate::RegisterSpec for CorectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`corectrl::R`](R) reader structure"]
impl crate::Readable for CorectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`corectrl::W`](W) writer structure"]
impl crate::Writable for CorectrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORECTRL to value 0x8000_0434"]
impl crate::Resettable for CorectrlSpec {
    const RESET_VALUE: u32 = 0x8000_0434;
}
