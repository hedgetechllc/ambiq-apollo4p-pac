#[doc = "Register `RNGISR` reader"]
pub type R = crate::R<RngisrSpec>;
#[doc = "Register `RNGISR` writer"]
pub type W = crate::W<RngisrSpec>;
#[doc = "Field `EHRVALID` reader - 0x1 indicates that 192 bits have been collected in the TRNG and are ready to be read."]
pub type EhrvalidR = crate::BitReader;
#[doc = "Field `EHRVALID` writer - 0x1 indicates that 192 bits have been collected in the TRNG and are ready to be read."]
pub type EhrvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORRERR` reader - 0x1 indicates Autocorrelation test failed four times in a row. When it set ,TRNG ceases to function until next reset."]
pub type AutocorrerrR = crate::BitReader;
#[doc = "Field `AUTOCORRERR` writer - 0x1 indicates Autocorrelation test failed four times in a row. When it set ,TRNG ceases to function until next reset."]
pub type AutocorrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGTERR` reader - 0x1 indicates CRNGT in the TRNG test failed. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
pub type CrngterrR = crate::BitReader;
#[doc = "Field `CRNGTERR` writer - 0x1 indicates CRNGT in the TRNG test failed. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
pub type CrngterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VNERR` reader - 0x1 indicates Von Neumann error. Error in von Neumann occurs if 32 consecutive collected bits are identical, ZERO, or ONE."]
pub type VnerrR = crate::BitReader;
#[doc = "Field `VNERR` writer - 0x1 indicates Von Neumann error. Error in von Neumann occurs if 32 consecutive collected bits are identical, ZERO, or ONE."]
pub type VnerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGDMADONE` reader - 0x1 indicates RNG DMA to SRAM is completed."]
pub type RngdmadoneR = crate::BitReader;
#[doc = "Field `RNGDMADONE` writer - 0x1 indicates RNG DMA to SRAM is completed."]
pub type RngdmadoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEEDINGDONE` reader - 0x1 indicates completion of reseeding algorithm with no errors."]
pub type ReseedingdoneR = crate::BitReader;
#[doc = "Field `RESEEDINGDONE` writer - 0x1 indicates completion of reseeding algorithm with no errors."]
pub type ReseedingdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTANTIATIONDONE` reader - 0x1 indicates completion of instantiation algorithm with no errors."]
pub type InstantiationdoneR = crate::BitReader;
#[doc = "Field `INSTANTIATIONDONE` writer - 0x1 indicates completion of instantiation algorithm with no errors."]
pub type InstantiationdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FINALUPDATEDONE` reader - 0x1 indicates completion of final update algorithm."]
pub type FinalupdatedoneR = crate::BitReader;
#[doc = "Field `FINALUPDATEDONE` writer - 0x1 indicates completion of final update algorithm."]
pub type FinalupdatedoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUTREADY` reader - 0x1 indicates that the result of PRNG is valid and ready to be read. The result can be read from the RNG_READOUT register."]
pub type OutputreadyR = crate::BitReader;
#[doc = "Field `OUTPUTREADY` writer - 0x1 indicates that the result of PRNG is valid and ready to be read. The result can be read from the RNG_READOUT register."]
pub type OutputreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEEDCNTRFULL` reader - 0x1 indicates that the reseed counter has reached 2^48, requiring to run the reseed algorithm before generating new random numbers."]
pub type ReseedcntrfullR = crate::BitReader;
#[doc = "Field `RESEEDCNTRFULL` writer - 0x1 indicates that the reseed counter has reached 2^48, requiring to run the reseed algorithm before generating new random numbers."]
pub type ReseedcntrfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEEDCNTRTOP40` reader - 0x1 indicates that the top 40 bits of the reseed counter are set (that is the reseed counter is larger than 2^48-2^8). This is a recommendation for running the reseed algorithm before the counter reaches its max value."]
pub type Reseedcntrtop40R = crate::BitReader;
#[doc = "Field `RESEEDCNTRTOP40` writer - 0x1 indicates that the top 40 bits of the reseed counter are set (that is the reseed counter is larger than 2^48-2^8). This is a recommendation for running the reseed algorithm before the counter reaches its max value."]
pub type Reseedcntrtop40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRNGCRNGTERR` reader - 0x1 indicates CRNGT in the PRNG test failed. Failure occurs when two consecutive results of AES are equal"]
pub type PrngcrngterrR = crate::BitReader;
#[doc = "Field `PRNGCRNGTERR` writer - 0x1 indicates CRNGT in the PRNG test failed. Failure occurs when two consecutive results of AES are equal"]
pub type PrngcrngterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQSIZE` reader - 0x1 indicates that the request size counter (which represents how many generations of random bits in the PRNG have been produced) has reached 2^12, thus requiring a working state update before generating new random numbers."]
pub type ReqsizeR = crate::BitReader;
#[doc = "Field `REQSIZE` writer - 0x1 indicates that the request size counter (which represents how many generations of random bits in the PRNG have been produced) has reached 2^12, thus requiring a working state update before generating new random numbers."]
pub type ReqsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KATERR` reader - 0x1 indicates that one of the KAT (Known Answer Tests) tests has failed. When set, the entire engine ceases to function."]
pub type KaterrR = crate::BitReader;
#[doc = "Field `KATERR` writer - 0x1 indicates that one of the KAT (Known Answer Tests) tests has failed. When set, the entire engine ceases to function."]
pub type KaterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "When the KAT_ERR bit is set, these bits represent which Known Answer Test had failed:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Whichkaterr {
    #[doc = "0: first test of instantiation"]
    Instant1 = 0,
    #[doc = "1: second test of instantiation"]
    Instant2 = 1,
    #[doc = "2: first test of reseeding"]
    Reseed1 = 2,
    #[doc = "3: second test of reseeding"]
    Reseed2 = 3,
}
impl From<Whichkaterr> for u8 {
    #[inline(always)]
    fn from(variant: Whichkaterr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Whichkaterr {
    type Ux = u8;
}
impl crate::IsEnum for Whichkaterr {}
#[doc = "Field `WHICHKATERR` reader - When the KAT_ERR bit is set, these bits represent which Known Answer Test had failed:"]
pub type WhichkaterrR = crate::FieldReader<Whichkaterr>;
impl WhichkaterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Whichkaterr {
        match self.bits {
            0 => Whichkaterr::Instant1,
            1 => Whichkaterr::Instant2,
            2 => Whichkaterr::Reseed1,
            3 => Whichkaterr::Reseed2,
            _ => unreachable!(),
        }
    }
    #[doc = "first test of instantiation"]
    #[inline(always)]
    pub fn is_instant_1(&self) -> bool {
        *self == Whichkaterr::Instant1
    }
    #[doc = "second test of instantiation"]
    #[inline(always)]
    pub fn is_instant_2(&self) -> bool {
        *self == Whichkaterr::Instant2
    }
    #[doc = "first test of reseeding"]
    #[inline(always)]
    pub fn is_reseed_1(&self) -> bool {
        *self == Whichkaterr::Reseed1
    }
    #[doc = "second test of reseeding"]
    #[inline(always)]
    pub fn is_reseed_2(&self) -> bool {
        *self == Whichkaterr::Reseed2
    }
}
#[doc = "Field `WHICHKATERR` writer - When the KAT_ERR bit is set, these bits represent which Known Answer Test had failed:"]
pub type WhichkaterrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Whichkaterr, crate::Safe>;
impl<'a, REG> WhichkaterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "first test of instantiation"]
    #[inline(always)]
    pub fn instant_1(self) -> &'a mut crate::W<REG> {
        self.variant(Whichkaterr::Instant1)
    }
    #[doc = "second test of instantiation"]
    #[inline(always)]
    pub fn instant_2(self) -> &'a mut crate::W<REG> {
        self.variant(Whichkaterr::Instant2)
    }
    #[doc = "first test of reseeding"]
    #[inline(always)]
    pub fn reseed_1(self) -> &'a mut crate::W<REG> {
        self.variant(Whichkaterr::Reseed1)
    }
    #[doc = "second test of reseeding"]
    #[inline(always)]
    pub fn reseed_2(self) -> &'a mut crate::W<REG> {
        self.variant(Whichkaterr::Reseed2)
    }
}
impl R {
    #[doc = "Bit 0 - 0x1 indicates that 192 bits have been collected in the TRNG and are ready to be read."]
    #[inline(always)]
    pub fn ehrvalid(&self) -> EhrvalidR {
        EhrvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0x1 indicates Autocorrelation test failed four times in a row. When it set ,TRNG ceases to function until next reset."]
    #[inline(always)]
    pub fn autocorrerr(&self) -> AutocorrerrR {
        AutocorrerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0x1 indicates CRNGT in the TRNG test failed. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
    #[inline(always)]
    pub fn crngterr(&self) -> CrngterrR {
        CrngterrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0x1 indicates Von Neumann error. Error in von Neumann occurs if 32 consecutive collected bits are identical, ZERO, or ONE."]
    #[inline(always)]
    pub fn vnerr(&self) -> VnerrR {
        VnerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 0x1 indicates RNG DMA to SRAM is completed."]
    #[inline(always)]
    pub fn rngdmadone(&self) -> RngdmadoneR {
        RngdmadoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - 0x1 indicates completion of reseeding algorithm with no errors."]
    #[inline(always)]
    pub fn reseedingdone(&self) -> ReseedingdoneR {
        ReseedingdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0x1 indicates completion of instantiation algorithm with no errors."]
    #[inline(always)]
    pub fn instantiationdone(&self) -> InstantiationdoneR {
        InstantiationdoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0x1 indicates completion of final update algorithm."]
    #[inline(always)]
    pub fn finalupdatedone(&self) -> FinalupdatedoneR {
        FinalupdatedoneR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 0x1 indicates that the result of PRNG is valid and ready to be read. The result can be read from the RNG_READOUT register."]
    #[inline(always)]
    pub fn outputready(&self) -> OutputreadyR {
        OutputreadyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 0x1 indicates that the reseed counter has reached 2^48, requiring to run the reseed algorithm before generating new random numbers."]
    #[inline(always)]
    pub fn reseedcntrfull(&self) -> ReseedcntrfullR {
        ReseedcntrfullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 0x1 indicates that the top 40 bits of the reseed counter are set (that is the reseed counter is larger than 2^48-2^8). This is a recommendation for running the reseed algorithm before the counter reaches its max value."]
    #[inline(always)]
    pub fn reseedcntrtop40(&self) -> Reseedcntrtop40R {
        Reseedcntrtop40R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 0x1 indicates CRNGT in the PRNG test failed. Failure occurs when two consecutive results of AES are equal"]
    #[inline(always)]
    pub fn prngcrngterr(&self) -> PrngcrngterrR {
        PrngcrngterrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 0x1 indicates that the request size counter (which represents how many generations of random bits in the PRNG have been produced) has reached 2^12, thus requiring a working state update before generating new random numbers."]
    #[inline(always)]
    pub fn reqsize(&self) -> ReqsizeR {
        ReqsizeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 0x1 indicates that one of the KAT (Known Answer Tests) tests has failed. When set, the entire engine ceases to function."]
    #[inline(always)]
    pub fn katerr(&self) -> KaterrR {
        KaterrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - When the KAT_ERR bit is set, these bits represent which Known Answer Test had failed:"]
    #[inline(always)]
    pub fn whichkaterr(&self) -> WhichkaterrR {
        WhichkaterrR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0x1 indicates that 192 bits have been collected in the TRNG and are ready to be read."]
    #[inline(always)]
    #[must_use]
    pub fn ehrvalid(&mut self) -> EhrvalidW<RngisrSpec> {
        EhrvalidW::new(self, 0)
    }
    #[doc = "Bit 1 - 0x1 indicates Autocorrelation test failed four times in a row. When it set ,TRNG ceases to function until next reset."]
    #[inline(always)]
    #[must_use]
    pub fn autocorrerr(&mut self) -> AutocorrerrW<RngisrSpec> {
        AutocorrerrW::new(self, 1)
    }
    #[doc = "Bit 2 - 0x1 indicates CRNGT in the TRNG test failed. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
    #[inline(always)]
    #[must_use]
    pub fn crngterr(&mut self) -> CrngterrW<RngisrSpec> {
        CrngterrW::new(self, 2)
    }
    #[doc = "Bit 3 - 0x1 indicates Von Neumann error. Error in von Neumann occurs if 32 consecutive collected bits are identical, ZERO, or ONE."]
    #[inline(always)]
    #[must_use]
    pub fn vnerr(&mut self) -> VnerrW<RngisrSpec> {
        VnerrW::new(self, 3)
    }
    #[doc = "Bit 5 - 0x1 indicates RNG DMA to SRAM is completed."]
    #[inline(always)]
    #[must_use]
    pub fn rngdmadone(&mut self) -> RngdmadoneW<RngisrSpec> {
        RngdmadoneW::new(self, 5)
    }
    #[doc = "Bit 16 - 0x1 indicates completion of reseeding algorithm with no errors."]
    #[inline(always)]
    #[must_use]
    pub fn reseedingdone(&mut self) -> ReseedingdoneW<RngisrSpec> {
        ReseedingdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - 0x1 indicates completion of instantiation algorithm with no errors."]
    #[inline(always)]
    #[must_use]
    pub fn instantiationdone(&mut self) -> InstantiationdoneW<RngisrSpec> {
        InstantiationdoneW::new(self, 17)
    }
    #[doc = "Bit 18 - 0x1 indicates completion of final update algorithm."]
    #[inline(always)]
    #[must_use]
    pub fn finalupdatedone(&mut self) -> FinalupdatedoneW<RngisrSpec> {
        FinalupdatedoneW::new(self, 18)
    }
    #[doc = "Bit 19 - 0x1 indicates that the result of PRNG is valid and ready to be read. The result can be read from the RNG_READOUT register."]
    #[inline(always)]
    #[must_use]
    pub fn outputready(&mut self) -> OutputreadyW<RngisrSpec> {
        OutputreadyW::new(self, 19)
    }
    #[doc = "Bit 20 - 0x1 indicates that the reseed counter has reached 2^48, requiring to run the reseed algorithm before generating new random numbers."]
    #[inline(always)]
    #[must_use]
    pub fn reseedcntrfull(&mut self) -> ReseedcntrfullW<RngisrSpec> {
        ReseedcntrfullW::new(self, 20)
    }
    #[doc = "Bit 21 - 0x1 indicates that the top 40 bits of the reseed counter are set (that is the reseed counter is larger than 2^48-2^8). This is a recommendation for running the reseed algorithm before the counter reaches its max value."]
    #[inline(always)]
    #[must_use]
    pub fn reseedcntrtop40(&mut self) -> Reseedcntrtop40W<RngisrSpec> {
        Reseedcntrtop40W::new(self, 21)
    }
    #[doc = "Bit 22 - 0x1 indicates CRNGT in the PRNG test failed. Failure occurs when two consecutive results of AES are equal"]
    #[inline(always)]
    #[must_use]
    pub fn prngcrngterr(&mut self) -> PrngcrngterrW<RngisrSpec> {
        PrngcrngterrW::new(self, 22)
    }
    #[doc = "Bit 23 - 0x1 indicates that the request size counter (which represents how many generations of random bits in the PRNG have been produced) has reached 2^12, thus requiring a working state update before generating new random numbers."]
    #[inline(always)]
    #[must_use]
    pub fn reqsize(&mut self) -> ReqsizeW<RngisrSpec> {
        ReqsizeW::new(self, 23)
    }
    #[doc = "Bit 24 - 0x1 indicates that one of the KAT (Known Answer Tests) tests has failed. When set, the entire engine ceases to function."]
    #[inline(always)]
    #[must_use]
    pub fn katerr(&mut self) -> KaterrW<RngisrSpec> {
        KaterrW::new(self, 24)
    }
    #[doc = "Bits 25:26 - When the KAT_ERR bit is set, these bits represent which Known Answer Test had failed:"]
    #[inline(always)]
    #[must_use]
    pub fn whichkaterr(&mut self) -> WhichkaterrW<RngisrSpec> {
        WhichkaterrW::new(self, 25)
    }
}
#[doc = "Status register. If corresponding RNG_IMR bit is unmasked, an interrupt is generated.Consists of trng_isr and prng_isr bit\\[15-0\\]
- TRNG bit\\[31-16\\]
- PRNG\n\nYou can [`read`](crate::Reg::read) this register and get [`rngisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngisrSpec;
impl crate::RegisterSpec for RngisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngisr::R`](R) reader structure"]
impl crate::Readable for RngisrSpec {}
#[doc = "`write(|w| ..)` method takes [`rngisr::W`](W) writer structure"]
impl crate::Writable for RngisrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNGISR to value 0"]
impl crate::Resettable for RngisrSpec {
    const RESET_VALUE: u32 = 0;
}
