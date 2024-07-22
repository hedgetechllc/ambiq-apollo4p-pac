#[doc = "Register `OUTCFG21` reader"]
pub type R = crate::R<Outcfg21Spec>;
#[doc = "Register `OUTCFG21` writer"]
pub type W = crate::W<Outcfg21Spec>;
#[doc = "Pad output 84 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg84 {
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
impl From<Outcfg84> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg84) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg84 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg84 {}
#[doc = "Field `OUTCFG84` reader - Pad output 84 configuration"]
pub type Outcfg84R = crate::FieldReader<Outcfg84>;
impl Outcfg84R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg84> {
        match self.bits {
            0 => Some(Outcfg84::Timer00),
            1 => Some(Outcfg84::Timer01),
            2 => Some(Outcfg84::Timer10),
            3 => Some(Outcfg84::Timer11),
            4 => Some(Outcfg84::Timer20),
            5 => Some(Outcfg84::Timer21),
            6 => Some(Outcfg84::Timer30),
            7 => Some(Outcfg84::Timer31),
            8 => Some(Outcfg84::Timer40),
            9 => Some(Outcfg84::Timer41),
            10 => Some(Outcfg84::Timer50),
            11 => Some(Outcfg84::Timer51),
            12 => Some(Outcfg84::Timer60),
            13 => Some(Outcfg84::Timer61),
            14 => Some(Outcfg84::Timer70),
            15 => Some(Outcfg84::Timer71),
            16 => Some(Outcfg84::Timer80),
            17 => Some(Outcfg84::Timer81),
            18 => Some(Outcfg84::Timer90),
            19 => Some(Outcfg84::Timer91),
            20 => Some(Outcfg84::Timer100),
            21 => Some(Outcfg84::Timer101),
            22 => Some(Outcfg84::Timer110),
            23 => Some(Outcfg84::Timer111),
            24 => Some(Outcfg84::Timer120),
            25 => Some(Outcfg84::Timer121),
            26 => Some(Outcfg84::Timer130),
            27 => Some(Outcfg84::Timer131),
            28 => Some(Outcfg84::Timer140),
            29 => Some(Outcfg84::Timer141),
            30 => Some(Outcfg84::Timer150),
            31 => Some(Outcfg84::Timer151),
            32 => Some(Outcfg84::Stimer0),
            33 => Some(Outcfg84::Stimer1),
            34 => Some(Outcfg84::Stimer2),
            35 => Some(Outcfg84::Stimer3),
            36 => Some(Outcfg84::Stimer4),
            37 => Some(Outcfg84::Stimer5),
            38 => Some(Outcfg84::Stimer6),
            39 => Some(Outcfg84::Stimer7),
            63 => Some(Outcfg84::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg84::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg84::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg84::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg84::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg84::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg84::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg84::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg84::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg84::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg84::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg84::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg84::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg84::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg84::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg84::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg84::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg84::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg84::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg84::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg84::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg84::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg84::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg84::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg84::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg84::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg84::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg84::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg84::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg84::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg84::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg84::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg84::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg84::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg84::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg84::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg84::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg84::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg84::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg84::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg84::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg84::Disabled
    }
}
#[doc = "Field `OUTCFG84` writer - Pad output 84 configuration"]
pub type Outcfg84W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg84>;
impl<'a, REG> Outcfg84W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg84::Disabled)
    }
}
#[doc = "Pad output 85 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg85 {
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
impl From<Outcfg85> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg85) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg85 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg85 {}
#[doc = "Field `OUTCFG85` reader - Pad output 85 configuration"]
pub type Outcfg85R = crate::FieldReader<Outcfg85>;
impl Outcfg85R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg85> {
        match self.bits {
            0 => Some(Outcfg85::Timer00),
            1 => Some(Outcfg85::Timer01),
            2 => Some(Outcfg85::Timer10),
            3 => Some(Outcfg85::Timer11),
            4 => Some(Outcfg85::Timer20),
            5 => Some(Outcfg85::Timer21),
            6 => Some(Outcfg85::Timer30),
            7 => Some(Outcfg85::Timer31),
            8 => Some(Outcfg85::Timer40),
            9 => Some(Outcfg85::Timer41),
            10 => Some(Outcfg85::Timer50),
            11 => Some(Outcfg85::Timer51),
            12 => Some(Outcfg85::Timer60),
            13 => Some(Outcfg85::Timer61),
            14 => Some(Outcfg85::Timer70),
            15 => Some(Outcfg85::Timer71),
            16 => Some(Outcfg85::Timer80),
            17 => Some(Outcfg85::Timer81),
            18 => Some(Outcfg85::Timer90),
            19 => Some(Outcfg85::Timer91),
            20 => Some(Outcfg85::Timer100),
            21 => Some(Outcfg85::Timer101),
            22 => Some(Outcfg85::Timer110),
            23 => Some(Outcfg85::Timer111),
            24 => Some(Outcfg85::Timer120),
            25 => Some(Outcfg85::Timer121),
            26 => Some(Outcfg85::Timer130),
            27 => Some(Outcfg85::Timer131),
            28 => Some(Outcfg85::Timer140),
            29 => Some(Outcfg85::Timer141),
            30 => Some(Outcfg85::Timer150),
            31 => Some(Outcfg85::Timer151),
            32 => Some(Outcfg85::Stimer0),
            33 => Some(Outcfg85::Stimer1),
            34 => Some(Outcfg85::Stimer2),
            35 => Some(Outcfg85::Stimer3),
            36 => Some(Outcfg85::Stimer4),
            37 => Some(Outcfg85::Stimer5),
            38 => Some(Outcfg85::Stimer6),
            39 => Some(Outcfg85::Stimer7),
            63 => Some(Outcfg85::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg85::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg85::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg85::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg85::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg85::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg85::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg85::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg85::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg85::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg85::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg85::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg85::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg85::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg85::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg85::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg85::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg85::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg85::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg85::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg85::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg85::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg85::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg85::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg85::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg85::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg85::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg85::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg85::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg85::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg85::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg85::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg85::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg85::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg85::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg85::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg85::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg85::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg85::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg85::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg85::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg85::Disabled
    }
}
#[doc = "Field `OUTCFG85` writer - Pad output 85 configuration"]
pub type Outcfg85W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg85>;
impl<'a, REG> Outcfg85W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg85::Disabled)
    }
}
#[doc = "Pad output 86 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg86 {
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
impl From<Outcfg86> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg86) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg86 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg86 {}
#[doc = "Field `OUTCFG86` reader - Pad output 86 configuration"]
pub type Outcfg86R = crate::FieldReader<Outcfg86>;
impl Outcfg86R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg86> {
        match self.bits {
            0 => Some(Outcfg86::Timer00),
            1 => Some(Outcfg86::Timer01),
            2 => Some(Outcfg86::Timer10),
            3 => Some(Outcfg86::Timer11),
            4 => Some(Outcfg86::Timer20),
            5 => Some(Outcfg86::Timer21),
            6 => Some(Outcfg86::Timer30),
            7 => Some(Outcfg86::Timer31),
            8 => Some(Outcfg86::Timer40),
            9 => Some(Outcfg86::Timer41),
            10 => Some(Outcfg86::Timer50),
            11 => Some(Outcfg86::Timer51),
            12 => Some(Outcfg86::Timer60),
            13 => Some(Outcfg86::Timer61),
            14 => Some(Outcfg86::Timer70),
            15 => Some(Outcfg86::Timer71),
            16 => Some(Outcfg86::Timer80),
            17 => Some(Outcfg86::Timer81),
            18 => Some(Outcfg86::Timer90),
            19 => Some(Outcfg86::Timer91),
            20 => Some(Outcfg86::Timer100),
            21 => Some(Outcfg86::Timer101),
            22 => Some(Outcfg86::Timer110),
            23 => Some(Outcfg86::Timer111),
            24 => Some(Outcfg86::Timer120),
            25 => Some(Outcfg86::Timer121),
            26 => Some(Outcfg86::Timer130),
            27 => Some(Outcfg86::Timer131),
            28 => Some(Outcfg86::Timer140),
            29 => Some(Outcfg86::Timer141),
            30 => Some(Outcfg86::Timer150),
            31 => Some(Outcfg86::Timer151),
            32 => Some(Outcfg86::Stimer0),
            33 => Some(Outcfg86::Stimer1),
            34 => Some(Outcfg86::Stimer2),
            35 => Some(Outcfg86::Stimer3),
            36 => Some(Outcfg86::Stimer4),
            37 => Some(Outcfg86::Stimer5),
            38 => Some(Outcfg86::Stimer6),
            39 => Some(Outcfg86::Stimer7),
            63 => Some(Outcfg86::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg86::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg86::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg86::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg86::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg86::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg86::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg86::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg86::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg86::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg86::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg86::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg86::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg86::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg86::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg86::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg86::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg86::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg86::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg86::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg86::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg86::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg86::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg86::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg86::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg86::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg86::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg86::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg86::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg86::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg86::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg86::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg86::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg86::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg86::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg86::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg86::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg86::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg86::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg86::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg86::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg86::Disabled
    }
}
#[doc = "Field `OUTCFG86` writer - Pad output 86 configuration"]
pub type Outcfg86W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg86>;
impl<'a, REG> Outcfg86W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg86::Disabled)
    }
}
#[doc = "Pad output 87 configuration\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcfg87 {
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
impl From<Outcfg87> for u8 {
    #[inline(always)]
    fn from(variant: Outcfg87) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outcfg87 {
    type Ux = u8;
}
impl crate::IsEnum for Outcfg87 {}
#[doc = "Field `OUTCFG87` reader - Pad output 87 configuration"]
pub type Outcfg87R = crate::FieldReader<Outcfg87>;
impl Outcfg87R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outcfg87> {
        match self.bits {
            0 => Some(Outcfg87::Timer00),
            1 => Some(Outcfg87::Timer01),
            2 => Some(Outcfg87::Timer10),
            3 => Some(Outcfg87::Timer11),
            4 => Some(Outcfg87::Timer20),
            5 => Some(Outcfg87::Timer21),
            6 => Some(Outcfg87::Timer30),
            7 => Some(Outcfg87::Timer31),
            8 => Some(Outcfg87::Timer40),
            9 => Some(Outcfg87::Timer41),
            10 => Some(Outcfg87::Timer50),
            11 => Some(Outcfg87::Timer51),
            12 => Some(Outcfg87::Timer60),
            13 => Some(Outcfg87::Timer61),
            14 => Some(Outcfg87::Timer70),
            15 => Some(Outcfg87::Timer71),
            16 => Some(Outcfg87::Timer80),
            17 => Some(Outcfg87::Timer81),
            18 => Some(Outcfg87::Timer90),
            19 => Some(Outcfg87::Timer91),
            20 => Some(Outcfg87::Timer100),
            21 => Some(Outcfg87::Timer101),
            22 => Some(Outcfg87::Timer110),
            23 => Some(Outcfg87::Timer111),
            24 => Some(Outcfg87::Timer120),
            25 => Some(Outcfg87::Timer121),
            26 => Some(Outcfg87::Timer130),
            27 => Some(Outcfg87::Timer131),
            28 => Some(Outcfg87::Timer140),
            29 => Some(Outcfg87::Timer141),
            30 => Some(Outcfg87::Timer150),
            31 => Some(Outcfg87::Timer151),
            32 => Some(Outcfg87::Stimer0),
            33 => Some(Outcfg87::Stimer1),
            34 => Some(Outcfg87::Stimer2),
            35 => Some(Outcfg87::Stimer3),
            36 => Some(Outcfg87::Stimer4),
            37 => Some(Outcfg87::Stimer5),
            38 => Some(Outcfg87::Stimer6),
            39 => Some(Outcfg87::Stimer7),
            63 => Some(Outcfg87::Disabled),
            _ => None,
        }
    }
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn is_timer00(&self) -> bool {
        *self == Outcfg87::Timer00
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn is_timer01(&self) -> bool {
        *self == Outcfg87::Timer01
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn is_timer10(&self) -> bool {
        *self == Outcfg87::Timer10
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn is_timer11(&self) -> bool {
        *self == Outcfg87::Timer11
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn is_timer20(&self) -> bool {
        *self == Outcfg87::Timer20
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn is_timer21(&self) -> bool {
        *self == Outcfg87::Timer21
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn is_timer30(&self) -> bool {
        *self == Outcfg87::Timer30
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn is_timer31(&self) -> bool {
        *self == Outcfg87::Timer31
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn is_timer40(&self) -> bool {
        *self == Outcfg87::Timer40
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn is_timer41(&self) -> bool {
        *self == Outcfg87::Timer41
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn is_timer50(&self) -> bool {
        *self == Outcfg87::Timer50
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn is_timer51(&self) -> bool {
        *self == Outcfg87::Timer51
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn is_timer60(&self) -> bool {
        *self == Outcfg87::Timer60
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn is_timer61(&self) -> bool {
        *self == Outcfg87::Timer61
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn is_timer70(&self) -> bool {
        *self == Outcfg87::Timer70
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn is_timer71(&self) -> bool {
        *self == Outcfg87::Timer71
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn is_timer80(&self) -> bool {
        *self == Outcfg87::Timer80
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn is_timer81(&self) -> bool {
        *self == Outcfg87::Timer81
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn is_timer90(&self) -> bool {
        *self == Outcfg87::Timer90
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn is_timer91(&self) -> bool {
        *self == Outcfg87::Timer91
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn is_timer100(&self) -> bool {
        *self == Outcfg87::Timer100
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn is_timer101(&self) -> bool {
        *self == Outcfg87::Timer101
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn is_timer110(&self) -> bool {
        *self == Outcfg87::Timer110
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn is_timer111(&self) -> bool {
        *self == Outcfg87::Timer111
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn is_timer120(&self) -> bool {
        *self == Outcfg87::Timer120
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn is_timer121(&self) -> bool {
        *self == Outcfg87::Timer121
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn is_timer130(&self) -> bool {
        *self == Outcfg87::Timer130
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn is_timer131(&self) -> bool {
        *self == Outcfg87::Timer131
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn is_timer140(&self) -> bool {
        *self == Outcfg87::Timer140
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn is_timer141(&self) -> bool {
        *self == Outcfg87::Timer141
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn is_timer150(&self) -> bool {
        *self == Outcfg87::Timer150
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn is_timer151(&self) -> bool {
        *self == Outcfg87::Timer151
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn is_stimer0(&self) -> bool {
        *self == Outcfg87::Stimer0
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn is_stimer1(&self) -> bool {
        *self == Outcfg87::Stimer1
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn is_stimer2(&self) -> bool {
        *self == Outcfg87::Stimer2
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn is_stimer3(&self) -> bool {
        *self == Outcfg87::Stimer3
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn is_stimer4(&self) -> bool {
        *self == Outcfg87::Stimer4
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn is_stimer5(&self) -> bool {
        *self == Outcfg87::Stimer5
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn is_stimer6(&self) -> bool {
        *self == Outcfg87::Stimer6
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn is_stimer7(&self) -> bool {
        *self == Outcfg87::Stimer7
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outcfg87::Disabled
    }
}
#[doc = "Field `OUTCFG87` writer - Pad output 87 configuration"]
pub type Outcfg87W<'a, REG> = crate::FieldWriter<'a, REG, 6, Outcfg87>;
impl<'a, REG> Outcfg87W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output is Timer 0, output 0"]
    #[inline(always)]
    pub fn timer00(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer00)
    }
    #[doc = "Output is Timer 0, output 1"]
    #[inline(always)]
    pub fn timer01(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer01)
    }
    #[doc = "Output is Timer 1, output 0"]
    #[inline(always)]
    pub fn timer10(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer10)
    }
    #[doc = "Output is Timer 1, output 1"]
    #[inline(always)]
    pub fn timer11(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer11)
    }
    #[doc = "Output is Timer 2, output 0"]
    #[inline(always)]
    pub fn timer20(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer20)
    }
    #[doc = "Output is Timer 2, output 1"]
    #[inline(always)]
    pub fn timer21(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer21)
    }
    #[doc = "Output is Timer 3, output 0"]
    #[inline(always)]
    pub fn timer30(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer30)
    }
    #[doc = "Output is Timer 3, output 1"]
    #[inline(always)]
    pub fn timer31(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer31)
    }
    #[doc = "Output is Timer 4, output 0"]
    #[inline(always)]
    pub fn timer40(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer40)
    }
    #[doc = "Output is Timer 4, output 1"]
    #[inline(always)]
    pub fn timer41(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer41)
    }
    #[doc = "Output is Timer 5, output 0"]
    #[inline(always)]
    pub fn timer50(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer50)
    }
    #[doc = "Output is Timer 5, output 1"]
    #[inline(always)]
    pub fn timer51(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer51)
    }
    #[doc = "Output is Timer 6, output 0"]
    #[inline(always)]
    pub fn timer60(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer60)
    }
    #[doc = "Output is Timer 6, output 1"]
    #[inline(always)]
    pub fn timer61(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer61)
    }
    #[doc = "Output is Timer 7, output 0"]
    #[inline(always)]
    pub fn timer70(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer70)
    }
    #[doc = "Output is Timer 7, output 1"]
    #[inline(always)]
    pub fn timer71(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer71)
    }
    #[doc = "Output is Timer 8, output 0"]
    #[inline(always)]
    pub fn timer80(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer80)
    }
    #[doc = "Output is Timer 8, output 1"]
    #[inline(always)]
    pub fn timer81(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer81)
    }
    #[doc = "Output is Timer 9, output 0"]
    #[inline(always)]
    pub fn timer90(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer90)
    }
    #[doc = "Output is Timer 9, output 1"]
    #[inline(always)]
    pub fn timer91(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer91)
    }
    #[doc = "Output is Timer 10, output 0"]
    #[inline(always)]
    pub fn timer100(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer100)
    }
    #[doc = "Output is Timer 10, output 1"]
    #[inline(always)]
    pub fn timer101(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer101)
    }
    #[doc = "Output is Timer 11, output 0"]
    #[inline(always)]
    pub fn timer110(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer110)
    }
    #[doc = "Output is Timer 11, output 1"]
    #[inline(always)]
    pub fn timer111(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer111)
    }
    #[doc = "Output is Timer 12, output 0"]
    #[inline(always)]
    pub fn timer120(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer120)
    }
    #[doc = "Output is Timer 12, output 1"]
    #[inline(always)]
    pub fn timer121(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer121)
    }
    #[doc = "Output is Timer 13, output 0"]
    #[inline(always)]
    pub fn timer130(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer130)
    }
    #[doc = "Output is Timer 13, output 1"]
    #[inline(always)]
    pub fn timer131(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer131)
    }
    #[doc = "Output is Timer 14, output 0"]
    #[inline(always)]
    pub fn timer140(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer140)
    }
    #[doc = "Output is Timer 14, output 1"]
    #[inline(always)]
    pub fn timer141(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer141)
    }
    #[doc = "Output is Timer 15, output 0"]
    #[inline(always)]
    pub fn timer150(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer150)
    }
    #[doc = "Output is Timer 15, output 1"]
    #[inline(always)]
    pub fn timer151(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Timer151)
    }
    #[doc = "Output is STimer 0"]
    #[inline(always)]
    pub fn stimer0(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Stimer0)
    }
    #[doc = "Output is STimer 1"]
    #[inline(always)]
    pub fn stimer1(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Stimer1)
    }
    #[doc = "Output is STimer 2"]
    #[inline(always)]
    pub fn stimer2(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Stimer2)
    }
    #[doc = "Output is STimer 3"]
    #[inline(always)]
    pub fn stimer3(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Stimer3)
    }
    #[doc = "Output is STimer 4"]
    #[inline(always)]
    pub fn stimer4(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Stimer4)
    }
    #[doc = "Output is STimer 5"]
    #[inline(always)]
    pub fn stimer5(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Stimer5)
    }
    #[doc = "Output is STimer 6"]
    #[inline(always)]
    pub fn stimer6(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Stimer6)
    }
    #[doc = "Output is STimer 7"]
    #[inline(always)]
    pub fn stimer7(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Stimer7)
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Outcfg87::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Pad output 84 configuration"]
    #[inline(always)]
    pub fn outcfg84(&self) -> Outcfg84R {
        Outcfg84R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Pad output 85 configuration"]
    #[inline(always)]
    pub fn outcfg85(&self) -> Outcfg85R {
        Outcfg85R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Pad output 86 configuration"]
    #[inline(always)]
    pub fn outcfg86(&self) -> Outcfg86R {
        Outcfg86R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Pad output 87 configuration"]
    #[inline(always)]
    pub fn outcfg87(&self) -> Outcfg87R {
        Outcfg87R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pad output 84 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg84(&mut self) -> Outcfg84W<Outcfg21Spec> {
        Outcfg84W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Pad output 85 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg85(&mut self) -> Outcfg85W<Outcfg21Spec> {
        Outcfg85W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Pad output 86 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg86(&mut self) -> Outcfg86W<Outcfg21Spec> {
        Outcfg86W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Pad output 87 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg87(&mut self) -> Outcfg87W<Outcfg21Spec> {
        Outcfg87W::new(self, 24)
    }
}
#[doc = "Pad output configuration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`outcfg21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcfg21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Outcfg21Spec;
impl crate::RegisterSpec for Outcfg21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outcfg21::R`](R) reader structure"]
impl crate::Readable for Outcfg21Spec {}
#[doc = "`write(|w| ..)` method takes [`outcfg21::W`](W) writer structure"]
impl crate::Writable for Outcfg21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTCFG21 to value 0x3f3f_3f3f"]
impl crate::Resettable for Outcfg21Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
