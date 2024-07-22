#[doc = "Register `VRDEMOTIONTHR` reader"]
pub type R = crate::R<VrdemotionthrSpec>;
#[doc = "Register `VRDEMOTIONTHR` writer"]
pub type W = crate::W<VrdemotionthrSpec>;
#[doc = "Field `VRDEMOTIONTHR` reader - VR Demotion Threshold"]
pub type VrdemotionthrR = crate::FieldReader<u32>;
#[doc = "Field `VRDEMOTIONTHR` writer - VR Demotion Threshold"]
pub type VrdemotionthrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VR Demotion Threshold"]
    #[inline(always)]
    pub fn vrdemotionthr(&self) -> VrdemotionthrR {
        VrdemotionthrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VR Demotion Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn vrdemotionthr(&mut self) -> VrdemotionthrW<VrdemotionthrSpec> {
        VrdemotionthrW::new(self, 0)
    }
}
#[doc = "Weights specified in PWRWEIGHT* registers are applied to each of the masters active requests. The aggregate of all the masters is compared against the this threshold value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`vrdemotionthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrdemotionthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrdemotionthrSpec;
impl crate::RegisterSpec for VrdemotionthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrdemotionthr::R`](R) reader structure"]
impl crate::Readable for VrdemotionthrSpec {}
#[doc = "`write(|w| ..)` method takes [`vrdemotionthr::W`](W) writer structure"]
impl crate::Writable for VrdemotionthrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VRDEMOTIONTHR to value 0"]
impl crate::Resettable for VrdemotionthrSpec {
    const RESET_VALUE: u32 = 0;
}
