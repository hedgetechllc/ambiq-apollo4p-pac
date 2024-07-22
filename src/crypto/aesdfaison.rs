#[doc = "Register `AESDFAISON` reader"]
pub type R = crate::R<AesdfaisonSpec>;
#[doc = "Register `AESDFAISON` writer"]
pub type W = crate::W<AesdfaisonSpec>;
#[doc = "Field `AESDFAISON` reader - writing to this register turns the DFA counter-measures on. this register exists only if DFA countermeasures are supported"]
pub type AesdfaisonR = crate::BitReader;
#[doc = "Field `AESDFAISON` writer - writing to this register turns the DFA counter-measures on. this register exists only if DFA countermeasures are supported"]
pub type AesdfaisonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - writing to this register turns the DFA counter-measures on. this register exists only if DFA countermeasures are supported"]
    #[inline(always)]
    pub fn aesdfaison(&self) -> AesdfaisonR {
        AesdfaisonR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - writing to this register turns the DFA counter-measures on. this register exists only if DFA countermeasures are supported"]
    #[inline(always)]
    #[must_use]
    pub fn aesdfaison(&mut self) -> AesdfaisonW<AesdfaisonSpec> {
        AesdfaisonW::new(self, 0)
    }
}
#[doc = "This register disable_enable the AES dfa. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`aesdfaison::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesdfaison::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesdfaisonSpec;
impl crate::RegisterSpec for AesdfaisonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aesdfaison::R`](R) reader structure"]
impl crate::Readable for AesdfaisonSpec {}
#[doc = "`write(|w| ..)` method takes [`aesdfaison::W`](W) writer structure"]
impl crate::Writable for AesdfaisonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESDFAISON to value 0"]
impl crate::Resettable for AesdfaisonSpec {
    const RESET_VALUE: u32 = 0;
}
