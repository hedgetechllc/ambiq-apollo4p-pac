#[doc = "Register `DINMEMDMABUSY` reader"]
pub type R = crate::R<DinmemdmabusySpec>;
#[doc = "Register `DINMEMDMABUSY` writer"]
pub type W = crate::W<DinmemdmabusySpec>;
#[doc = "DIN memory DMA busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dinmemdmabusy {
    #[doc = "1: DMA busy"]
    Busy = 1,
    #[doc = "0: DMA not busy"]
    Not = 0,
}
impl From<Dinmemdmabusy> for bool {
    #[inline(always)]
    fn from(variant: Dinmemdmabusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINMEMDMABUSY` reader - DIN memory DMA busy"]
pub type DinmemdmabusyR = crate::BitReader<Dinmemdmabusy>;
impl DinmemdmabusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dinmemdmabusy {
        match self.bits {
            true => Dinmemdmabusy::Busy,
            false => Dinmemdmabusy::Not,
        }
    }
    #[doc = "DMA busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Dinmemdmabusy::Busy
    }
    #[doc = "DMA not busy"]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == Dinmemdmabusy::Not
    }
}
#[doc = "Field `DINMEMDMABUSY` writer - DIN memory DMA busy"]
pub type DinmemdmabusyW<'a, REG> = crate::BitWriter<'a, REG, Dinmemdmabusy>;
impl<'a, REG> DinmemdmabusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Dinmemdmabusy::Busy)
    }
    #[doc = "DMA not busy"]
    #[inline(always)]
    pub fn not(self) -> &'a mut crate::W<REG> {
        self.variant(Dinmemdmabusy::Not)
    }
}
impl R {
    #[doc = "Bit 0 - DIN memory DMA busy"]
    #[inline(always)]
    pub fn dinmemdmabusy(&self) -> DinmemdmabusyR {
        DinmemdmabusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIN memory DMA busy"]
    #[inline(always)]
    #[must_use]
    pub fn dinmemdmabusy(&mut self) -> DinmemdmabusyW<DinmemdmabusySpec> {
        DinmemdmabusyW::new(self, 0)
    }
}
#[doc = "Indicates whether memory (AXI) source DMA (DIN) is busy.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinmemdmabusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinmemdmabusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinmemdmabusySpec;
impl crate::RegisterSpec for DinmemdmabusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinmemdmabusy::R`](R) reader structure"]
impl crate::Readable for DinmemdmabusySpec {}
#[doc = "`write(|w| ..)` method takes [`dinmemdmabusy::W`](W) writer structure"]
impl crate::Writable for DinmemdmabusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINMEMDMABUSY to value 0"]
impl crate::Resettable for DinmemdmabusySpec {
    const RESET_VALUE: u32 = 0;
}
