#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "OUT Endpoint 0 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0outIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep0outIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep0outIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0OutIntStat` reader - OUT Endpoint 0 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep0outIntStatR = crate::BitReader<Ep0outIntStat>;
impl Ep0outIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0outIntStat {
        match self.bits {
            false => Ep0outIntStat::Inactive,
            true => Ep0outIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep0outIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep0outIntStat::Active
    }
}
#[doc = "Field `EP0OutIntStat` writer - OUT Endpoint 0 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep0outIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep0outIntStat>;
impl<'a, REG> Ep0outIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0outIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0outIntStat::Active)
    }
}
#[doc = "OUT Endpoint 1 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep1outIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep1outIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep1outIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1OutIntStat` reader - OUT Endpoint 1 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep1outIntStatR = crate::BitReader<Ep1outIntStat>;
impl Ep1outIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep1outIntStat {
        match self.bits {
            false => Ep1outIntStat::Inactive,
            true => Ep1outIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep1outIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep1outIntStat::Active
    }
}
#[doc = "Field `EP1OutIntStat` writer - OUT Endpoint 1 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep1outIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep1outIntStat>;
impl<'a, REG> Ep1outIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1outIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1outIntStat::Active)
    }
}
#[doc = "OUT Endpoint 2 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep2outIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep2outIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep2outIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2OutIntStat` reader - OUT Endpoint 2 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep2outIntStatR = crate::BitReader<Ep2outIntStat>;
impl Ep2outIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep2outIntStat {
        match self.bits {
            false => Ep2outIntStat::Inactive,
            true => Ep2outIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep2outIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep2outIntStat::Active
    }
}
#[doc = "Field `EP2OutIntStat` writer - OUT Endpoint 2 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep2outIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep2outIntStat>;
impl<'a, REG> Ep2outIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2outIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2outIntStat::Active)
    }
}
#[doc = "OUT Endpoint 3 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep3outIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep3outIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep3outIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3OutIntStat` reader - OUT Endpoint 3 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep3outIntStatR = crate::BitReader<Ep3outIntStat>;
impl Ep3outIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep3outIntStat {
        match self.bits {
            false => Ep3outIntStat::Inactive,
            true => Ep3outIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep3outIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep3outIntStat::Active
    }
}
#[doc = "Field `EP3OutIntStat` writer - OUT Endpoint 3 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep3outIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep3outIntStat>;
impl<'a, REG> Ep3outIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3outIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3outIntStat::Active)
    }
}
#[doc = "OUT Endpoint 4 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep4outIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep4outIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep4outIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4OutIntStat` reader - OUT Endpoint 4 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep4outIntStatR = crate::BitReader<Ep4outIntStat>;
impl Ep4outIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep4outIntStat {
        match self.bits {
            false => Ep4outIntStat::Inactive,
            true => Ep4outIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep4outIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep4outIntStat::Active
    }
}
#[doc = "Field `EP4OutIntStat` writer - OUT Endpoint 4 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep4outIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep4outIntStat>;
impl<'a, REG> Ep4outIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4outIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4outIntStat::Active)
    }
}
#[doc = "OUT Endpoint 5 interrupt status. All interrupts are cleared when the register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep5outIntStat {
    #[doc = "0: Interrupt inactive."]
    Inactive = 0,
    #[doc = "1: Interrupt active."]
    Active = 1,
}
impl From<Ep5outIntStat> for bool {
    #[inline(always)]
    fn from(variant: Ep5outIntStat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP5OutIntStat` reader - OUT Endpoint 5 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep5outIntStatR = crate::BitReader<Ep5outIntStat>;
impl Ep5outIntStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep5outIntStat {
        match self.bits {
            false => Ep5outIntStat::Inactive,
            true => Ep5outIntStat::Active,
        }
    }
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ep5outIntStat::Inactive
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ep5outIntStat::Active
    }
}
#[doc = "Field `EP5OutIntStat` writer - OUT Endpoint 5 interrupt status. All interrupts are cleared when the register is read."]
pub type Ep5outIntStatW<'a, REG> = crate::BitWriter<'a, REG, Ep5outIntStat>;
impl<'a, REG> Ep5outIntStatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5outIntStat::Inactive)
    }
    #[doc = "Interrupt active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5outIntStat::Active)
    }
}
#[doc = "IN Endpoint 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep0inIntEn {
    #[doc = "0: IN Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: IN Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep0inIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep0inIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0InIntEn` reader - IN Endpoint 0 Interrupt Enable"]
pub type Ep0inIntEnR = crate::BitReader<Ep0inIntEn>;
impl Ep0inIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep0inIntEn {
        match self.bits {
            false => Ep0inIntEn::Dis,
            true => Ep0inIntEn::En,
        }
    }
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep0inIntEn::Dis
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep0inIntEn::En
    }
}
#[doc = "Field `EP0InIntEn` writer - IN Endpoint 0 Interrupt Enable"]
pub type Ep0inIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep0inIntEn>;
impl<'a, REG> Ep0inIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0inIntEn::Dis)
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep0inIntEn::En)
    }
}
#[doc = "IN Endpoint 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep1inIntEn {
    #[doc = "0: IN Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: IN Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep1inIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep1inIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1InIntEn` reader - IN Endpoint 1 Interrupt Enable"]
pub type Ep1inIntEnR = crate::BitReader<Ep1inIntEn>;
impl Ep1inIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep1inIntEn {
        match self.bits {
            false => Ep1inIntEn::Dis,
            true => Ep1inIntEn::En,
        }
    }
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep1inIntEn::Dis
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep1inIntEn::En
    }
}
#[doc = "Field `EP1InIntEn` writer - IN Endpoint 1 Interrupt Enable"]
pub type Ep1inIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep1inIntEn>;
impl<'a, REG> Ep1inIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1inIntEn::Dis)
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1inIntEn::En)
    }
}
#[doc = "IN Endpoint 2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep2inIntEn {
    #[doc = "0: IN Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: IN Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep2inIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep2inIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2InIntEn` reader - IN Endpoint 2 Interrupt Enable"]
pub type Ep2inIntEnR = crate::BitReader<Ep2inIntEn>;
impl Ep2inIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep2inIntEn {
        match self.bits {
            false => Ep2inIntEn::Dis,
            true => Ep2inIntEn::En,
        }
    }
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep2inIntEn::Dis
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep2inIntEn::En
    }
}
#[doc = "Field `EP2InIntEn` writer - IN Endpoint 2 Interrupt Enable"]
pub type Ep2inIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep2inIntEn>;
impl<'a, REG> Ep2inIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2inIntEn::Dis)
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2inIntEn::En)
    }
}
#[doc = "IN Endpoint 3 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep3inIntEn {
    #[doc = "0: IN Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: IN Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep3inIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep3inIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3InIntEn` reader - IN Endpoint 3 Interrupt Enable"]
pub type Ep3inIntEnR = crate::BitReader<Ep3inIntEn>;
impl Ep3inIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep3inIntEn {
        match self.bits {
            false => Ep3inIntEn::Dis,
            true => Ep3inIntEn::En,
        }
    }
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep3inIntEn::Dis
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep3inIntEn::En
    }
}
#[doc = "Field `EP3InIntEn` writer - IN Endpoint 3 Interrupt Enable"]
pub type Ep3inIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep3inIntEn>;
impl<'a, REG> Ep3inIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3inIntEn::Dis)
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3inIntEn::En)
    }
}
#[doc = "IN Endpoint 4 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep4inIntEn {
    #[doc = "0: IN Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: IN Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep4inIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep4inIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4InIntEn` reader - IN Endpoint 4 Interrupt Enable"]
pub type Ep4inIntEnR = crate::BitReader<Ep4inIntEn>;
impl Ep4inIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep4inIntEn {
        match self.bits {
            false => Ep4inIntEn::Dis,
            true => Ep4inIntEn::En,
        }
    }
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep4inIntEn::Dis
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep4inIntEn::En
    }
}
#[doc = "Field `EP4InIntEn` writer - IN Endpoint 4 Interrupt Enable"]
pub type Ep4inIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep4inIntEn>;
impl<'a, REG> Ep4inIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4inIntEn::Dis)
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4inIntEn::En)
    }
}
#[doc = "IN Endpoint 5 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep5inIntEn {
    #[doc = "0: IN Endpoint interrupt disabled."]
    Dis = 0,
    #[doc = "1: IN Endpoint interrupt enabled."]
    En = 1,
}
impl From<Ep5inIntEn> for bool {
    #[inline(always)]
    fn from(variant: Ep5inIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP5InIntEn` reader - IN Endpoint 5 Interrupt Enable"]
pub type Ep5inIntEnR = crate::BitReader<Ep5inIntEn>;
impl Ep5inIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep5inIntEn {
        match self.bits {
            false => Ep5inIntEn::Dis,
            true => Ep5inIntEn::En,
        }
    }
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ep5inIntEn::Dis
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ep5inIntEn::En
    }
}
#[doc = "Field `EP5InIntEn` writer - IN Endpoint 5 Interrupt Enable"]
pub type Ep5inIntEnW<'a, REG> = crate::BitWriter<'a, REG, Ep5inIntEn>;
impl<'a, REG> Ep5inIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN Endpoint interrupt disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5inIntEn::Dis)
    }
    #[doc = "IN Endpoint interrupt enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5inIntEn::En)
    }
}
impl R {
    #[doc = "Bit 0 - OUT Endpoint 0 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep0out_int_stat(&self) -> Ep0outIntStatR {
        Ep0outIntStatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUT Endpoint 1 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep1out_int_stat(&self) -> Ep1outIntStatR {
        Ep1outIntStatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OUT Endpoint 2 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep2out_int_stat(&self) -> Ep2outIntStatR {
        Ep2outIntStatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OUT Endpoint 3 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep3out_int_stat(&self) -> Ep3outIntStatR {
        Ep3outIntStatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Endpoint 4 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep4out_int_stat(&self) -> Ep4outIntStatR {
        Ep4outIntStatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OUT Endpoint 5 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    pub fn ep5out_int_stat(&self) -> Ep5outIntStatR {
        Ep5outIntStatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - IN Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ep0in_int_en(&self) -> Ep0inIntEnR {
        Ep0inIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IN Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ep1in_int_en(&self) -> Ep1inIntEnR {
        Ep1inIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ep2in_int_en(&self) -> Ep2inIntEnR {
        Ep2inIntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IN Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ep3in_int_en(&self) -> Ep3inIntEnR {
        Ep3inIntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IN Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ep4in_int_en(&self) -> Ep4inIntEnR {
        Ep4inIntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IN Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ep5in_int_en(&self) -> Ep5inIntEnR {
        Ep5inIntEnR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUT Endpoint 0 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep0out_int_stat(&mut self) -> Ep0outIntStatW<Cfg1Spec> {
        Ep0outIntStatW::new(self, 0)
    }
    #[doc = "Bit 1 - OUT Endpoint 1 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep1out_int_stat(&mut self) -> Ep1outIntStatW<Cfg1Spec> {
        Ep1outIntStatW::new(self, 1)
    }
    #[doc = "Bit 2 - OUT Endpoint 2 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep2out_int_stat(&mut self) -> Ep2outIntStatW<Cfg1Spec> {
        Ep2outIntStatW::new(self, 2)
    }
    #[doc = "Bit 3 - OUT Endpoint 3 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep3out_int_stat(&mut self) -> Ep3outIntStatW<Cfg1Spec> {
        Ep3outIntStatW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT Endpoint 4 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep4out_int_stat(&mut self) -> Ep4outIntStatW<Cfg1Spec> {
        Ep4outIntStatW::new(self, 4)
    }
    #[doc = "Bit 5 - OUT Endpoint 5 interrupt status. All interrupts are cleared when the register is read."]
    #[inline(always)]
    #[must_use]
    pub fn ep5out_int_stat(&mut self) -> Ep5outIntStatW<Cfg1Spec> {
        Ep5outIntStatW::new(self, 5)
    }
    #[doc = "Bit 16 - IN Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ep0in_int_en(&mut self) -> Ep0inIntEnW<Cfg1Spec> {
        Ep0inIntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - IN Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ep1in_int_en(&mut self) -> Ep1inIntEnW<Cfg1Spec> {
        Ep1inIntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - IN Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ep2in_int_en(&mut self) -> Ep2inIntEnW<Cfg1Spec> {
        Ep2inIntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - IN Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ep3in_int_en(&mut self) -> Ep3inIntEnW<Cfg1Spec> {
        Ep3inIntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - IN Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ep4in_int_en(&mut self) -> Ep4inIntEnW<Cfg1Spec> {
        Ep4inIntEnW::new(self, 20)
    }
    #[doc = "Bit 21 - IN Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ep5in_int_en(&mut self) -> Ep5inIntEnW<Cfg1Spec> {
        Ep5inIntEnW::new(self, 21)
    }
}
#[doc = "Indicates which of the IN Endpoint 1 - 5 interrupts and the single Endpoint 0 interrupt are currently active. Also indicates which of the interrupts for OUT Endpoint 1 - 5 are currently active. All active interrupts are cleared when this register is read.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
