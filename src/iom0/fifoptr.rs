#[doc = "Register `FIFOPTR` reader"]
pub type R = crate::R<FifoptrSpec>;
#[doc = "Register `FIFOPTR` writer"]
pub type W = crate::W<FifoptrSpec>;
#[doc = "Field `FIFO0SIZ` reader - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
pub type Fifo0sizR = crate::FieldReader;
#[doc = "Field `FIFO0SIZ` writer - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
pub type Fifo0sizW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIFO0REM` reader - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
pub type Fifo0remR = crate::FieldReader;
#[doc = "Field `FIFO0REM` writer - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
pub type Fifo0remW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIFO1SIZ` reader - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
pub type Fifo1sizR = crate::FieldReader;
#[doc = "Field `FIFO1SIZ` writer - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
pub type Fifo1sizW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIFO1REM` reader - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
pub type Fifo1remR = crate::FieldReader;
#[doc = "Field `FIFO1REM` writer - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
pub type Fifo1remW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0siz(&self) -> Fifo0sizR {
        Fifo0sizR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    pub fn fifo0rem(&self) -> Fifo0remR {
        Fifo0remR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1siz(&self) -> Fifo1sizR {
        Fifo1sizR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    pub fn fifo1rem(&self) -> Fifo1remR {
        Fifo1remR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    #[must_use]
    pub fn fifo0siz(&mut self) -> Fifo0sizW<FifoptrSpec> {
        Fifo0sizW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)"]
    #[inline(always)]
    #[must_use]
    pub fn fifo0rem(&mut self) -> Fifo0remW<FifoptrSpec> {
        Fifo0remW::new(self, 8)
    }
    #[doc = "Bits 16:23 - The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    #[must_use]
    pub fn fifo1siz(&mut self) -> Fifo1sizW<FifoptrSpec> {
        Fifo1sizW::new(self, 16)
    }
    #[doc = "Bits 24:31 - The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)"]
    #[inline(always)]
    #[must_use]
    pub fn fifo1rem(&mut self) -> Fifo1remW<FifoptrSpec> {
        Fifo1remW::new(self, 24)
    }
}
#[doc = "Provides the current valid byte count of data within the FIFO as seen from the internal state machines. FIFO0 is dedicated to outgoing transactions and FIFO1 is dedicated to incoming transactions. All counts are specified in units of bytes.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoptrSpec;
impl crate::RegisterSpec for FifoptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoptr::R`](R) reader structure"]
impl crate::Readable for FifoptrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoptr::W`](W) writer structure"]
impl crate::Writable for FifoptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOPTR to value 0"]
impl crate::Resettable for FifoptrSpec {
    const RESET_VALUE: u32 = 0;
}
