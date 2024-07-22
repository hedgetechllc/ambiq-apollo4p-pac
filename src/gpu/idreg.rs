#[doc = "Register `IDREG` reader"]
pub type R = crate::R<IdregSpec>;
#[doc = "Register `IDREG` writer"]
pub type W = crate::W<IdregSpec>;
#[doc = "Field `GPUID` reader - Fixed value for GPU ID"]
pub type GpuidR = crate::FieldReader<u32>;
#[doc = "Field `GPUID` writer - Fixed value for GPU ID"]
pub type GpuidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fixed value for GPU ID"]
    #[inline(always)]
    pub fn gpuid(&self) -> GpuidR {
        GpuidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fixed value for GPU ID"]
    #[inline(always)]
    #[must_use]
    pub fn gpuid(&mut self) -> GpuidW<IdregSpec> {
        GpuidW::new(self, 0)
    }
}
#[doc = "Fixed value\n\nYou can [`read`](crate::Reg::read) this register and get [`idreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdregSpec;
impl crate::RegisterSpec for IdregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idreg::R`](R) reader structure"]
impl crate::Readable for IdregSpec {}
#[doc = "`write(|w| ..)` method takes [`idreg::W`](W) writer structure"]
impl crate::Writable for IdregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDREG to value 0x8636_2000"]
impl crate::Resettable for IdregSpec {
    const RESET_VALUE: u32 = 0x8636_2000;
}
