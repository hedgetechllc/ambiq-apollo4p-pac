#[doc = "Register `PERIPHERALID1` reader"]
pub type R = crate::R<Peripheralid1Spec>;
#[doc = "Register `PERIPHERALID1` writer"]
pub type W = crate::W<Peripheralid1Spec>;
#[doc = "Field `PART1` reader - Identification register part number, bits\\[11:8\\]"]
pub type Part1R = crate::FieldReader;
#[doc = "Field `PART1` writer - Identification register part number, bits\\[11:8\\]"]
pub type Part1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DES0JEP106` reader - for ARM products."]
pub type Des0jep106R = crate::FieldReader;
#[doc = "Field `DES0JEP106` writer - for ARM products."]
pub type Des0jep106W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Identification register part number, bits\\[11:8\\]"]
    #[inline(always)]
    pub fn part1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - for ARM products."]
    #[inline(always)]
    pub fn des0jep106(&self) -> Des0jep106R {
        Des0jep106R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Identification register part number, bits\\[11:8\\]"]
    #[inline(always)]
    #[must_use]
    pub fn part1(&mut self) -> Part1W<Peripheralid1Spec> {
        Part1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - for ARM products."]
    #[inline(always)]
    #[must_use]
    pub fn des0jep106(&mut self) -> Des0jep106W<Peripheralid1Spec> {
        Des0jep106W::new(self, 4)
    }
}
#[doc = "Peripheral ID 1 (PID1).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Peripheralid1Spec;
impl crate::RegisterSpec for Peripheralid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peripheralid1::R`](R) reader structure"]
impl crate::Readable for Peripheralid1Spec {}
#[doc = "`write(|w| ..)` method takes [`peripheralid1::W`](W) writer structure"]
impl crate::Writable for Peripheralid1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIPHERALID1 to value 0xb0"]
impl crate::Resettable for Peripheralid1Spec {
    const RESET_VALUE: u32 = 0xb0;
}
