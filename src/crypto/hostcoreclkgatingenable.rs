#[doc = "Register `HOSTCORECLKGATINGENABLE` reader"]
pub type R = crate::R<HostcoreclkgatingenableSpec>;
#[doc = "Register `HOSTCORECLKGATINGENABLE` writer"]
pub type W = crate::W<HostcoreclkgatingenableSpec>;
#[doc = "Field `HOSTCORECLKGATINGENABLE` reader - Enable the core clk gating,"]
pub type HostcoreclkgatingenableR = crate::BitReader;
#[doc = "Field `HOSTCORECLKGATINGENABLE` writer - Enable the core clk gating,"]
pub type HostcoreclkgatingenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the core clk gating,"]
    #[inline(always)]
    pub fn hostcoreclkgatingenable(&self) -> HostcoreclkgatingenableR {
        HostcoreclkgatingenableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the core clk gating,"]
    #[inline(always)]
    #[must_use]
    pub fn hostcoreclkgatingenable(
        &mut self,
    ) -> HostcoreclkgatingenableW<HostcoreclkgatingenableSpec> {
        HostcoreclkgatingenableW::new(self, 0)
    }
}
#[doc = "This register enables the core clk gating by masking_enabling the cc_idle_state output signal.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostcoreclkgatingenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostcoreclkgatingenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostcoreclkgatingenableSpec;
impl crate::RegisterSpec for HostcoreclkgatingenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostcoreclkgatingenable::R`](R) reader structure"]
impl crate::Readable for HostcoreclkgatingenableSpec {}
#[doc = "`write(|w| ..)` method takes [`hostcoreclkgatingenable::W`](W) writer structure"]
impl crate::Writable for HostcoreclkgatingenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTCORECLKGATINGENABLE to value 0"]
impl crate::Resettable for HostcoreclkgatingenableSpec {
    const RESET_VALUE: u32 = 0;
}
