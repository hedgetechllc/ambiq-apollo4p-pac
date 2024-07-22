#[doc = "Register `RNGDMASTATUS` reader"]
pub type R = crate::R<RngdmastatusSpec>;
#[doc = "Register `RNGDMASTATUS` writer"]
pub type W = crate::W<RngdmastatusSpec>;
#[doc = "Field `RNGDMABUSY` reader - Indicates whether DMA is busy."]
pub type RngdmabusyR = crate::BitReader;
#[doc = "Field `RNGDMABUSY` writer - Indicates whether DMA is busy."]
pub type RngdmabusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASRCSEL` reader - The active ring oscillator length using by DMA"]
pub type DmasrcselR = crate::FieldReader;
#[doc = "Field `DMASRCSEL` writer - The active ring oscillator length using by DMA"]
pub type DmasrcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NUMOFSAMPLES` reader - Number of samples already collected in the current ring oscillator chain length."]
pub type NumofsamplesR = crate::FieldReader;
#[doc = "Field `NUMOFSAMPLES` writer - Number of samples already collected in the current ring oscillator chain length."]
pub type NumofsamplesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Indicates whether DMA is busy."]
    #[inline(always)]
    pub fn rngdmabusy(&self) -> RngdmabusyR {
        RngdmabusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - The active ring oscillator length using by DMA"]
    #[inline(always)]
    pub fn dmasrcsel(&self) -> DmasrcselR {
        DmasrcselR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Number of samples already collected in the current ring oscillator chain length."]
    #[inline(always)]
    pub fn numofsamples(&self) -> NumofsamplesR {
        NumofsamplesR::new(((self.bits >> 3) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether DMA is busy."]
    #[inline(always)]
    #[must_use]
    pub fn rngdmabusy(&mut self) -> RngdmabusyW<RngdmastatusSpec> {
        RngdmabusyW::new(self, 0)
    }
    #[doc = "Bits 1:2 - The active ring oscillator length using by DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dmasrcsel(&mut self) -> DmasrcselW<RngdmastatusSpec> {
        DmasrcselW::new(self, 1)
    }
    #[doc = "Bits 3:10 - Number of samples already collected in the current ring oscillator chain length."]
    #[inline(always)]
    #[must_use]
    pub fn numofsamples(&mut self) -> NumofsamplesW<RngdmastatusSpec> {
        NumofsamplesW::new(self, 3)
    }
}
#[doc = "This register holds the RNG DMA status.\n\nYou can [`read`](crate::Reg::read) this register and get [`rngdmastatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngdmastatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngdmastatusSpec;
impl crate::RegisterSpec for RngdmastatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngdmastatus::R`](R) reader structure"]
impl crate::Readable for RngdmastatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rngdmastatus::W`](W) writer structure"]
impl crate::Writable for RngdmastatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGDMASTATUS to value 0"]
impl crate::Resettable for RngdmastatusSpec {
    const RESET_VALUE: u32 = 0;
}
