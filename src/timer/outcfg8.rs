#[doc = "Register `OUTCFG8` reader"]
pub type R = crate::R<Outcfg8Spec>;
#[doc = "Register `OUTCFG8` writer"]
pub type W = crate::W<Outcfg8Spec>;
#[doc = "Pad output 32 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg32 {
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
impl From<Outcfg32> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg32) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg32 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg32 {}
#[doc = "Field `OUTCFG32` reader - Pad output 32 configuration"]
pub type Outcfg32R = crate::FieldReader<Outcfg32>;
impl Outcfg32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg32> {
        match self.bits {
            0 => Some(Outcfg32::Timer00),
            1 => Some(Outcfg32::Timer01),
            2 => Some(Outcfg32::Timer10),
            3 => Some(Outcfg32::Timer11),
            4 => Some(Outcfg32::Timer20),
            5 => Some(Outcfg32::Timer21),
            6 => Some(Outcfg32::Timer30),
            7 => Some(Outcfg32::Timer31),
            8 => Some(Outcfg32::Timer40),
            9 => Some(Outcfg32::Timer41),
            10 => Some(Outcfg32::Timer50),
            11 => Some(Outcfg32::Timer51),
            12 => Some(Outcfg32::Timer60),
            13 => Some(Outcfg32::Timer61),
            14 => Some(Outcfg32::Timer70),
            15 => Some(Outcfg32::Timer71),
            16 => Some(Outcfg32::Timer80),
            17 => Some(Outcfg32::Timer81),
            18 => Some(Outcfg32::Timer90),
            19 => Some(Outcfg32::Timer91),
            20 => Some(Outcfg32::Timer100),
            21 => Some(Outcfg32::Timer101),
            22 => Some(Outcfg32::Timer110),
            23 => Some(Outcfg32::Timer111),
            24 => Some(Outcfg32::Timer120),
            25 => Some(Outcfg32::Timer121),
            26 => Some(Outcfg32::Timer130),
            27 => Some(Outcfg32::Timer131),
            28 => Some(Outcfg32::Timer140),
            29 => Some(Outcfg32::Timer141),
            30 => Some(Outcfg32::Timer150),
            31 => Some(Outcfg32::Timer151),
            32 => Some(Outcfg32::Stimer0),
            33 => Some(Outcfg32::Stimer1),
            34 => Some(Outcfg32::Stimer2),
            35 => Some(Outcfg32::Stimer3),
            36 => Some(Outcfg32::Stimer4),
            37 => Some(Outcfg32::Stimer5),
            38 => Some(Outcfg32::Stimer6),
            39 => Some(Outcfg32::Stimer7),
            63 => Some(Outcfg32::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg32::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg32::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg32::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg32::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg32::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg32::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg32::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg32::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg32::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg32::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg32::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg32::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg32::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg32::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg32::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg32::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg32::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg32::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg32::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg32::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg32::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg32::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg32::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg32::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg32::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg32::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg32::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg32::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg32::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg32::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg32::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg32::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg32::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg32::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg32::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg32::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg32::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg32::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg32::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg32::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg32::Disabled
    }
}
#[doc = "Field `OUTCFG32` writer - Pad output 32 configuration"]
pub type Outcfg32W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg32>;
impl<'a, REG> Outcfg32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg32::Disabled)
    }
}
#[doc = "Pad output 33 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg33 {
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
impl From<Outcfg33> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg33 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg33 {}
#[doc = "Field `OUTCFG33` reader - Pad output 33 configuration"]
pub type Outcfg33R = crate::FieldReader<Outcfg33>;
impl Outcfg33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg33> {
        match self.bits {
            0 => Some(Outcfg33::Timer00),
            1 => Some(Outcfg33::Timer01),
            2 => Some(Outcfg33::Timer10),
            3 => Some(Outcfg33::Timer11),
            4 => Some(Outcfg33::Timer20),
            5 => Some(Outcfg33::Timer21),
            6 => Some(Outcfg33::Timer30),
            7 => Some(Outcfg33::Timer31),
            8 => Some(Outcfg33::Timer40),
            9 => Some(Outcfg33::Timer41),
            10 => Some(Outcfg33::Timer50),
            11 => Some(Outcfg33::Timer51),
            12 => Some(Outcfg33::Timer60),
            13 => Some(Outcfg33::Timer61),
            14 => Some(Outcfg33::Timer70),
            15 => Some(Outcfg33::Timer71),
            16 => Some(Outcfg33::Timer80),
            17 => Some(Outcfg33::Timer81),
            18 => Some(Outcfg33::Timer90),
            19 => Some(Outcfg33::Timer91),
            20 => Some(Outcfg33::Timer100),
            21 => Some(Outcfg33::Timer101),
            22 => Some(Outcfg33::Timer110),
            23 => Some(Outcfg33::Timer111),
            24 => Some(Outcfg33::Timer120),
            25 => Some(Outcfg33::Timer121),
            26 => Some(Outcfg33::Timer130),
            27 => Some(Outcfg33::Timer131),
            28 => Some(Outcfg33::Timer140),
            29 => Some(Outcfg33::Timer141),
            30 => Some(Outcfg33::Timer150),
            31 => Some(Outcfg33::Timer151),
            32 => Some(Outcfg33::Stimer0),
            33 => Some(Outcfg33::Stimer1),
            34 => Some(Outcfg33::Stimer2),
            35 => Some(Outcfg33::Stimer3),
            36 => Some(Outcfg33::Stimer4),
            37 => Some(Outcfg33::Stimer5),
            38 => Some(Outcfg33::Stimer6),
            39 => Some(Outcfg33::Stimer7),
            63 => Some(Outcfg33::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg33::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg33::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg33::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg33::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg33::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg33::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg33::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg33::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg33::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg33::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg33::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg33::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg33::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg33::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg33::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg33::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg33::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg33::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg33::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg33::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg33::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg33::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg33::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg33::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg33::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg33::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg33::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg33::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg33::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg33::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg33::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg33::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg33::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg33::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg33::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg33::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg33::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg33::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg33::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg33::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg33::Disabled
    }
}
#[doc = "Field `OUTCFG33` writer - Pad output 33 configuration"]
pub type Outcfg33W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg33>;
impl<'a, REG> Outcfg33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg33::Disabled)
    }
}
#[doc = "Pad output 34 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg34 {
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
impl From<Outcfg34> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg34) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg34 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg34 {}
#[doc = "Field `OUTCFG34` reader - Pad output 34 configuration"]
pub type Outcfg34R = crate::FieldReader<Outcfg34>;
impl Outcfg34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg34> {
        match self.bits {
            0 => Some(Outcfg34::Timer00),
            1 => Some(Outcfg34::Timer01),
            2 => Some(Outcfg34::Timer10),
            3 => Some(Outcfg34::Timer11),
            4 => Some(Outcfg34::Timer20),
            5 => Some(Outcfg34::Timer21),
            6 => Some(Outcfg34::Timer30),
            7 => Some(Outcfg34::Timer31),
            8 => Some(Outcfg34::Timer40),
            9 => Some(Outcfg34::Timer41),
            10 => Some(Outcfg34::Timer50),
            11 => Some(Outcfg34::Timer51),
            12 => Some(Outcfg34::Timer60),
            13 => Some(Outcfg34::Timer61),
            14 => Some(Outcfg34::Timer70),
            15 => Some(Outcfg34::Timer71),
            16 => Some(Outcfg34::Timer80),
            17 => Some(Outcfg34::Timer81),
            18 => Some(Outcfg34::Timer90),
            19 => Some(Outcfg34::Timer91),
            20 => Some(Outcfg34::Timer100),
            21 => Some(Outcfg34::Timer101),
            22 => Some(Outcfg34::Timer110),
            23 => Some(Outcfg34::Timer111),
            24 => Some(Outcfg34::Timer120),
            25 => Some(Outcfg34::Timer121),
            26 => Some(Outcfg34::Timer130),
            27 => Some(Outcfg34::Timer131),
            28 => Some(Outcfg34::Timer140),
            29 => Some(Outcfg34::Timer141),
            30 => Some(Outcfg34::Timer150),
            31 => Some(Outcfg34::Timer151),
            32 => Some(Outcfg34::Stimer0),
            33 => Some(Outcfg34::Stimer1),
            34 => Some(Outcfg34::Stimer2),
            35 => Some(Outcfg34::Stimer3),
            36 => Some(Outcfg34::Stimer4),
            37 => Some(Outcfg34::Stimer5),
            38 => Some(Outcfg34::Stimer6),
            39 => Some(Outcfg34::Stimer7),
            63 => Some(Outcfg34::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg34::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg34::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg34::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg34::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg34::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg34::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg34::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg34::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg34::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg34::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg34::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg34::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg34::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg34::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg34::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg34::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg34::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg34::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg34::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg34::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg34::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg34::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg34::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg34::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg34::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg34::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg34::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg34::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg34::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg34::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg34::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg34::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg34::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg34::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg34::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg34::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg34::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg34::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg34::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg34::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg34::Disabled
    }
}
#[doc = "Field `OUTCFG34` writer - Pad output 34 configuration"]
pub type Outcfg34W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg34>;
impl<'a, REG> Outcfg34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg34::Disabled)
    }
}
#[doc = "Pad output 35 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg35 {
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
impl From<Outcfg35> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg35) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg35 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg35 {}
#[doc = "Field `OUTCFG35` reader - Pad output 35 configuration"]
pub type Outcfg35R = crate::FieldReader<Outcfg35>;
impl Outcfg35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg35> {
        match self.bits {
            0 => Some(Outcfg35::Timer00),
            1 => Some(Outcfg35::Timer01),
            2 => Some(Outcfg35::Timer10),
            3 => Some(Outcfg35::Timer11),
            4 => Some(Outcfg35::Timer20),
            5 => Some(Outcfg35::Timer21),
            6 => Some(Outcfg35::Timer30),
            7 => Some(Outcfg35::Timer31),
            8 => Some(Outcfg35::Timer40),
            9 => Some(Outcfg35::Timer41),
            10 => Some(Outcfg35::Timer50),
            11 => Some(Outcfg35::Timer51),
            12 => Some(Outcfg35::Timer60),
            13 => Some(Outcfg35::Timer61),
            14 => Some(Outcfg35::Timer70),
            15 => Some(Outcfg35::Timer71),
            16 => Some(Outcfg35::Timer80),
            17 => Some(Outcfg35::Timer81),
            18 => Some(Outcfg35::Timer90),
            19 => Some(Outcfg35::Timer91),
            20 => Some(Outcfg35::Timer100),
            21 => Some(Outcfg35::Timer101),
            22 => Some(Outcfg35::Timer110),
            23 => Some(Outcfg35::Timer111),
            24 => Some(Outcfg35::Timer120),
            25 => Some(Outcfg35::Timer121),
            26 => Some(Outcfg35::Timer130),
            27 => Some(Outcfg35::Timer131),
            28 => Some(Outcfg35::Timer140),
            29 => Some(Outcfg35::Timer141),
            30 => Some(Outcfg35::Timer150),
            31 => Some(Outcfg35::Timer151),
            32 => Some(Outcfg35::Stimer0),
            33 => Some(Outcfg35::Stimer1),
            34 => Some(Outcfg35::Stimer2),
            35 => Some(Outcfg35::Stimer3),
            36 => Some(Outcfg35::Stimer4),
            37 => Some(Outcfg35::Stimer5),
            38 => Some(Outcfg35::Stimer6),
            39 => Some(Outcfg35::Stimer7),
            63 => Some(Outcfg35::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg35::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg35::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg35::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg35::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg35::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg35::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg35::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg35::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg35::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg35::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg35::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg35::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg35::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg35::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg35::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg35::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg35::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg35::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg35::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg35::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg35::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg35::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg35::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg35::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg35::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg35::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg35::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg35::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg35::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg35::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg35::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg35::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg35::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg35::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg35::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg35::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg35::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg35::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg35::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg35::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg35::Disabled
    }
}
#[doc = "Field `OUTCFG35` writer - Pad output 35 configuration"]
pub type Outcfg35W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg35>;
impl<'a, REG> Outcfg35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg35::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Pad output 32 configuration"]
    #[inline(always)]
    pub fn outcfg32(&self) -> Outcfg32R {
        Outcfg32R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Pad output 33 configuration"]
    #[inline(always)]
    pub fn outcfg33(&self) -> Outcfg33R {
        Outcfg33R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Pad output 34 configuration"]
    #[inline(always)]
    pub fn outcfg34(&self) -> Outcfg34R {
        Outcfg34R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Pad output 35 configuration"]
    #[inline(always)]
    pub fn outcfg35(&self) -> Outcfg35R {
        Outcfg35R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pad output 32 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg32(&mut self) -> Outcfg32W<Outcfg8Spec> {
        Outcfg32W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Pad output 33 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg33(&mut self) -> Outcfg33W<Outcfg8Spec> {
        Outcfg33W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Pad output 34 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg34(&mut self) -> Outcfg34W<Outcfg8Spec> {
        Outcfg34W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Pad output 35 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg35(&mut self) -> Outcfg35W<Outcfg8Spec> {
        Outcfg35W::new(self, 24)
    }
}
#[doc = "Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Outcfg8Spec;
impl crate::RegisterSpec for Outcfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outcfg8::R`](R) reader structure"]
impl crate::Readable for Outcfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`outcfg8::W`](W) writer structure"]
impl crate::Writable for Outcfg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTCFG8 to value 0x3f3f_3f3f"]
impl crate::Resettable for Outcfg8Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
