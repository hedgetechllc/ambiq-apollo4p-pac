#[doc = "Register `AOCCSECDEBUGRESET` reader"]
pub type R = crate::R<AoccsecdebugresetSpec>;
#[doc = "Register `AOCCSECDEBUGRESET` writer"]
pub type W = crate::W<AoccsecdebugresetSpec>;
#[doc = "Field `AOCCSECDEBUGRESET` reader - For resets Cerberus, and prevents loading the HW keys after that reset"]
pub type AoccsecdebugresetR = crate::BitReader;
#[doc = "Field `AOCCSECDEBUGRESET` writer - For resets Cerberus, and prevents loading the HW keys after that reset"]
pub type AoccsecdebugresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For resets Cerberus, and prevents loading the HW keys after that reset"]
    #[inline(always)]
    pub fn aoccsecdebugreset(&self) -> AoccsecdebugresetR {
        AoccsecdebugresetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For resets Cerberus, and prevents loading the HW keys after that reset"]
    #[inline(always)]
    #[must_use]
    pub fn aoccsecdebugreset(&mut self) -> AoccsecdebugresetW<AoccsecdebugresetSpec> {
        AoccsecdebugresetW::new(self, 0)
    }
}
#[doc = "The reset-upon-debug indication\n\nYou can [`read`](crate::Reg::read) this register and get [`aoccsecdebugreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aoccsecdebugreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AoccsecdebugresetSpec;
impl crate::RegisterSpec for AoccsecdebugresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoccsecdebugreset::R`](R) reader structure"]
impl crate::Readable for AoccsecdebugresetSpec {}
#[doc = "`write(|w| ..)` method takes [`aoccsecdebugreset::W`](W) writer structure"]
impl crate::Writable for AoccsecdebugresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AOCCSECDEBUGRESET to value 0"]
impl crate::Resettable for AoccsecdebugresetSpec {
    const RESET_VALUE: u32 = 0;
}
