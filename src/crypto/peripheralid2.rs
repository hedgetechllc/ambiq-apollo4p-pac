#[doc = "Register `PERIPHERALID2` reader"]
pub type R = crate::R<Peripheralid2Spec>;
#[doc = "Register `PERIPHERALID2` writer"]
pub type W = crate::W<Peripheralid2Spec>;
#[doc = "Field `DES1JEP106` reader - for ARM products."]
pub type Des1jep106R = crate::FieldReader;
#[doc = "Field `DES1JEP106` writer - for ARM products."]
pub type Des1jep106W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JEDEC` reader - constant 0x1. Indicates that a JEDEC assigned value is used."]
pub type JedecR = crate::BitReader;
#[doc = "Field `JEDEC` writer - constant 0x1. Indicates that a JEDEC assigned value is used."]
pub type JedecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REVISION` reader - starts at zero and increments for every new IP release."]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - starts at zero and increments for every new IP release."]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - for ARM products."]
    #[inline(always)]
    pub fn des1jep106(&self) -> Des1jep106R {
        Des1jep106R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - constant 0x1. Indicates that a JEDEC assigned value is used."]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - starts at zero and increments for every new IP release."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - for ARM products."]
    #[inline(always)]
    #[must_use]
    pub fn des1jep106(&mut self) -> Des1jep106W<Peripheralid2Spec> {
        Des1jep106W::new(self, 0)
    }
    #[doc = "Bit 3 - constant 0x1. Indicates that a JEDEC assigned value is used."]
    #[inline(always)]
    #[must_use]
    pub fn jedec(&mut self) -> JedecW<Peripheralid2Spec> {
        JedecW::new(self, 3)
    }
    #[doc = "Bits 4:7 - starts at zero and increments for every new IP release."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<Peripheralid2Spec> {
        RevisionW::new(self, 4)
    }
}
#[doc = "Peripheral ID 2 (PID2).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Peripheralid2Spec;
impl crate::RegisterSpec for Peripheralid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peripheralid2::R`](R) reader structure"]
impl crate::Readable for Peripheralid2Spec {}
#[doc = "`write(|w| ..)` method takes [`peripheralid2::W`](W) writer structure"]
impl crate::Writable for Peripheralid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIPHERALID2 to value 0x2b"]
impl crate::Resettable for Peripheralid2Spec {
    const RESET_VALUE: u32 = 0x2b;
}
