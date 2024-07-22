#[doc = "Register `SSRAMPWREN` reader"]
pub type R = crate::R<SsrampwrenSpec>;
#[doc = "Register `SSRAMPWREN` writer"]
pub type W = crate::W<SsrampwrenSpec>;
#[doc = "Power up SRAM groups\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pwrenssram {
    #[doc = "0: Do not power ON any of the SRAM banks"]
    None = 0,
    #[doc = "1: Power ON only SRAM0 group (lower 1M)"]
    Group0 = 1,
    #[doc = "2: Power ON only SRAM1 group (upper 1M)"]
    Group1 = 2,
    #[doc = "3: All shared SRAM banks (SSRAM0 1M + SSRAM1 1M) powered ON"]
    All = 3,
}
impl From<Pwrenssram> for u8 {
    #[inline(always)]
    fn from(variant: Pwrenssram) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pwrenssram {
    type Ux = u8;
}
impl crate::IsEnum for Pwrenssram {}
#[doc = "Field `PWRENSSRAM` reader - Power up SRAM groups"]
pub type PwrenssramR = crate::FieldReader<Pwrenssram>;
impl PwrenssramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenssram {
        match self.bits {
            0 => Pwrenssram::None,
            1 => Pwrenssram::Group0,
            2 => Pwrenssram::Group1,
            3 => Pwrenssram::All,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not power ON any of the SRAM banks"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pwrenssram::None
    }
    #[doc = "Power ON only SRAM0 group (lower 1M)"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        *self == Pwrenssram::Group0
    }
    #[doc = "Power ON only SRAM1 group (upper 1M)"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == Pwrenssram::Group1
    }
    #[doc = "All shared SRAM banks (SSRAM0 1M + SSRAM1 1M) powered ON"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Pwrenssram::All
    }
}
#[doc = "Field `PWRENSSRAM` writer - Power up SRAM groups"]
pub type PwrenssramW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pwrenssram, crate::Safe>;
impl<'a, REG> PwrenssramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not power ON any of the SRAM banks"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenssram::None)
    }
    #[doc = "Power ON only SRAM0 group (lower 1M)"]
    #[inline(always)]
    pub fn group0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenssram::Group0)
    }
    #[doc = "Power ON only SRAM1 group (upper 1M)"]
    #[inline(always)]
    pub fn group1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenssram::Group1)
    }
    #[doc = "All shared SRAM banks (SSRAM0 1M + SSRAM1 1M) powered ON"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenssram::All)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power up SRAM groups"]
    #[inline(always)]
    pub fn pwrenssram(&self) -> PwrenssramR {
        PwrenssramR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power up SRAM groups"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenssram(&mut self) -> PwrenssramW<SsrampwrenSpec> {
        PwrenssramW::new(self, 0)
    }
}
#[doc = "This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the SSRAMRETCFG register. If this register is not set, then power will always be disabled to the memory bank.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrampwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrampwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrampwrenSpec;
impl crate::RegisterSpec for SsrampwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssrampwren::R`](R) reader structure"]
impl crate::Readable for SsrampwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`ssrampwren::W`](W) writer structure"]
impl crate::Writable for SsrampwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSRAMPWREN to value 0"]
impl crate::Resettable for SsrampwrenSpec {
    const RESET_VALUE: u32 = 0;
}
