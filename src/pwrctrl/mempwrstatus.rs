#[doc = "Register `MEMPWRSTATUS` reader"]
pub type R = crate::R<MempwrstatusSpec>;
#[doc = "Register `MEMPWRSTATUS` writer"]
pub type W = crate::W<MempwrstatusSpec>;
#[doc = "Power status for DTCM. Each bit corresponds to one of the TCMs. bit0=DTCM0_0, bit1=DTCM0_1, bit2=DTCM1.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pwrstdtcm {
    #[doc = "0: Do not enable power to any DTCMs"]
    None = 0,
    #[doc = "1: Only lower 8k is powered up"]
    Tcm8k = 1,
    #[doc = "3: Only lower 128k is powered up"]
    Tcm128k = 3,
    #[doc = "7: All 384k is powered up"]
    Tcm384k = 7,
}
impl From<Pwrstdtcm> for u8 {
    #[inline(always)]
    fn from(variant: Pwrstdtcm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pwrstdtcm {
    type Ux = u8;
}
impl crate::IsEnum for Pwrstdtcm {}
#[doc = "Field `PWRSTDTCM` reader - Power status for DTCM. Each bit corresponds to one of the TCMs. bit0=DTCM0_0, bit1=DTCM0_1, bit2=DTCM1."]
pub type PwrstdtcmR = crate::FieldReader<Pwrstdtcm>;
impl PwrstdtcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pwrstdtcm> {
        match self.bits {
            0 => Some(Pwrstdtcm::None),
            1 => Some(Pwrstdtcm::Tcm8k),
            3 => Some(Pwrstdtcm::Tcm128k),
            7 => Some(Pwrstdtcm::Tcm384k),
            _ => None,
        }
    }
    #[doc = "Do not enable power to any DTCMs"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pwrstdtcm::None
    }
    #[doc = "Only lower 8k is powered up"]
    #[inline(always)]
    pub fn is_tcm8k(&self) -> bool {
        *self == Pwrstdtcm::Tcm8k
    }
    #[doc = "Only lower 128k is powered up"]
    #[inline(always)]
    pub fn is_tcm128k(&self) -> bool {
        *self == Pwrstdtcm::Tcm128k
    }
    #[doc = "All 384k is powered up"]
    #[inline(always)]
    pub fn is_tcm384k(&self) -> bool {
        *self == Pwrstdtcm::Tcm384k
    }
}
#[doc = "Field `PWRSTDTCM` writer - Power status for DTCM. Each bit corresponds to one of the TCMs. bit0=DTCM0_0, bit1=DTCM0_1, bit2=DTCM1."]
pub type PwrstdtcmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pwrstdtcm>;
impl<'a, REG> PwrstdtcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not enable power to any DTCMs"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdtcm::None)
    }
    #[doc = "Only lower 8k is powered up"]
    #[inline(always)]
    pub fn tcm8k(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdtcm::Tcm8k)
    }
    #[doc = "Only lower 128k is powered up"]
    #[inline(always)]
    pub fn tcm128k(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdtcm::Tcm128k)
    }
    #[doc = "All 384k is powered up"]
    #[inline(always)]
    pub fn tcm384k(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdtcm::Tcm384k)
    }
}
#[doc = "Field `PWRSTNVM0` reader - This bit is 1 if power is supplied to NVM 0"]
pub type Pwrstnvm0R = crate::BitReader;
#[doc = "Field `PWRSTNVM0` writer - This bit is 1 if power is supplied to NVM 0"]
pub type Pwrstnvm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTCACHEB0` reader - This bit is 1 if power is supplied to Cache Bank 0"]
pub type Pwrstcacheb0R = crate::BitReader;
#[doc = "Field `PWRSTCACHEB0` writer - This bit is 1 if power is supplied to Cache Bank 0"]
pub type Pwrstcacheb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTCACHEB2` reader - This bit is 1 if power is supplied to Cache Bank 2"]
pub type Pwrstcacheb2R = crate::BitReader;
#[doc = "Field `PWRSTCACHEB2` writer - This bit is 1 if power is supplied to Cache Bank 2"]
pub type Pwrstcacheb2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Power status for DTCM. Each bit corresponds to one of the TCMs. bit0=DTCM0_0, bit1=DTCM0_1, bit2=DTCM1."]
    #[inline(always)]
    pub fn pwrstdtcm(&self) -> PwrstdtcmR {
        PwrstdtcmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to NVM 0"]
    #[inline(always)]
    pub fn pwrstnvm0(&self) -> Pwrstnvm0R {
        Pwrstnvm0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to Cache Bank 0"]
    #[inline(always)]
    pub fn pwrstcacheb0(&self) -> Pwrstcacheb0R {
        Pwrstcacheb0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to Cache Bank 2"]
    #[inline(always)]
    pub fn pwrstcacheb2(&self) -> Pwrstcacheb2R {
        Pwrstcacheb2R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Power status for DTCM. Each bit corresponds to one of the TCMs. bit0=DTCM0_0, bit1=DTCM0_1, bit2=DTCM1."]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdtcm(&mut self) -> PwrstdtcmW<MempwrstatusSpec> {
        PwrstdtcmW::new(self, 0)
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to NVM 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstnvm0(&mut self) -> Pwrstnvm0W<MempwrstatusSpec> {
        Pwrstnvm0W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to Cache Bank 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstcacheb0(&mut self) -> Pwrstcacheb0W<MempwrstatusSpec> {
        Pwrstcacheb0W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to Cache Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstcacheb2(&mut self) -> Pwrstcacheb2W<MempwrstatusSpec> {
        Pwrstcacheb2W::new(self, 5)
    }
}
#[doc = "It provides the power status for all the memory banks including- caches, nvm (0 and 1) and all the SRAM groups. The status here should reflect the enable provided by the MEMPWREN register. There may be a lag time between setting the bits in MEMPWREN register and MEMPWRSTATUS register, due to the need to cycle the power gate and isolation seqeunces to the memory banks.\n\nYou can [`read`](crate::Reg::read) this register and get [`mempwrstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mempwrstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MempwrstatusSpec;
impl crate::RegisterSpec for MempwrstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mempwrstatus::R`](R) reader structure"]
impl crate::Readable for MempwrstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mempwrstatus::W`](W) writer structure"]
impl crate::Writable for MempwrstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMPWRSTATUS to value 0x3f"]
impl crate::Resettable for MempwrstatusSpec {
    const RESET_VALUE: u32 = 0x3f;
}
