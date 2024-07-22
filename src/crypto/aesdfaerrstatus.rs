#[doc = "Register `AESDFAERRSTATUS` reader"]
pub type R = crate::R<AesdfaerrstatusSpec>;
#[doc = "Register `AESDFAERRSTATUS` writer"]
pub type W = crate::W<AesdfaerrstatusSpec>;
#[doc = "Field `AESDFAERRSTATUS` reader - after a DFA violation this register is set and the AES block is disabled) until the next reset. this register only exists if DFA countermeasures is are supported"]
pub type AesdfaerrstatusR = crate::BitReader;
#[doc = "Field `AESDFAERRSTATUS` writer - after a DFA violation this register is set and the AES block is disabled) until the next reset. this register only exists if DFA countermeasures is are supported"]
pub type AesdfaerrstatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - after a DFA violation this register is set and the AES block is disabled) until the next reset. this register only exists if DFA countermeasures is are supported"]
    #[inline(always)]
    pub fn aesdfaerrstatus(&self) -> AesdfaerrstatusR {
        AesdfaerrstatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - after a DFA violation this register is set and the AES block is disabled) until the next reset. this register only exists if DFA countermeasures is are supported"]
    #[inline(always)]
    #[must_use]
    pub fn aesdfaerrstatus(&mut self) -> AesdfaerrstatusW<AesdfaerrstatusSpec> {
        AesdfaerrstatusW::new(self, 0)
    }
}
#[doc = "dfa error status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesdfaerrstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesdfaerrstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesdfaerrstatusSpec;
impl crate::RegisterSpec for AesdfaerrstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesdfaerrstatus::R`](R) reader structure"]
impl crate::Readable for AesdfaerrstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`aesdfaerrstatus::W`](W) writer structure"]
impl crate::Writable for AesdfaerrstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESDFAERRSTATUS to value 0"]
impl crate::Resettable for AesdfaerrstatusSpec {
    const RESET_VALUE: u32 = 0;
}
