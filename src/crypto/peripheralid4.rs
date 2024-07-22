#[doc = "Register `PERIPHERALID4` reader"]
pub type R = crate::R<Peripheralid4Spec>;
#[doc = "Register `PERIPHERALID4` writer"]
pub type W = crate::W<Peripheralid4Spec>;
#[doc = "Field `DES2JEP106` reader - for ARM products."]
pub type Des2jep106R = crate::FieldReader;
#[doc = "Field `DES2JEP106` writer - for ARM products."]
pub type Des2jep106W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - for ARM products."]
    #[inline(always)]
    pub fn des2jep106(&self) -> Des2jep106R {
        Des2jep106R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - for ARM products."]
    #[inline(always)]
    #[must_use]
    pub fn des2jep106(&mut self) -> Des2jep106W<Peripheralid4Spec> {
        Des2jep106W::new(self, 0)
    }
}
#[doc = "Peripheral ID 4 (PID4).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Peripheralid4Spec;
impl crate::RegisterSpec for Peripheralid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peripheralid4::R`](R) reader structure"]
impl crate::Readable for Peripheralid4Spec {}
#[doc = "`write(|w| ..)` method takes [`peripheralid4::W`](W) writer structure"]
impl crate::Writable for Peripheralid4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIPHERALID4 to value 0x04"]
impl crate::Resettable for Peripheralid4Spec {
    const RESET_VALUE: u32 = 0x04;
}
