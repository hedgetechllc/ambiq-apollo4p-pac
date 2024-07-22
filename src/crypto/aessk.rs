#[doc = "Register `AESSK` reader"]
pub type R = crate::R<AesskSpec>;
#[doc = "Register `AESSK` writer"]
pub type W = crate::W<AesskSpec>;
#[doc = "Field `AESSK` reader - writing to this address causes sampling of the HW key to the AES_KEY0 register"]
pub type AesskR = crate::BitReader;
#[doc = "Field `AESSK` writer - writing to this address causes sampling of the HW key to the AES_KEY0 register"]
pub type AesskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - writing to this address causes sampling of the HW key to the AES_KEY0 register"]
    #[inline(always)]
    pub fn aessk(&self) -> AesskR {
        AesskR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - writing to this address causes sampling of the HW key to the AES_KEY0 register"]
    #[inline(always)]
    #[must_use]
    pub fn aessk(&mut self) -> AesskW<AesskSpec> {
        AesskW::new(self, 0)
    }
}
#[doc = "writing to this address causes sampling of the HW key to the AES_KEY0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`aessk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aessk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesskSpec;
impl crate::RegisterSpec for AesskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aessk::R`](R) reader structure"]
impl crate::Readable for AesskSpec {}
#[doc = "`write(|w| ..)` method takes [`aessk::W`](W) writer structure"]
impl crate::Writable for AesskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESSK to value 0"]
impl crate::Resettable for AesskSpec {
    const RESET_VALUE: u32 = 0;
}
