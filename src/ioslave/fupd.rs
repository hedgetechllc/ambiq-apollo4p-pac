#[doc = "Register `FUPD` reader"]
pub type R = crate::R<FupdSpec>;
#[doc = "Register `FUPD` writer"]
pub type W = crate::W<FupdSpec>;
#[doc = "Field `FIFOUPD` reader - This bit indicates that a FIFO update is underway."]
pub type FifoupdR = crate::BitReader;
#[doc = "Field `FIFOUPD` writer - This bit indicates that a FIFO update is underway."]
pub type FifoupdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOREAD` reader - This bitfield indicates an IO read is active."]
pub type IoreadR = crate::BitReader;
#[doc = "Field `IOREAD` writer - This bitfield indicates an IO read is active."]
pub type IoreadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit indicates that a FIFO update is underway."]
    #[inline(always)]
    pub fn fifoupd(&self) -> FifoupdR {
        FifoupdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bitfield indicates an IO read is active."]
    #[inline(always)]
    pub fn ioread(&self) -> IoreadR {
        IoreadR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates that a FIFO update is underway."]
    #[inline(always)]
    #[must_use]
    pub fn fifoupd(&mut self) -> FifoupdW<FupdSpec> {
        FifoupdW::new(self, 0)
    }
    #[doc = "Bit 1 - This bitfield indicates an IO read is active."]
    #[inline(always)]
    #[must_use]
    pub fn ioread(&mut self) -> IoreadW<FupdSpec> {
        IoreadW::new(self, 1)
    }
}
#[doc = "FIFO Update Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fupd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fupd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FupdSpec;
impl crate::RegisterSpec for FupdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fupd::R`](R) reader structure"]
impl crate::Readable for FupdSpec {}
#[doc = "`write(|w| ..)` method takes [`fupd::W`](W) writer structure"]
impl crate::Writable for FupdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUPD to value 0"]
impl crate::Resettable for FupdSpec {
    const RESET_VALUE: u32 = 0;
}
