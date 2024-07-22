#[doc = "Register `FIFO` reader"]
pub type R = crate::R<FifoSpec>;
#[doc = "Register `FIFO` writer"]
pub type W = crate::W<FifoSpec>;
#[doc = "Field `METALO` reader - Meta data about this sample which represents the lower 4 bits of the PGA gain code"]
pub type MetaloR = crate::FieldReader;
#[doc = "Field `METALO` writer - Meta data about this sample which represents the lower 4 bits of the PGA gain code"]
pub type MetaloW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LGDATA` reader - Low-gain PGA sample data"]
pub type LgdataR = crate::FieldReader<u16>;
#[doc = "Field `LGDATA` writer - Low-gain PGA sample data"]
pub type LgdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `METAHI` reader - Meta data about this sample which represents the upper 3 bits of the PGA gain code"]
pub type MetahiR = crate::FieldReader;
#[doc = "Field `METAHI` writer - Meta data about this sample which represents the upper 3 bits of the PGA gain code"]
pub type MetahiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MIC` reader - Which audio channel this data is from encoded as int(slot number/2). In other words, this is 1 if data is from slots 2 or 3, or 0 if from slots 0 or 1."]
pub type MicR = crate::BitReader;
#[doc = "Field `MIC` writer - Which audio channel this data is from encoded as int(slot number/2). In other words, this is 1 if data is from slots 2 or 3, or 0 if from slots 0 or 1."]
pub type MicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HGDATA` reader - High-gain PGA sample data"]
pub type HgdataR = crate::FieldReader<u16>;
#[doc = "Field `HGDATA` writer - High-gain PGA sample data"]
pub type HgdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - Meta data about this sample which represents the lower 4 bits of the PGA gain code"]
    #[inline(always)]
    pub fn metalo(&self) -> MetaloR {
        MetaloR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Low-gain PGA sample data"]
    #[inline(always)]
    pub fn lgdata(&self) -> LgdataR {
        LgdataR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Meta data about this sample which represents the upper 3 bits of the PGA gain code"]
    #[inline(always)]
    pub fn metahi(&self) -> MetahiR {
        MetahiR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Which audio channel this data is from encoded as int(slot number/2). In other words, this is 1 if data is from slots 2 or 3, or 0 if from slots 0 or 1."]
    #[inline(always)]
    pub fn mic(&self) -> MicR {
        MicR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - High-gain PGA sample data"]
    #[inline(always)]
    pub fn hgdata(&self) -> HgdataR {
        HgdataR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Meta data about this sample which represents the lower 4 bits of the PGA gain code"]
    #[inline(always)]
    #[must_use]
    pub fn metalo(&mut self) -> MetaloW<FifoSpec> {
        MetaloW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Low-gain PGA sample data"]
    #[inline(always)]
    #[must_use]
    pub fn lgdata(&mut self) -> LgdataW<FifoSpec> {
        LgdataW::new(self, 4)
    }
    #[doc = "Bits 16:18 - Meta data about this sample which represents the upper 3 bits of the PGA gain code"]
    #[inline(always)]
    #[must_use]
    pub fn metahi(&mut self) -> MetahiW<FifoSpec> {
        MetahiW::new(self, 16)
    }
    #[doc = "Bit 19 - Which audio channel this data is from encoded as int(slot number/2). In other words, this is 1 if data is from slots 2 or 3, or 0 if from slots 0 or 1."]
    #[inline(always)]
    #[must_use]
    pub fn mic(&mut self) -> MicW<FifoSpec> {
        MicW::new(self, 19)
    }
    #[doc = "Bits 20:31 - High-gain PGA sample data"]
    #[inline(always)]
    #[must_use]
    pub fn hgdata(&mut self) -> HgdataW<FifoSpec> {
        HgdataW::new(self, 20)
    }
}
#[doc = "The AUDADC FIFO Register contains up to 2 samples for a single channel (high and low gain PGA samples), each sample up to 12-bits. It also contains meta data in the form of which audio channel the sample(s) are from along with the PGA gain code for that sample pair. When no data is present, FIFO entry reads back as all 1s (0xFFFFFFFF).\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoSpec;
impl crate::RegisterSpec for FifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO to value 0"]
impl crate::Resettable for FifoSpec {
    const RESET_VALUE: u32 = 0;
}
