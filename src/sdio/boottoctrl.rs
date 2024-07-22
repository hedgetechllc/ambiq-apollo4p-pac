#[doc = "Register `BOOTTOCTRL` reader"]
pub type R = crate::R<BoottoctrlSpec>;
#[doc = "Register `BOOTTOCTRL` writer"]
pub type W = crate::W<BoottoctrlSpec>;
#[doc = "Field `BOOTDATATO` reader - This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC card. The value is in number of sd clock."]
pub type BootdatatoR = crate::FieldReader<u32>;
#[doc = "Field `BOOTDATATO` writer - This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC card. The value is in number of sd clock."]
pub type BootdatatoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC card. The value is in number of sd clock."]
    #[inline(always)]
    pub fn bootdatato(&self) -> BootdatatoR {
        BootdatatoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This value determines the interval by which DAT line time-outs are detected during boot operation for eMMC card. The value is in number of sd clock."]
    #[inline(always)]
    #[must_use]
    pub fn bootdatato(&mut self) -> BootdatatoW<BoottoctrlSpec> {
        BootdatatoW::new(self, 0)
    }
}
#[doc = "Boot Data Timeout control\n\nYou can [`read`](crate::Reg::read) this register and get [`boottoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boottoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BoottoctrlSpec;
impl crate::RegisterSpec for BoottoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boottoctrl::R`](R) reader structure"]
impl crate::Readable for BoottoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`boottoctrl::W`](W) writer structure"]
impl crate::Writable for BoottoctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTTOCTRL to value 0"]
impl crate::Resettable for BoottoctrlSpec {
    const RESET_VALUE: u32 = 0;
}
