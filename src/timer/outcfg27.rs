#[doc = "Register `OUTCFG27` reader"]
pub type R = crate::R<Outcfg27Spec>;
#[doc = "Register `OUTCFG27` writer"]
pub type W = crate::W<Outcfg27Spec>;
#[doc = "Pad output 108 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg108 {
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
impl From<Outcfg108> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg108) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg108 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg108 {}
#[doc = "Field `OUTCFG108` reader - Pad output 108 configuration"]
pub type Outcfg108R = crate::FieldReader<Outcfg108>;
impl Outcfg108R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg108> {
        match self.bits {
            0 => Some(Outcfg108::Timer00),
            1 => Some(Outcfg108::Timer01),
            2 => Some(Outcfg108::Timer10),
            3 => Some(Outcfg108::Timer11),
            4 => Some(Outcfg108::Timer20),
            5 => Some(Outcfg108::Timer21),
            6 => Some(Outcfg108::Timer30),
            7 => Some(Outcfg108::Timer31),
            8 => Some(Outcfg108::Timer40),
            9 => Some(Outcfg108::Timer41),
            10 => Some(Outcfg108::Timer50),
            11 => Some(Outcfg108::Timer51),
            12 => Some(Outcfg108::Timer60),
            13 => Some(Outcfg108::Timer61),
            14 => Some(Outcfg108::Timer70),
            15 => Some(Outcfg108::Timer71),
            16 => Some(Outcfg108::Timer80),
            17 => Some(Outcfg108::Timer81),
            18 => Some(Outcfg108::Timer90),
            19 => Some(Outcfg108::Timer91),
            20 => Some(Outcfg108::Timer100),
            21 => Some(Outcfg108::Timer101),
            22 => Some(Outcfg108::Timer110),
            23 => Some(Outcfg108::Timer111),
            24 => Some(Outcfg108::Timer120),
            25 => Some(Outcfg108::Timer121),
            26 => Some(Outcfg108::Timer130),
            27 => Some(Outcfg108::Timer131),
            28 => Some(Outcfg108::Timer140),
            29 => Some(Outcfg108::Timer141),
            30 => Some(Outcfg108::Timer150),
            31 => Some(Outcfg108::Timer151),
            32 => Some(Outcfg108::Stimer0),
            33 => Some(Outcfg108::Stimer1),
            34 => Some(Outcfg108::Stimer2),
            35 => Some(Outcfg108::Stimer3),
            36 => Some(Outcfg108::Stimer4),
            37 => Some(Outcfg108::Stimer5),
            38 => Some(Outcfg108::Stimer6),
            39 => Some(Outcfg108::Stimer7),
            63 => Some(Outcfg108::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg108::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg108::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg108::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg108::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg108::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg108::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg108::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg108::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg108::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg108::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg108::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg108::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg108::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg108::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg108::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg108::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg108::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg108::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg108::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg108::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg108::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg108::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg108::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg108::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg108::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg108::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg108::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg108::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg108::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg108::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg108::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg108::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg108::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg108::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg108::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg108::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg108::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg108::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg108::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg108::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg108::Disabled
    }
}
#[doc = "Field `OUTCFG108` writer - Pad output 108 configuration"]
pub type Outcfg108W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg108>;
impl<'a, REG> Outcfg108W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg108::Disabled)
    }
}
#[doc = "Pad output 109 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg109 {
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
impl From<Outcfg109> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg109) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg109 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg109 {}
#[doc = "Field `OUTCFG109` reader - Pad output 109 configuration"]
pub type Outcfg109R = crate::FieldReader<Outcfg109>;
impl Outcfg109R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg109> {
        match self.bits {
            0 => Some(Outcfg109::Timer00),
            1 => Some(Outcfg109::Timer01),
            2 => Some(Outcfg109::Timer10),
            3 => Some(Outcfg109::Timer11),
            4 => Some(Outcfg109::Timer20),
            5 => Some(Outcfg109::Timer21),
            6 => Some(Outcfg109::Timer30),
            7 => Some(Outcfg109::Timer31),
            8 => Some(Outcfg109::Timer40),
            9 => Some(Outcfg109::Timer41),
            10 => Some(Outcfg109::Timer50),
            11 => Some(Outcfg109::Timer51),
            12 => Some(Outcfg109::Timer60),
            13 => Some(Outcfg109::Timer61),
            14 => Some(Outcfg109::Timer70),
            15 => Some(Outcfg109::Timer71),
            16 => Some(Outcfg109::Timer80),
            17 => Some(Outcfg109::Timer81),
            18 => Some(Outcfg109::Timer90),
            19 => Some(Outcfg109::Timer91),
            20 => Some(Outcfg109::Timer100),
            21 => Some(Outcfg109::Timer101),
            22 => Some(Outcfg109::Timer110),
            23 => Some(Outcfg109::Timer111),
            24 => Some(Outcfg109::Timer120),
            25 => Some(Outcfg109::Timer121),
            26 => Some(Outcfg109::Timer130),
            27 => Some(Outcfg109::Timer131),
            28 => Some(Outcfg109::Timer140),
            29 => Some(Outcfg109::Timer141),
            30 => Some(Outcfg109::Timer150),
            31 => Some(Outcfg109::Timer151),
            32 => Some(Outcfg109::Stimer0),
            33 => Some(Outcfg109::Stimer1),
            34 => Some(Outcfg109::Stimer2),
            35 => Some(Outcfg109::Stimer3),
            36 => Some(Outcfg109::Stimer4),
            37 => Some(Outcfg109::Stimer5),
            38 => Some(Outcfg109::Stimer6),
            39 => Some(Outcfg109::Stimer7),
            63 => Some(Outcfg109::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg109::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg109::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg109::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg109::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg109::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg109::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg109::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg109::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg109::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg109::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg109::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg109::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg109::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg109::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg109::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg109::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg109::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg109::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg109::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg109::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg109::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg109::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg109::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg109::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg109::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg109::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg109::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg109::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg109::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg109::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg109::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg109::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg109::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg109::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg109::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg109::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg109::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg109::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg109::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg109::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg109::Disabled
    }
}
#[doc = "Field `OUTCFG109` writer - Pad output 109 configuration"]
pub type Outcfg109W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg109>;
impl<'a, REG> Outcfg109W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg109::Disabled)
    }
}
#[doc = "Pad output 110 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg110 {
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
impl From<Outcfg110> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg110) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg110 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg110 {}
#[doc = "Field `OUTCFG110` reader - Pad output 110 configuration"]
pub type Outcfg110R = crate::FieldReader<Outcfg110>;
impl Outcfg110R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg110> {
        match self.bits {
            0 => Some(Outcfg110::Timer00),
            1 => Some(Outcfg110::Timer01),
            2 => Some(Outcfg110::Timer10),
            3 => Some(Outcfg110::Timer11),
            4 => Some(Outcfg110::Timer20),
            5 => Some(Outcfg110::Timer21),
            6 => Some(Outcfg110::Timer30),
            7 => Some(Outcfg110::Timer31),
            8 => Some(Outcfg110::Timer40),
            9 => Some(Outcfg110::Timer41),
            10 => Some(Outcfg110::Timer50),
            11 => Some(Outcfg110::Timer51),
            12 => Some(Outcfg110::Timer60),
            13 => Some(Outcfg110::Timer61),
            14 => Some(Outcfg110::Timer70),
            15 => Some(Outcfg110::Timer71),
            16 => Some(Outcfg110::Timer80),
            17 => Some(Outcfg110::Timer81),
            18 => Some(Outcfg110::Timer90),
            19 => Some(Outcfg110::Timer91),
            20 => Some(Outcfg110::Timer100),
            21 => Some(Outcfg110::Timer101),
            22 => Some(Outcfg110::Timer110),
            23 => Some(Outcfg110::Timer111),
            24 => Some(Outcfg110::Timer120),
            25 => Some(Outcfg110::Timer121),
            26 => Some(Outcfg110::Timer130),
            27 => Some(Outcfg110::Timer131),
            28 => Some(Outcfg110::Timer140),
            29 => Some(Outcfg110::Timer141),
            30 => Some(Outcfg110::Timer150),
            31 => Some(Outcfg110::Timer151),
            32 => Some(Outcfg110::Stimer0),
            33 => Some(Outcfg110::Stimer1),
            34 => Some(Outcfg110::Stimer2),
            35 => Some(Outcfg110::Stimer3),
            36 => Some(Outcfg110::Stimer4),
            37 => Some(Outcfg110::Stimer5),
            38 => Some(Outcfg110::Stimer6),
            39 => Some(Outcfg110::Stimer7),
            63 => Some(Outcfg110::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg110::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg110::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg110::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg110::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg110::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg110::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg110::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg110::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg110::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg110::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg110::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg110::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg110::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg110::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg110::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg110::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg110::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg110::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg110::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg110::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg110::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg110::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg110::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg110::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg110::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg110::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg110::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg110::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg110::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg110::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg110::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg110::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg110::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg110::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg110::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg110::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg110::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg110::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg110::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg110::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg110::Disabled
    }
}
#[doc = "Field `OUTCFG110` writer - Pad output 110 configuration"]
pub type Outcfg110W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg110>;
impl<'a, REG> Outcfg110W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg110::Disabled)
    }
}
#[doc = "Pad output 111 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg111 {
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
impl From<Outcfg111> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg111) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg111 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg111 {}
#[doc = "Field `OUTCFG111` reader - Pad output 111 configuration"]
pub type Outcfg111R = crate::FieldReader<Outcfg111>;
impl Outcfg111R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg111> {
        match self.bits {
            0 => Some(Outcfg111::Timer00),
            1 => Some(Outcfg111::Timer01),
            2 => Some(Outcfg111::Timer10),
            3 => Some(Outcfg111::Timer11),
            4 => Some(Outcfg111::Timer20),
            5 => Some(Outcfg111::Timer21),
            6 => Some(Outcfg111::Timer30),
            7 => Some(Outcfg111::Timer31),
            8 => Some(Outcfg111::Timer40),
            9 => Some(Outcfg111::Timer41),
            10 => Some(Outcfg111::Timer50),
            11 => Some(Outcfg111::Timer51),
            12 => Some(Outcfg111::Timer60),
            13 => Some(Outcfg111::Timer61),
            14 => Some(Outcfg111::Timer70),
            15 => Some(Outcfg111::Timer71),
            16 => Some(Outcfg111::Timer80),
            17 => Some(Outcfg111::Timer81),
            18 => Some(Outcfg111::Timer90),
            19 => Some(Outcfg111::Timer91),
            20 => Some(Outcfg111::Timer100),
            21 => Some(Outcfg111::Timer101),
            22 => Some(Outcfg111::Timer110),
            23 => Some(Outcfg111::Timer111),
            24 => Some(Outcfg111::Timer120),
            25 => Some(Outcfg111::Timer121),
            26 => Some(Outcfg111::Timer130),
            27 => Some(Outcfg111::Timer131),
            28 => Some(Outcfg111::Timer140),
            29 => Some(Outcfg111::Timer141),
            30 => Some(Outcfg111::Timer150),
            31 => Some(Outcfg111::Timer151),
            32 => Some(Outcfg111::Stimer0),
            33 => Some(Outcfg111::Stimer1),
            34 => Some(Outcfg111::Stimer2),
            35 => Some(Outcfg111::Stimer3),
            36 => Some(Outcfg111::Stimer4),
            37 => Some(Outcfg111::Stimer5),
            38 => Some(Outcfg111::Stimer6),
            39 => Some(Outcfg111::Stimer7),
            63 => Some(Outcfg111::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg111::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg111::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg111::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg111::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg111::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg111::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg111::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg111::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg111::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg111::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg111::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg111::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg111::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg111::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg111::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg111::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg111::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg111::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg111::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg111::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg111::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg111::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg111::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg111::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg111::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg111::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg111::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg111::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg111::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg111::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg111::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg111::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg111::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg111::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg111::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg111::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg111::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg111::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg111::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg111::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg111::Disabled
    }
}
#[doc = "Field `OUTCFG111` writer - Pad output 111 configuration"]
pub type Outcfg111W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg111>;
impl<'a, REG> Outcfg111W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg111::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Pad output 108 configuration"]
    #[inline(always)]
    pub fn outcfg108(&self) -> Outcfg108R {
        Outcfg108R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Pad output 109 configuration"]
    #[inline(always)]
    pub fn outcfg109(&self) -> Outcfg109R {
        Outcfg109R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Pad output 110 configuration"]
    #[inline(always)]
    pub fn outcfg110(&self) -> Outcfg110R {
        Outcfg110R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Pad output 111 configuration"]
    #[inline(always)]
    pub fn outcfg111(&self) -> Outcfg111R {
        Outcfg111R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pad output 108 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg108(&mut self) -> Outcfg108W<Outcfg27Spec> {
        Outcfg108W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Pad output 109 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg109(&mut self) -> Outcfg109W<Outcfg27Spec> {
        Outcfg109W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Pad output 110 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg110(&mut self) -> Outcfg110W<Outcfg27Spec> {
        Outcfg110W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Pad output 111 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg111(&mut self) -> Outcfg111W<Outcfg27Spec> {
        Outcfg111W::new(self, 24)
    }
}
#[doc = "Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Outcfg27Spec;
impl crate::RegisterSpec for Outcfg27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outcfg27::R`](R) reader structure"]
impl crate::Readable for Outcfg27Spec {}
#[doc = "`write(|w| ..)` method takes [`outcfg27::W`](W) writer structure"]
impl crate::Writable for Outcfg27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTCFG27 to value 0x3f3f_3f3f"]
impl crate::Resettable for Outcfg27Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
