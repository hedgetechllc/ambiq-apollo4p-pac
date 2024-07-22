#[doc = "Register `CQSTAT` reader"]
pub type R = crate::R<CqstatSpec>;
#[doc = "Register `CQSTAT` writer"]
pub type W = crate::W<CqstatSpec>;
#[doc = "Field `CQTIP` reader - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
pub type CqtipR = crate::BitReader;
#[doc = "Field `CQTIP` writer - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
pub type CqtipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQCPL` reader - Command queue operation Complete. This signals the end of the command queue operation."]
pub type CqcplR = crate::BitReader;
#[doc = "Field `CQCPL` writer - Command queue operation Complete. This signals the end of the command queue operation."]
pub type CqcplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQERR` reader - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
pub type CqerrR = crate::BitReader;
#[doc = "Field `CQERR` writer - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
pub type CqerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQPAUSED` reader - Command queue is currently paused status."]
pub type CqpausedR = crate::BitReader;
#[doc = "Field `CQPAUSED` writer - Command queue is currently paused status."]
pub type CqpausedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
    #[inline(always)]
    pub fn cqtip(&self) -> CqtipR {
        CqtipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command queue operation Complete. This signals the end of the command queue operation."]
    #[inline(always)]
    pub fn cqcpl(&self) -> CqcplR {
        CqcplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
    #[inline(always)]
    pub fn cqerr(&self) -> CqerrR {
        CqerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command queue is currently paused status."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CqpausedR {
        CqpausedR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
    #[inline(always)]
    #[must_use]
    pub fn cqtip(&mut self) -> CqtipW<CqstatSpec> {
        CqtipW::new(self, 0)
    }
    #[doc = "Bit 1 - Command queue operation Complete. This signals the end of the command queue operation."]
    #[inline(always)]
    #[must_use]
    pub fn cqcpl(&mut self) -> CqcplW<CqstatSpec> {
        CqcplW::new(self, 1)
    }
    #[doc = "Bit 2 - Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
    #[inline(always)]
    #[must_use]
    pub fn cqerr(&mut self) -> CqerrW<CqstatSpec> {
        CqerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Command queue is currently paused status."]
    #[inline(always)]
    #[must_use]
    pub fn cqpaused(&mut self) -> CqpausedW<CqstatSpec> {
        CqpausedW::new(self, 3)
    }
}
#[doc = "Command Queue Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cqstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqstatSpec;
impl crate::RegisterSpec for CqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqstat::R`](R) reader structure"]
impl crate::Readable for CqstatSpec {}
#[doc = "`write(|w| ..)` method takes [`cqstat::W`](W) writer structure"]
impl crate::Writable for CqstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQSTAT to value 0"]
impl crate::Resettable for CqstatSpec {
    const RESET_VALUE: u32 = 0;
}
