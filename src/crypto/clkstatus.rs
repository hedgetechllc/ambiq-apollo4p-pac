#[doc = "Register `CLKSTATUS` reader"]
pub type R = crate::R<ClkstatusSpec>;
#[doc = "Register `CLKSTATUS` writer"]
pub type W = crate::W<ClkstatusSpec>;
#[doc = "Status of AES clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesclkstatus {
    #[doc = "1: the AES clock is enabled."]
    ClkE = 1,
    #[doc = "0: the AES clock is disabled."]
    ClkD = 0,
}
impl From<Aesclkstatus> for bool {
    #[inline(always)]
    fn from(variant: Aesclkstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESCLKSTATUS` reader - Status of AES clock enable."]
pub type AesclkstatusR = crate::BitReader<Aesclkstatus>;
impl AesclkstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesclkstatus {
        match self.bits {
            true => Aesclkstatus::ClkE,
            false => Aesclkstatus::ClkD,
        }
    }
    #[doc = "the AES clock is enabled."]
    #[inline(always)]
    pub fn is_clk_e(&self) -> bool {
        *self == Aesclkstatus::ClkE
    }
    #[doc = "the AES clock is disabled."]
    #[inline(always)]
    pub fn is_clk_d(&self) -> bool {
        *self == Aesclkstatus::ClkD
    }
}
#[doc = "Field `AESCLKSTATUS` writer - Status of AES clock enable."]
pub type AesclkstatusW<'a, REG> = crate::BitWriter<'a, REG, Aesclkstatus>;
impl<'a, REG> AesclkstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the AES clock is enabled."]
    #[inline(always)]
    pub fn clk_e(self) -> &'a mut crate::W<REG> {
        self.variant(Aesclkstatus::ClkE)
    }
    #[doc = "the AES clock is disabled."]
    #[inline(always)]
    pub fn clk_d(self) -> &'a mut crate::W<REG> {
        self.variant(Aesclkstatus::ClkD)
    }
}
#[doc = "Status of HASH clock clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hashclkstatus {
    #[doc = "1: the HASH clock is enabled."]
    HashE = 1,
    #[doc = "0: the HASH clock is disabled."]
    HashD = 0,
}
impl From<Hashclkstatus> for bool {
    #[inline(always)]
    fn from(variant: Hashclkstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASHCLKSTATUS` reader - Status of HASH clock clock enable."]
pub type HashclkstatusR = crate::BitReader<Hashclkstatus>;
impl HashclkstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hashclkstatus {
        match self.bits {
            true => Hashclkstatus::HashE,
            false => Hashclkstatus::HashD,
        }
    }
    #[doc = "the HASH clock is enabled."]
    #[inline(always)]
    pub fn is_hash_e(&self) -> bool {
        *self == Hashclkstatus::HashE
    }
    #[doc = "the HASH clock is disabled."]
    #[inline(always)]
    pub fn is_hash_d(&self) -> bool {
        *self == Hashclkstatus::HashD
    }
}
#[doc = "Field `HASHCLKSTATUS` writer - Status of HASH clock clock enable."]
pub type HashclkstatusW<'a, REG> = crate::BitWriter<'a, REG, Hashclkstatus>;
impl<'a, REG> HashclkstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the HASH clock is enabled."]
    #[inline(always)]
    pub fn hash_e(self) -> &'a mut crate::W<REG> {
        self.variant(Hashclkstatus::HashE)
    }
    #[doc = "the HASH clock is disabled."]
    #[inline(always)]
    pub fn hash_d(self) -> &'a mut crate::W<REG> {
        self.variant(Hashclkstatus::HashD)
    }
}
#[doc = "Status of PKA clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pkaclkstatus {
    #[doc = "1: the PKA clock is enabled."]
    PkaE = 1,
    #[doc = "0: the PKA clock is disabled."]
    PkaD = 0,
}
impl From<Pkaclkstatus> for bool {
    #[inline(always)]
    fn from(variant: Pkaclkstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKACLKSTATUS` reader - Status of PKA clock enable."]
pub type PkaclkstatusR = crate::BitReader<Pkaclkstatus>;
impl PkaclkstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pkaclkstatus {
        match self.bits {
            true => Pkaclkstatus::PkaE,
            false => Pkaclkstatus::PkaD,
        }
    }
    #[doc = "the PKA clock is enabled."]
    #[inline(always)]
    pub fn is_pka_e(&self) -> bool {
        *self == Pkaclkstatus::PkaE
    }
    #[doc = "the PKA clock is disabled."]
    #[inline(always)]
    pub fn is_pka_d(&self) -> bool {
        *self == Pkaclkstatus::PkaD
    }
}
#[doc = "Field `PKACLKSTATUS` writer - Status of PKA clock enable."]
pub type PkaclkstatusW<'a, REG> = crate::BitWriter<'a, REG, Pkaclkstatus>;
impl<'a, REG> PkaclkstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the PKA clock is enabled."]
    #[inline(always)]
    pub fn pka_e(self) -> &'a mut crate::W<REG> {
        self.variant(Pkaclkstatus::PkaE)
    }
    #[doc = "the PKA clock is disabled."]
    #[inline(always)]
    pub fn pka_d(self) -> &'a mut crate::W<REG> {
        self.variant(Pkaclkstatus::PkaD)
    }
}
#[doc = "Status of CHACHA clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chachaclkstatus {
    #[doc = "1: the CHACHA clock is enabled."]
    ChachaE = 1,
    #[doc = "0: the CHACHA clock is disabled."]
    ChachaD = 0,
}
impl From<Chachaclkstatus> for bool {
    #[inline(always)]
    fn from(variant: Chachaclkstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHACLKSTATUS` reader - Status of CHACHA clock enable."]
pub type ChachaclkstatusR = crate::BitReader<Chachaclkstatus>;
impl ChachaclkstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chachaclkstatus {
        match self.bits {
            true => Chachaclkstatus::ChachaE,
            false => Chachaclkstatus::ChachaD,
        }
    }
    #[doc = "the CHACHA clock is enabled."]
    #[inline(always)]
    pub fn is_chacha_e(&self) -> bool {
        *self == Chachaclkstatus::ChachaE
    }
    #[doc = "the CHACHA clock is disabled."]
    #[inline(always)]
    pub fn is_chacha_d(&self) -> bool {
        *self == Chachaclkstatus::ChachaD
    }
}
#[doc = "Field `CHACHACLKSTATUS` writer - Status of CHACHA clock enable."]
pub type ChachaclkstatusW<'a, REG> = crate::BitWriter<'a, REG, Chachaclkstatus>;
impl<'a, REG> ChachaclkstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the CHACHA clock is enabled."]
    #[inline(always)]
    pub fn chacha_e(self) -> &'a mut crate::W<REG> {
        self.variant(Chachaclkstatus::ChachaE)
    }
    #[doc = "the CHACHA clock is disabled."]
    #[inline(always)]
    pub fn chacha_d(self) -> &'a mut crate::W<REG> {
        self.variant(Chachaclkstatus::ChachaD)
    }
}
#[doc = "Status of DMA clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaclkstatus {
    #[doc = "1: the DMA clock is enabled."]
    DmaE = 1,
    #[doc = "0: the DMA clock is disabled."]
    DmaD = 0,
}
impl From<Dmaclkstatus> for bool {
    #[inline(always)]
    fn from(variant: Dmaclkstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACLKSTATUS` reader - Status of DMA clock enable."]
pub type DmaclkstatusR = crate::BitReader<Dmaclkstatus>;
impl DmaclkstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaclkstatus {
        match self.bits {
            true => Dmaclkstatus::DmaE,
            false => Dmaclkstatus::DmaD,
        }
    }
    #[doc = "the DMA clock is enabled."]
    #[inline(always)]
    pub fn is_dma_e(&self) -> bool {
        *self == Dmaclkstatus::DmaE
    }
    #[doc = "the DMA clock is disabled."]
    #[inline(always)]
    pub fn is_dma_d(&self) -> bool {
        *self == Dmaclkstatus::DmaD
    }
}
#[doc = "Field `DMACLKSTATUS` writer - Status of DMA clock enable."]
pub type DmaclkstatusW<'a, REG> = crate::BitWriter<'a, REG, Dmaclkstatus>;
impl<'a, REG> DmaclkstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the DMA clock is enabled."]
    #[inline(always)]
    pub fn dma_e(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaclkstatus::DmaE)
    }
    #[doc = "the DMA clock is disabled."]
    #[inline(always)]
    pub fn dma_d(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaclkstatus::DmaD)
    }
}
impl R {
    #[doc = "Bit 0 - Status of AES clock enable."]
    #[inline(always)]
    pub fn aesclkstatus(&self) -> AesclkstatusR {
        AesclkstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Status of HASH clock clock enable."]
    #[inline(always)]
    pub fn hashclkstatus(&self) -> HashclkstatusR {
        HashclkstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of PKA clock enable."]
    #[inline(always)]
    pub fn pkaclkstatus(&self) -> PkaclkstatusR {
        PkaclkstatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of CHACHA clock enable."]
    #[inline(always)]
    pub fn chachaclkstatus(&self) -> ChachaclkstatusR {
        ChachaclkstatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of DMA clock enable."]
    #[inline(always)]
    pub fn dmaclkstatus(&self) -> DmaclkstatusR {
        DmaclkstatusR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status of AES clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn aesclkstatus(&mut self) -> AesclkstatusW<ClkstatusSpec> {
        AesclkstatusW::new(self, 0)
    }
    #[doc = "Bit 2 - Status of HASH clock clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn hashclkstatus(&mut self) -> HashclkstatusW<ClkstatusSpec> {
        HashclkstatusW::new(self, 2)
    }
    #[doc = "Bit 3 - Status of PKA clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn pkaclkstatus(&mut self) -> PkaclkstatusW<ClkstatusSpec> {
        PkaclkstatusW::new(self, 3)
    }
    #[doc = "Bit 7 - Status of CHACHA clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn chachaclkstatus(&mut self) -> ChachaclkstatusW<ClkstatusSpec> {
        ChachaclkstatusW::new(self, 7)
    }
    #[doc = "Bit 8 - Status of DMA clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn dmaclkstatus(&mut self) -> DmaclkstatusW<ClkstatusSpec> {
        DmaclkstatusW::new(self, 8)
    }
}
#[doc = "The CryptoCell clocks status register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkstatusSpec;
impl crate::RegisterSpec for ClkstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkstatus::R`](R) reader structure"]
impl crate::Readable for ClkstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`clkstatus::W`](W) writer structure"]
impl crate::Writable for ClkstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSTATUS to value 0"]
impl crate::Resettable for ClkstatusSpec {
    const RESET_VALUE: u32 = 0;
}
