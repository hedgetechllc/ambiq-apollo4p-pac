#[doc = "Register `OUTCFG0` reader"]
pub type R = crate::R<Outcfg0Spec>;
#[doc = "Register `OUTCFG0` writer"]
pub type W = crate::W<Outcfg0Spec>;
#[doc = "Pad output 0 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg0 {
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
impl From<Outcfg0> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg0 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg0 {}
#[doc = "Field `OUTCFG0` reader - Pad output 0 configuration"]
pub type Outcfg0R = crate::FieldReader<Outcfg0>;
impl Outcfg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg0> {
        match self.bits {
            0 => Some(Outcfg0::Timer00),
            1 => Some(Outcfg0::Timer01),
            2 => Some(Outcfg0::Timer10),
            3 => Some(Outcfg0::Timer11),
            4 => Some(Outcfg0::Timer20),
            5 => Some(Outcfg0::Timer21),
            6 => Some(Outcfg0::Timer30),
            7 => Some(Outcfg0::Timer31),
            8 => Some(Outcfg0::Timer40),
            9 => Some(Outcfg0::Timer41),
            10 => Some(Outcfg0::Timer50),
            11 => Some(Outcfg0::Timer51),
            12 => Some(Outcfg0::Timer60),
            13 => Some(Outcfg0::Timer61),
            14 => Some(Outcfg0::Timer70),
            15 => Some(Outcfg0::Timer71),
            16 => Some(Outcfg0::Timer80),
            17 => Some(Outcfg0::Timer81),
            18 => Some(Outcfg0::Timer90),
            19 => Some(Outcfg0::Timer91),
            20 => Some(Outcfg0::Timer100),
            21 => Some(Outcfg0::Timer101),
            22 => Some(Outcfg0::Timer110),
            23 => Some(Outcfg0::Timer111),
            24 => Some(Outcfg0::Timer120),
            25 => Some(Outcfg0::Timer121),
            26 => Some(Outcfg0::Timer130),
            27 => Some(Outcfg0::Timer131),
            28 => Some(Outcfg0::Timer140),
            29 => Some(Outcfg0::Timer141),
            30 => Some(Outcfg0::Timer150),
            31 => Some(Outcfg0::Timer151),
            32 => Some(Outcfg0::Stimer0),
            33 => Some(Outcfg0::Stimer1),
            34 => Some(Outcfg0::Stimer2),
            35 => Some(Outcfg0::Stimer3),
            36 => Some(Outcfg0::Stimer4),
            37 => Some(Outcfg0::Stimer5),
            38 => Some(Outcfg0::Stimer6),
            39 => Some(Outcfg0::Stimer7),
            63 => Some(Outcfg0::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg0::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg0::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg0::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg0::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg0::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg0::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg0::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg0::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg0::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg0::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg0::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg0::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg0::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg0::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg0::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg0::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg0::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg0::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg0::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg0::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg0::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg0::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg0::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg0::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg0::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg0::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg0::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg0::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg0::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg0::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg0::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg0::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg0::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg0::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg0::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg0::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg0::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg0::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg0::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg0::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg0::Disabled
    }
}
#[doc = "Field `OUTCFG0` writer - Pad output 0 configuration"]
pub type Outcfg0W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg0>;
impl<'a, REG> Outcfg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg0::Disabled)
    }
}
#[doc = "Pad output 1 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg1 {
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
impl From<Outcfg1> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg1 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg1 {}
#[doc = "Field `OUTCFG1` reader - Pad output 1 configuration"]
pub type Outcfg1R = crate::FieldReader<Outcfg1>;
impl Outcfg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg1> {
        match self.bits {
            0 => Some(Outcfg1::Timer00),
            1 => Some(Outcfg1::Timer01),
            2 => Some(Outcfg1::Timer10),
            3 => Some(Outcfg1::Timer11),
            4 => Some(Outcfg1::Timer20),
            5 => Some(Outcfg1::Timer21),
            6 => Some(Outcfg1::Timer30),
            7 => Some(Outcfg1::Timer31),
            8 => Some(Outcfg1::Timer40),
            9 => Some(Outcfg1::Timer41),
            10 => Some(Outcfg1::Timer50),
            11 => Some(Outcfg1::Timer51),
            12 => Some(Outcfg1::Timer60),
            13 => Some(Outcfg1::Timer61),
            14 => Some(Outcfg1::Timer70),
            15 => Some(Outcfg1::Timer71),
            16 => Some(Outcfg1::Timer80),
            17 => Some(Outcfg1::Timer81),
            18 => Some(Outcfg1::Timer90),
            19 => Some(Outcfg1::Timer91),
            20 => Some(Outcfg1::Timer100),
            21 => Some(Outcfg1::Timer101),
            22 => Some(Outcfg1::Timer110),
            23 => Some(Outcfg1::Timer111),
            24 => Some(Outcfg1::Timer120),
            25 => Some(Outcfg1::Timer121),
            26 => Some(Outcfg1::Timer130),
            27 => Some(Outcfg1::Timer131),
            28 => Some(Outcfg1::Timer140),
            29 => Some(Outcfg1::Timer141),
            30 => Some(Outcfg1::Timer150),
            31 => Some(Outcfg1::Timer151),
            32 => Some(Outcfg1::Stimer0),
            33 => Some(Outcfg1::Stimer1),
            34 => Some(Outcfg1::Stimer2),
            35 => Some(Outcfg1::Stimer3),
            36 => Some(Outcfg1::Stimer4),
            37 => Some(Outcfg1::Stimer5),
            38 => Some(Outcfg1::Stimer6),
            39 => Some(Outcfg1::Stimer7),
            63 => Some(Outcfg1::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg1::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg1::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg1::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg1::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg1::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg1::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg1::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg1::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg1::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg1::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg1::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg1::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg1::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg1::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg1::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg1::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg1::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg1::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg1::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg1::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg1::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg1::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg1::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg1::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg1::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg1::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg1::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg1::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg1::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg1::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg1::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg1::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg1::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg1::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg1::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg1::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg1::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg1::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg1::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg1::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg1::Disabled
    }
}
#[doc = "Field `OUTCFG1` writer - Pad output 1 configuration"]
pub type Outcfg1W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg1>;
impl<'a, REG> Outcfg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg1::Disabled)
    }
}
#[doc = "Pad output 2 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg2 {
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
impl From<Outcfg2> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg2 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg2 {}
#[doc = "Field `OUTCFG2` reader - Pad output 2 configuration"]
pub type Outcfg2R = crate::FieldReader<Outcfg2>;
impl Outcfg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg2> {
        match self.bits {
            0 => Some(Outcfg2::Timer00),
            1 => Some(Outcfg2::Timer01),
            2 => Some(Outcfg2::Timer10),
            3 => Some(Outcfg2::Timer11),
            4 => Some(Outcfg2::Timer20),
            5 => Some(Outcfg2::Timer21),
            6 => Some(Outcfg2::Timer30),
            7 => Some(Outcfg2::Timer31),
            8 => Some(Outcfg2::Timer40),
            9 => Some(Outcfg2::Timer41),
            10 => Some(Outcfg2::Timer50),
            11 => Some(Outcfg2::Timer51),
            12 => Some(Outcfg2::Timer60),
            13 => Some(Outcfg2::Timer61),
            14 => Some(Outcfg2::Timer70),
            15 => Some(Outcfg2::Timer71),
            16 => Some(Outcfg2::Timer80),
            17 => Some(Outcfg2::Timer81),
            18 => Some(Outcfg2::Timer90),
            19 => Some(Outcfg2::Timer91),
            20 => Some(Outcfg2::Timer100),
            21 => Some(Outcfg2::Timer101),
            22 => Some(Outcfg2::Timer110),
            23 => Some(Outcfg2::Timer111),
            24 => Some(Outcfg2::Timer120),
            25 => Some(Outcfg2::Timer121),
            26 => Some(Outcfg2::Timer130),
            27 => Some(Outcfg2::Timer131),
            28 => Some(Outcfg2::Timer140),
            29 => Some(Outcfg2::Timer141),
            30 => Some(Outcfg2::Timer150),
            31 => Some(Outcfg2::Timer151),
            32 => Some(Outcfg2::Stimer0),
            33 => Some(Outcfg2::Stimer1),
            34 => Some(Outcfg2::Stimer2),
            35 => Some(Outcfg2::Stimer3),
            36 => Some(Outcfg2::Stimer4),
            37 => Some(Outcfg2::Stimer5),
            38 => Some(Outcfg2::Stimer6),
            39 => Some(Outcfg2::Stimer7),
            63 => Some(Outcfg2::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg2::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg2::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg2::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg2::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg2::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg2::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg2::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg2::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg2::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg2::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg2::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg2::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg2::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg2::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg2::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg2::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg2::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg2::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg2::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg2::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg2::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg2::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg2::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg2::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg2::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg2::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg2::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg2::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg2::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg2::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg2::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg2::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg2::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg2::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg2::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg2::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg2::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg2::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg2::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg2::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg2::Disabled
    }
}
#[doc = "Field `OUTCFG2` writer - Pad output 2 configuration"]
pub type Outcfg2W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg2>;
impl<'a, REG> Outcfg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg2::Disabled)
    }
}
#[doc = "Pad output 3 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg3 {
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
impl From<Outcfg3> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg3 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg3 {}
#[doc = "Field `OUTCFG3` reader - Pad output 3 configuration"]
pub type Outcfg3R = crate::FieldReader<Outcfg3>;
impl Outcfg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg3> {
        match self.bits {
            0 => Some(Outcfg3::Timer00),
            1 => Some(Outcfg3::Timer01),
            2 => Some(Outcfg3::Timer10),
            3 => Some(Outcfg3::Timer11),
            4 => Some(Outcfg3::Timer20),
            5 => Some(Outcfg3::Timer21),
            6 => Some(Outcfg3::Timer30),
            7 => Some(Outcfg3::Timer31),
            8 => Some(Outcfg3::Timer40),
            9 => Some(Outcfg3::Timer41),
            10 => Some(Outcfg3::Timer50),
            11 => Some(Outcfg3::Timer51),
            12 => Some(Outcfg3::Timer60),
            13 => Some(Outcfg3::Timer61),
            14 => Some(Outcfg3::Timer70),
            15 => Some(Outcfg3::Timer71),
            16 => Some(Outcfg3::Timer80),
            17 => Some(Outcfg3::Timer81),
            18 => Some(Outcfg3::Timer90),
            19 => Some(Outcfg3::Timer91),
            20 => Some(Outcfg3::Timer100),
            21 => Some(Outcfg3::Timer101),
            22 => Some(Outcfg3::Timer110),
            23 => Some(Outcfg3::Timer111),
            24 => Some(Outcfg3::Timer120),
            25 => Some(Outcfg3::Timer121),
            26 => Some(Outcfg3::Timer130),
            27 => Some(Outcfg3::Timer131),
            28 => Some(Outcfg3::Timer140),
            29 => Some(Outcfg3::Timer141),
            30 => Some(Outcfg3::Timer150),
            31 => Some(Outcfg3::Timer151),
            32 => Some(Outcfg3::Stimer0),
            33 => Some(Outcfg3::Stimer1),
            34 => Some(Outcfg3::Stimer2),
            35 => Some(Outcfg3::Stimer3),
            36 => Some(Outcfg3::Stimer4),
            37 => Some(Outcfg3::Stimer5),
            38 => Some(Outcfg3::Stimer6),
            39 => Some(Outcfg3::Stimer7),
            63 => Some(Outcfg3::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg3::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg3::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg3::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg3::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg3::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg3::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg3::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg3::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg3::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg3::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg3::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg3::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg3::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg3::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg3::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg3::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg3::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg3::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg3::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg3::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg3::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg3::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg3::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg3::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg3::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg3::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg3::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg3::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg3::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg3::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg3::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg3::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg3::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg3::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg3::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg3::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg3::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg3::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg3::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg3::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg3::Disabled
    }
}
#[doc = "Field `OUTCFG3` writer - Pad output 3 configuration"]
pub type Outcfg3W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg3>;
impl<'a, REG> Outcfg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg3::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Pad output 0 configuration"]
    #[inline(always)]
    pub fn outcfg0(&self) -> Outcfg0R {
        Outcfg0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Pad output 1 configuration"]
    #[inline(always)]
    pub fn outcfg1(&self) -> Outcfg1R {
        Outcfg1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Pad output 2 configuration"]
    #[inline(always)]
    pub fn outcfg2(&self) -> Outcfg2R {
        Outcfg2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Pad output 3 configuration"]
    #[inline(always)]
    pub fn outcfg3(&self) -> Outcfg3R {
        Outcfg3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pad output 0 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg0(&mut self) -> Outcfg0W<Outcfg0Spec> {
        Outcfg0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Pad output 1 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg1(&mut self) -> Outcfg1W<Outcfg0Spec> {
        Outcfg1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Pad output 2 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg2(&mut self) -> Outcfg2W<Outcfg0Spec> {
        Outcfg2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Pad output 3 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg3(&mut self) -> Outcfg3W<Outcfg0Spec> {
        Outcfg3W::new(self, 24)
    }
}
#[doc = "Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Outcfg0Spec;
impl crate::RegisterSpec for Outcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outcfg0::R`](R) reader structure"]
impl crate::Readable for Outcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`outcfg0::W`](W) writer structure"]
impl crate::Writable for Outcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTCFG0 to value 0x3f3f_3f3f"]
impl crate::Resettable for Outcfg0Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
