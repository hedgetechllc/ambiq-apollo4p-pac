#[doc = "Register `DAXICTRL` reader"]
pub type R = crate::R<DaxictrlSpec>;
#[doc = "Register `DAXICTRL` writer"]
pub type W = crate::W<DaxictrlSpec>;
#[doc = "Field `DAXIFLUSHWRITE` reader - Writing a 1 to this bitfield forces a flush of WRITE (W->I) or MODIFIED buffers (M->S)."]
pub type DaxiflushwriteR = crate::BitReader;
#[doc = "Field `DAXIFLUSHWRITE` writer - Writing a 1 to this bitfield forces a flush of WRITE (W->I) or MODIFIED buffers (M->S)."]
pub type DaxiflushwriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIINVALIDATE` reader - Writing a 1 to this bitfield invalidates any SHARED data buffers (S->I)."]
pub type DaxiinvalidateR = crate::BitReader;
#[doc = "Field `DAXIINVALIDATE` writer - Writing a 1 to this bitfield invalidates any SHARED data buffers (S->I)."]
pub type DaxiinvalidateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIREADY` reader - DAXI Ready Status (enabled and not processing a flush of WRITE or MODIFIED buffers)"]
pub type DaxireadyR = crate::BitReader;
#[doc = "Field `DAXIREADY` writer - DAXI Ready Status (enabled and not processing a flush of WRITE or MODIFIED buffers)"]
pub type DaxireadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIBUSY` reader - DAXI status indicating DAXI is busy."]
pub type DaxibusyR = crate::BitReader;
#[doc = "Field `DAXIBUSY` writer - DAXI status indicating DAXI is busy."]
pub type DaxibusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIAHBBUSY` reader - DAXI status indicating DAXI AHB interface is busy."]
pub type DaxiahbbusyR = crate::BitReader;
#[doc = "Field `DAXIAHBBUSY` writer - DAXI status indicating DAXI AHB interface is busy."]
pub type DaxiahbbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXISHARED` reader - DAXI status indicating at least one full buffer is shared."]
pub type DaxisharedR = crate::BitReader;
#[doc = "Field `DAXISHARED` writer - DAXI status indicating at least one full buffer is shared."]
pub type DaxisharedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIMODIFIED` reader - DAXI status indicating at least one full buffer has modified data."]
pub type DaximodifiedR = crate::BitReader;
#[doc = "Field `DAXIMODIFIED` writer - DAXI status indicating at least one full buffer has modified data."]
pub type DaximodifiedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIWRITE` reader - DAXI status indicating at least one partially written buffer has modified data."]
pub type DaxiwriteR = crate::BitReader;
#[doc = "Field `DAXIWRITE` writer - DAXI status indicating at least one partially written buffer has modified data."]
pub type DaxiwriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIWALLOC` reader - DAXI status indicating at least one write allocation is waiting for prior store to complete."]
pub type DaxiwallocR = crate::BitReader;
#[doc = "Field `DAXIWALLOC` writer - DAXI status indicating at least one write allocation is waiting for prior store to complete."]
pub type DaxiwallocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIWRLOAD` reader - DAXI status indicating at least one partially written buffer is waiting for load to convert to modified."]
pub type DaxiwrloadR = crate::BitReader;
#[doc = "Field `DAXIWRLOAD` writer - DAXI status indicating at least one partially written buffer is waiting for load to convert to modified."]
pub type DaxiwrloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXISTORE` reader - DAXI status indicating at least one buffer has outstanding store waiting to complete."]
pub type DaxistoreR = crate::BitReader;
#[doc = "Field `DAXISTORE` writer - DAXI status indicating at least one buffer has outstanding store waiting to complete."]
pub type DaxistoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIBRESPPENDING` reader - DAXI status indicating at least one AXI B repsonse for a store is pending."]
pub type DaxibresppendingR = crate::BitReader;
#[doc = "Field `DAXIBRESPPENDING` writer - DAXI status indicating at least one AXI B repsonse for a store is pending."]
pub type DaxibresppendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAXIRAXIBUSY` reader - DAXI status indicating the DAXI RAXI interface is busy."]
pub type DaxiraxibusyR = crate::BitReader;
#[doc = "Field `DAXIRAXIBUSY` writer - DAXI status indicating the DAXI RAXI interface is busy."]
pub type DaxiraxibusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing a 1 to this bitfield forces a flush of WRITE (W->I) or MODIFIED buffers (M->S)."]
    #[inline(always)]
    pub fn daxiflushwrite(&self) -> DaxiflushwriteR {
        DaxiflushwriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing a 1 to this bitfield invalidates any SHARED data buffers (S->I)."]
    #[inline(always)]
    pub fn daxiinvalidate(&self) -> DaxiinvalidateR {
        DaxiinvalidateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAXI Ready Status (enabled and not processing a flush of WRITE or MODIFIED buffers)"]
    #[inline(always)]
    pub fn daxiready(&self) -> DaxireadyR {
        DaxireadyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAXI status indicating DAXI is busy."]
    #[inline(always)]
    pub fn daxibusy(&self) -> DaxibusyR {
        DaxibusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DAXI status indicating DAXI AHB interface is busy."]
    #[inline(always)]
    pub fn daxiahbbusy(&self) -> DaxiahbbusyR {
        DaxiahbbusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAXI status indicating at least one full buffer is shared."]
    #[inline(always)]
    pub fn daxishared(&self) -> DaxisharedR {
        DaxisharedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DAXI status indicating at least one full buffer has modified data."]
    #[inline(always)]
    pub fn daximodified(&self) -> DaximodifiedR {
        DaximodifiedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAXI status indicating at least one partially written buffer has modified data."]
    #[inline(always)]
    pub fn daxiwrite(&self) -> DaxiwriteR {
        DaxiwriteR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAXI status indicating at least one write allocation is waiting for prior store to complete."]
    #[inline(always)]
    pub fn daxiwalloc(&self) -> DaxiwallocR {
        DaxiwallocR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAXI status indicating at least one partially written buffer is waiting for load to convert to modified."]
    #[inline(always)]
    pub fn daxiwrload(&self) -> DaxiwrloadR {
        DaxiwrloadR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DAXI status indicating at least one buffer has outstanding store waiting to complete."]
    #[inline(always)]
    pub fn daxistore(&self) -> DaxistoreR {
        DaxistoreR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAXI status indicating at least one AXI B repsonse for a store is pending."]
    #[inline(always)]
    pub fn daxibresppending(&self) -> DaxibresppendingR {
        DaxibresppendingR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DAXI status indicating the DAXI RAXI interface is busy."]
    #[inline(always)]
    pub fn daxiraxibusy(&self) -> DaxiraxibusyR {
        DaxiraxibusyR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bitfield forces a flush of WRITE (W->I) or MODIFIED buffers (M->S)."]
    #[inline(always)]
    #[must_use]
    pub fn daxiflushwrite(&mut self) -> DaxiflushwriteW<DaxictrlSpec> {
        DaxiflushwriteW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 to this bitfield invalidates any SHARED data buffers (S->I)."]
    #[inline(always)]
    #[must_use]
    pub fn daxiinvalidate(&mut self) -> DaxiinvalidateW<DaxictrlSpec> {
        DaxiinvalidateW::new(self, 1)
    }
    #[doc = "Bit 2 - DAXI Ready Status (enabled and not processing a flush of WRITE or MODIFIED buffers)"]
    #[inline(always)]
    #[must_use]
    pub fn daxiready(&mut self) -> DaxireadyW<DaxictrlSpec> {
        DaxireadyW::new(self, 2)
    }
    #[doc = "Bit 3 - DAXI status indicating DAXI is busy."]
    #[inline(always)]
    #[must_use]
    pub fn daxibusy(&mut self) -> DaxibusyW<DaxictrlSpec> {
        DaxibusyW::new(self, 3)
    }
    #[doc = "Bit 4 - DAXI status indicating DAXI AHB interface is busy."]
    #[inline(always)]
    #[must_use]
    pub fn daxiahbbusy(&mut self) -> DaxiahbbusyW<DaxictrlSpec> {
        DaxiahbbusyW::new(self, 4)
    }
    #[doc = "Bit 5 - DAXI status indicating at least one full buffer is shared."]
    #[inline(always)]
    #[must_use]
    pub fn daxishared(&mut self) -> DaxisharedW<DaxictrlSpec> {
        DaxisharedW::new(self, 5)
    }
    #[doc = "Bit 6 - DAXI status indicating at least one full buffer has modified data."]
    #[inline(always)]
    #[must_use]
    pub fn daximodified(&mut self) -> DaximodifiedW<DaxictrlSpec> {
        DaximodifiedW::new(self, 6)
    }
    #[doc = "Bit 7 - DAXI status indicating at least one partially written buffer has modified data."]
    #[inline(always)]
    #[must_use]
    pub fn daxiwrite(&mut self) -> DaxiwriteW<DaxictrlSpec> {
        DaxiwriteW::new(self, 7)
    }
    #[doc = "Bit 8 - DAXI status indicating at least one write allocation is waiting for prior store to complete."]
    #[inline(always)]
    #[must_use]
    pub fn daxiwalloc(&mut self) -> DaxiwallocW<DaxictrlSpec> {
        DaxiwallocW::new(self, 8)
    }
    #[doc = "Bit 9 - DAXI status indicating at least one partially written buffer is waiting for load to convert to modified."]
    #[inline(always)]
    #[must_use]
    pub fn daxiwrload(&mut self) -> DaxiwrloadW<DaxictrlSpec> {
        DaxiwrloadW::new(self, 9)
    }
    #[doc = "Bit 10 - DAXI status indicating at least one buffer has outstanding store waiting to complete."]
    #[inline(always)]
    #[must_use]
    pub fn daxistore(&mut self) -> DaxistoreW<DaxictrlSpec> {
        DaxistoreW::new(self, 10)
    }
    #[doc = "Bit 11 - DAXI status indicating at least one AXI B repsonse for a store is pending."]
    #[inline(always)]
    #[must_use]
    pub fn daxibresppending(&mut self) -> DaxibresppendingW<DaxictrlSpec> {
        DaxibresppendingW::new(self, 11)
    }
    #[doc = "Bit 12 - DAXI status indicating the DAXI RAXI interface is busy."]
    #[inline(always)]
    #[must_use]
    pub fn daxiraxibusy(&mut self) -> DaxiraxibusyW<DaxictrlSpec> {
        DaxiraxibusyW::new(self, 12)
    }
}
#[doc = "DAXI Control\n\nYou can [`read`](crate::Reg::read) this register and get [`daxictrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daxictrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaxictrlSpec;
impl crate::RegisterSpec for DaxictrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daxictrl::R`](R) reader structure"]
impl crate::Readable for DaxictrlSpec {}
#[doc = "`write(|w| ..)` method takes [`daxictrl::W`](W) writer structure"]
impl crate::Writable for DaxictrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAXICTRL to value 0x04"]
impl crate::Resettable for DaxictrlSpec {
    const RESET_VALUE: u32 = 0x04;
}
