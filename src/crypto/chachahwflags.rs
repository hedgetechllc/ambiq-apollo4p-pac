#[doc = "Register `CHACHAHWFLAGS` reader"]
pub type R = crate::R<ChachahwflagsSpec>;
#[doc = "Register `CHACHAHWFLAGS` writer"]
pub type W = crate::W<ChachahwflagsSpec>;
#[doc = "If this flag is set, the Salsa_ChaCha engine include ChaCha implementation:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chachaexists {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable."]
    Enable = 1,
}
impl From<Chachaexists> for bool {
    #[inline(always)]
    fn from(variant: Chachaexists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHAEXISTS` reader - If this flag is set, the Salsa_ChaCha engine include ChaCha implementation:"]
pub type ChachaexistsR = crate::BitReader<Chachaexists>;
impl ChachaexistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chachaexists {
        match self.bits {
            false => Chachaexists::Disable,
            true => Chachaexists::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Chachaexists::Disable
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Chachaexists::Enable
    }
}
#[doc = "Field `CHACHAEXISTS` writer - If this flag is set, the Salsa_ChaCha engine include ChaCha implementation:"]
pub type ChachaexistsW<'a, REG> = crate::BitWriter<'a, REG, Chachaexists>;
impl<'a, REG> ChachaexistsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachaexists::Disable)
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Chachaexists::Enable)
    }
}
#[doc = "If this flag is set, the Salsa_ChaCha engine include Salsa implementation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Salsaexists {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable."]
    Enable = 1,
}
impl From<Salsaexists> for bool {
    #[inline(always)]
    fn from(variant: Salsaexists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SALSAEXISTS` reader - If this flag is set, the Salsa_ChaCha engine include Salsa implementation:"]
pub type SalsaexistsR = crate::BitReader<Salsaexists>;
impl SalsaexistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Salsaexists {
        match self.bits {
            false => Salsaexists::Disable,
            true => Salsaexists::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Salsaexists::Disable
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Salsaexists::Enable
    }
}
#[doc = "Field `SALSAEXISTS` writer - If this flag is set, the Salsa_ChaCha engine include Salsa implementation:"]
pub type SalsaexistsW<'a, REG> = crate::BitWriter<'a, REG, Salsaexists>;
impl<'a, REG> SalsaexistsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Salsaexists::Disable)
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Salsaexists::Enable)
    }
}
#[doc = "If this flag is set, the next matrix calculated when the current one is written to data output path (same flag for Salsa core):\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fastchacha {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable."]
    Enable = 1,
}
impl From<Fastchacha> for bool {
    #[inline(always)]
    fn from(variant: Fastchacha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTCHACHA` reader - If this flag is set, the next matrix calculated when the current one is written to data output path (same flag for Salsa core):"]
pub type FastchachaR = crate::BitReader<Fastchacha>;
impl FastchachaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fastchacha {
        match self.bits {
            false => Fastchacha::Disable,
            true => Fastchacha::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fastchacha::Disable
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fastchacha::Enable
    }
}
#[doc = "Field `FASTCHACHA` writer - If this flag is set, the next matrix calculated when the current one is written to data output path (same flag for Salsa core):"]
pub type FastchachaW<'a, REG> = crate::BitWriter<'a, REG, Fastchacha>;
impl<'a, REG> FastchachaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fastchacha::Disable)
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fastchacha::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - If this flag is set, the Salsa_ChaCha engine include ChaCha implementation:"]
    #[inline(always)]
    pub fn chachaexists(&self) -> ChachaexistsR {
        ChachaexistsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this flag is set, the Salsa_ChaCha engine include Salsa implementation:"]
    #[inline(always)]
    pub fn salsaexists(&self) -> SalsaexistsR {
        SalsaexistsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this flag is set, the next matrix calculated when the current one is written to data output path (same flag for Salsa core):"]
    #[inline(always)]
    pub fn fastchacha(&self) -> FastchachaR {
        FastchachaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this flag is set, the Salsa_ChaCha engine include ChaCha implementation:"]
    #[inline(always)]
    #[must_use]
    pub fn chachaexists(&mut self) -> ChachaexistsW<ChachahwflagsSpec> {
        ChachaexistsW::new(self, 0)
    }
    #[doc = "Bit 1 - If this flag is set, the Salsa_ChaCha engine include Salsa implementation:"]
    #[inline(always)]
    #[must_use]
    pub fn salsaexists(&mut self) -> SalsaexistsW<ChachahwflagsSpec> {
        SalsaexistsW::new(self, 1)
    }
    #[doc = "Bit 2 - If this flag is set, the next matrix calculated when the current one is written to data output path (same flag for Salsa core):"]
    #[inline(always)]
    #[must_use]
    pub fn fastchacha(&mut self) -> FastchachaW<ChachahwflagsSpec> {
        FastchachaW::new(self, 2)
    }
}
#[doc = "This register holds the pre-synthesis HW flag configuration of the CHACHA_SALSA engine\n\nYou can [`read`](crate::Reg::read) this register and get [`chachahwflags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachahwflags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachahwflagsSpec;
impl crate::RegisterSpec for ChachahwflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachahwflags::R`](R) reader structure"]
impl crate::Readable for ChachahwflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`chachahwflags::W`](W) writer structure"]
impl crate::Writable for ChachahwflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHAHWFLAGS to value 0x01"]
impl crate::Resettable for ChachahwflagsSpec {
    const RESET_VALUE: u32 = 0x01;
}
