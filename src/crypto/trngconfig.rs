#[doc = "Register `TRNGCONFIG` reader"]
pub type R = crate::R<TrngconfigSpec>;
#[doc = "Register `TRNGCONFIG` writer"]
pub type W = crate::W<TrngconfigSpec>;
#[doc = "Field `RNDSRCSEL` reader - Defines the length of the oscillator ring (= the number of inverters) out of four possible selections."]
pub type RndsrcselR = crate::FieldReader;
#[doc = "Field `RNDSRCSEL` writer - Defines the length of the oscillator ring (= the number of inverters) out of four possible selections."]
pub type RndsrcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Secure Output Port selection:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sopsel {
    #[doc = "1: sop_data port reflects TRNG output (EHR_DATA)."]
    SopData1 = 1,
    #[doc = "0: sop_data port reflects PRNG output (RNG_READOUT). Note: Secure output is used for direct connection of the RNG block outputs to an engine input key."]
    SopData2 = 0,
}
impl From<Sopsel> for bool {
    #[inline(always)]
    fn from(variant: Sopsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOPSEL` reader - Secure Output Port selection:"]
pub type SopselR = crate::BitReader<Sopsel>;
impl SopselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sopsel {
        match self.bits {
            true => Sopsel::SopData1,
            false => Sopsel::SopData2,
        }
    }
    #[doc = "sop_data port reflects TRNG output (EHR_DATA)."]
    #[inline(always)]
    pub fn is_sop_data_1(&self) -> bool {
        *self == Sopsel::SopData1
    }
    #[doc = "sop_data port reflects PRNG output (RNG_READOUT). Note: Secure output is used for direct connection of the RNG block outputs to an engine input key."]
    #[inline(always)]
    pub fn is_sop_data_2(&self) -> bool {
        *self == Sopsel::SopData2
    }
}
#[doc = "Field `SOPSEL` writer - Secure Output Port selection:"]
pub type SopselW<'a, REG> = crate::BitWriter<'a, REG, Sopsel>;
impl<'a, REG> SopselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sop_data port reflects TRNG output (EHR_DATA)."]
    #[inline(always)]
    pub fn sop_data_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sopsel::SopData1)
    }
    #[doc = "sop_data port reflects PRNG output (RNG_READOUT). Note: Secure output is used for direct connection of the RNG block outputs to an engine input key."]
    #[inline(always)]
    pub fn sop_data_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sopsel::SopData2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Defines the length of the oscillator ring (= the number of inverters) out of four possible selections."]
    #[inline(always)]
    pub fn rndsrcsel(&self) -> RndsrcselR {
        RndsrcselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Secure Output Port selection:"]
    #[inline(always)]
    pub fn sopsel(&self) -> SopselR {
        SopselR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the length of the oscillator ring (= the number of inverters) out of four possible selections."]
    #[inline(always)]
    #[must_use]
    pub fn rndsrcsel(&mut self) -> RndsrcselW<TrngconfigSpec> {
        RndsrcselW::new(self, 0)
    }
    #[doc = "Bit 2 - Secure Output Port selection:"]
    #[inline(always)]
    #[must_use]
    pub fn sopsel(&mut self) -> SopselW<TrngconfigSpec> {
        SopselW::new(self, 2)
    }
}
#[doc = "This register handles TRNG configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`trngconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngconfigSpec;
impl crate::RegisterSpec for TrngconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trngconfig::R`](R) reader structure"]
impl crate::Readable for TrngconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`trngconfig::W`](W) writer structure"]
impl crate::Writable for TrngconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNGCONFIG to value 0"]
impl crate::Resettable for TrngconfigSpec {
    const RESET_VALUE: u32 = 0;
}
