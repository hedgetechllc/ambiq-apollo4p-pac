#[doc = "Register `OUTCFG10` reader"]
pub type R = crate::R<Outcfg10Spec>;
#[doc = "Register `OUTCFG10` writer"]
pub type W = crate::W<Outcfg10Spec>;
#[doc = "Pad output 40 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg40 {
    #[doc = "0: Output is Timer 0, output 0"]
    Timer00 = 0,
    #[doc = "1: Output is Timer 0, output 1"]
    Timer01 = 1,
    #[doc = "2: Output is Timer 1, output 0"]
    Timer10 = 2,
    #[doc = "3: Output is Timer 1, output 1"]
    Timer11 = 3,
    #[doc = "4: Output is Timer 2, output 0"]
    Timer20 = 4,
    #[doc = "5: Output is Timer 2, output 1"]
    Timer21 = 5,
    #[doc = "6: Output is Timer 3, output 0"]
    Timer30 = 6,
    #[doc = "7: Output is Timer 3, output 1"]
    Timer31 = 7,
    #[doc = "8: Output is Timer 4, output 0"]
    Timer40 = 8,
    #[doc = "9: Output is Timer 4, output 1"]
    Timer41 = 9,
    #[doc = "10: Output is Timer 5, output 0"]
    Timer50 = 10,
    #[doc = "11: Output is Timer 5, output 1"]
    Timer51 = 11,
    #[doc = "12: Output is Timer 6, output 0"]
    Timer60 = 12,
    #[doc = "13: Output is Timer 6, output 1"]
    Timer61 = 13,
    #[doc = "14: Output is Timer 7, output 0"]
    Timer70 = 14,
    #[doc = "15: Output is Timer 7, output 1"]
    Timer71 = 15,
    #[doc = "16: Output is Timer 8, output 0"]
    Timer80 = 16,
    #[doc = "17: Output is Timer 8, output 1"]
    Timer81 = 17,
    #[doc = "18: Output is Timer 9, output 0"]
    Timer90 = 18,
    #[doc = "19: Output is Timer 9, output 1"]
    Timer91 = 19,
    #[doc = "20: Output is Timer 10, output 0"]
    Timer100 = 20,
    #[doc = "21: Output is Timer 10, output 1"]
    Timer101 = 21,
    #[doc = "22: Output is Timer 11, output 0"]
    Timer110 = 22,
    #[doc = "23: Output is Timer 11, output 1"]
    Timer111 = 23,
    #[doc = "24: Output is Timer 12, output 0"]
    Timer120 = 24,
    #[doc = "25: Output is Timer 12, output 1"]
    Timer121 = 25,
    #[doc = "26: Output is Timer 13, output 0"]
    Timer130 = 26,
    #[doc = "27: Output is Timer 13, output 1"]
    Timer131 = 27,
    #[doc = "28: Output is Timer 14, output 0"]
    Timer140 = 28,
    #[doc = "29: Output is Timer 14, output 1"]
    Timer141 = 29,
    #[doc = "30: Output is Timer 15, output 0"]
    Timer150 = 30,
    #[doc = "31: Output is Timer 15, output 1"]
    Timer151 = 31,
    #[doc = "32: Output is STimer 0"]
    Stimer0 = 32,
    #[doc = "33: Output is STimer 1"]
    Stimer1 = 33,
    #[doc = "34: Output is STimer 2"]
    Stimer2 = 34,
    #[doc = "35: Output is STimer 3"]
    Stimer3 = 35,
    #[doc = "36: Output is STimer 4"]
    Stimer4 = 36,
    #[doc = "37: Output is STimer 5"]
    Stimer5 = 37,
    #[doc = "38: Output is STimer 6"]
    Stimer6 = 38,
    #[doc = "39: Output is STimer 7"]
    Stimer7 = 39,
    #[doc = "63: Output is disabled"]
    Disabled = 63,
}
impl From<Outcfg40> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg40) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg40 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg40 {}
#[doc = "Field `OUTCFG40` reader - Pad output 40 configuration"]
pub type Outcfg40R = crate::FieldReader<Outcfg40>;
impl Outcfg40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg40> {
        match self.bits {
            0 => Some(Outcfg40::Timer00),
            1 => Some(Outcfg40::Timer01),
            2 => Some(Outcfg40::Timer10),
            3 => Some(Outcfg40::Timer11),
            4 => Some(Outcfg40::Timer20),
            5 => Some(Outcfg40::Timer21),
            6 => Some(Outcfg40::Timer30),
            7 => Some(Outcfg40::Timer31),
            8 => Some(Outcfg40::Timer40),
            9 => Some(Outcfg40::Timer41),
            10 => Some(Outcfg40::Timer50),
            11 => Some(Outcfg40::Timer51),
            12 => Some(Outcfg40::Timer60),
            13 => Some(Outcfg40::Timer61),
            14 => Some(Outcfg40::Timer70),
            15 => Some(Outcfg40::Timer71),
            16 => Some(Outcfg40::Timer80),
            17 => Some(Outcfg40::Timer81),
            18 => Some(Outcfg40::Timer90),
            19 => Some(Outcfg40::Timer91),
            20 => Some(Outcfg40::Timer100),
            21 => Some(Outcfg40::Timer101),
            22 => Some(Outcfg40::Timer110),
            23 => Some(Outcfg40::Timer111),
            24 => Some(Outcfg40::Timer120),
            25 => Some(Outcfg40::Timer121),
            26 => Some(Outcfg40::Timer130),
            27 => Some(Outcfg40::Timer131),
            28 => Some(Outcfg40::Timer140),
            29 => Some(Outcfg40::Timer141),
            30 => Some(Outcfg40::Timer150),
            31 => Some(Outcfg40::Timer151),
            32 => Some(Outcfg40::Stimer0),
            33 => Some(Outcfg40::Stimer1),
            34 => Some(Outcfg40::Stimer2),
            35 => Some(Outcfg40::Stimer3),
            36 => Some(Outcfg40::Stimer4),
            37 => Some(Outcfg40::Stimer5),
            38 => Some(Outcfg40::Stimer6),
            39 => Some(Outcfg40::Stimer7),
            63 => Some(Outcfg40::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg40::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg40::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg40::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg40::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg40::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg40::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg40::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg40::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg40::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg40::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg40::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg40::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg40::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg40::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg40::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg40::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg40::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg40::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg40::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg40::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg40::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg40::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg40::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg40::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg40::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg40::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg40::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg40::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg40::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg40::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg40::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg40::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg40::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg40::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg40::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg40::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg40::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg40::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg40::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg40::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg40::Disabled
    }
}
#[doc = "Field `OUTCFG40` writer - Pad output 40 configuration"]
pub type Outcfg40W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg40>;
impl<'a, REG> Outcfg40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg40::Disabled)
    }
}
#[doc = "Pad output 41 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg41 {
    #[doc = "0: Output is Timer 0, output 0"]
    Timer00 = 0,
    #[doc = "1: Output is Timer 0, output 1"]
    Timer01 = 1,
    #[doc = "2: Output is Timer 1, output 0"]
    Timer10 = 2,
    #[doc = "3: Output is Timer 1, output 1"]
    Timer11 = 3,
    #[doc = "4: Output is Timer 2, output 0"]
    Timer20 = 4,
    #[doc = "5: Output is Timer 2, output 1"]
    Timer21 = 5,
    #[doc = "6: Output is Timer 3, output 0"]
    Timer30 = 6,
    #[doc = "7: Output is Timer 3, output 1"]
    Timer31 = 7,
    #[doc = "8: Output is Timer 4, output 0"]
    Timer40 = 8,
    #[doc = "9: Output is Timer 4, output 1"]
    Timer41 = 9,
    #[doc = "10: Output is Timer 5, output 0"]
    Timer50 = 10,
    #[doc = "11: Output is Timer 5, output 1"]
    Timer51 = 11,
    #[doc = "12: Output is Timer 6, output 0"]
    Timer60 = 12,
    #[doc = "13: Output is Timer 6, output 1"]
    Timer61 = 13,
    #[doc = "14: Output is Timer 7, output 0"]
    Timer70 = 14,
    #[doc = "15: Output is Timer 7, output 1"]
    Timer71 = 15,
    #[doc = "16: Output is Timer 8, output 0"]
    Timer80 = 16,
    #[doc = "17: Output is Timer 8, output 1"]
    Timer81 = 17,
    #[doc = "18: Output is Timer 9, output 0"]
    Timer90 = 18,
    #[doc = "19: Output is Timer 9, output 1"]
    Timer91 = 19,
    #[doc = "20: Output is Timer 10, output 0"]
    Timer100 = 20,
    #[doc = "21: Output is Timer 10, output 1"]
    Timer101 = 21,
    #[doc = "22: Output is Timer 11, output 0"]
    Timer110 = 22,
    #[doc = "23: Output is Timer 11, output 1"]
    Timer111 = 23,
    #[doc = "24: Output is Timer 12, output 0"]
    Timer120 = 24,
    #[doc = "25: Output is Timer 12, output 1"]
    Timer121 = 25,
    #[doc = "26: Output is Timer 13, output 0"]
    Timer130 = 26,
    #[doc = "27: Output is Timer 13, output 1"]
    Timer131 = 27,
    #[doc = "28: Output is Timer 14, output 0"]
    Timer140 = 28,
    #[doc = "29: Output is Timer 14, output 1"]
    Timer141 = 29,
    #[doc = "30: Output is Timer 15, output 0"]
    Timer150 = 30,
    #[doc = "31: Output is Timer 15, output 1"]
    Timer151 = 31,
    #[doc = "32: Output is STimer 0"]
    Stimer0 = 32,
    #[doc = "33: Output is STimer 1"]
    Stimer1 = 33,
    #[doc = "34: Output is STimer 2"]
    Stimer2 = 34,
    #[doc = "35: Output is STimer 3"]
    Stimer3 = 35,
    #[doc = "36: Output is STimer 4"]
    Stimer4 = 36,
    #[doc = "37: Output is STimer 5"]
    Stimer5 = 37,
    #[doc = "38: Output is STimer 6"]
    Stimer6 = 38,
    #[doc = "39: Output is STimer 7"]
    Stimer7 = 39,
    #[doc = "63: Output is disabled"]
    Disabled = 63,
}
impl From<Outcfg41> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg41) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg41 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg41 {}
#[doc = "Field `OUTCFG41` reader - Pad output 41 configuration"]
pub type Outcfg41R = crate::FieldReader<Outcfg41>;
impl Outcfg41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg41> {
        match self.bits {
            0 => Some(Outcfg41::Timer00),
            1 => Some(Outcfg41::Timer01),
            2 => Some(Outcfg41::Timer10),
            3 => Some(Outcfg41::Timer11),
            4 => Some(Outcfg41::Timer20),
            5 => Some(Outcfg41::Timer21),
            6 => Some(Outcfg41::Timer30),
            7 => Some(Outcfg41::Timer31),
            8 => Some(Outcfg41::Timer40),
            9 => Some(Outcfg41::Timer41),
            10 => Some(Outcfg41::Timer50),
            11 => Some(Outcfg41::Timer51),
            12 => Some(Outcfg41::Timer60),
            13 => Some(Outcfg41::Timer61),
            14 => Some(Outcfg41::Timer70),
            15 => Some(Outcfg41::Timer71),
            16 => Some(Outcfg41::Timer80),
            17 => Some(Outcfg41::Timer81),
            18 => Some(Outcfg41::Timer90),
            19 => Some(Outcfg41::Timer91),
            20 => Some(Outcfg41::Timer100),
            21 => Some(Outcfg41::Timer101),
            22 => Some(Outcfg41::Timer110),
            23 => Some(Outcfg41::Timer111),
            24 => Some(Outcfg41::Timer120),
            25 => Some(Outcfg41::Timer121),
            26 => Some(Outcfg41::Timer130),
            27 => Some(Outcfg41::Timer131),
            28 => Some(Outcfg41::Timer140),
            29 => Some(Outcfg41::Timer141),
            30 => Some(Outcfg41::Timer150),
            31 => Some(Outcfg41::Timer151),
            32 => Some(Outcfg41::Stimer0),
            33 => Some(Outcfg41::Stimer1),
            34 => Some(Outcfg41::Stimer2),
            35 => Some(Outcfg41::Stimer3),
            36 => Some(Outcfg41::Stimer4),
            37 => Some(Outcfg41::Stimer5),
            38 => Some(Outcfg41::Stimer6),
            39 => Some(Outcfg41::Stimer7),
            63 => Some(Outcfg41::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg41::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg41::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg41::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg41::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg41::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg41::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg41::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg41::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg41::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg41::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg41::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg41::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg41::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg41::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg41::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg41::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg41::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg41::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg41::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg41::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg41::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg41::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg41::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg41::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg41::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg41::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg41::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg41::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg41::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg41::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg41::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg41::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg41::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg41::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg41::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg41::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg41::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg41::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg41::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg41::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg41::Disabled
    }
}
#[doc = "Field `OUTCFG41` writer - Pad output 41 configuration"]
pub type Outcfg41W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg41>;
impl<'a, REG> Outcfg41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg41::Disabled)
    }
}
#[doc = "Pad output 42 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg42 {
    #[doc = "0: Output is Timer 0, output 0"]
    Timer00 = 0,
    #[doc = "1: Output is Timer 0, output 1"]
    Timer01 = 1,
    #[doc = "2: Output is Timer 1, output 0"]
    Timer10 = 2,
    #[doc = "3: Output is Timer 1, output 1"]
    Timer11 = 3,
    #[doc = "4: Output is Timer 2, output 0"]
    Timer20 = 4,
    #[doc = "5: Output is Timer 2, output 1"]
    Timer21 = 5,
    #[doc = "6: Output is Timer 3, output 0"]
    Timer30 = 6,
    #[doc = "7: Output is Timer 3, output 1"]
    Timer31 = 7,
    #[doc = "8: Output is Timer 4, output 0"]
    Timer40 = 8,
    #[doc = "9: Output is Timer 4, output 1"]
    Timer41 = 9,
    #[doc = "10: Output is Timer 5, output 0"]
    Timer50 = 10,
    #[doc = "11: Output is Timer 5, output 1"]
    Timer51 = 11,
    #[doc = "12: Output is Timer 6, output 0"]
    Timer60 = 12,
    #[doc = "13: Output is Timer 6, output 1"]
    Timer61 = 13,
    #[doc = "14: Output is Timer 7, output 0"]
    Timer70 = 14,
    #[doc = "15: Output is Timer 7, output 1"]
    Timer71 = 15,
    #[doc = "16: Output is Timer 8, output 0"]
    Timer80 = 16,
    #[doc = "17: Output is Timer 8, output 1"]
    Timer81 = 17,
    #[doc = "18: Output is Timer 9, output 0"]
    Timer90 = 18,
    #[doc = "19: Output is Timer 9, output 1"]
    Timer91 = 19,
    #[doc = "20: Output is Timer 10, output 0"]
    Timer100 = 20,
    #[doc = "21: Output is Timer 10, output 1"]
    Timer101 = 21,
    #[doc = "22: Output is Timer 11, output 0"]
    Timer110 = 22,
    #[doc = "23: Output is Timer 11, output 1"]
    Timer111 = 23,
    #[doc = "24: Output is Timer 12, output 0"]
    Timer120 = 24,
    #[doc = "25: Output is Timer 12, output 1"]
    Timer121 = 25,
    #[doc = "26: Output is Timer 13, output 0"]
    Timer130 = 26,
    #[doc = "27: Output is Timer 13, output 1"]
    Timer131 = 27,
    #[doc = "28: Output is Timer 14, output 0"]
    Timer140 = 28,
    #[doc = "29: Output is Timer 14, output 1"]
    Timer141 = 29,
    #[doc = "30: Output is Timer 15, output 0"]
    Timer150 = 30,
    #[doc = "31: Output is Timer 15, output 1"]
    Timer151 = 31,
    #[doc = "32: Output is STimer 0"]
    Stimer0 = 32,
    #[doc = "33: Output is STimer 1"]
    Stimer1 = 33,
    #[doc = "34: Output is STimer 2"]
    Stimer2 = 34,
    #[doc = "35: Output is STimer 3"]
    Stimer3 = 35,
    #[doc = "36: Output is STimer 4"]
    Stimer4 = 36,
    #[doc = "37: Output is STimer 5"]
    Stimer5 = 37,
    #[doc = "38: Output is STimer 6"]
    Stimer6 = 38,
    #[doc = "39: Output is STimer 7"]
    Stimer7 = 39,
    #[doc = "63: Output is disabled"]
    Disabled = 63,
}
impl From<Outcfg42> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg42) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg42 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg42 {}
#[doc = "Field `OUTCFG42` reader - Pad output 42 configuration"]
pub type Outcfg42R = crate::FieldReader<Outcfg42>;
impl Outcfg42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg42> {
        match self.bits {
            0 => Some(Outcfg42::Timer00),
            1 => Some(Outcfg42::Timer01),
            2 => Some(Outcfg42::Timer10),
            3 => Some(Outcfg42::Timer11),
            4 => Some(Outcfg42::Timer20),
            5 => Some(Outcfg42::Timer21),
            6 => Some(Outcfg42::Timer30),
            7 => Some(Outcfg42::Timer31),
            8 => Some(Outcfg42::Timer40),
            9 => Some(Outcfg42::Timer41),
            10 => Some(Outcfg42::Timer50),
            11 => Some(Outcfg42::Timer51),
            12 => Some(Outcfg42::Timer60),
            13 => Some(Outcfg42::Timer61),
            14 => Some(Outcfg42::Timer70),
            15 => Some(Outcfg42::Timer71),
            16 => Some(Outcfg42::Timer80),
            17 => Some(Outcfg42::Timer81),
            18 => Some(Outcfg42::Timer90),
            19 => Some(Outcfg42::Timer91),
            20 => Some(Outcfg42::Timer100),
            21 => Some(Outcfg42::Timer101),
            22 => Some(Outcfg42::Timer110),
            23 => Some(Outcfg42::Timer111),
            24 => Some(Outcfg42::Timer120),
            25 => Some(Outcfg42::Timer121),
            26 => Some(Outcfg42::Timer130),
            27 => Some(Outcfg42::Timer131),
            28 => Some(Outcfg42::Timer140),
            29 => Some(Outcfg42::Timer141),
            30 => Some(Outcfg42::Timer150),
            31 => Some(Outcfg42::Timer151),
            32 => Some(Outcfg42::Stimer0),
            33 => Some(Outcfg42::Stimer1),
            34 => Some(Outcfg42::Stimer2),
            35 => Some(Outcfg42::Stimer3),
            36 => Some(Outcfg42::Stimer4),
            37 => Some(Outcfg42::Stimer5),
            38 => Some(Outcfg42::Stimer6),
            39 => Some(Outcfg42::Stimer7),
            63 => Some(Outcfg42::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg42::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg42::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg42::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg42::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg42::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg42::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg42::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg42::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg42::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg42::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg42::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg42::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg42::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg42::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg42::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg42::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg42::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg42::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg42::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg42::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg42::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg42::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg42::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg42::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg42::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg42::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg42::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg42::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg42::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg42::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg42::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg42::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg42::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg42::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg42::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg42::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg42::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg42::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg42::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg42::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg42::Disabled
    }
}
#[doc = "Field `OUTCFG42` writer - Pad output 42 configuration"]
pub type Outcfg42W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg42>;
impl<'a, REG> Outcfg42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg42::Disabled)
    }
}
#[doc = "Pad output 43 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg43 {
    #[doc = "0: Output is Timer 0, output 0"]
    Timer00 = 0,
    #[doc = "1: Output is Timer 0, output 1"]
    Timer01 = 1,
    #[doc = "2: Output is Timer 1, output 0"]
    Timer10 = 2,
    #[doc = "3: Output is Timer 1, output 1"]
    Timer11 = 3,
    #[doc = "4: Output is Timer 2, output 0"]
    Timer20 = 4,
    #[doc = "5: Output is Timer 2, output 1"]
    Timer21 = 5,
    #[doc = "6: Output is Timer 3, output 0"]
    Timer30 = 6,
    #[doc = "7: Output is Timer 3, output 1"]
    Timer31 = 7,
    #[doc = "8: Output is Timer 4, output 0"]
    Timer40 = 8,
    #[doc = "9: Output is Timer 4, output 1"]
    Timer41 = 9,
    #[doc = "10: Output is Timer 5, output 0"]
    Timer50 = 10,
    #[doc = "11: Output is Timer 5, output 1"]
    Timer51 = 11,
    #[doc = "12: Output is Timer 6, output 0"]
    Timer60 = 12,
    #[doc = "13: Output is Timer 6, output 1"]
    Timer61 = 13,
    #[doc = "14: Output is Timer 7, output 0"]
    Timer70 = 14,
    #[doc = "15: Output is Timer 7, output 1"]
    Timer71 = 15,
    #[doc = "16: Output is Timer 8, output 0"]
    Timer80 = 16,
    #[doc = "17: Output is Timer 8, output 1"]
    Timer81 = 17,
    #[doc = "18: Output is Timer 9, output 0"]
    Timer90 = 18,
    #[doc = "19: Output is Timer 9, output 1"]
    Timer91 = 19,
    #[doc = "20: Output is Timer 10, output 0"]
    Timer100 = 20,
    #[doc = "21: Output is Timer 10, output 1"]
    Timer101 = 21,
    #[doc = "22: Output is Timer 11, output 0"]
    Timer110 = 22,
    #[doc = "23: Output is Timer 11, output 1"]
    Timer111 = 23,
    #[doc = "24: Output is Timer 12, output 0"]
    Timer120 = 24,
    #[doc = "25: Output is Timer 12, output 1"]
    Timer121 = 25,
    #[doc = "26: Output is Timer 13, output 0"]
    Timer130 = 26,
    #[doc = "27: Output is Timer 13, output 1"]
    Timer131 = 27,
    #[doc = "28: Output is Timer 14, output 0"]
    Timer140 = 28,
    #[doc = "29: Output is Timer 14, output 1"]
    Timer141 = 29,
    #[doc = "30: Output is Timer 15, output 0"]
    Timer150 = 30,
    #[doc = "31: Output is Timer 15, output 1"]
    Timer151 = 31,
    #[doc = "32: Output is STimer 0"]
    Stimer0 = 32,
    #[doc = "33: Output is STimer 1"]
    Stimer1 = 33,
    #[doc = "34: Output is STimer 2"]
    Stimer2 = 34,
    #[doc = "35: Output is STimer 3"]
    Stimer3 = 35,
    #[doc = "36: Output is STimer 4"]
    Stimer4 = 36,
    #[doc = "37: Output is STimer 5"]
    Stimer5 = 37,
    #[doc = "38: Output is STimer 6"]
    Stimer6 = 38,
    #[doc = "39: Output is STimer 7"]
    Stimer7 = 39,
    #[doc = "63: Output is disabled"]
    Disabled = 63,
}
impl From<Outcfg43> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg43) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg43 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg43 {}
#[doc = "Field `OUTCFG43` reader - Pad output 43 configuration"]
pub type Outcfg43R = crate::FieldReader<Outcfg43>;
impl Outcfg43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg43> {
        match self.bits {
            0 => Some(Outcfg43::Timer00),
            1 => Some(Outcfg43::Timer01),
            2 => Some(Outcfg43::Timer10),
            3 => Some(Outcfg43::Timer11),
            4 => Some(Outcfg43::Timer20),
            5 => Some(Outcfg43::Timer21),
            6 => Some(Outcfg43::Timer30),
            7 => Some(Outcfg43::Timer31),
            8 => Some(Outcfg43::Timer40),
            9 => Some(Outcfg43::Timer41),
            10 => Some(Outcfg43::Timer50),
            11 => Some(Outcfg43::Timer51),
            12 => Some(Outcfg43::Timer60),
            13 => Some(Outcfg43::Timer61),
            14 => Some(Outcfg43::Timer70),
            15 => Some(Outcfg43::Timer71),
            16 => Some(Outcfg43::Timer80),
            17 => Some(Outcfg43::Timer81),
            18 => Some(Outcfg43::Timer90),
            19 => Some(Outcfg43::Timer91),
            20 => Some(Outcfg43::Timer100),
            21 => Some(Outcfg43::Timer101),
            22 => Some(Outcfg43::Timer110),
            23 => Some(Outcfg43::Timer111),
            24 => Some(Outcfg43::Timer120),
            25 => Some(Outcfg43::Timer121),
            26 => Some(Outcfg43::Timer130),
            27 => Some(Outcfg43::Timer131),
            28 => Some(Outcfg43::Timer140),
            29 => Some(Outcfg43::Timer141),
            30 => Some(Outcfg43::Timer150),
            31 => Some(Outcfg43::Timer151),
            32 => Some(Outcfg43::Stimer0),
            33 => Some(Outcfg43::Stimer1),
            34 => Some(Outcfg43::Stimer2),
            35 => Some(Outcfg43::Stimer3),
            36 => Some(Outcfg43::Stimer4),
            37 => Some(Outcfg43::Stimer5),
            38 => Some(Outcfg43::Stimer6),
            39 => Some(Outcfg43::Stimer7),
            63 => Some(Outcfg43::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg43::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg43::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg43::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg43::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg43::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg43::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg43::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg43::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg43::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg43::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg43::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg43::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg43::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg43::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg43::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg43::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg43::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg43::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg43::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg43::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg43::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg43::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg43::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg43::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg43::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg43::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg43::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg43::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg43::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg43::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg43::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg43::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg43::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg43::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg43::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg43::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg43::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg43::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg43::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg43::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg43::Disabled
    }
}
#[doc = "Field `OUTCFG43` writer - Pad output 43 configuration"]
pub type Outcfg43W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg43>;
impl<'a, REG> Outcfg43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg43::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Pad output 40 configuration"]
    #[inline(always)]
    pub fn outcfg40(&self) -> Outcfg40R {
        Outcfg40R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Pad output 41 configuration"]
    #[inline(always)]
    pub fn outcfg41(&self) -> Outcfg41R {
        Outcfg41R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Pad output 42 configuration"]
    #[inline(always)]
    pub fn outcfg42(&self) -> Outcfg42R {
        Outcfg42R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Pad output 43 configuration"]
    #[inline(always)]
    pub fn outcfg43(&self) -> Outcfg43R {
        Outcfg43R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pad output 40 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg40(&mut self) -> Outcfg40W<Outcfg10Spec> {
        Outcfg40W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Pad output 41 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg41(&mut self) -> Outcfg41W<Outcfg10Spec> {
        Outcfg41W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Pad output 42 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg42(&mut self) -> Outcfg42W<Outcfg10Spec> {
        Outcfg42W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Pad output 43 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg43(&mut self) -> Outcfg43W<Outcfg10Spec> {
        Outcfg43W::new(self, 24)
    }
}
#[doc = "Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Outcfg10Spec;
impl crate::RegisterSpec for Outcfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outcfg10::R`](R) reader structure"]
impl crate::Readable for Outcfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`outcfg10::W`](W) writer structure"]
impl crate::Writable for Outcfg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTCFG10 to value 0x3f3f_3f3f"]
impl crate::Resettable for Outcfg10Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
