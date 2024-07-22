#[doc = "Register `SYSCLEAR` reader"]
pub type R = crate::R<SysclearSpec>;
#[doc = "Register `SYSCLEAR` writer"]
pub type W = crate::W<SysclearSpec>;
#[doc = "Field `RESETGPU` reader - On write, resets the GPU (CHECK address!)."]
pub type ResetgpuR = crate::FieldReader<u32>;
#[doc = "Field `RESETGPU` writer - On write, resets the GPU (CHECK address!)."]
pub type ResetgpuW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - On write, resets the GPU (CHECK address!)."]
    #[inline(always)]
    pub fn resetgpu(&self) -> ResetgpuR {
        ResetgpuR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - On write, resets the GPU (CHECK address!)."]
    #[inline(always)]
    #[must_use]
    pub fn resetgpu(&mut self) -> ResetgpuW<SysclearSpec> {
        ResetgpuW::new(self, 0)
    }
}
#[doc = "On write, resets the GPU (CHECK address!).\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysclearSpec;
impl crate::RegisterSpec for SysclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclear::R`](R) reader structure"]
impl crate::Readable for SysclearSpec {}
#[doc = "`write(|w| ..)` method takes [`sysclear::W`](W) writer structure"]
impl crate::Writable for SysclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCLEAR to value 0"]
impl crate::Resettable for SysclearSpec {
    const RESET_VALUE: u32 = 0;
}
