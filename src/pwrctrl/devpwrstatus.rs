#[doc = "Register `DEVPWRSTATUS` reader"]
pub type R = crate::R<DevpwrstatusSpec>;
#[doc = "Register `DEVPWRSTATUS` writer"]
pub type W = crate::W<DevpwrstatusSpec>;
#[doc = "Power status IO Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstios {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstios> for bool {
    #[inline(always)]
    fn from(variant: Pwrstios) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOS` reader - Power status IO Slave"]
pub type PwrstiosR = crate::BitReader<Pwrstios>;
impl PwrstiosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstios {
        match self.bits {
            true => Pwrstios::On,
            false => Pwrstios::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstios::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstios::Off
    }
}
#[doc = "Field `PWRSTIOS` writer - Power status IO Slave"]
pub type PwrstiosW<'a, REG> = crate::BitWriter<'a, REG, Pwrstios>;
impl<'a, REG> PwrstiosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstios::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstios::Off)
    }
}
#[doc = "Power status IO Master 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstiom0 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstiom0> for bool {
    #[inline(always)]
    fn from(variant: Pwrstiom0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOM0` reader - Power status IO Master 0"]
pub type Pwrstiom0R = crate::BitReader<Pwrstiom0>;
impl Pwrstiom0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstiom0 {
        match self.bits {
            true => Pwrstiom0::On,
            false => Pwrstiom0::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstiom0::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstiom0::Off
    }
}
#[doc = "Field `PWRSTIOM0` writer - Power status IO Master 0"]
pub type Pwrstiom0W<'a, REG> = crate::BitWriter<'a, REG, Pwrstiom0>;
impl<'a, REG> Pwrstiom0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom0::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom0::Off)
    }
}
#[doc = "Power status IO Master 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstiom1 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstiom1> for bool {
    #[inline(always)]
    fn from(variant: Pwrstiom1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOM1` reader - Power status IO Master 1"]
pub type Pwrstiom1R = crate::BitReader<Pwrstiom1>;
impl Pwrstiom1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstiom1 {
        match self.bits {
            true => Pwrstiom1::On,
            false => Pwrstiom1::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstiom1::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstiom1::Off
    }
}
#[doc = "Field `PWRSTIOM1` writer - Power status IO Master 1"]
pub type Pwrstiom1W<'a, REG> = crate::BitWriter<'a, REG, Pwrstiom1>;
impl<'a, REG> Pwrstiom1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom1::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom1::Off)
    }
}
#[doc = "Power status IO Master 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstiom2 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstiom2> for bool {
    #[inline(always)]
    fn from(variant: Pwrstiom2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOM2` reader - Power status IO Master 2"]
pub type Pwrstiom2R = crate::BitReader<Pwrstiom2>;
impl Pwrstiom2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstiom2 {
        match self.bits {
            true => Pwrstiom2::On,
            false => Pwrstiom2::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstiom2::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstiom2::Off
    }
}
#[doc = "Field `PWRSTIOM2` writer - Power status IO Master 2"]
pub type Pwrstiom2W<'a, REG> = crate::BitWriter<'a, REG, Pwrstiom2>;
impl<'a, REG> Pwrstiom2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom2::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom2::Off)
    }
}
#[doc = "Power status IO Master 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstiom3 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstiom3> for bool {
    #[inline(always)]
    fn from(variant: Pwrstiom3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOM3` reader - Power status IO Master 3"]
pub type Pwrstiom3R = crate::BitReader<Pwrstiom3>;
impl Pwrstiom3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstiom3 {
        match self.bits {
            true => Pwrstiom3::On,
            false => Pwrstiom3::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstiom3::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstiom3::Off
    }
}
#[doc = "Field `PWRSTIOM3` writer - Power status IO Master 3"]
pub type Pwrstiom3W<'a, REG> = crate::BitWriter<'a, REG, Pwrstiom3>;
impl<'a, REG> Pwrstiom3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom3::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom3::Off)
    }
}
#[doc = "Power status IO Master 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstiom4 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstiom4> for bool {
    #[inline(always)]
    fn from(variant: Pwrstiom4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOM4` reader - Power status IO Master 4"]
pub type Pwrstiom4R = crate::BitReader<Pwrstiom4>;
impl Pwrstiom4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstiom4 {
        match self.bits {
            true => Pwrstiom4::On,
            false => Pwrstiom4::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstiom4::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstiom4::Off
    }
}
#[doc = "Field `PWRSTIOM4` writer - Power status IO Master 4"]
pub type Pwrstiom4W<'a, REG> = crate::BitWriter<'a, REG, Pwrstiom4>;
impl<'a, REG> Pwrstiom4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom4::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom4::Off)
    }
}
#[doc = "Power Status IO Master 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstiom5 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstiom5> for bool {
    #[inline(always)]
    fn from(variant: Pwrstiom5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOM5` reader - Power Status IO Master 5"]
pub type Pwrstiom5R = crate::BitReader<Pwrstiom5>;
impl Pwrstiom5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstiom5 {
        match self.bits {
            true => Pwrstiom5::On,
            false => Pwrstiom5::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstiom5::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstiom5::Off
    }
}
#[doc = "Field `PWRSTIOM5` writer - Power Status IO Master 5"]
pub type Pwrstiom5W<'a, REG> = crate::BitWriter<'a, REG, Pwrstiom5>;
impl<'a, REG> Pwrstiom5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom5::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom5::Off)
    }
}
#[doc = "Power Status IO Master 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstiom6 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstiom6> for bool {
    #[inline(always)]
    fn from(variant: Pwrstiom6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOM6` reader - Power Status IO Master 6"]
pub type Pwrstiom6R = crate::BitReader<Pwrstiom6>;
impl Pwrstiom6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstiom6 {
        match self.bits {
            true => Pwrstiom6::On,
            false => Pwrstiom6::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstiom6::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstiom6::Off
    }
}
#[doc = "Field `PWRSTIOM6` writer - Power Status IO Master 6"]
pub type Pwrstiom6W<'a, REG> = crate::BitWriter<'a, REG, Pwrstiom6>;
impl<'a, REG> Pwrstiom6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom6::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom6::Off)
    }
}
#[doc = "Power Status IO Master 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstiom7 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstiom7> for bool {
    #[inline(always)]
    fn from(variant: Pwrstiom7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTIOM7` reader - Power Status IO Master 7"]
pub type Pwrstiom7R = crate::BitReader<Pwrstiom7>;
impl Pwrstiom7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstiom7 {
        match self.bits {
            true => Pwrstiom7::On,
            false => Pwrstiom7::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstiom7::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstiom7::Off
    }
}
#[doc = "Field `PWRSTIOM7` writer - Power Status IO Master 7"]
pub type Pwrstiom7W<'a, REG> = crate::BitWriter<'a, REG, Pwrstiom7>;
impl<'a, REG> Pwrstiom7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom7::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstiom7::Off)
    }
}
#[doc = "Power Status UART Controller 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstuart0 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstuart0> for bool {
    #[inline(always)]
    fn from(variant: Pwrstuart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTUART0` reader - Power Status UART Controller 0"]
pub type Pwrstuart0R = crate::BitReader<Pwrstuart0>;
impl Pwrstuart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstuart0 {
        match self.bits {
            true => Pwrstuart0::On,
            false => Pwrstuart0::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstuart0::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstuart0::Off
    }
}
#[doc = "Field `PWRSTUART0` writer - Power Status UART Controller 0"]
pub type Pwrstuart0W<'a, REG> = crate::BitWriter<'a, REG, Pwrstuart0>;
impl<'a, REG> Pwrstuart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstuart0::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstuart0::Off)
    }
}
#[doc = "Power Status UART Controller 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstuart1 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstuart1> for bool {
    #[inline(always)]
    fn from(variant: Pwrstuart1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTUART1` reader - Power Status UART Controller 1"]
pub type Pwrstuart1R = crate::BitReader<Pwrstuart1>;
impl Pwrstuart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstuart1 {
        match self.bits {
            true => Pwrstuart1::On,
            false => Pwrstuart1::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstuart1::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstuart1::Off
    }
}
#[doc = "Field `PWRSTUART1` writer - Power Status UART Controller 1"]
pub type Pwrstuart1W<'a, REG> = crate::BitWriter<'a, REG, Pwrstuart1>;
impl<'a, REG> Pwrstuart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstuart1::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstuart1::Off)
    }
}
#[doc = "Power Status UART Controller 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstuart2 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstuart2> for bool {
    #[inline(always)]
    fn from(variant: Pwrstuart2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTUART2` reader - Power Status UART Controller 2"]
pub type Pwrstuart2R = crate::BitReader<Pwrstuart2>;
impl Pwrstuart2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstuart2 {
        match self.bits {
            true => Pwrstuart2::On,
            false => Pwrstuart2::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstuart2::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstuart2::Off
    }
}
#[doc = "Field `PWRSTUART2` writer - Power Status UART Controller 2"]
pub type Pwrstuart2W<'a, REG> = crate::BitWriter<'a, REG, Pwrstuart2>;
impl<'a, REG> Pwrstuart2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstuart2::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstuart2::Off)
    }
}
#[doc = "Power Status UART Controller 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstuart3 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstuart3> for bool {
    #[inline(always)]
    fn from(variant: Pwrstuart3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTUART3` reader - Power Status UART Controller 3"]
pub type Pwrstuart3R = crate::BitReader<Pwrstuart3>;
impl Pwrstuart3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstuart3 {
        match self.bits {
            true => Pwrstuart3::On,
            false => Pwrstuart3::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstuart3::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstuart3::Off
    }
}
#[doc = "Field `PWRSTUART3` writer - Power Status UART Controller 3"]
pub type Pwrstuart3W<'a, REG> = crate::BitWriter<'a, REG, Pwrstuart3>;
impl<'a, REG> Pwrstuart3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstuart3::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstuart3::Off)
    }
}
#[doc = "Power Status ADC Digital Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstadc {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstadc> for bool {
    #[inline(always)]
    fn from(variant: Pwrstadc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTADC` reader - Power Status ADC Digital Controller"]
pub type PwrstadcR = crate::BitReader<Pwrstadc>;
impl PwrstadcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstadc {
        match self.bits {
            true => Pwrstadc::On,
            false => Pwrstadc::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstadc::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstadc::Off
    }
}
#[doc = "Field `PWRSTADC` writer - Power Status ADC Digital Controller"]
pub type PwrstadcW<'a, REG> = crate::BitWriter<'a, REG, Pwrstadc>;
impl<'a, REG> PwrstadcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstadc::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstadc::Off)
    }
}
#[doc = "Power Status MSPI Controller0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstmspi0 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstmspi0> for bool {
    #[inline(always)]
    fn from(variant: Pwrstmspi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTMSPI0` reader - Power Status MSPI Controller0"]
pub type Pwrstmspi0R = crate::BitReader<Pwrstmspi0>;
impl Pwrstmspi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstmspi0 {
        match self.bits {
            true => Pwrstmspi0::On,
            false => Pwrstmspi0::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstmspi0::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstmspi0::Off
    }
}
#[doc = "Field `PWRSTMSPI0` writer - Power Status MSPI Controller0"]
pub type Pwrstmspi0W<'a, REG> = crate::BitWriter<'a, REG, Pwrstmspi0>;
impl<'a, REG> Pwrstmspi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstmspi0::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstmspi0::Off)
    }
}
#[doc = "Power Status MSPI Controller1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstmspi1 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstmspi1> for bool {
    #[inline(always)]
    fn from(variant: Pwrstmspi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTMSPI1` reader - Power Status MSPI Controller1"]
pub type Pwrstmspi1R = crate::BitReader<Pwrstmspi1>;
impl Pwrstmspi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstmspi1 {
        match self.bits {
            true => Pwrstmspi1::On,
            false => Pwrstmspi1::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstmspi1::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstmspi1::Off
    }
}
#[doc = "Field `PWRSTMSPI1` writer - Power Status MSPI Controller1"]
pub type Pwrstmspi1W<'a, REG> = crate::BitWriter<'a, REG, Pwrstmspi1>;
impl<'a, REG> Pwrstmspi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstmspi1::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstmspi1::Off)
    }
}
#[doc = "Power Status MSPI Controller2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstmspi2 {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstmspi2> for bool {
    #[inline(always)]
    fn from(variant: Pwrstmspi2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTMSPI2` reader - Power Status MSPI Controller2"]
pub type Pwrstmspi2R = crate::BitReader<Pwrstmspi2>;
impl Pwrstmspi2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstmspi2 {
        match self.bits {
            true => Pwrstmspi2::On,
            false => Pwrstmspi2::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstmspi2::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstmspi2::Off
    }
}
#[doc = "Field `PWRSTMSPI2` writer - Power Status MSPI Controller2"]
pub type Pwrstmspi2W<'a, REG> = crate::BitWriter<'a, REG, Pwrstmspi2>;
impl<'a, REG> Pwrstmspi2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstmspi2::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstmspi2::Off)
    }
}
#[doc = "Power Status GFX controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstgfx {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstgfx> for bool {
    #[inline(always)]
    fn from(variant: Pwrstgfx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTGFX` reader - Power Status GFX controller"]
pub type PwrstgfxR = crate::BitReader<Pwrstgfx>;
impl PwrstgfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstgfx {
        match self.bits {
            true => Pwrstgfx::On,
            false => Pwrstgfx::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstgfx::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstgfx::Off
    }
}
#[doc = "Field `PWRSTGFX` writer - Power Status GFX controller"]
pub type PwrstgfxW<'a, REG> = crate::BitWriter<'a, REG, Pwrstgfx>;
impl<'a, REG> PwrstgfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstgfx::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstgfx::Off)
    }
}
#[doc = "Power Status DISP controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstdisp {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstdisp> for bool {
    #[inline(always)]
    fn from(variant: Pwrstdisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTDISP` reader - Power Status DISP controller"]
pub type PwrstdispR = crate::BitReader<Pwrstdisp>;
impl PwrstdispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstdisp {
        match self.bits {
            true => Pwrstdisp::On,
            false => Pwrstdisp::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstdisp::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstdisp::Off
    }
}
#[doc = "Field `PWRSTDISP` writer - Power Status DISP controller"]
pub type PwrstdispW<'a, REG> = crate::BitWriter<'a, REG, Pwrstdisp>;
impl<'a, REG> PwrstdispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdisp::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdisp::Off)
    }
}
#[doc = "Power Status DISP PHY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstdispphy {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstdispphy> for bool {
    #[inline(always)]
    fn from(variant: Pwrstdispphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTDISPPHY` reader - Power Status DISP PHY"]
pub type PwrstdispphyR = crate::BitReader<Pwrstdispphy>;
impl PwrstdispphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstdispphy {
        match self.bits {
            true => Pwrstdispphy::On,
            false => Pwrstdispphy::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstdispphy::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstdispphy::Off
    }
}
#[doc = "Field `PWRSTDISPPHY` writer - Power Status DISP PHY"]
pub type PwrstdispphyW<'a, REG> = crate::BitWriter<'a, REG, Pwrstdispphy>;
impl<'a, REG> PwrstdispphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdispphy::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdispphy::Off)
    }
}
#[doc = "Power Status CRYPTO module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstcrypto {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstcrypto> for bool {
    #[inline(always)]
    fn from(variant: Pwrstcrypto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTCRYPTO` reader - Power Status CRYPTO module"]
pub type PwrstcryptoR = crate::BitReader<Pwrstcrypto>;
impl PwrstcryptoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstcrypto {
        match self.bits {
            true => Pwrstcrypto::On,
            false => Pwrstcrypto::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstcrypto::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstcrypto::Off
    }
}
#[doc = "Field `PWRSTCRYPTO` writer - Power Status CRYPTO module"]
pub type PwrstcryptoW<'a, REG> = crate::BitWriter<'a, REG, Pwrstcrypto>;
impl<'a, REG> PwrstcryptoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstcrypto::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstcrypto::Off)
    }
}
#[doc = "Power Status SDIO controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstsdio {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstsdio> for bool {
    #[inline(always)]
    fn from(variant: Pwrstsdio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTSDIO` reader - Power Status SDIO controller"]
pub type PwrstsdioR = crate::BitReader<Pwrstsdio>;
impl PwrstsdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstsdio {
        match self.bits {
            true => Pwrstsdio::On,
            false => Pwrstsdio::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstsdio::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstsdio::Off
    }
}
#[doc = "Field `PWRSTSDIO` writer - Power Status SDIO controller"]
pub type PwrstsdioW<'a, REG> = crate::BitWriter<'a, REG, Pwrstsdio>;
impl<'a, REG> PwrstsdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstsdio::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstsdio::Off)
    }
}
#[doc = "Power Status USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstusb {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstusb> for bool {
    #[inline(always)]
    fn from(variant: Pwrstusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTUSB` reader - Power Status USB controller"]
pub type PwrstusbR = crate::BitReader<Pwrstusb>;
impl PwrstusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstusb {
        match self.bits {
            true => Pwrstusb::On,
            false => Pwrstusb::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstusb::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstusb::Off
    }
}
#[doc = "Field `PWRSTUSB` writer - Power Status USB controller"]
pub type PwrstusbW<'a, REG> = crate::BitWriter<'a, REG, Pwrstusb>;
impl<'a, REG> PwrstusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstusb::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstusb::Off)
    }
}
#[doc = "Power Status USB PHY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstusbphy {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstusbphy> for bool {
    #[inline(always)]
    fn from(variant: Pwrstusbphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTUSBPHY` reader - Power Status USB PHY"]
pub type PwrstusbphyR = crate::BitReader<Pwrstusbphy>;
impl PwrstusbphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstusbphy {
        match self.bits {
            true => Pwrstusbphy::On,
            false => Pwrstusbphy::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstusbphy::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstusbphy::Off
    }
}
#[doc = "Field `PWRSTUSBPHY` writer - Power Status USB PHY"]
pub type PwrstusbphyW<'a, REG> = crate::BitWriter<'a, REG, Pwrstusbphy>;
impl<'a, REG> PwrstusbphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstusbphy::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstusbphy::Off)
    }
}
#[doc = "Power Status DBG subsystem\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrstdbg {
    #[doc = "1: Domain powered on"]
    On = 1,
    #[doc = "0: Domain powered off"]
    Off = 0,
}
impl From<Pwrstdbg> for bool {
    #[inline(always)]
    fn from(variant: Pwrstdbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSTDBG` reader - Power Status DBG subsystem"]
pub type PwrstdbgR = crate::BitReader<Pwrstdbg>;
impl PwrstdbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrstdbg {
        match self.bits {
            true => Pwrstdbg::On,
            false => Pwrstdbg::Off,
        }
    }
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pwrstdbg::On
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pwrstdbg::Off
    }
}
#[doc = "Field `PWRSTDBG` writer - Power Status DBG subsystem"]
pub type PwrstdbgW<'a, REG> = crate::BitWriter<'a, REG, Pwrstdbg>;
impl<'a, REG> PwrstdbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Domain powered on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdbg::On)
    }
    #[doc = "Domain powered off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrstdbg::Off)
    }
}
impl R {
    #[doc = "Bit 0 - Power status IO Slave"]
    #[inline(always)]
    pub fn pwrstios(&self) -> PwrstiosR {
        PwrstiosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power status IO Master 0"]
    #[inline(always)]
    pub fn pwrstiom0(&self) -> Pwrstiom0R {
        Pwrstiom0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power status IO Master 1"]
    #[inline(always)]
    pub fn pwrstiom1(&self) -> Pwrstiom1R {
        Pwrstiom1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power status IO Master 2"]
    #[inline(always)]
    pub fn pwrstiom2(&self) -> Pwrstiom2R {
        Pwrstiom2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power status IO Master 3"]
    #[inline(always)]
    pub fn pwrstiom3(&self) -> Pwrstiom3R {
        Pwrstiom3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power status IO Master 4"]
    #[inline(always)]
    pub fn pwrstiom4(&self) -> Pwrstiom4R {
        Pwrstiom4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power Status IO Master 5"]
    #[inline(always)]
    pub fn pwrstiom5(&self) -> Pwrstiom5R {
        Pwrstiom5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power Status IO Master 6"]
    #[inline(always)]
    pub fn pwrstiom6(&self) -> Pwrstiom6R {
        Pwrstiom6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Power Status IO Master 7"]
    #[inline(always)]
    pub fn pwrstiom7(&self) -> Pwrstiom7R {
        Pwrstiom7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power Status UART Controller 0"]
    #[inline(always)]
    pub fn pwrstuart0(&self) -> Pwrstuart0R {
        Pwrstuart0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Power Status UART Controller 1"]
    #[inline(always)]
    pub fn pwrstuart1(&self) -> Pwrstuart1R {
        Pwrstuart1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power Status UART Controller 2"]
    #[inline(always)]
    pub fn pwrstuart2(&self) -> Pwrstuart2R {
        Pwrstuart2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Power Status UART Controller 3"]
    #[inline(always)]
    pub fn pwrstuart3(&self) -> Pwrstuart3R {
        Pwrstuart3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Power Status ADC Digital Controller"]
    #[inline(always)]
    pub fn pwrstadc(&self) -> PwrstadcR {
        PwrstadcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Power Status MSPI Controller0"]
    #[inline(always)]
    pub fn pwrstmspi0(&self) -> Pwrstmspi0R {
        Pwrstmspi0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Power Status MSPI Controller1"]
    #[inline(always)]
    pub fn pwrstmspi1(&self) -> Pwrstmspi1R {
        Pwrstmspi1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Power Status MSPI Controller2"]
    #[inline(always)]
    pub fn pwrstmspi2(&self) -> Pwrstmspi2R {
        Pwrstmspi2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Power Status GFX controller"]
    #[inline(always)]
    pub fn pwrstgfx(&self) -> PwrstgfxR {
        PwrstgfxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power Status DISP controller"]
    #[inline(always)]
    pub fn pwrstdisp(&self) -> PwrstdispR {
        PwrstdispR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Power Status DISP PHY"]
    #[inline(always)]
    pub fn pwrstdispphy(&self) -> PwrstdispphyR {
        PwrstdispphyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Power Status CRYPTO module"]
    #[inline(always)]
    pub fn pwrstcrypto(&self) -> PwrstcryptoR {
        PwrstcryptoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Power Status SDIO controller"]
    #[inline(always)]
    pub fn pwrstsdio(&self) -> PwrstsdioR {
        PwrstsdioR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Power Status USB controller"]
    #[inline(always)]
    pub fn pwrstusb(&self) -> PwrstusbR {
        PwrstusbR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Power Status USB PHY"]
    #[inline(always)]
    pub fn pwrstusbphy(&self) -> PwrstusbphyR {
        PwrstusbphyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Power Status DBG subsystem"]
    #[inline(always)]
    pub fn pwrstdbg(&self) -> PwrstdbgR {
        PwrstdbgR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power status IO Slave"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstios(&mut self) -> PwrstiosW<DevpwrstatusSpec> {
        PwrstiosW::new(self, 0)
    }
    #[doc = "Bit 1 - Power status IO Master 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstiom0(&mut self) -> Pwrstiom0W<DevpwrstatusSpec> {
        Pwrstiom0W::new(self, 1)
    }
    #[doc = "Bit 2 - Power status IO Master 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstiom1(&mut self) -> Pwrstiom1W<DevpwrstatusSpec> {
        Pwrstiom1W::new(self, 2)
    }
    #[doc = "Bit 3 - Power status IO Master 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstiom2(&mut self) -> Pwrstiom2W<DevpwrstatusSpec> {
        Pwrstiom2W::new(self, 3)
    }
    #[doc = "Bit 4 - Power status IO Master 3"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstiom3(&mut self) -> Pwrstiom3W<DevpwrstatusSpec> {
        Pwrstiom3W::new(self, 4)
    }
    #[doc = "Bit 5 - Power status IO Master 4"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstiom4(&mut self) -> Pwrstiom4W<DevpwrstatusSpec> {
        Pwrstiom4W::new(self, 5)
    }
    #[doc = "Bit 6 - Power Status IO Master 5"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstiom5(&mut self) -> Pwrstiom5W<DevpwrstatusSpec> {
        Pwrstiom5W::new(self, 6)
    }
    #[doc = "Bit 7 - Power Status IO Master 6"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstiom6(&mut self) -> Pwrstiom6W<DevpwrstatusSpec> {
        Pwrstiom6W::new(self, 7)
    }
    #[doc = "Bit 8 - Power Status IO Master 7"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstiom7(&mut self) -> Pwrstiom7W<DevpwrstatusSpec> {
        Pwrstiom7W::new(self, 8)
    }
    #[doc = "Bit 9 - Power Status UART Controller 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstuart0(&mut self) -> Pwrstuart0W<DevpwrstatusSpec> {
        Pwrstuart0W::new(self, 9)
    }
    #[doc = "Bit 10 - Power Status UART Controller 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstuart1(&mut self) -> Pwrstuart1W<DevpwrstatusSpec> {
        Pwrstuart1W::new(self, 10)
    }
    #[doc = "Bit 11 - Power Status UART Controller 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstuart2(&mut self) -> Pwrstuart2W<DevpwrstatusSpec> {
        Pwrstuart2W::new(self, 11)
    }
    #[doc = "Bit 12 - Power Status UART Controller 3"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstuart3(&mut self) -> Pwrstuart3W<DevpwrstatusSpec> {
        Pwrstuart3W::new(self, 12)
    }
    #[doc = "Bit 13 - Power Status ADC Digital Controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstadc(&mut self) -> PwrstadcW<DevpwrstatusSpec> {
        PwrstadcW::new(self, 13)
    }
    #[doc = "Bit 14 - Power Status MSPI Controller0"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstmspi0(&mut self) -> Pwrstmspi0W<DevpwrstatusSpec> {
        Pwrstmspi0W::new(self, 14)
    }
    #[doc = "Bit 15 - Power Status MSPI Controller1"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstmspi1(&mut self) -> Pwrstmspi1W<DevpwrstatusSpec> {
        Pwrstmspi1W::new(self, 15)
    }
    #[doc = "Bit 16 - Power Status MSPI Controller2"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstmspi2(&mut self) -> Pwrstmspi2W<DevpwrstatusSpec> {
        Pwrstmspi2W::new(self, 16)
    }
    #[doc = "Bit 17 - Power Status GFX controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstgfx(&mut self) -> PwrstgfxW<DevpwrstatusSpec> {
        PwrstgfxW::new(self, 17)
    }
    #[doc = "Bit 18 - Power Status DISP controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdisp(&mut self) -> PwrstdispW<DevpwrstatusSpec> {
        PwrstdispW::new(self, 18)
    }
    #[doc = "Bit 19 - Power Status DISP PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdispphy(&mut self) -> PwrstdispphyW<DevpwrstatusSpec> {
        PwrstdispphyW::new(self, 19)
    }
    #[doc = "Bit 20 - Power Status CRYPTO module"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstcrypto(&mut self) -> PwrstcryptoW<DevpwrstatusSpec> {
        PwrstcryptoW::new(self, 20)
    }
    #[doc = "Bit 21 - Power Status SDIO controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstsdio(&mut self) -> PwrstsdioW<DevpwrstatusSpec> {
        PwrstsdioW::new(self, 21)
    }
    #[doc = "Bit 22 - Power Status USB controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstusb(&mut self) -> PwrstusbW<DevpwrstatusSpec> {
        PwrstusbW::new(self, 22)
    }
    #[doc = "Bit 23 - Power Status USB PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstusbphy(&mut self) -> PwrstusbphyW<DevpwrstatusSpec> {
        PwrstusbphyW::new(self, 23)
    }
    #[doc = "Bit 24 - Power Status DBG subsystem"]
    #[inline(always)]
    #[must_use]
    pub fn pwrstdbg(&mut self) -> PwrstdbgW<DevpwrstatusSpec> {
        PwrstdbgW::new(self, 24)
    }
}
#[doc = "This provides the power status for the peripheral device domains controlled through DEVPWREN register. Value of 1 means the device is powred up and ready to be used and 0 means its not powered up.\n\nYou can [`read`](crate::Reg::read) this register and get [`devpwrstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devpwrstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevpwrstatusSpec;
impl crate::RegisterSpec for DevpwrstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devpwrstatus::R`](R) reader structure"]
impl crate::Readable for DevpwrstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`devpwrstatus::W`](W) writer structure"]
impl crate::Writable for DevpwrstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVPWRSTATUS to value 0"]
impl crate::Resettable for DevpwrstatusSpec {
    const RESET_VALUE: u32 = 0;
}
