#[doc = "Register `AESCMACINIT` reader"]
pub type R = crate::R<AescmacinitSpec>;
#[doc = "Register `AESCMACINIT` writer"]
pub type W = crate::W<AescmacinitSpec>;
#[doc = "Field `AESCMACINIT` reader - Writing to this address starts the generating of K1 and K2 for AES CMAC operations"]
pub type AescmacinitR = crate::BitReader;
#[doc = "Field `AESCMACINIT` writer - Writing to this address starts the generating of K1 and K2 for AES CMAC operations"]
pub type AescmacinitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing to this address starts the generating of K1 and K2 for AES CMAC operations"]
    #[inline(always)]
    pub fn aescmacinit(&self) -> AescmacinitR {
        AescmacinitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing to this address starts the generating of K1 and K2 for AES CMAC operations"]
    #[inline(always)]
    #[must_use]
    pub fn aescmacinit(&mut self) -> AescmacinitW<AescmacinitSpec> {
        AescmacinitW::new(self, 0)
    }
}
#[doc = "Writing to this address triggers the AES engine generating of K1 and K2 for AES CMAC operations. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aescmacinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aescmacinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AescmacinitSpec;
impl crate::RegisterSpec for AescmacinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aescmacinit::R`](R) reader structure"]
impl crate::Readable for AescmacinitSpec {}
#[doc = "`write(|w| ..)` method takes [`aescmacinit::W`](W) writer structure"]
impl crate::Writable for AescmacinitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESCMACINIT to value 0"]
impl crate::Resettable for AescmacinitSpec {
    const RESET_VALUE: u32 = 0;
}
