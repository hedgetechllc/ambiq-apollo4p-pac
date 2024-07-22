#[doc = "Register `MEMPWREN` reader"]
pub type R = crate::R<MempwrenSpec>;
#[doc = "Register `MEMPWREN` writer"]
pub type W = crate::W<MempwrenSpec>;
#[doc = "Power up DTCM\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pwrendtcm {
    #[doc = "0: Do not enable power to any DTCMs"]
    None = 0,
    #[doc = "1: Power ON only lower 8k"]
    Tcm8k = 1,
    #[doc = "3: Power ON only lower 128k"]
    Tcm128k = 3,
    #[doc = "7: Power ON 384k"]
    Tcm384k = 7,
}
impl From<Pwrendtcm> for u8 {
    #[inline(always)]
    fn from(variant: Pwrendtcm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pwrendtcm {
    type Ux = u8;
}
impl crate::IsEnum for Pwrendtcm {}
#[doc = "Field `PWRENDTCM` reader - Power up DTCM"]
pub type PwrendtcmR = crate::FieldReader<Pwrendtcm>;
impl PwrendtcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pwrendtcm> {
        match self.bits {
            0 => Some(Pwrendtcm::None),
            1 => Some(Pwrendtcm::Tcm8k),
            3 => Some(Pwrendtcm::Tcm128k),
            7 => Some(Pwrendtcm::Tcm384k),
            _ => None,
        }
    }
    #[doc = "Do not enable power to any DTCMs"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pwrendtcm::None
    }
    #[doc = "Power ON only lower 8k"]
    #[inline(always)]
    pub fn is_tcm8k(&self) -> bool {
        *self == Pwrendtcm::Tcm8k
    }
    #[doc = "Power ON only lower 128k"]
    #[inline(always)]
    pub fn is_tcm128k(&self) -> bool {
        *self == Pwrendtcm::Tcm128k
    }
    #[doc = "Power ON 384k"]
    #[inline(always)]
    pub fn is_tcm384k(&self) -> bool {
        *self == Pwrendtcm::Tcm384k
    }
}
#[doc = "Field `PWRENDTCM` writer - Power up DTCM"]
pub type PwrendtcmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pwrendtcm>;
impl<'a, REG> PwrendtcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not enable power to any DTCMs"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendtcm::None)
    }
    #[doc = "Power ON only lower 8k"]
    #[inline(always)]
    pub fn tcm8k(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendtcm::Tcm8k)
    }
    #[doc = "Power ON only lower 128k"]
    #[inline(always)]
    pub fn tcm128k(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendtcm::Tcm128k)
    }
    #[doc = "Power ON 384k"]
    #[inline(always)]
    pub fn tcm384k(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendtcm::Tcm384k)
    }
}
#[doc = "Power up NVM0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrennvm0 {
    #[doc = "1: Power up NVM0"]
    En = 1,
    #[doc = "0: Power down NVM0"]
    Dis = 0,
}
impl From<Pwrennvm0> for bool {
    #[inline(always)]
    fn from(variant: Pwrennvm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENNVM0` reader - Power up NVM0"]
pub type Pwrennvm0R = crate::BitReader<Pwrennvm0>;
impl Pwrennvm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrennvm0 {
        match self.bits {
            true => Pwrennvm0::En,
            false => Pwrennvm0::Dis,
        }
    }
    #[doc = "Power up NVM0"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrennvm0::En
    }
    #[doc = "Power down NVM0"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrennvm0::Dis
    }
}
#[doc = "Field `PWRENNVM0` writer - Power up NVM0"]
pub type Pwrennvm0W<'a, REG> = crate::BitWriter<'a, REG, Pwrennvm0>;
impl<'a, REG> Pwrennvm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up NVM0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrennvm0::En)
    }
    #[doc = "Power down NVM0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrennvm0::Dis)
    }
}
#[doc = "Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrencacheb0 {
    #[doc = "1: Power up Cache Bank 0"]
    En = 1,
    #[doc = "0: Power down Cache Bank 0"]
    Dis = 0,
}
impl From<Pwrencacheb0> for bool {
    #[inline(always)]
    fn from(variant: Pwrencacheb0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENCACHEB0` reader - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
pub type Pwrencacheb0R = crate::BitReader<Pwrencacheb0>;
impl Pwrencacheb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrencacheb0 {
        match self.bits {
            true => Pwrencacheb0::En,
            false => Pwrencacheb0::Dis,
        }
    }
    #[doc = "Power up Cache Bank 0"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrencacheb0::En
    }
    #[doc = "Power down Cache Bank 0"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrencacheb0::Dis
    }
}
#[doc = "Field `PWRENCACHEB0` writer - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
pub type Pwrencacheb0W<'a, REG> = crate::BitWriter<'a, REG, Pwrencacheb0>;
impl<'a, REG> Pwrencacheb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up Cache Bank 0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrencacheb0::En)
    }
    #[doc = "Power down Cache Bank 0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrencacheb0::Dis)
    }
}
#[doc = "Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrencacheb2 {
    #[doc = "1: Power up Cache Bank 2"]
    En = 1,
    #[doc = "0: Power down Cache Bank 2"]
    Dis = 0,
}
impl From<Pwrencacheb2> for bool {
    #[inline(always)]
    fn from(variant: Pwrencacheb2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENCACHEB2` reader - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
pub type Pwrencacheb2R = crate::BitReader<Pwrencacheb2>;
impl Pwrencacheb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrencacheb2 {
        match self.bits {
            true => Pwrencacheb2::En,
            false => Pwrencacheb2::Dis,
        }
    }
    #[doc = "Power up Cache Bank 2"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrencacheb2::En
    }
    #[doc = "Power down Cache Bank 2"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrencacheb2::Dis
    }
}
#[doc = "Field `PWRENCACHEB2` writer - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
pub type Pwrencacheb2W<'a, REG> = crate::BitWriter<'a, REG, Pwrencacheb2>;
impl<'a, REG> Pwrencacheb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up Cache Bank 2"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrencacheb2::En)
    }
    #[doc = "Power down Cache Bank 2"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrencacheb2::Dis)
    }
}
impl R {
    #[doc = "Bits 0:2 - Power up DTCM"]
    #[inline(always)]
    pub fn pwrendtcm(&self) -> PwrendtcmR {
        PwrendtcmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Power up NVM0"]
    #[inline(always)]
    pub fn pwrennvm0(&self) -> Pwrennvm0R {
        Pwrennvm0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn pwrencacheb0(&self) -> Pwrencacheb0R {
        Pwrencacheb0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn pwrencacheb2(&self) -> Pwrencacheb2R {
        Pwrencacheb2R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Power up DTCM"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendtcm(&mut self) -> PwrendtcmW<MempwrenSpec> {
        PwrendtcmW::new(self, 0)
    }
    #[doc = "Bit 3 - Power up NVM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwrennvm0(&mut self) -> Pwrennvm0W<MempwrenSpec> {
        Pwrennvm0W::new(self, 3)
    }
    #[doc = "Bit 4 - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank0, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    #[must_use]
    pub fn pwrencacheb0(&mut self) -> Pwrencacheb0W<MempwrenSpec> {
        Pwrencacheb0W::new(self, 4)
    }
    #[doc = "Bit 5 - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank2, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    #[must_use]
    pub fn pwrencacheb2(&mut self) -> Pwrencacheb2W<MempwrenSpec> {
        Pwrencacheb2W::new(self, 5)
    }
}
#[doc = "This register enables the individual banks for the memories. When set, power will be enabled to the banks. This register works in conjunction with the MEMRETCFG register. If this register is not set, then power will always be disabled to the memory bank.\n\nYou can [`read`](crate::Reg::read) this register and get [`mempwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mempwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MempwrenSpec;
impl crate::RegisterSpec for MempwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mempwren::R`](R) reader structure"]
impl crate::Readable for MempwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`mempwren::W`](W) writer structure"]
impl crate::Writable for MempwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMPWREN to value 0x3f"]
impl crate::Resettable for MempwrenSpec {
    const RESET_VALUE: u32 = 0x3f;
}
