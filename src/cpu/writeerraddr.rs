#[doc = "Register `WRITEERRADDR` reader"]
pub type R = crate::R<WriteerraddrSpec>;
#[doc = "Register `WRITEERRADDR` writer"]
pub type W = crate::W<WriteerraddrSpec>;
#[doc = "Field `WERRADDR` reader - This address will be approximate since multiple write transactions might be in flight at any given time. However, it should be accurate when debugging/single-stepping"]
pub type WerraddrR = crate::FieldReader<u32>;
#[doc = "Field `WERRADDR` writer - This address will be approximate since multiple write transactions might be in flight at any given time. However, it should be accurate when debugging/single-stepping"]
pub type WerraddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This address will be approximate since multiple write transactions might be in flight at any given time. However, it should be accurate when debugging/single-stepping"]
    #[inline(always)]
    pub fn werraddr(&self) -> WerraddrR {
        WerraddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This address will be approximate since multiple write transactions might be in flight at any given time. However, it should be accurate when debugging/single-stepping"]
    #[inline(always)]
    #[must_use]
    pub fn werraddr(&mut self) -> WerraddrW<WriteerraddrSpec> {
        WerraddrW::new(self, 0)
    }
}
#[doc = "DAXI Write Error Address\n\nYou can [`read`](crate::Reg::read) this register and get [`writeerraddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writeerraddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteerraddrSpec;
impl crate::RegisterSpec for WriteerraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writeerraddr::R`](R) reader structure"]
impl crate::Readable for WriteerraddrSpec {}
#[doc = "`write(|w| ..)` method takes [`writeerraddr::W`](W) writer structure"]
impl crate::Writable for WriteerraddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITEERRADDR to value 0"]
impl crate::Resettable for WriteerraddrSpec {
    const RESET_VALUE: u32 = 0;
}
