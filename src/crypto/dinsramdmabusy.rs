#[doc = "Register `DINSRAMDMABUSY` reader"]
pub type R = crate::R<DinsramdmabusySpec>;
#[doc = "Register `DINSRAMDMABUSY` writer"]
pub type W = crate::W<DinsramdmabusySpec>;
#[doc = "DIN SRAM DMA busy:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "1: busy"]
    Busy = 1,
    #[doc = "0: not busy"]
    NotBusy = 0,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - DIN SRAM DMA busy:"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            true => Busy::Busy,
            false => Busy::NotBusy,
        }
    }
    #[doc = "busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
    #[doc = "not busy"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == Busy::NotBusy
    }
}
#[doc = "Field `BUSY` writer - DIN SRAM DMA busy:"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG, Busy>;
impl<'a, REG> BusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Busy::Busy)
    }
    #[doc = "not busy"]
    #[inline(always)]
    pub fn not_busy(self) -> &'a mut crate::W<REG> {
        self.variant(Busy::NotBusy)
    }
}
impl R {
    #[doc = "Bit 0 - DIN SRAM DMA busy:"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIN SRAM DMA busy:"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<DinsramdmabusySpec> {
        BusyW::new(self, 0)
    }
}
#[doc = "This register holds the status of the SRAM DMA DIN.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinsramdmabusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinsramdmabusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinsramdmabusySpec;
impl crate::RegisterSpec for DinsramdmabusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinsramdmabusy::R`](R) reader structure"]
impl crate::Readable for DinsramdmabusySpec {}
#[doc = "`write(|w| ..)` method takes [`dinsramdmabusy::W`](W) writer structure"]
impl crate::Writable for DinsramdmabusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINSRAMDMABUSY to value 0"]
impl crate::Resettable for DinsramdmabusySpec {
    const RESET_VALUE: u32 = 0;
}
