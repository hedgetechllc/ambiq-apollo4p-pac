#[doc = "Register `LOADINITSTATE` reader"]
pub type R = crate::R<LoadinitstateSpec>;
#[doc = "Register `LOADINITSTATE` writer"]
pub type W = crate::W<LoadinitstateSpec>;
#[doc = "Field `LOAD` reader - Load data to initial state registers. digest_iv for hash_aes_mac. When done loading data this bit should be reset"]
pub type LoadR = crate::BitReader;
#[doc = "Field `LOAD` writer - Load data to initial state registers. digest_iv for hash_aes_mac. When done loading data this bit should be reset"]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Load data to initial state registers. digest_iv for hash_aes_mac. When done loading data this bit should be reset"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Load data to initial state registers. digest_iv for hash_aes_mac. When done loading data this bit should be reset"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LoadW<LoadinitstateSpec> {
        LoadW::new(self, 0)
    }
}
#[doc = "Indication to HASH that the following data is to be loaded into initial value registers in HASH(H0:H15) or IV to AES MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`loadinitstate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loadinitstate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadinitstateSpec;
impl crate::RegisterSpec for LoadinitstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loadinitstate::R`](R) reader structure"]
impl crate::Readable for LoadinitstateSpec {}
#[doc = "`write(|w| ..)` method takes [`loadinitstate::W`](W) writer structure"]
impl crate::Writable for LoadinitstateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOADINITSTATE to value 0"]
impl crate::Resettable for LoadinitstateSpec {
    const RESET_VALUE: u32 = 0;
}
