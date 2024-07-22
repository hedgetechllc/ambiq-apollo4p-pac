#[doc = "Register `FIFOPR` reader"]
pub type R = crate::R<FifoprSpec>;
#[doc = "Register `FIFOPR` writer"]
pub type W = crate::W<FifoprSpec>;
#[doc = "Field `METALOPR` reader - Meta data about this sample which represents the lower 4 bits of the PGA gain code"]
pub type MetaloprR = crate::FieldReader;
#[doc = "Field `METALOPR` writer - Meta data about this sample which represents the lower 4 bits of the PGA gain code"]
pub type MetaloprW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LGDATAPR` reader - Low-gain PGA sample data"]
pub type LgdataprR = crate::FieldReader<u16>;
#[doc = "Field `LGDATAPR` writer - Low-gain PGA sample data"]
pub type LgdataprW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `METAHIPR` reader - Meta data about this sample which represents the upper 3 bits of the PGA gain code"]
pub type MetahiprR = crate::FieldReader;
#[doc = "Field `METAHIPR` writer - Meta data about this sample which represents the upper 3 bits of the PGA gain code"]
pub type MetahiprW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MICPR` reader - Which audio channel this data is from encoded as int(slot number/2). In other words, this is 1 if data is from slots 2 or 3, or 0 if from slots 0 or 1."]
pub type MicprR = crate::BitReader;
#[doc = "Field `MICPR` writer - Which audio channel this data is from encoded as int(slot number/2). In other words, this is 1 if data is from slots 2 or 3, or 0 if from slots 0 or 1."]
pub type MicprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HGDATAPR` reader - High-gain PGA sample data"]
pub type HgdataprR = crate::FieldReader<u16>;
#[doc = "Field `HGDATAPR` writer - High-gain PGA sample data"]
pub type HgdataprW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - Meta data about this sample which represents the lower 4 bits of the PGA gain code"]
    #[inline(always)]
    pub fn metalopr(&self) -> MetaloprR {
        MetaloprR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Low-gain PGA sample data"]
    #[inline(always)]
    pub fn lgdatapr(&self) -> LgdataprR {
        LgdataprR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Meta data about this sample which represents the upper 3 bits of the PGA gain code"]
    #[inline(always)]
    pub fn metahipr(&self) -> MetahiprR {
        MetahiprR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Which audio channel this data is from encoded as int(slot number/2). In other words, this is 1 if data is from slots 2 or 3, or 0 if from slots 0 or 1."]
    #[inline(always)]
    pub fn micpr(&self) -> MicprR {
        MicprR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - High-gain PGA sample data"]
    #[inline(always)]
    pub fn hgdatapr(&self) -> HgdataprR {
        HgdataprR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Meta data about this sample which represents the lower 4 bits of the PGA gain code"]
    #[inline(always)]
    #[must_use]
    pub fn metalopr(&mut self) -> MetaloprW<FifoprSpec> {
        MetaloprW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Low-gain PGA sample data"]
    #[inline(always)]
    #[must_use]
    pub fn lgdatapr(&mut self) -> LgdataprW<FifoprSpec> {
        LgdataprW::new(self, 4)
    }
    #[doc = "Bits 16:18 - Meta data about this sample which represents the upper 3 bits of the PGA gain code"]
    #[inline(always)]
    #[must_use]
    pub fn metahipr(&mut self) -> MetahiprW<FifoprSpec> {
        MetahiprW::new(self, 16)
    }
    #[doc = "Bit 19 - Which audio channel this data is from encoded as int(slot number/2). In other words, this is 1 if data is from slots 2 or 3, or 0 if from slots 0 or 1."]
    #[inline(always)]
    #[must_use]
    pub fn micpr(&mut self) -> MicprW<FifoprSpec> {
        MicprW::new(self, 19)
    }
    #[doc = "Bits 20:31 - High-gain PGA sample data"]
    #[inline(always)]
    #[must_use]
    pub fn hgdatapr(&mut self) -> HgdataprW<FifoprSpec> {
        HgdataprW::new(self, 20)
    }
}
#[doc = "This is a pop-on-read mirrored copy of the ADCFIFO register with the only difference being that reading this register will result in a simultaneous FIFO POP which is also achieved by writing to the ADCFIFO Register. Note: The DFIFORDEN bit must be set in the CFG register for the the destructive read to be enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifopr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifopr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoprSpec;
impl crate::RegisterSpec for FifoprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifopr::R`](R) reader structure"]
impl crate::Readable for FifoprSpec {}
#[doc = "`write(|w| ..)` method takes [`fifopr::W`](W) writer structure"]
impl crate::Writable for FifoprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOPR to value 0"]
impl crate::Resettable for FifoprSpec {
    const RESET_VALUE: u32 = 0;
}
