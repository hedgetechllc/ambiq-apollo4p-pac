#[doc = "Register `DOUTMEMDMABUSY` reader"]
pub type R = crate::R<DoutmemdmabusySpec>;
#[doc = "Register `DOUTMEMDMABUSY` writer"]
pub type W = crate::W<DoutmemdmabusySpec>;
#[doc = "DOUT memory DMA busy:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutmemdmabusy {
    #[doc = "1: busy"]
    Busy = 1,
    #[doc = "0: not busy"]
    NotBusy = 0,
}
impl From<Doutmemdmabusy> for bool {
    #[inline(always)]
    fn from(variant: Doutmemdmabusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTMEMDMABUSY` reader - DOUT memory DMA busy:"]
pub type DoutmemdmabusyR = crate::BitReader<Doutmemdmabusy>;
impl DoutmemdmabusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doutmemdmabusy {
        match self.bits {
            true => Doutmemdmabusy::Busy,
            false => Doutmemdmabusy::NotBusy,
        }
    }
    #[doc = "busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Doutmemdmabusy::Busy
    }
    #[doc = "not busy"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == Doutmemdmabusy::NotBusy
    }
}
#[doc = "Field `DOUTMEMDMABUSY` writer - DOUT memory DMA busy:"]
pub type DoutmemdmabusyW<'a, REG> = crate::BitWriter<'a, REG, Doutmemdmabusy>;
impl<'a, REG> DoutmemdmabusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Doutmemdmabusy::Busy)
    }
    #[doc = "not busy"]
    #[inline(always)]
    pub fn not_busy(self) -> &'a mut crate::W<REG> {
        self.variant(Doutmemdmabusy::NotBusy)
    }
}
impl R {
    #[doc = "Bit 0 - DOUT memory DMA busy:"]
    #[inline(always)]
    pub fn doutmemdmabusy(&self) -> DoutmemdmabusyR {
        DoutmemdmabusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DOUT memory DMA busy:"]
    #[inline(always)]
    #[must_use]
    pub fn doutmemdmabusy(&mut self) -> DoutmemdmabusyW<DoutmemdmabusySpec> {
        DoutmemdmabusyW::new(self, 0)
    }
}
#[doc = "DOUT memory DMA busy - Indicates that memory (AXI) destination DMA (DOUT) is busy,\n\nYou can [`read`](crate::Reg::read) this register and get [`doutmemdmabusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutmemdmabusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutmemdmabusySpec;
impl crate::RegisterSpec for DoutmemdmabusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutmemdmabusy::R`](R) reader structure"]
impl crate::Readable for DoutmemdmabusySpec {}
#[doc = "`write(|w| ..)` method takes [`doutmemdmabusy::W`](W) writer structure"]
impl crate::Writable for DoutmemdmabusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTMEMDMABUSY to value 0"]
impl crate::Resettable for DoutmemdmabusySpec {
    const RESET_VALUE: u32 = 0;
}
