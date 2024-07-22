#[doc = "Register `NVMISIDLE` reader"]
pub type R = crate::R<NvmisidleSpec>;
#[doc = "Register `NVMISIDLE` writer"]
pub type W = crate::W<NvmisidleSpec>;
#[doc = "Field `NVMISIDLEREG` reader - Indicates whether the NVM manager finishes its operation, calculates the LCS, reads the HW keys, compares the number of zeros and clears the keys"]
pub type NvmisidleregR = crate::BitReader;
#[doc = "Field `NVMISIDLEREG` writer - Indicates whether the NVM manager finishes its operation, calculates the LCS, reads the HW keys, compares the number of zeros and clears the keys"]
pub type NvmisidleregW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates whether the NVM manager finishes its operation, calculates the LCS, reads the HW keys, compares the number of zeros and clears the keys"]
    #[inline(always)]
    pub fn nvmisidlereg(&self) -> NvmisidleregR {
        NvmisidleregR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether the NVM manager finishes its operation, calculates the LCS, reads the HW keys, compares the number of zeros and clears the keys"]
    #[inline(always)]
    #[must_use]
    pub fn nvmisidlereg(&mut self) -> NvmisidleregW<NvmisidleSpec> {
        NvmisidleregW::new(self, 0)
    }
}
#[doc = "Indicates that the LCS register holds a valid value.Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`nvmisidle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvmisidle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvmisidleSpec;
impl crate::RegisterSpec for NvmisidleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvmisidle::R`](R) reader structure"]
impl crate::Readable for NvmisidleSpec {}
#[doc = "`write(|w| ..)` method takes [`nvmisidle::W`](W) writer structure"]
impl crate::Writable for NvmisidleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVMISIDLE to value 0x01"]
impl crate::Resettable for NvmisidleSpec {
    const RESET_VALUE: u32 = 0x01;
}
