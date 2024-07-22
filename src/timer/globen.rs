#[doc = "Register `GLOBEN` reader"]
pub type R = crate::R<GlobenSpec>;
#[doc = "Register `GLOBEN` writer"]
pub type W = crate::W<GlobenSpec>;
#[doc = "Alternate enable for timer 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb0 {
    #[doc = "1: Timer Enabled. TMR0EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 0."]
    Dis = 0,
}
impl From<Enb0> for bool {
    #[inline(always)]
    fn from(variant: Enb0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB0` reader - Alternate enable for timer 0"]
pub type Enb0R = crate::BitReader<Enb0>;
impl Enb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb0 {
        match self.bits {
            true => Enb0::En,
            false => Enb0::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR0EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb0::En
    }
    #[doc = "Disable TIMER 0."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb0::Dis
    }
}
#[doc = "Field `ENB0` writer - Alternate enable for timer 0"]
pub type Enb0W<'a, REG> = crate::BitWriter<'a, REG, Enb0>;
impl<'a, REG> Enb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR0EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb0::En)
    }
    #[doc = "Disable TIMER 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb0::Dis)
    }
}
#[doc = "Alternate enable for timer 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb1 {
    #[doc = "1: Timer Enabled. TMR1EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 1."]
    Dis = 0,
}
impl From<Enb1> for bool {
    #[inline(always)]
    fn from(variant: Enb1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB1` reader - Alternate enable for timer 1"]
pub type Enb1R = crate::BitReader<Enb1>;
impl Enb1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb1 {
        match self.bits {
            true => Enb1::En,
            false => Enb1::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR1EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb1::En
    }
    #[doc = "Disable TIMER 1."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb1::Dis
    }
}
#[doc = "Field `ENB1` writer - Alternate enable for timer 1"]
pub type Enb1W<'a, REG> = crate::BitWriter<'a, REG, Enb1>;
impl<'a, REG> Enb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR1EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb1::En)
    }
    #[doc = "Disable TIMER 1."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb1::Dis)
    }
}
#[doc = "Alternate enable for timer 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb2 {
    #[doc = "1: Timer Enabled. TMR2EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 2."]
    Dis = 0,
}
impl From<Enb2> for bool {
    #[inline(always)]
    fn from(variant: Enb2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB2` reader - Alternate enable for timer 2"]
pub type Enb2R = crate::BitReader<Enb2>;
impl Enb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb2 {
        match self.bits {
            true => Enb2::En,
            false => Enb2::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR2EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb2::En
    }
    #[doc = "Disable TIMER 2."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb2::Dis
    }
}
#[doc = "Field `ENB2` writer - Alternate enable for timer 2"]
pub type Enb2W<'a, REG> = crate::BitWriter<'a, REG, Enb2>;
impl<'a, REG> Enb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR2EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb2::En)
    }
    #[doc = "Disable TIMER 2."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb2::Dis)
    }
}
#[doc = "Alternate enable for timer 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb3 {
    #[doc = "1: Timer Enabled. TMR3EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 3."]
    Dis = 0,
}
impl From<Enb3> for bool {
    #[inline(always)]
    fn from(variant: Enb3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB3` reader - Alternate enable for timer 3"]
pub type Enb3R = crate::BitReader<Enb3>;
impl Enb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb3 {
        match self.bits {
            true => Enb3::En,
            false => Enb3::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR3EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb3::En
    }
    #[doc = "Disable TIMER 3."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb3::Dis
    }
}
#[doc = "Field `ENB3` writer - Alternate enable for timer 3"]
pub type Enb3W<'a, REG> = crate::BitWriter<'a, REG, Enb3>;
impl<'a, REG> Enb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR3EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb3::En)
    }
    #[doc = "Disable TIMER 3."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb3::Dis)
    }
}
#[doc = "Alternate enable for timer 4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb4 {
    #[doc = "1: Timer Enabled. TMR4EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 4."]
    Dis = 0,
}
impl From<Enb4> for bool {
    #[inline(always)]
    fn from(variant: Enb4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB4` reader - Alternate enable for timer 4"]
pub type Enb4R = crate::BitReader<Enb4>;
impl Enb4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb4 {
        match self.bits {
            true => Enb4::En,
            false => Enb4::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR4EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb4::En
    }
    #[doc = "Disable TIMER 4."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb4::Dis
    }
}
#[doc = "Field `ENB4` writer - Alternate enable for timer 4"]
pub type Enb4W<'a, REG> = crate::BitWriter<'a, REG, Enb4>;
impl<'a, REG> Enb4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR4EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb4::En)
    }
    #[doc = "Disable TIMER 4."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb4::Dis)
    }
}
#[doc = "Alternate enable for timer 5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb5 {
    #[doc = "1: Timer Enabled. TMR5EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 5."]
    Dis = 0,
}
impl From<Enb5> for bool {
    #[inline(always)]
    fn from(variant: Enb5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB5` reader - Alternate enable for timer 5"]
pub type Enb5R = crate::BitReader<Enb5>;
impl Enb5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb5 {
        match self.bits {
            true => Enb5::En,
            false => Enb5::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR5EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb5::En
    }
    #[doc = "Disable TIMER 5."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb5::Dis
    }
}
#[doc = "Field `ENB5` writer - Alternate enable for timer 5"]
pub type Enb5W<'a, REG> = crate::BitWriter<'a, REG, Enb5>;
impl<'a, REG> Enb5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR5EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb5::En)
    }
    #[doc = "Disable TIMER 5."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb5::Dis)
    }
}
#[doc = "Alternate enable for timer 6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb6 {
    #[doc = "1: Timer Enabled. TMR6EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 6."]
    Dis = 0,
}
impl From<Enb6> for bool {
    #[inline(always)]
    fn from(variant: Enb6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB6` reader - Alternate enable for timer 6"]
pub type Enb6R = crate::BitReader<Enb6>;
impl Enb6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb6 {
        match self.bits {
            true => Enb6::En,
            false => Enb6::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR6EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb6::En
    }
    #[doc = "Disable TIMER 6."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb6::Dis
    }
}
#[doc = "Field `ENB6` writer - Alternate enable for timer 6"]
pub type Enb6W<'a, REG> = crate::BitWriter<'a, REG, Enb6>;
impl<'a, REG> Enb6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR6EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb6::En)
    }
    #[doc = "Disable TIMER 6."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb6::Dis)
    }
}
#[doc = "Alternate enable for timer 7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb7 {
    #[doc = "1: Timer Enabled. TMR7EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 7."]
    Dis = 0,
}
impl From<Enb7> for bool {
    #[inline(always)]
    fn from(variant: Enb7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB7` reader - Alternate enable for timer 7"]
pub type Enb7R = crate::BitReader<Enb7>;
impl Enb7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb7 {
        match self.bits {
            true => Enb7::En,
            false => Enb7::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR7EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb7::En
    }
    #[doc = "Disable TIMER 7."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb7::Dis
    }
}
#[doc = "Field `ENB7` writer - Alternate enable for timer 7"]
pub type Enb7W<'a, REG> = crate::BitWriter<'a, REG, Enb7>;
impl<'a, REG> Enb7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR7EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb7::En)
    }
    #[doc = "Disable TIMER 7."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb7::Dis)
    }
}
#[doc = "Alternate enable for timer 8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb8 {
    #[doc = "1: Timer Enabled. TMR8EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 8."]
    Dis = 0,
}
impl From<Enb8> for bool {
    #[inline(always)]
    fn from(variant: Enb8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB8` reader - Alternate enable for timer 8"]
pub type Enb8R = crate::BitReader<Enb8>;
impl Enb8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb8 {
        match self.bits {
            true => Enb8::En,
            false => Enb8::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR8EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb8::En
    }
    #[doc = "Disable TIMER 8."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb8::Dis
    }
}
#[doc = "Field `ENB8` writer - Alternate enable for timer 8"]
pub type Enb8W<'a, REG> = crate::BitWriter<'a, REG, Enb8>;
impl<'a, REG> Enb8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR8EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb8::En)
    }
    #[doc = "Disable TIMER 8."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb8::Dis)
    }
}
#[doc = "Alternate enable for timer 9\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb9 {
    #[doc = "1: Timer Enabled. TMR9EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 9."]
    Dis = 0,
}
impl From<Enb9> for bool {
    #[inline(always)]
    fn from(variant: Enb9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB9` reader - Alternate enable for timer 9"]
pub type Enb9R = crate::BitReader<Enb9>;
impl Enb9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb9 {
        match self.bits {
            true => Enb9::En,
            false => Enb9::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR9EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb9::En
    }
    #[doc = "Disable TIMER 9."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb9::Dis
    }
}
#[doc = "Field `ENB9` writer - Alternate enable for timer 9"]
pub type Enb9W<'a, REG> = crate::BitWriter<'a, REG, Enb9>;
impl<'a, REG> Enb9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR9EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb9::En)
    }
    #[doc = "Disable TIMER 9."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb9::Dis)
    }
}
#[doc = "Alternate enable for timer 10\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb10 {
    #[doc = "1: Timer Enabled. TMR10EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 10."]
    Dis = 0,
}
impl From<Enb10> for bool {
    #[inline(always)]
    fn from(variant: Enb10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB10` reader - Alternate enable for timer 10"]
pub type Enb10R = crate::BitReader<Enb10>;
impl Enb10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb10 {
        match self.bits {
            true => Enb10::En,
            false => Enb10::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR10EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb10::En
    }
    #[doc = "Disable TIMER 10."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb10::Dis
    }
}
#[doc = "Field `ENB10` writer - Alternate enable for timer 10"]
pub type Enb10W<'a, REG> = crate::BitWriter<'a, REG, Enb10>;
impl<'a, REG> Enb10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR10EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb10::En)
    }
    #[doc = "Disable TIMER 10."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb10::Dis)
    }
}
#[doc = "Alternate enable for timer 11\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb11 {
    #[doc = "1: Timer Enabled. TMR11EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 11."]
    Dis = 0,
}
impl From<Enb11> for bool {
    #[inline(always)]
    fn from(variant: Enb11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB11` reader - Alternate enable for timer 11"]
pub type Enb11R = crate::BitReader<Enb11>;
impl Enb11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb11 {
        match self.bits {
            true => Enb11::En,
            false => Enb11::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR11EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb11::En
    }
    #[doc = "Disable TIMER 11."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb11::Dis
    }
}
#[doc = "Field `ENB11` writer - Alternate enable for timer 11"]
pub type Enb11W<'a, REG> = crate::BitWriter<'a, REG, Enb11>;
impl<'a, REG> Enb11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR11EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb11::En)
    }
    #[doc = "Disable TIMER 11."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb11::Dis)
    }
}
#[doc = "Alternate enable for timer 12\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb12 {
    #[doc = "1: Timer Enabled. TMR12EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 12."]
    Dis = 0,
}
impl From<Enb12> for bool {
    #[inline(always)]
    fn from(variant: Enb12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB12` reader - Alternate enable for timer 12"]
pub type Enb12R = crate::BitReader<Enb12>;
impl Enb12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb12 {
        match self.bits {
            true => Enb12::En,
            false => Enb12::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR12EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb12::En
    }
    #[doc = "Disable TIMER 12."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb12::Dis
    }
}
#[doc = "Field `ENB12` writer - Alternate enable for timer 12"]
pub type Enb12W<'a, REG> = crate::BitWriter<'a, REG, Enb12>;
impl<'a, REG> Enb12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR12EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb12::En)
    }
    #[doc = "Disable TIMER 12."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb12::Dis)
    }
}
#[doc = "Alternate enable for timer 13\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb13 {
    #[doc = "1: Timer Enabled. TMR13EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 13."]
    Dis = 0,
}
impl From<Enb13> for bool {
    #[inline(always)]
    fn from(variant: Enb13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB13` reader - Alternate enable for timer 13"]
pub type Enb13R = crate::BitReader<Enb13>;
impl Enb13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb13 {
        match self.bits {
            true => Enb13::En,
            false => Enb13::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR13EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb13::En
    }
    #[doc = "Disable TIMER 13."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb13::Dis
    }
}
#[doc = "Field `ENB13` writer - Alternate enable for timer 13"]
pub type Enb13W<'a, REG> = crate::BitWriter<'a, REG, Enb13>;
impl<'a, REG> Enb13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR13EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb13::En)
    }
    #[doc = "Disable TIMER 13."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb13::Dis)
    }
}
#[doc = "Alternate enable for timer 14\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb14 {
    #[doc = "1: Timer Enabled. TMR14EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 14."]
    Dis = 0,
}
impl From<Enb14> for bool {
    #[inline(always)]
    fn from(variant: Enb14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB14` reader - Alternate enable for timer 14"]
pub type Enb14R = crate::BitReader<Enb14>;
impl Enb14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb14 {
        match self.bits {
            true => Enb14::En,
            false => Enb14::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR14EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb14::En
    }
    #[doc = "Disable TIMER 14."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb14::Dis
    }
}
#[doc = "Field `ENB14` writer - Alternate enable for timer 14"]
pub type Enb14W<'a, REG> = crate::BitWriter<'a, REG, Enb14>;
impl<'a, REG> Enb14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR14EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb14::En)
    }
    #[doc = "Disable TIMER 14."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb14::Dis)
    }
}
#[doc = "Alternate enable for timer 15\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb15 {
    #[doc = "1: Timer Enabled. TMR15EN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER 15."]
    Dis = 0,
}
impl From<Enb15> for bool {
    #[inline(always)]
    fn from(variant: Enb15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB15` reader - Alternate enable for timer 15"]
pub type Enb15R = crate::BitReader<Enb15>;
impl Enb15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb15 {
        match self.bits {
            true => Enb15::En,
            false => Enb15::Dis,
        }
    }
    #[doc = "Timer Enabled. TMR15EN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enb15::En
    }
    #[doc = "Disable TIMER 15."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enb15::Dis
    }
}
#[doc = "Field `ENB15` writer - Alternate enable for timer 15"]
pub type Enb15W<'a, REG> = crate::BitWriter<'a, REG, Enb15>;
impl<'a, REG> Enb15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMR15EN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enb15::En)
    }
    #[doc = "Disable TIMER 15."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enb15::Dis)
    }
}
#[doc = "Override to enable all GPIO inputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enableallinputs {
    #[doc = "1: Override to enable all inputs from GPIO"]
    En = 1,
    #[doc = "0: Normal mode where inputs from GPIO are enabled based on enabled clock and triggers."]
    Dis = 0,
}
impl From<Enableallinputs> for bool {
    #[inline(always)]
    fn from(variant: Enableallinputs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLEALLINPUTS` reader - Override to enable all GPIO inputs"]
pub type EnableallinputsR = crate::BitReader<Enableallinputs>;
impl EnableallinputsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enableallinputs {
        match self.bits {
            true => Enableallinputs::En,
            false => Enableallinputs::Dis,
        }
    }
    #[doc = "Override to enable all inputs from GPIO"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Enableallinputs::En
    }
    #[doc = "Normal mode where inputs from GPIO are enabled based on enabled clock and triggers."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Enableallinputs::Dis
    }
}
#[doc = "Field `ENABLEALLINPUTS` writer - Override to enable all GPIO inputs"]
pub type EnableallinputsW<'a, REG> = crate::BitWriter<'a, REG, Enableallinputs>;
impl<'a, REG> EnableallinputsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Override to enable all inputs from GPIO"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Enableallinputs::En)
    }
    #[doc = "Normal mode where inputs from GPIO are enabled based on enabled clock and triggers."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enableallinputs::Dis)
    }
}
#[doc = "Audio ADC controls enable for timer 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Audadcen {
    #[doc = "1: Timer Enabled. TMREN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER ."]
    Dis = 0,
}
impl From<Audadcen> for bool {
    #[inline(always)]
    fn from(variant: Audadcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDADCEN` reader - Audio ADC controls enable for timer 6"]
pub type AudadcenR = crate::BitReader<Audadcen>;
impl AudadcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Audadcen {
        match self.bits {
            true => Audadcen::En,
            false => Audadcen::Dis,
        }
    }
    #[doc = "Timer Enabled. TMREN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Audadcen::En
    }
    #[doc = "Disable TIMER ."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Audadcen::Dis
    }
}
#[doc = "Field `AUDADCEN` writer - Audio ADC controls enable for timer 6"]
pub type AudadcenW<'a, REG> = crate::BitWriter<'a, REG, Audadcen>;
impl<'a, REG> AudadcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMREN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Audadcen::En)
    }
    #[doc = "Disable TIMER ."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Audadcen::Dis)
    }
}
#[doc = "ADC controls enable for timer 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcen {
    #[doc = "1: Timer Enabled. TMREN enable is used."]
    En = 1,
    #[doc = "0: Disable TIMER ."]
    Dis = 0,
}
impl From<Adcen> for bool {
    #[inline(always)]
    fn from(variant: Adcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC controls enable for timer 7"]
pub type AdcenR = crate::BitReader<Adcen>;
impl AdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcen {
        match self.bits {
            true => Adcen::En,
            false => Adcen::Dis,
        }
    }
    #[doc = "Timer Enabled. TMREN enable is used."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Adcen::En
    }
    #[doc = "Disable TIMER ."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Adcen::Dis
    }
}
#[doc = "Field `ADCEN` writer - ADC controls enable for timer 7"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG, Adcen>;
impl<'a, REG> AdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer Enabled. TMREN enable is used."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::En)
    }
    #[doc = "Disable TIMER ."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Adcen::Dis)
    }
}
impl R {
    #[doc = "Bit 0 - Alternate enable for timer 0"]
    #[inline(always)]
    pub fn enb0(&self) -> Enb0R {
        Enb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alternate enable for timer 1"]
    #[inline(always)]
    pub fn enb1(&self) -> Enb1R {
        Enb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternate enable for timer 2"]
    #[inline(always)]
    pub fn enb2(&self) -> Enb2R {
        Enb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alternate enable for timer 3"]
    #[inline(always)]
    pub fn enb3(&self) -> Enb3R {
        Enb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Alternate enable for timer 4"]
    #[inline(always)]
    pub fn enb4(&self) -> Enb4R {
        Enb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alternate enable for timer 5"]
    #[inline(always)]
    pub fn enb5(&self) -> Enb5R {
        Enb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alternate enable for timer 6"]
    #[inline(always)]
    pub fn enb6(&self) -> Enb6R {
        Enb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Alternate enable for timer 7"]
    #[inline(always)]
    pub fn enb7(&self) -> Enb7R {
        Enb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alternate enable for timer 8"]
    #[inline(always)]
    pub fn enb8(&self) -> Enb8R {
        Enb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alternate enable for timer 9"]
    #[inline(always)]
    pub fn enb9(&self) -> Enb9R {
        Enb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Alternate enable for timer 10"]
    #[inline(always)]
    pub fn enb10(&self) -> Enb10R {
        Enb10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Alternate enable for timer 11"]
    #[inline(always)]
    pub fn enb11(&self) -> Enb11R {
        Enb11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alternate enable for timer 12"]
    #[inline(always)]
    pub fn enb12(&self) -> Enb12R {
        Enb12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alternate enable for timer 13"]
    #[inline(always)]
    pub fn enb13(&self) -> Enb13R {
        Enb13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Alternate enable for timer 14"]
    #[inline(always)]
    pub fn enb14(&self) -> Enb14R {
        Enb14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternate enable for timer 15"]
    #[inline(always)]
    pub fn enb15(&self) -> Enb15R {
        Enb15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - Override to enable all GPIO inputs"]
    #[inline(always)]
    pub fn enableallinputs(&self) -> EnableallinputsR {
        EnableallinputsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Audio ADC controls enable for timer 6"]
    #[inline(always)]
    pub fn audadcen(&self) -> AudadcenR {
        AudadcenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC controls enable for timer 7"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate enable for timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn enb0(&mut self) -> Enb0W<GlobenSpec> {
        Enb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Alternate enable for timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn enb1(&mut self) -> Enb1W<GlobenSpec> {
        Enb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Alternate enable for timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn enb2(&mut self) -> Enb2W<GlobenSpec> {
        Enb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Alternate enable for timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn enb3(&mut self) -> Enb3W<GlobenSpec> {
        Enb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Alternate enable for timer 4"]
    #[inline(always)]
    #[must_use]
    pub fn enb4(&mut self) -> Enb4W<GlobenSpec> {
        Enb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Alternate enable for timer 5"]
    #[inline(always)]
    #[must_use]
    pub fn enb5(&mut self) -> Enb5W<GlobenSpec> {
        Enb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Alternate enable for timer 6"]
    #[inline(always)]
    #[must_use]
    pub fn enb6(&mut self) -> Enb6W<GlobenSpec> {
        Enb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Alternate enable for timer 7"]
    #[inline(always)]
    #[must_use]
    pub fn enb7(&mut self) -> Enb7W<GlobenSpec> {
        Enb7W::new(self, 7)
    }
    #[doc = "Bit 8 - Alternate enable for timer 8"]
    #[inline(always)]
    #[must_use]
    pub fn enb8(&mut self) -> Enb8W<GlobenSpec> {
        Enb8W::new(self, 8)
    }
    #[doc = "Bit 9 - Alternate enable for timer 9"]
    #[inline(always)]
    #[must_use]
    pub fn enb9(&mut self) -> Enb9W<GlobenSpec> {
        Enb9W::new(self, 9)
    }
    #[doc = "Bit 10 - Alternate enable for timer 10"]
    #[inline(always)]
    #[must_use]
    pub fn enb10(&mut self) -> Enb10W<GlobenSpec> {
        Enb10W::new(self, 10)
    }
    #[doc = "Bit 11 - Alternate enable for timer 11"]
    #[inline(always)]
    #[must_use]
    pub fn enb11(&mut self) -> Enb11W<GlobenSpec> {
        Enb11W::new(self, 11)
    }
    #[doc = "Bit 12 - Alternate enable for timer 12"]
    #[inline(always)]
    #[must_use]
    pub fn enb12(&mut self) -> Enb12W<GlobenSpec> {
        Enb12W::new(self, 12)
    }
    #[doc = "Bit 13 - Alternate enable for timer 13"]
    #[inline(always)]
    #[must_use]
    pub fn enb13(&mut self) -> Enb13W<GlobenSpec> {
        Enb13W::new(self, 13)
    }
    #[doc = "Bit 14 - Alternate enable for timer 14"]
    #[inline(always)]
    #[must_use]
    pub fn enb14(&mut self) -> Enb14W<GlobenSpec> {
        Enb14W::new(self, 14)
    }
    #[doc = "Bit 15 - Alternate enable for timer 15"]
    #[inline(always)]
    #[must_use]
    pub fn enb15(&mut self) -> Enb15W<GlobenSpec> {
        Enb15W::new(self, 15)
    }
    #[doc = "Bit 29 - Override to enable all GPIO inputs"]
    #[inline(always)]
    #[must_use]
    pub fn enableallinputs(&mut self) -> EnableallinputsW<GlobenSpec> {
        EnableallinputsW::new(self, 29)
    }
    #[doc = "Bit 30 - Audio ADC controls enable for timer 6"]
    #[inline(always)]
    #[must_use]
    pub fn audadcen(&mut self) -> AudadcenW<GlobenSpec> {
        AudadcenW::new(self, 30)
    }
    #[doc = "Bit 31 - ADC controls enable for timer 7"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> AdcenW<GlobenSpec> {
        AdcenW::new(self, 31)
    }
}
#[doc = "Alternate enables for all TIMERs.\n\nYou can [`read`](crate::Reg::read) this register and get [`globen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`globen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobenSpec;
impl crate::RegisterSpec for GlobenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globen::R`](R) reader structure"]
impl crate::Readable for GlobenSpec {}
#[doc = "`write(|w| ..)` method takes [`globen::W`](W) writer structure"]
impl crate::Writable for GlobenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBEN to value 0xffff"]
impl crate::Resettable for GlobenSpec {
    const RESET_VALUE: u32 = 0xffff;
}
