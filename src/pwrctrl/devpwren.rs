#[doc = "Register `DEVPWREN` reader"]
pub type R = crate::R<DevpwrenSpec>;
#[doc = "Register `DEVPWREN` writer"]
pub type W = crate::W<DevpwrenSpec>;
#[doc = "Power up IO Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenios {
    #[doc = "1: Power up IO slave"]
    En = 1,
    #[doc = "0: Power down IO slave"]
    Dis = 0,
}
impl From<Pwrenios> for bool {
    #[inline(always)]
    fn from(variant: Pwrenios) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOS` reader - Power up IO Slave"]
pub type PwreniosR = crate::BitReader<Pwrenios>;
impl PwreniosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenios {
        match self.bits {
            true => Pwrenios::En,
            false => Pwrenios::Dis,
        }
    }
    #[doc = "Power up IO slave"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenios::En
    }
    #[doc = "Power down IO slave"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenios::Dis
    }
}
#[doc = "Field `PWRENIOS` writer - Power up IO Slave"]
pub type PwreniosW<'a, REG> = crate::BitWriter<'a, REG, Pwrenios>;
impl<'a, REG> PwreniosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO slave"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenios::En)
    }
    #[doc = "Power down IO slave"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenios::Dis)
    }
}
#[doc = "Power up IO Master 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreniom0 {
    #[doc = "1: Power up IO Master 0"]
    En = 1,
    #[doc = "0: Power down IO Master 0"]
    Dis = 0,
}
impl From<Pwreniom0> for bool {
    #[inline(always)]
    fn from(variant: Pwreniom0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOM0` reader - Power up IO Master 0"]
pub type Pwreniom0R = crate::BitReader<Pwreniom0>;
impl Pwreniom0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreniom0 {
        match self.bits {
            true => Pwreniom0::En,
            false => Pwreniom0::Dis,
        }
    }
    #[doc = "Power up IO Master 0"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreniom0::En
    }
    #[doc = "Power down IO Master 0"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreniom0::Dis
    }
}
#[doc = "Field `PWRENIOM0` writer - Power up IO Master 0"]
pub type Pwreniom0W<'a, REG> = crate::BitWriter<'a, REG, Pwreniom0>;
impl<'a, REG> Pwreniom0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO Master 0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom0::En)
    }
    #[doc = "Power down IO Master 0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom0::Dis)
    }
}
#[doc = "Power up IO Master 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreniom1 {
    #[doc = "1: Power up IO Master 1"]
    En = 1,
    #[doc = "0: Power down IO Master 1"]
    Dis = 0,
}
impl From<Pwreniom1> for bool {
    #[inline(always)]
    fn from(variant: Pwreniom1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOM1` reader - Power up IO Master 1"]
pub type Pwreniom1R = crate::BitReader<Pwreniom1>;
impl Pwreniom1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreniom1 {
        match self.bits {
            true => Pwreniom1::En,
            false => Pwreniom1::Dis,
        }
    }
    #[doc = "Power up IO Master 1"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreniom1::En
    }
    #[doc = "Power down IO Master 1"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreniom1::Dis
    }
}
#[doc = "Field `PWRENIOM1` writer - Power up IO Master 1"]
pub type Pwreniom1W<'a, REG> = crate::BitWriter<'a, REG, Pwreniom1>;
impl<'a, REG> Pwreniom1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO Master 1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom1::En)
    }
    #[doc = "Power down IO Master 1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom1::Dis)
    }
}
#[doc = "Power up IO Master 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreniom2 {
    #[doc = "1: Power up IO Master 2"]
    En = 1,
    #[doc = "0: Power down IO Master 2"]
    Dis = 0,
}
impl From<Pwreniom2> for bool {
    #[inline(always)]
    fn from(variant: Pwreniom2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOM2` reader - Power up IO Master 2"]
pub type Pwreniom2R = crate::BitReader<Pwreniom2>;
impl Pwreniom2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreniom2 {
        match self.bits {
            true => Pwreniom2::En,
            false => Pwreniom2::Dis,
        }
    }
    #[doc = "Power up IO Master 2"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreniom2::En
    }
    #[doc = "Power down IO Master 2"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreniom2::Dis
    }
}
#[doc = "Field `PWRENIOM2` writer - Power up IO Master 2"]
pub type Pwreniom2W<'a, REG> = crate::BitWriter<'a, REG, Pwreniom2>;
impl<'a, REG> Pwreniom2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO Master 2"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom2::En)
    }
    #[doc = "Power down IO Master 2"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom2::Dis)
    }
}
#[doc = "Power up IO Master 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreniom3 {
    #[doc = "1: Power up IO Master 3"]
    En = 1,
    #[doc = "0: Power down IO Master 3"]
    Dis = 0,
}
impl From<Pwreniom3> for bool {
    #[inline(always)]
    fn from(variant: Pwreniom3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOM3` reader - Power up IO Master 3"]
pub type Pwreniom3R = crate::BitReader<Pwreniom3>;
impl Pwreniom3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreniom3 {
        match self.bits {
            true => Pwreniom3::En,
            false => Pwreniom3::Dis,
        }
    }
    #[doc = "Power up IO Master 3"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreniom3::En
    }
    #[doc = "Power down IO Master 3"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreniom3::Dis
    }
}
#[doc = "Field `PWRENIOM3` writer - Power up IO Master 3"]
pub type Pwreniom3W<'a, REG> = crate::BitWriter<'a, REG, Pwreniom3>;
impl<'a, REG> Pwreniom3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO Master 3"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom3::En)
    }
    #[doc = "Power down IO Master 3"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom3::Dis)
    }
}
#[doc = "Power up IO Master 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreniom4 {
    #[doc = "1: Power up IO Master 4"]
    En = 1,
    #[doc = "0: Power down IO Master 4"]
    Dis = 0,
}
impl From<Pwreniom4> for bool {
    #[inline(always)]
    fn from(variant: Pwreniom4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOM4` reader - Power up IO Master 4"]
pub type Pwreniom4R = crate::BitReader<Pwreniom4>;
impl Pwreniom4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreniom4 {
        match self.bits {
            true => Pwreniom4::En,
            false => Pwreniom4::Dis,
        }
    }
    #[doc = "Power up IO Master 4"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreniom4::En
    }
    #[doc = "Power down IO Master 4"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreniom4::Dis
    }
}
#[doc = "Field `PWRENIOM4` writer - Power up IO Master 4"]
pub type Pwreniom4W<'a, REG> = crate::BitWriter<'a, REG, Pwreniom4>;
impl<'a, REG> Pwreniom4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO Master 4"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom4::En)
    }
    #[doc = "Power down IO Master 4"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom4::Dis)
    }
}
#[doc = "Power up IO Master 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreniom5 {
    #[doc = "1: Power up IO Master 5"]
    En = 1,
    #[doc = "0: Power down IO Master 5"]
    Dis = 0,
}
impl From<Pwreniom5> for bool {
    #[inline(always)]
    fn from(variant: Pwreniom5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOM5` reader - Power up IO Master 5"]
pub type Pwreniom5R = crate::BitReader<Pwreniom5>;
impl Pwreniom5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreniom5 {
        match self.bits {
            true => Pwreniom5::En,
            false => Pwreniom5::Dis,
        }
    }
    #[doc = "Power up IO Master 5"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreniom5::En
    }
    #[doc = "Power down IO Master 5"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreniom5::Dis
    }
}
#[doc = "Field `PWRENIOM5` writer - Power up IO Master 5"]
pub type Pwreniom5W<'a, REG> = crate::BitWriter<'a, REG, Pwreniom5>;
impl<'a, REG> Pwreniom5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO Master 5"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom5::En)
    }
    #[doc = "Power down IO Master 5"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom5::Dis)
    }
}
#[doc = "Power up IO Master 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreniom6 {
    #[doc = "1: Power up IO Master 6"]
    En = 1,
    #[doc = "0: Power down IO Master 6"]
    Dis = 0,
}
impl From<Pwreniom6> for bool {
    #[inline(always)]
    fn from(variant: Pwreniom6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOM6` reader - Power up IO Master 6"]
pub type Pwreniom6R = crate::BitReader<Pwreniom6>;
impl Pwreniom6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreniom6 {
        match self.bits {
            true => Pwreniom6::En,
            false => Pwreniom6::Dis,
        }
    }
    #[doc = "Power up IO Master 6"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreniom6::En
    }
    #[doc = "Power down IO Master 6"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreniom6::Dis
    }
}
#[doc = "Field `PWRENIOM6` writer - Power up IO Master 6"]
pub type Pwreniom6W<'a, REG> = crate::BitWriter<'a, REG, Pwreniom6>;
impl<'a, REG> Pwreniom6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO Master 6"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom6::En)
    }
    #[doc = "Power down IO Master 6"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom6::Dis)
    }
}
#[doc = "Power up IO Master 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreniom7 {
    #[doc = "1: Power up IO Master 7"]
    En = 1,
    #[doc = "0: Power down IO Master 7"]
    Dis = 0,
}
impl From<Pwreniom7> for bool {
    #[inline(always)]
    fn from(variant: Pwreniom7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENIOM7` reader - Power up IO Master 7"]
pub type Pwreniom7R = crate::BitReader<Pwreniom7>;
impl Pwreniom7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreniom7 {
        match self.bits {
            true => Pwreniom7::En,
            false => Pwreniom7::Dis,
        }
    }
    #[doc = "Power up IO Master 7"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreniom7::En
    }
    #[doc = "Power down IO Master 7"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreniom7::Dis
    }
}
#[doc = "Field `PWRENIOM7` writer - Power up IO Master 7"]
pub type Pwreniom7W<'a, REG> = crate::BitWriter<'a, REG, Pwreniom7>;
impl<'a, REG> Pwreniom7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up IO Master 7"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom7::En)
    }
    #[doc = "Power down IO Master 7"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreniom7::Dis)
    }
}
#[doc = "Power up UART Controller 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenuart0 {
    #[doc = "1: Power up UART 0"]
    En = 1,
    #[doc = "0: Power down UART 0"]
    Dis = 0,
}
impl From<Pwrenuart0> for bool {
    #[inline(always)]
    fn from(variant: Pwrenuart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENUART0` reader - Power up UART Controller 0"]
pub type Pwrenuart0R = crate::BitReader<Pwrenuart0>;
impl Pwrenuart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenuart0 {
        match self.bits {
            true => Pwrenuart0::En,
            false => Pwrenuart0::Dis,
        }
    }
    #[doc = "Power up UART 0"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenuart0::En
    }
    #[doc = "Power down UART 0"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenuart0::Dis
    }
}
#[doc = "Field `PWRENUART0` writer - Power up UART Controller 0"]
pub type Pwrenuart0W<'a, REG> = crate::BitWriter<'a, REG, Pwrenuart0>;
impl<'a, REG> Pwrenuart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up UART 0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenuart0::En)
    }
    #[doc = "Power down UART 0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenuart0::Dis)
    }
}
#[doc = "Power up UART Controller 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenuart1 {
    #[doc = "1: Power up UART 1"]
    En = 1,
    #[doc = "0: Power down UART 1"]
    Dis = 0,
}
impl From<Pwrenuart1> for bool {
    #[inline(always)]
    fn from(variant: Pwrenuart1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENUART1` reader - Power up UART Controller 1"]
pub type Pwrenuart1R = crate::BitReader<Pwrenuart1>;
impl Pwrenuart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenuart1 {
        match self.bits {
            true => Pwrenuart1::En,
            false => Pwrenuart1::Dis,
        }
    }
    #[doc = "Power up UART 1"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenuart1::En
    }
    #[doc = "Power down UART 1"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenuart1::Dis
    }
}
#[doc = "Field `PWRENUART1` writer - Power up UART Controller 1"]
pub type Pwrenuart1W<'a, REG> = crate::BitWriter<'a, REG, Pwrenuart1>;
impl<'a, REG> Pwrenuart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up UART 1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenuart1::En)
    }
    #[doc = "Power down UART 1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenuart1::Dis)
    }
}
#[doc = "Power up UART Controller 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenuart2 {
    #[doc = "1: Power up UART 2"]
    En = 1,
    #[doc = "0: Power down UART 2"]
    Dis = 0,
}
impl From<Pwrenuart2> for bool {
    #[inline(always)]
    fn from(variant: Pwrenuart2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENUART2` reader - Power up UART Controller 2"]
pub type Pwrenuart2R = crate::BitReader<Pwrenuart2>;
impl Pwrenuart2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenuart2 {
        match self.bits {
            true => Pwrenuart2::En,
            false => Pwrenuart2::Dis,
        }
    }
    #[doc = "Power up UART 2"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenuart2::En
    }
    #[doc = "Power down UART 2"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenuart2::Dis
    }
}
#[doc = "Field `PWRENUART2` writer - Power up UART Controller 2"]
pub type Pwrenuart2W<'a, REG> = crate::BitWriter<'a, REG, Pwrenuart2>;
impl<'a, REG> Pwrenuart2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up UART 2"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenuart2::En)
    }
    #[doc = "Power down UART 2"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenuart2::Dis)
    }
}
#[doc = "Power up UART Controller 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenuart3 {
    #[doc = "1: Power up UART 3"]
    En = 1,
    #[doc = "0: Power down UART 3"]
    Dis = 0,
}
impl From<Pwrenuart3> for bool {
    #[inline(always)]
    fn from(variant: Pwrenuart3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENUART3` reader - Power up UART Controller 3"]
pub type Pwrenuart3R = crate::BitReader<Pwrenuart3>;
impl Pwrenuart3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenuart3 {
        match self.bits {
            true => Pwrenuart3::En,
            false => Pwrenuart3::Dis,
        }
    }
    #[doc = "Power up UART 3"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenuart3::En
    }
    #[doc = "Power down UART 3"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenuart3::Dis
    }
}
#[doc = "Field `PWRENUART3` writer - Power up UART Controller 3"]
pub type Pwrenuart3W<'a, REG> = crate::BitWriter<'a, REG, Pwrenuart3>;
impl<'a, REG> Pwrenuart3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up UART 3"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenuart3::En)
    }
    #[doc = "Power down UART 3"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenuart3::Dis)
    }
}
#[doc = "Power up ADC Digital Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenadc {
    #[doc = "1: Power up ADC"]
    En = 1,
    #[doc = "0: Power Down ADC"]
    Dis = 0,
}
impl From<Pwrenadc> for bool {
    #[inline(always)]
    fn from(variant: Pwrenadc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENADC` reader - Power up ADC Digital Controller"]
pub type PwrenadcR = crate::BitReader<Pwrenadc>;
impl PwrenadcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenadc {
        match self.bits {
            true => Pwrenadc::En,
            false => Pwrenadc::Dis,
        }
    }
    #[doc = "Power up ADC"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenadc::En
    }
    #[doc = "Power Down ADC"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenadc::Dis
    }
}
#[doc = "Field `PWRENADC` writer - Power up ADC Digital Controller"]
pub type PwrenadcW<'a, REG> = crate::BitWriter<'a, REG, Pwrenadc>;
impl<'a, REG> PwrenadcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up ADC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenadc::En)
    }
    #[doc = "Power Down ADC"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenadc::Dis)
    }
}
#[doc = "Power up MSPI Controller0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenmspi0 {
    #[doc = "1: Power up MSPI0"]
    En = 1,
    #[doc = "0: Power down MSPI0"]
    Dis = 0,
}
impl From<Pwrenmspi0> for bool {
    #[inline(always)]
    fn from(variant: Pwrenmspi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENMSPI0` reader - Power up MSPI Controller0"]
pub type Pwrenmspi0R = crate::BitReader<Pwrenmspi0>;
impl Pwrenmspi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenmspi0 {
        match self.bits {
            true => Pwrenmspi0::En,
            false => Pwrenmspi0::Dis,
        }
    }
    #[doc = "Power up MSPI0"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenmspi0::En
    }
    #[doc = "Power down MSPI0"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenmspi0::Dis
    }
}
#[doc = "Field `PWRENMSPI0` writer - Power up MSPI Controller0"]
pub type Pwrenmspi0W<'a, REG> = crate::BitWriter<'a, REG, Pwrenmspi0>;
impl<'a, REG> Pwrenmspi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up MSPI0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenmspi0::En)
    }
    #[doc = "Power down MSPI0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenmspi0::Dis)
    }
}
#[doc = "Power up MSPI Controller1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenmspi1 {
    #[doc = "1: Power up MSPI1"]
    En = 1,
    #[doc = "0: Power down MSPI1"]
    Dis = 0,
}
impl From<Pwrenmspi1> for bool {
    #[inline(always)]
    fn from(variant: Pwrenmspi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENMSPI1` reader - Power up MSPI Controller1"]
pub type Pwrenmspi1R = crate::BitReader<Pwrenmspi1>;
impl Pwrenmspi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenmspi1 {
        match self.bits {
            true => Pwrenmspi1::En,
            false => Pwrenmspi1::Dis,
        }
    }
    #[doc = "Power up MSPI1"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenmspi1::En
    }
    #[doc = "Power down MSPI1"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenmspi1::Dis
    }
}
#[doc = "Field `PWRENMSPI1` writer - Power up MSPI Controller1"]
pub type Pwrenmspi1W<'a, REG> = crate::BitWriter<'a, REG, Pwrenmspi1>;
impl<'a, REG> Pwrenmspi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up MSPI1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenmspi1::En)
    }
    #[doc = "Power down MSPI1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenmspi1::Dis)
    }
}
#[doc = "Power up MSPI Controller2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenmspi2 {
    #[doc = "1: Power up MSPI2"]
    En = 1,
    #[doc = "0: Power down MSPI2"]
    Dis = 0,
}
impl From<Pwrenmspi2> for bool {
    #[inline(always)]
    fn from(variant: Pwrenmspi2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENMSPI2` reader - Power up MSPI Controller2"]
pub type Pwrenmspi2R = crate::BitReader<Pwrenmspi2>;
impl Pwrenmspi2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenmspi2 {
        match self.bits {
            true => Pwrenmspi2::En,
            false => Pwrenmspi2::Dis,
        }
    }
    #[doc = "Power up MSPI2"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenmspi2::En
    }
    #[doc = "Power down MSPI2"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenmspi2::Dis
    }
}
#[doc = "Field `PWRENMSPI2` writer - Power up MSPI Controller2"]
pub type Pwrenmspi2W<'a, REG> = crate::BitWriter<'a, REG, Pwrenmspi2>;
impl<'a, REG> Pwrenmspi2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up MSPI2"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenmspi2::En)
    }
    #[doc = "Power down MSPI2"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenmspi2::Dis)
    }
}
#[doc = "Power up GFX controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrengfx {
    #[doc = "1: Power up GFX"]
    En = 1,
    #[doc = "0: Power down GFX"]
    Dis = 0,
}
impl From<Pwrengfx> for bool {
    #[inline(always)]
    fn from(variant: Pwrengfx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENGFX` reader - Power up GFX controller"]
pub type PwrengfxR = crate::BitReader<Pwrengfx>;
impl PwrengfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrengfx {
        match self.bits {
            true => Pwrengfx::En,
            false => Pwrengfx::Dis,
        }
    }
    #[doc = "Power up GFX"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrengfx::En
    }
    #[doc = "Power down GFX"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrengfx::Dis
    }
}
#[doc = "Field `PWRENGFX` writer - Power up GFX controller"]
pub type PwrengfxW<'a, REG> = crate::BitWriter<'a, REG, Pwrengfx>;
impl<'a, REG> PwrengfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up GFX"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrengfx::En)
    }
    #[doc = "Power down GFX"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrengfx::Dis)
    }
}
#[doc = "Power up DISP controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrendisp {
    #[doc = "1: Power up DISP"]
    En = 1,
    #[doc = "0: Power down DISP"]
    Dis = 0,
}
impl From<Pwrendisp> for bool {
    #[inline(always)]
    fn from(variant: Pwrendisp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENDISP` reader - Power up DISP controller"]
pub type PwrendispR = crate::BitReader<Pwrendisp>;
impl PwrendispR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrendisp {
        match self.bits {
            true => Pwrendisp::En,
            false => Pwrendisp::Dis,
        }
    }
    #[doc = "Power up DISP"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrendisp::En
    }
    #[doc = "Power down DISP"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrendisp::Dis
    }
}
#[doc = "Field `PWRENDISP` writer - Power up DISP controller"]
pub type PwrendispW<'a, REG> = crate::BitWriter<'a, REG, Pwrendisp>;
impl<'a, REG> PwrendispW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up DISP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendisp::En)
    }
    #[doc = "Power down DISP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendisp::Dis)
    }
}
#[doc = "Power up DISP PHY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrendispphy {
    #[doc = "1: Power up DISP PHY"]
    En = 1,
    #[doc = "0: Power down DISP PHY"]
    Dis = 0,
}
impl From<Pwrendispphy> for bool {
    #[inline(always)]
    fn from(variant: Pwrendispphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENDISPPHY` reader - Power up DISP PHY"]
pub type PwrendispphyR = crate::BitReader<Pwrendispphy>;
impl PwrendispphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrendispphy {
        match self.bits {
            true => Pwrendispphy::En,
            false => Pwrendispphy::Dis,
        }
    }
    #[doc = "Power up DISP PHY"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrendispphy::En
    }
    #[doc = "Power down DISP PHY"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrendispphy::Dis
    }
}
#[doc = "Field `PWRENDISPPHY` writer - Power up DISP PHY"]
pub type PwrendispphyW<'a, REG> = crate::BitWriter<'a, REG, Pwrendispphy>;
impl<'a, REG> PwrendispphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up DISP PHY"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendispphy::En)
    }
    #[doc = "Power down DISP PHY"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendispphy::Dis)
    }
}
#[doc = "Power up CRYPTO module\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrencrypto {
    #[doc = "1: Power up CRYPTO"]
    En = 1,
    #[doc = "0: Power down CRYPTO"]
    Dis = 0,
}
impl From<Pwrencrypto> for bool {
    #[inline(always)]
    fn from(variant: Pwrencrypto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENCRYPTO` reader - Power up CRYPTO module"]
pub type PwrencryptoR = crate::BitReader<Pwrencrypto>;
impl PwrencryptoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrencrypto {
        match self.bits {
            true => Pwrencrypto::En,
            false => Pwrencrypto::Dis,
        }
    }
    #[doc = "Power up CRYPTO"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrencrypto::En
    }
    #[doc = "Power down CRYPTO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrencrypto::Dis
    }
}
#[doc = "Field `PWRENCRYPTO` writer - Power up CRYPTO module"]
pub type PwrencryptoW<'a, REG> = crate::BitWriter<'a, REG, Pwrencrypto>;
impl<'a, REG> PwrencryptoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up CRYPTO"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrencrypto::En)
    }
    #[doc = "Power down CRYPTO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrencrypto::Dis)
    }
}
#[doc = "Power up SDIO controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrensdio {
    #[doc = "1: Power up SDIO"]
    En = 1,
    #[doc = "0: Power down SDIO"]
    Dis = 0,
}
impl From<Pwrensdio> for bool {
    #[inline(always)]
    fn from(variant: Pwrensdio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENSDIO` reader - Power up SDIO controller"]
pub type PwrensdioR = crate::BitReader<Pwrensdio>;
impl PwrensdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrensdio {
        match self.bits {
            true => Pwrensdio::En,
            false => Pwrensdio::Dis,
        }
    }
    #[doc = "Power up SDIO"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrensdio::En
    }
    #[doc = "Power down SDIO"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrensdio::Dis
    }
}
#[doc = "Field `PWRENSDIO` writer - Power up SDIO controller"]
pub type PwrensdioW<'a, REG> = crate::BitWriter<'a, REG, Pwrensdio>;
impl<'a, REG> PwrensdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up SDIO"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrensdio::En)
    }
    #[doc = "Power down SDIO"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrensdio::Dis)
    }
}
#[doc = "Power up USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenusb {
    #[doc = "1: Power up USB"]
    En = 1,
    #[doc = "0: Power down USB"]
    Dis = 0,
}
impl From<Pwrenusb> for bool {
    #[inline(always)]
    fn from(variant: Pwrenusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENUSB` reader - Power up USB controller"]
pub type PwrenusbR = crate::BitReader<Pwrenusb>;
impl PwrenusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenusb {
        match self.bits {
            true => Pwrenusb::En,
            false => Pwrenusb::Dis,
        }
    }
    #[doc = "Power up USB"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenusb::En
    }
    #[doc = "Power down USB"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenusb::Dis
    }
}
#[doc = "Field `PWRENUSB` writer - Power up USB controller"]
pub type PwrenusbW<'a, REG> = crate::BitWriter<'a, REG, Pwrenusb>;
impl<'a, REG> PwrenusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up USB"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenusb::En)
    }
    #[doc = "Power down USB"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenusb::Dis)
    }
}
#[doc = "Power up USB PHY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenusbphy {
    #[doc = "1: Power up USB PHY"]
    En = 1,
    #[doc = "0: Power down USB PHY"]
    Dis = 0,
}
impl From<Pwrenusbphy> for bool {
    #[inline(always)]
    fn from(variant: Pwrenusbphy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENUSBPHY` reader - Power up USB PHY"]
pub type PwrenusbphyR = crate::BitReader<Pwrenusbphy>;
impl PwrenusbphyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenusbphy {
        match self.bits {
            true => Pwrenusbphy::En,
            false => Pwrenusbphy::Dis,
        }
    }
    #[doc = "Power up USB PHY"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenusbphy::En
    }
    #[doc = "Power down USB PHY"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenusbphy::Dis
    }
}
#[doc = "Field `PWRENUSBPHY` writer - Power up USB PHY"]
pub type PwrenusbphyW<'a, REG> = crate::BitWriter<'a, REG, Pwrenusbphy>;
impl<'a, REG> PwrenusbphyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up USB PHY"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenusbphy::En)
    }
    #[doc = "Power down USB PHY"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenusbphy::Dis)
    }
}
#[doc = "Powerup DBG power domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrendbg {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<Pwrendbg> for bool {
    #[inline(always)]
    fn from(variant: Pwrendbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENDBG` reader - Powerup DBG power domain"]
pub type PwrendbgR = crate::BitReader<Pwrendbg>;
impl PwrendbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrendbg {
        match self.bits {
            true => Pwrendbg::En,
            false => Pwrendbg::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrendbg::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrendbg::Dis
    }
}
#[doc = "Field `PWRENDBG` writer - Powerup DBG power domain"]
pub type PwrendbgW<'a, REG> = crate::BitWriter<'a, REG, Pwrendbg>;
impl<'a, REG> PwrendbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendbg::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendbg::Dis)
    }
}
impl R {
    #[doc = "Bit 0 - Power up IO Slave"]
    #[inline(always)]
    pub fn pwrenios(&self) -> PwreniosR {
        PwreniosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power up IO Master 0"]
    #[inline(always)]
    pub fn pwreniom0(&self) -> Pwreniom0R {
        Pwreniom0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power up IO Master 1"]
    #[inline(always)]
    pub fn pwreniom1(&self) -> Pwreniom1R {
        Pwreniom1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power up IO Master 2"]
    #[inline(always)]
    pub fn pwreniom2(&self) -> Pwreniom2R {
        Pwreniom2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power up IO Master 3"]
    #[inline(always)]
    pub fn pwreniom3(&self) -> Pwreniom3R {
        Pwreniom3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power up IO Master 4"]
    #[inline(always)]
    pub fn pwreniom4(&self) -> Pwreniom4R {
        Pwreniom4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power up IO Master 5"]
    #[inline(always)]
    pub fn pwreniom5(&self) -> Pwreniom5R {
        Pwreniom5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power up IO Master 6"]
    #[inline(always)]
    pub fn pwreniom6(&self) -> Pwreniom6R {
        Pwreniom6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Power up IO Master 7"]
    #[inline(always)]
    pub fn pwreniom7(&self) -> Pwreniom7R {
        Pwreniom7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power up UART Controller 0"]
    #[inline(always)]
    pub fn pwrenuart0(&self) -> Pwrenuart0R {
        Pwrenuart0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Power up UART Controller 1"]
    #[inline(always)]
    pub fn pwrenuart1(&self) -> Pwrenuart1R {
        Pwrenuart1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power up UART Controller 2"]
    #[inline(always)]
    pub fn pwrenuart2(&self) -> Pwrenuart2R {
        Pwrenuart2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Power up UART Controller 3"]
    #[inline(always)]
    pub fn pwrenuart3(&self) -> Pwrenuart3R {
        Pwrenuart3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Power up ADC Digital Controller"]
    #[inline(always)]
    pub fn pwrenadc(&self) -> PwrenadcR {
        PwrenadcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Power up MSPI Controller0"]
    #[inline(always)]
    pub fn pwrenmspi0(&self) -> Pwrenmspi0R {
        Pwrenmspi0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Power up MSPI Controller1"]
    #[inline(always)]
    pub fn pwrenmspi1(&self) -> Pwrenmspi1R {
        Pwrenmspi1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Power up MSPI Controller2"]
    #[inline(always)]
    pub fn pwrenmspi2(&self) -> Pwrenmspi2R {
        Pwrenmspi2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Power up GFX controller"]
    #[inline(always)]
    pub fn pwrengfx(&self) -> PwrengfxR {
        PwrengfxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power up DISP controller"]
    #[inline(always)]
    pub fn pwrendisp(&self) -> PwrendispR {
        PwrendispR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Power up DISP PHY"]
    #[inline(always)]
    pub fn pwrendispphy(&self) -> PwrendispphyR {
        PwrendispphyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Power up CRYPTO module"]
    #[inline(always)]
    pub fn pwrencrypto(&self) -> PwrencryptoR {
        PwrencryptoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Power up SDIO controller"]
    #[inline(always)]
    pub fn pwrensdio(&self) -> PwrensdioR {
        PwrensdioR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Power up USB controller"]
    #[inline(always)]
    pub fn pwrenusb(&self) -> PwrenusbR {
        PwrenusbR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Power up USB PHY"]
    #[inline(always)]
    pub fn pwrenusbphy(&self) -> PwrenusbphyR {
        PwrenusbphyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Powerup DBG power domain"]
    #[inline(always)]
    pub fn pwrendbg(&self) -> PwrendbgR {
        PwrendbgR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power up IO Slave"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenios(&mut self) -> PwreniosW<DevpwrenSpec> {
        PwreniosW::new(self, 0)
    }
    #[doc = "Bit 1 - Power up IO Master 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwreniom0(&mut self) -> Pwreniom0W<DevpwrenSpec> {
        Pwreniom0W::new(self, 1)
    }
    #[doc = "Bit 2 - Power up IO Master 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwreniom1(&mut self) -> Pwreniom1W<DevpwrenSpec> {
        Pwreniom1W::new(self, 2)
    }
    #[doc = "Bit 3 - Power up IO Master 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwreniom2(&mut self) -> Pwreniom2W<DevpwrenSpec> {
        Pwreniom2W::new(self, 3)
    }
    #[doc = "Bit 4 - Power up IO Master 3"]
    #[inline(always)]
    #[must_use]
    pub fn pwreniom3(&mut self) -> Pwreniom3W<DevpwrenSpec> {
        Pwreniom3W::new(self, 4)
    }
    #[doc = "Bit 5 - Power up IO Master 4"]
    #[inline(always)]
    #[must_use]
    pub fn pwreniom4(&mut self) -> Pwreniom4W<DevpwrenSpec> {
        Pwreniom4W::new(self, 5)
    }
    #[doc = "Bit 6 - Power up IO Master 5"]
    #[inline(always)]
    #[must_use]
    pub fn pwreniom5(&mut self) -> Pwreniom5W<DevpwrenSpec> {
        Pwreniom5W::new(self, 6)
    }
    #[doc = "Bit 7 - Power up IO Master 6"]
    #[inline(always)]
    #[must_use]
    pub fn pwreniom6(&mut self) -> Pwreniom6W<DevpwrenSpec> {
        Pwreniom6W::new(self, 7)
    }
    #[doc = "Bit 8 - Power up IO Master 7"]
    #[inline(always)]
    #[must_use]
    pub fn pwreniom7(&mut self) -> Pwreniom7W<DevpwrenSpec> {
        Pwreniom7W::new(self, 8)
    }
    #[doc = "Bit 9 - Power up UART Controller 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenuart0(&mut self) -> Pwrenuart0W<DevpwrenSpec> {
        Pwrenuart0W::new(self, 9)
    }
    #[doc = "Bit 10 - Power up UART Controller 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenuart1(&mut self) -> Pwrenuart1W<DevpwrenSpec> {
        Pwrenuart1W::new(self, 10)
    }
    #[doc = "Bit 11 - Power up UART Controller 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenuart2(&mut self) -> Pwrenuart2W<DevpwrenSpec> {
        Pwrenuart2W::new(self, 11)
    }
    #[doc = "Bit 12 - Power up UART Controller 3"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenuart3(&mut self) -> Pwrenuart3W<DevpwrenSpec> {
        Pwrenuart3W::new(self, 12)
    }
    #[doc = "Bit 13 - Power up ADC Digital Controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenadc(&mut self) -> PwrenadcW<DevpwrenSpec> {
        PwrenadcW::new(self, 13)
    }
    #[doc = "Bit 14 - Power up MSPI Controller0"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenmspi0(&mut self) -> Pwrenmspi0W<DevpwrenSpec> {
        Pwrenmspi0W::new(self, 14)
    }
    #[doc = "Bit 15 - Power up MSPI Controller1"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenmspi1(&mut self) -> Pwrenmspi1W<DevpwrenSpec> {
        Pwrenmspi1W::new(self, 15)
    }
    #[doc = "Bit 16 - Power up MSPI Controller2"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenmspi2(&mut self) -> Pwrenmspi2W<DevpwrenSpec> {
        Pwrenmspi2W::new(self, 16)
    }
    #[doc = "Bit 17 - Power up GFX controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrengfx(&mut self) -> PwrengfxW<DevpwrenSpec> {
        PwrengfxW::new(self, 17)
    }
    #[doc = "Bit 18 - Power up DISP controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendisp(&mut self) -> PwrendispW<DevpwrenSpec> {
        PwrendispW::new(self, 18)
    }
    #[doc = "Bit 19 - Power up DISP PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendispphy(&mut self) -> PwrendispphyW<DevpwrenSpec> {
        PwrendispphyW::new(self, 19)
    }
    #[doc = "Bit 20 - Power up CRYPTO module"]
    #[inline(always)]
    #[must_use]
    pub fn pwrencrypto(&mut self) -> PwrencryptoW<DevpwrenSpec> {
        PwrencryptoW::new(self, 20)
    }
    #[doc = "Bit 21 - Power up SDIO controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrensdio(&mut self) -> PwrensdioW<DevpwrenSpec> {
        PwrensdioW::new(self, 21)
    }
    #[doc = "Bit 22 - Power up USB controller"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenusb(&mut self) -> PwrenusbW<DevpwrenSpec> {
        PwrenusbW::new(self, 22)
    }
    #[doc = "Bit 23 - Power up USB PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenusbphy(&mut self) -> PwrenusbphyW<DevpwrenSpec> {
        PwrenusbphyW::new(self, 23)
    }
    #[doc = "Bit 24 - Powerup DBG power domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendbg(&mut self) -> PwrendbgW<DevpwrenSpec> {
        PwrendbgW::new(self, 24)
    }
}
#[doc = "This enables various peripherals power domains.\n\nYou can [`read`](crate::Reg::read) this register and get [`devpwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devpwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevpwrenSpec;
impl crate::RegisterSpec for DevpwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devpwren::R`](R) reader structure"]
impl crate::Readable for DevpwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`devpwren::W`](W) writer structure"]
impl crate::Writable for DevpwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVPWREN to value 0x0010_0000"]
impl crate::Resettable for DevpwrenSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
