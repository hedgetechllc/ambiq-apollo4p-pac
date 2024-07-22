#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `THR` reader - This is the FIFO threshold interrupt."]
pub type ThrR = crate::BitReader;
#[doc = "Field `THR` writer - This is the FIFO threshold interrupt."]
pub type ThrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - This is the FIFO overflow interrupt."]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - This is the FIFO overflow interrupt."]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDFL` reader - This is the FIFO underflow interrupt."]
pub type UndflR = crate::BitReader;
#[doc = "Field `UNDFL` writer - This is the FIFO underflow interrupt."]
pub type UndflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMP` reader - DMA completed a transfer"]
pub type DcmpR = crate::BitReader;
#[doc = "Field `DCMP` writer - DMA completed a transfer"]
pub type DcmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DERR` reader - DMA Error receieved"]
pub type DerrR = crate::BitReader;
#[doc = "Field `DERR` writer - DMA Error receieved"]
pub type DerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is the FIFO threshold interrupt."]
    #[inline(always)]
    pub fn thr(&self) -> ThrR {
        ThrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the FIFO overflow interrupt."]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the FIFO underflow interrupt."]
    #[inline(always)]
    pub fn undfl(&self) -> UndflR {
        UndflR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA completed a transfer"]
    #[inline(always)]
    pub fn dcmp(&self) -> DcmpR {
        DcmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Error receieved"]
    #[inline(always)]
    pub fn derr(&self) -> DerrR {
        DerrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the FIFO threshold interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn thr(&mut self) -> ThrW<IntenSpec> {
        ThrW::new(self, 0)
    }
    #[doc = "Bit 1 - This is the FIFO overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<IntenSpec> {
        OvfW::new(self, 1)
    }
    #[doc = "Bit 2 - This is the FIFO underflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn undfl(&mut self) -> UndflW<IntenSpec> {
        UndflW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA completed a transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dcmp(&mut self) -> DcmpW<IntenSpec> {
        DcmpW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Error receieved"]
    #[inline(always)]
    #[must_use]
    pub fn derr(&mut self) -> DerrW<IntenSpec> {
        DerrW::new(self, 4)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
