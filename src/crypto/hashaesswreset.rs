#[doc = "Register `HASHAESSWRESET` reader"]
pub type R = crate::R<HashaesswresetSpec>;
#[doc = "Register `HASHAESSWRESET` writer"]
pub type W = crate::W<HashaesswresetSpec>;
#[doc = "Field `HASHAESSWRESET` reader - Hash receive reset internally."]
pub type HashaesswresetR = crate::BitReader;
#[doc = "Field `HASHAESSWRESET` writer - Hash receive reset internally."]
pub type HashaesswresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hash receive reset internally."]
    #[inline(always)]
    pub fn hashaesswreset(&self) -> HashaesswresetR {
        HashaesswresetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hash receive reset internally."]
    #[inline(always)]
    #[must_use]
    pub fn hashaesswreset(&mut self) -> HashaesswresetW<HashaesswresetSpec> {
        HashaesswresetW::new(self, 0)
    }
}
#[doc = "Software reset of the AES.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashaesswreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashaesswreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashaesswresetSpec;
impl crate::RegisterSpec for HashaesswresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashaesswreset::R`](R) reader structure"]
impl crate::Readable for HashaesswresetSpec {}
#[doc = "`write(|w| ..)` method takes [`hashaesswreset::W`](W) writer structure"]
impl crate::Writable for HashaesswresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHAESSWRESET to value 0"]
impl crate::Resettable for HashaesswresetSpec {
    const RESET_VALUE: u32 = 0;
}
