#[doc = "Register `HOSTPOWERDOWN` reader"]
pub type R = crate::R<HostpowerdownSpec>;
#[doc = "Register `HOSTPOWERDOWN` writer"]
pub type W = crate::W<HostpowerdownSpec>;
#[doc = "Field `HOSTPOWERDOWN` reader - Power down enable register."]
pub type HostpowerdownR = crate::BitReader;
#[doc = "Field `HOSTPOWERDOWN` writer - Power down enable register."]
pub type HostpowerdownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power down enable register."]
    #[inline(always)]
    pub fn hostpowerdown(&self) -> HostpowerdownR {
        HostpowerdownR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down enable register."]
    #[inline(always)]
    #[must_use]
    pub fn hostpowerdown(&mut self) -> HostpowerdownW<HostpowerdownSpec> {
        HostpowerdownW::new(self, 0)
    }
}
#[doc = "This register start the power-down sequence. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostpowerdown::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostpowerdown::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostpowerdownSpec;
impl crate::RegisterSpec for HostpowerdownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostpowerdown::R`](R) reader structure"]
impl crate::Readable for HostpowerdownSpec {}
#[doc = "`write(|w| ..)` method takes [`hostpowerdown::W`](W) writer structure"]
impl crate::Writable for HostpowerdownSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTPOWERDOWN to value 0"]
impl crate::Resettable for HostpowerdownSpec {
    const RESET_VALUE: u32 = 0;
}
