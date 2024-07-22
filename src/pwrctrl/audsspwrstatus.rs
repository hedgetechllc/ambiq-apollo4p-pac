#[doc = "Register `AUDSSPWRSTATUS` reader"]
pub type R = crate::R<AudsspwrstatusSpec>;
#[doc = "Register `AUDSSPWRSTATUS` writer"]
pub type W = crate::W<AudsspwrstatusSpec>;
#[doc = "Power Status Audio Record block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstaudrec {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstaudrec> for bool {
    #[inline(always)]
    fn from(variant: Pwrstaudrec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTAUDREC` reader - Power Status Audio Record block"]
pub type PwrstaudrecR = crate::BitReader<Pwrstaudrec>;
impl PwrstaudrecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstaudrec {
        match self.bits {
            true => Pwrstaudrec::On,
            false => Pwrstaudrec::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstaudrec::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstaudrec::Off
    }
}
#[doc = "Field `PWRSTAUDREC` writer - Power Status Audio Record block"]
pub type PwrstaudrecW<'a, REG> = crate::BitWriter<'a, REG, Pwrstaudrec>;
impl<'a, REG> PwrstaudrecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstaudrec::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstaudrec::Off)
    }
}
#[doc = "Power Status Audio Playback block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstaudpb {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstaudpb> for bool {
    #[inline(always)]
    fn from(variant: Pwrstaudpb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTAUDPB` reader - Power Status Audio Playback block"]
pub type PwrstaudpbR = crate::BitReader<Pwrstaudpb>;
impl PwrstaudpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstaudpb {
        match self.bits {
            true => Pwrstaudpb::On,
            false => Pwrstaudpb::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstaudpb::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstaudpb::Off
    }
}
#[doc = "Field `PWRSTAUDPB` writer - Power Status Audio Playback block"]
pub type PwrstaudpbW<'a, REG> = crate::BitWriter<'a, REG, Pwrstaudpb>;
impl<'a, REG> PwrstaudpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstaudpb::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstaudpb::Off)
    }
}
#[doc = "Power Status audio subsystem PDM0 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstpdm0 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstpdm0> for bool {
    #[inline(always)]
    fn from(variant: Pwrstpdm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTPDM0` reader - Power Status audio subsystem PDM0 domain"]
pub type Pwrstpdm0R = crate::BitReader<Pwrstpdm0>;
impl Pwrstpdm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstpdm0 {
        match self.bits {
            true => Pwrstpdm0::On,
            false => Pwrstpdm0::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstpdm0::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstpdm0::Off
    }
}
#[doc = "Field `PWRSTPDM0` writer - Power Status audio subsystem PDM0 domain"]
pub type Pwrstpdm0W<'a, REG> = crate::BitWriter<'a, REG, Pwrstpdm0>;
impl<'a, REG> Pwrstpdm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstpdm0::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstpdm0::Off)
    }
}
#[doc = "Power Status audio subsystem PDM1 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstpdm1 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstpdm1> for bool {
    #[inline(always)]
    fn from(variant: Pwrstpdm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTPDM1` reader - Power Status audio subsystem PDM1 domain"]
pub type Pwrstpdm1R = crate::BitReader<Pwrstpdm1>;
impl Pwrstpdm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstpdm1 {
        match self.bits {
            true => Pwrstpdm1::On,
            false => Pwrstpdm1::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstpdm1::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstpdm1::Off
    }
}
#[doc = "Field `PWRSTPDM1` writer - Power Status audio subsystem PDM1 domain"]
pub type Pwrstpdm1W<'a, REG> = crate::BitWriter<'a, REG, Pwrstpdm1>;
impl<'a, REG> Pwrstpdm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstpdm1::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstpdm1::Off)
    }
}
#[doc = "Power Status audio subsystem PDM2 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstpdm2 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstpdm2> for bool {
    #[inline(always)]
    fn from(variant: Pwrstpdm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTPDM2` reader - Power Status audio subsystem PDM2 domain"]
pub type Pwrstpdm2R = crate::BitReader<Pwrstpdm2>;
impl Pwrstpdm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstpdm2 {
        match self.bits {
            true => Pwrstpdm2::On,
            false => Pwrstpdm2::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstpdm2::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstpdm2::Off
    }
}
#[doc = "Field `PWRSTPDM2` writer - Power Status audio subsystem PDM2 domain"]
pub type Pwrstpdm2W<'a, REG> = crate::BitWriter<'a, REG, Pwrstpdm2>;
impl<'a, REG> Pwrstpdm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstpdm2::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstpdm2::Off)
    }
}
#[doc = "Power Status audio subsystem PDM3 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstpdm3 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstpdm3> for bool {
    #[inline(always)]
    fn from(variant: Pwrstpdm3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTPDM3` reader - Power Status audio subsystem PDM3 domain"]
pub type Pwrstpdm3R = crate::BitReader<Pwrstpdm3>;
impl Pwrstpdm3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstpdm3 {
        match self.bits {
            true => Pwrstpdm3::On,
            false => Pwrstpdm3::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstpdm3::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstpdm3::Off
    }
}
#[doc = "Field `PWRSTPDM3` writer - Power Status audio subsystem PDM3 domain"]
pub type Pwrstpdm3W<'a, REG> = crate::BitWriter<'a, REG, Pwrstpdm3>;
impl<'a, REG> Pwrstpdm3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstpdm3::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstpdm3::Off)
    }
}
#[doc = "Power Status audio subsystem I2S0 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrsti2s0 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrsti2s0> for bool {
    #[inline(always)]
    fn from(variant: Pwrsti2s0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTI2S0` reader - Power Status audio subsystem I2S0 domain"]
pub type Pwrsti2s0R = crate::BitReader<Pwrsti2s0>;
impl Pwrsti2s0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrsti2s0 {
        match self.bits {
            true => Pwrsti2s0::On,
            false => Pwrsti2s0::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrsti2s0::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrsti2s0::Off
    }
}
#[doc = "Field `PWRSTI2S0` writer - Power Status audio subsystem I2S0 domain"]
pub type Pwrsti2s0W<'a, REG> = crate::BitWriter<'a, REG, Pwrsti2s0>;
impl<'a, REG> Pwrsti2s0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsti2s0::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsti2s0::Off)
    }
}
#[doc = "Power Status audio subsystem I2S1 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrsti2s1 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrsti2s1> for bool {
    #[inline(always)]
    fn from(variant: Pwrsti2s1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTI2S1` reader - Power Status audio subsystem I2S1 domain"]
pub type Pwrsti2s1R = crate::BitReader<Pwrsti2s1>;
impl Pwrsti2s1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrsti2s1 {
        match self.bits {
            true => Pwrsti2s1::On,
            false => Pwrsti2s1::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrsti2s1::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrsti2s1::Off
    }
}
#[doc = "Field `PWRSTI2S1` writer - Power Status audio subsystem I2S1 domain"]
pub type Pwrsti2s1W<'a, REG> = crate::BitWriter<'a, REG, Pwrsti2s1>;
impl<'a, REG> Pwrsti2s1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsti2s1::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsti2s1::Off)
    }
}
#[doc = "Power Status audio subsystem ADC domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstaudadc {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstaudadc> for bool {
    #[inline(always)]
    fn from(variant: Pwrstaudadc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTAUDADC` reader - Power Status audio subsystem ADC domain"]
pub type PwrstaudadcR = crate::BitReader<Pwrstaudadc>;
impl PwrstaudadcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstaudadc {
        match self.bits {
            true => Pwrstaudadc::On,
            false => Pwrstaudadc::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstaudadc::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstaudadc::Off
    }
}
#[doc = "Field `PWRSTAUDADC` writer - Power Status audio subsystem ADC domain"]
pub type PwrstaudadcW<'a, REG> = crate::BitWriter<'a, REG, Pwrstaudadc>;
impl<'a, REG> PwrstaudadcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstaudadc::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstaudadc::Off)
    }
}
#[doc = "Power Status DSPA subsystem\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstdspa {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstdspa> for bool {
    #[inline(always)]
    fn from(variant: Pwrstdspa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTDSPA` reader - Power Status DSPA subsystem"]
pub type PwrstdspaR = crate::BitReader<Pwrstdspa>;
impl PwrstdspaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstdspa {
        match self.bits {
            true => Pwrstdspa::On,
            false => Pwrstdspa::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstdspa::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstdspa::Off
    }
}
#[doc = "Field `PWRSTDSPA` writer - Power Status DSPA subsystem"]
pub type PwrstdspaW<'a, REG> = crate::BitWriter<'a, REG, Pwrstdspa>;
impl<'a, REG> PwrstdspaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdspa::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdspa::Off)
    }
}
impl R {
    #[doc = "Bit 0 - Power Status Audio Record block"]
    #[inline(always)]
    pub fn pwrstaudrec(&self) -> PwrstaudrecR {
        PwrstaudrecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Status Audio Playback block"]
    #[inline(always)]
    pub fn pwrstaudpb(&self) -> PwrstaudpbR {
        PwrstaudpbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Status audio subsystem PDM0 domain"]
    #[inline(always)]
    pub fn pwrstpdm0(&self) -> Pwrstpdm0R {
        Pwrstpdm0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Status audio subsystem PDM1 domain"]
    #[inline(always)]
    pub fn pwrstpdm1(&self) -> Pwrstpdm1R {
        Pwrstpdm1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Status audio subsystem PDM2 domain"]
    #[inline(always)]
    pub fn pwrstpdm2(&self) -> Pwrstpdm2R {
        Pwrstpdm2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power Status audio subsystem PDM3 domain"]
    #[inline(always)]
    pub fn pwrstpdm3(&self) -> Pwrstpdm3R {
        Pwrstpdm3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power Status audio subsystem I2S0 domain"]
    #[inline(always)]
    pub fn pwrsti2s0(&self) -> Pwrsti2s0R {
        Pwrsti2s0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power Status audio subsystem I2S1 domain"]
    #[inline(always)]
    pub fn pwrsti2s1(&self) -> Pwrsti2s1R {
        Pwrsti2s1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Power Status audio subsystem ADC domain"]
    #[inline(always)]
    pub fn pwrstaudadc(&self) -> PwrstaudadcR {
        PwrstaudadcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power Status DSPA subsystem"]
    #[inline(always)]
    pub fn pwrstdspa(&self) -> PwrstdspaR {
        PwrstdspaR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Status Audio Record block"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstaudrec(&mut self) -> PwrstaudrecW<AudsspwrstatusSpec> {
        PwrstaudrecW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Status Audio Playback block"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstaudpb(&mut self) -> PwrstaudpbW<AudsspwrstatusSpec> {
        PwrstaudpbW::new(self, 1)
    }
    #[doc = "Bit 2 - Power Status audio subsystem PDM0 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstpdm0(&mut self) -> Pwrstpdm0W<AudsspwrstatusSpec> {
        Pwrstpdm0W::new(self, 2)
    }
    #[doc = "Bit 3 - Power Status audio subsystem PDM1 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstpdm1(&mut self) -> Pwrstpdm1W<AudsspwrstatusSpec> {
        Pwrstpdm1W::new(self, 3)
    }
    #[doc = "Bit 4 - Power Status audio subsystem PDM2 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstpdm2(&mut self) -> Pwrstpdm2W<AudsspwrstatusSpec> {
        Pwrstpdm2W::new(self, 4)
    }
    #[doc = "Bit 5 - Power Status audio subsystem PDM3 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstpdm3(&mut self) -> Pwrstpdm3W<AudsspwrstatusSpec> {
        Pwrstpdm3W::new(self, 5)
    }
    #[doc = "Bit 6 - Power Status audio subsystem I2S0 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsti2s0(&mut self) -> Pwrsti2s0W<AudsspwrstatusSpec> {
        Pwrsti2s0W::new(self, 6)
    }
    #[doc = "Bit 7 - Power Status audio subsystem I2S1 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsti2s1(&mut self) -> Pwrsti2s1W<AudsspwrstatusSpec> {
        Pwrsti2s1W::new(self, 7)
    }
    #[doc = "Bit 10 - Power Status audio subsystem ADC domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstaudadc(&mut self) -> PwrstaudadcW<AudsspwrstatusSpec> {
        PwrstaudadcW::new(self, 10)
    }
    #[doc = "Bit 11 - Power Status DSPA subsystem"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdspa(&mut self) -> PwrstdspaW<AudsspwrstatusSpec> {
        PwrstdspaW::new(self, 11)
    }
}
#[doc = "This provides the power status for the peripheral domains controlled through AUDSSPWREN register. Value of 1 means the device is powred up and ready to be used and 0 means its not powered up.\n\nYou can [`read`](crate::Reg::read) this register and get [`audsspwrstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audsspwrstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudsspwrstatusSpec;
impl crate::RegisterSpec for AudsspwrstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audsspwrstatus::R`](R) reader structure"]
impl crate::Readable for AudsspwrstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`audsspwrstatus::W`](W) writer structure"]
impl crate::Writable for AudsspwrstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDSSPWRSTATUS to value 0"]
impl crate::Resettable for AudsspwrstatusSpec {
    const RESET_VALUE: u32 = 0;
}
