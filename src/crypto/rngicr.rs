#[doc = "Register `RNGICR` reader"]
pub type R = crate::R<RngicrSpec>;
#[doc = "Register `RNGICR` writer"]
pub type W = crate::W<RngicrSpec>;
#[doc = "Field `EHRVALID` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type EhrvalidR = crate::BitReader;
#[doc = "Field `EHRVALID` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type EhrvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORRERR` reader - Cannot be cleared by SW! Only RNG reset clears this bit."]
pub type AutocorrerrR = crate::BitReader;
#[doc = "Field `AUTOCORRERR` writer - Cannot be cleared by SW! Only RNG reset clears this bit."]
pub type AutocorrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGTERR` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type CrngterrR = crate::BitReader;
#[doc = "Field `CRNGTERR` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type CrngterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VNERR` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type VnerrR = crate::BitReader;
#[doc = "Field `VNERR` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type VnerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGWATCHDOG` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type RngwatchdogR = crate::BitReader;
#[doc = "Field `RNGWATCHDOG` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type RngwatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGDMADONE` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type RngdmadoneR = crate::BitReader;
#[doc = "Field `RNGDMADONE` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type RngdmadoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEEDINGDONE` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type ReseedingdoneR = crate::BitReader;
#[doc = "Field `RESEEDINGDONE` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type ReseedingdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTANTIATIONDONE` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type InstantiationdoneR = crate::BitReader;
#[doc = "Field `INSTANTIATIONDONE` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type InstantiationdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FINALUPDATEDONE` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type FinalupdatedoneR = crate::BitReader;
#[doc = "Field `FINALUPDATEDONE` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type FinalupdatedoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUTREADY` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type OutputreadyR = crate::BitReader;
#[doc = "Field `OUTPUTREADY` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type OutputreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEEDCNTRFULL` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type ReseedcntrfullR = crate::BitReader;
#[doc = "Field `RESEEDCNTRFULL` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type ReseedcntrfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEEDCNTRTOP40` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type Reseedcntrtop40R = crate::BitReader;
#[doc = "Field `RESEEDCNTRTOP40` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type Reseedcntrtop40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRNGCRNGTERR` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type PrngcrngterrR = crate::BitReader;
#[doc = "Field `PRNGCRNGTERR` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type PrngcrngterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQSIZE` reader - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type ReqsizeR = crate::BitReader;
#[doc = "Field `REQSIZE` writer - Writing value 0x1 - clears corresponding bit in RNGISR"]
pub type ReqsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KATERR` reader - Cannot be cleared by SW! Only RNG reset clears this bit."]
pub type KaterrR = crate::BitReader;
#[doc = "Field `KATERR` writer - Cannot be cleared by SW! Only RNG reset clears this bit."]
pub type KaterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHICHKATERR` reader - Cannot be cleared by SW! Only RNG reset clears this bit."]
pub type WhichkaterrR = crate::FieldReader;
#[doc = "Field `WHICHKATERR` writer - Cannot be cleared by SW! Only RNG reset clears this bit."]
pub type WhichkaterrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn ehrvalid(&self) -> EhrvalidR {
        EhrvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    pub fn autocorrerr(&self) -> AutocorrerrR {
        AutocorrerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn crngterr(&self) -> CrngterrR {
        CrngterrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn vnerr(&self) -> VnerrR {
        VnerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn rngwatchdog(&self) -> RngwatchdogR {
        RngwatchdogR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn rngdmadone(&self) -> RngdmadoneR {
        RngdmadoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn reseedingdone(&self) -> ReseedingdoneR {
        ReseedingdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn instantiationdone(&self) -> InstantiationdoneR {
        InstantiationdoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn finalupdatedone(&self) -> FinalupdatedoneR {
        FinalupdatedoneR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn outputready(&self) -> OutputreadyR {
        OutputreadyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn reseedcntrfull(&self) -> ReseedcntrfullR {
        ReseedcntrfullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn reseedcntrtop40(&self) -> Reseedcntrtop40R {
        Reseedcntrtop40R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn prngcrngterr(&self) -> PrngcrngterrR {
        PrngcrngterrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    pub fn reqsize(&self) -> ReqsizeR {
        ReqsizeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    pub fn katerr(&self) -> KaterrR {
        KaterrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    pub fn whichkaterr(&self) -> WhichkaterrR {
        WhichkaterrR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn ehrvalid(&mut self) -> EhrvalidW<RngicrSpec> {
        EhrvalidW::new(self, 0)
    }
    #[doc = "Bit 1 - Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn autocorrerr(&mut self) -> AutocorrerrW<RngicrSpec> {
        AutocorrerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn crngterr(&mut self) -> CrngterrW<RngicrSpec> {
        CrngterrW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn vnerr(&mut self) -> VnerrW<RngicrSpec> {
        VnerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn rngwatchdog(&mut self) -> RngwatchdogW<RngicrSpec> {
        RngwatchdogW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn rngdmadone(&mut self) -> RngdmadoneW<RngicrSpec> {
        RngdmadoneW::new(self, 5)
    }
    #[doc = "Bit 16 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn reseedingdone(&mut self) -> ReseedingdoneW<RngicrSpec> {
        ReseedingdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn instantiationdone(&mut self) -> InstantiationdoneW<RngicrSpec> {
        InstantiationdoneW::new(self, 17)
    }
    #[doc = "Bit 18 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn finalupdatedone(&mut self) -> FinalupdatedoneW<RngicrSpec> {
        FinalupdatedoneW::new(self, 18)
    }
    #[doc = "Bit 19 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn outputready(&mut self) -> OutputreadyW<RngicrSpec> {
        OutputreadyW::new(self, 19)
    }
    #[doc = "Bit 20 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn reseedcntrfull(&mut self) -> ReseedcntrfullW<RngicrSpec> {
        ReseedcntrfullW::new(self, 20)
    }
    #[doc = "Bit 21 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn reseedcntrtop40(&mut self) -> Reseedcntrtop40W<RngicrSpec> {
        Reseedcntrtop40W::new(self, 21)
    }
    #[doc = "Bit 22 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn prngcrngterr(&mut self) -> PrngcrngterrW<RngicrSpec> {
        PrngcrngterrW::new(self, 22)
    }
    #[doc = "Bit 23 - Writing value 0x1 - clears corresponding bit in RNGISR"]
    #[inline(always)]
    #[must_use]
    pub fn reqsize(&mut self) -> ReqsizeW<RngicrSpec> {
        ReqsizeW::new(self, 23)
    }
    #[doc = "Bit 24 - Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn katerr(&mut self) -> KaterrW<RngicrSpec> {
        KaterrW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn whichkaterr(&mut self) -> WhichkaterrW<RngicrSpec> {
        WhichkaterrW::new(self, 25)
    }
}
#[doc = "Interrupt_status bit clear Register. Consists of trng_icr and prng_icr bit\\[15-0\\]
- TRNG bit\\[31-16\\]
- PRNG\n\nYou can [`read`](crate::Reg::read) this register and get [`rngicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngicrSpec;
impl crate::RegisterSpec for RngicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngicr::R`](R) reader structure"]
impl crate::Readable for RngicrSpec {}
#[doc = "`write(|w| ..)` method takes [`rngicr::W`](W) writer structure"]
impl crate::Writable for RngicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGICR to value 0"]
impl crate::Resettable for RngicrSpec {
    const RESET_VALUE: u32 = 0;
}
