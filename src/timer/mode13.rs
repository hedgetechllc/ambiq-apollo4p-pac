#[doc = "Register `MODE13` reader"]
pub type R = crate::R<Mode13Spec>;
#[doc = "Register `MODE13` writer"]
pub type W = crate::W<Mode13Spec>;
#[doc = "Counter/Timer 13 Trigger Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmr13trigsel {
    #[doc = "0: Trigger source is TIMER 0 Output 0"]
    Tmr00 = 0,
    #[doc = "1: Trigger source is TIMER 0 Output 1"]
    Tmr01 = 1,
    #[doc = "2: Trigger source is TIMER 1 Output 0"]
    Tmr10 = 2,
    #[doc = "3: Trigger source is TIMER 1 Output 1"]
    Tmr11 = 3,
    #[doc = "4: Trigger source is TIMER 2 Output 0"]
    Tmr20 = 4,
    #[doc = "5: Trigger source is TIMER 2 Output 1"]
    Tmr21 = 5,
    #[doc = "6: Trigger source is TIMER 3 Output 0"]
    Tmr30 = 6,
    #[doc = "7: Trigger source is TIMER 3 Output 1"]
    Tmr31 = 7,
    #[doc = "8: Trigger source is TIMER 4 Output 0"]
    Tmr40 = 8,
    #[doc = "9: Trigger source is TIMER 4 Output 1"]
    Tmr41 = 9,
    #[doc = "10: Trigger source is TIMER 5 Output 0"]
    Tmr50 = 10,
    #[doc = "11: Trigger source is TIMER 5 Output 1"]
    Tmr51 = 11,
    #[doc = "12: Trigger source is TIMER 6 Output 0"]
    Tmr60 = 12,
    #[doc = "13: Trigger source is TIMER 6 Output 1"]
    Tmr61 = 13,
    #[doc = "14: Trigger source is TIMER 7 Output 0"]
    Tmr70 = 14,
    #[doc = "15: Trigger source is TIMER 7 Output 1"]
    Tmr71 = 15,
    #[doc = "16: Trigger source is TIMER 8 Output 0"]
    Tmr80 = 16,
    #[doc = "17: Trigger source is TIMER 8 Output 1"]
    Tmr81 = 17,
    #[doc = "18: Trigger source is TIMER 9 Output 0"]
    Tmr90 = 18,
    #[doc = "19: Trigger source is TIMER 9 Output 1"]
    Tmr91 = 19,
    #[doc = "20: Trigger source is TIMER 10 Output 0"]
    Tmr100 = 20,
    #[doc = "21: Trigger source is TIMER 10 Output 1"]
    Tmr101 = 21,
    #[doc = "22: Trigger source is TIMER 11 Output 0"]
    Tmr110 = 22,
    #[doc = "23: Trigger source is TIMER 11 Output 1"]
    Tmr111 = 23,
    #[doc = "24: Trigger source is TIMER 12 Output 0"]
    Tmr120 = 24,
    #[doc = "25: Trigger source is TIMER 12 Output 1"]
    Tmr121 = 25,
    #[doc = "26: Trigger source is TIMER 13 Output 0"]
    Tmr130 = 26,
    #[doc = "27: Trigger source is TIMER 13 Output 1"]
    Tmr131 = 27,
    #[doc = "28: Trigger source is TIMER 14 Output 0"]
    Tmr140 = 28,
    #[doc = "29: Trigger source is TIMER 14 Output 1"]
    Tmr141 = 29,
    #[doc = "30: Trigger source is TIMER 15 Output 0"]
    Tmr150 = 30,
    #[doc = "31: Trigger source is TIMER 15 Output 1"]
    Tmr151 = 31,
    #[doc = "128: Trigger source is GPIO #0"]
    Gpio0 = 128,
    #[doc = "255: Trigger source is GPIO #127"]
    Gpio127 = 255,
}
impl From<Tmr13trigsel> for u8 {
    #[inline(always)]
    fn from(variant: Tmr13trigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmr13trigsel {
    type Ux = u8;
}
impl crate::IsEnum for Tmr13trigsel {}
#[doc = "Field `TMR13TRIGSEL` reader - Counter/Timer 13 Trigger Source Selection"]
pub type Tmr13trigselR = crate::FieldReader<Tmr13trigsel>;
impl Tmr13trigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tmr13trigsel> {
        match self.bits {
            0 => Some(Tmr13trigsel::Tmr00),
            1 => Some(Tmr13trigsel::Tmr01),
            2 => Some(Tmr13trigsel::Tmr10),
            3 => Some(Tmr13trigsel::Tmr11),
            4 => Some(Tmr13trigsel::Tmr20),
            5 => Some(Tmr13trigsel::Tmr21),
            6 => Some(Tmr13trigsel::Tmr30),
            7 => Some(Tmr13trigsel::Tmr31),
            8 => Some(Tmr13trigsel::Tmr40),
            9 => Some(Tmr13trigsel::Tmr41),
            10 => Some(Tmr13trigsel::Tmr50),
            11 => Some(Tmr13trigsel::Tmr51),
            12 => Some(Tmr13trigsel::Tmr60),
            13 => Some(Tmr13trigsel::Tmr61),
            14 => Some(Tmr13trigsel::Tmr70),
            15 => Some(Tmr13trigsel::Tmr71),
            16 => Some(Tmr13trigsel::Tmr80),
            17 => Some(Tmr13trigsel::Tmr81),
            18 => Some(Tmr13trigsel::Tmr90),
            19 => Some(Tmr13trigsel::Tmr91),
            20 => Some(Tmr13trigsel::Tmr100),
            21 => Some(Tmr13trigsel::Tmr101),
            22 => Some(Tmr13trigsel::Tmr110),
            23 => Some(Tmr13trigsel::Tmr111),
            24 => Some(Tmr13trigsel::Tmr120),
            25 => Some(Tmr13trigsel::Tmr121),
            26 => Some(Tmr13trigsel::Tmr130),
            27 => Some(Tmr13trigsel::Tmr131),
            28 => Some(Tmr13trigsel::Tmr140),
            29 => Some(Tmr13trigsel::Tmr141),
            30 => Some(Tmr13trigsel::Tmr150),
            31 => Some(Tmr13trigsel::Tmr151),
            128 => Some(Tmr13trigsel::Gpio0),
            255 => Some(Tmr13trigsel::Gpio127),
            _ => None,
        }
    }
    #[doc = "Trigger source is TIMER 0 Output 0"]
    #[inline(always)]
    pub fn is_tmr00(&self) -> bool {
        *self == Tmr13trigsel::Tmr00
    }
    #[doc = "Trigger source is TIMER 0 Output 1"]
    #[inline(always)]
    pub fn is_tmr01(&self) -> bool {
        *self == Tmr13trigsel::Tmr01
    }
    #[doc = "Trigger source is TIMER 1 Output 0"]
    #[inline(always)]
    pub fn is_tmr10(&self) -> bool {
        *self == Tmr13trigsel::Tmr10
    }
    #[doc = "Trigger source is TIMER 1 Output 1"]
    #[inline(always)]
    pub fn is_tmr11(&self) -> bool {
        *self == Tmr13trigsel::Tmr11
    }
    #[doc = "Trigger source is TIMER 2 Output 0"]
    #[inline(always)]
    pub fn is_tmr20(&self) -> bool {
        *self == Tmr13trigsel::Tmr20
    }
    #[doc = "Trigger source is TIMER 2 Output 1"]
    #[inline(always)]
    pub fn is_tmr21(&self) -> bool {
        *self == Tmr13trigsel::Tmr21
    }
    #[doc = "Trigger source is TIMER 3 Output 0"]
    #[inline(always)]
    pub fn is_tmr30(&self) -> bool {
        *self == Tmr13trigsel::Tmr30
    }
    #[doc = "Trigger source is TIMER 3 Output 1"]
    #[inline(always)]
    pub fn is_tmr31(&self) -> bool {
        *self == Tmr13trigsel::Tmr31
    }
    #[doc = "Trigger source is TIMER 4 Output 0"]
    #[inline(always)]
    pub fn is_tmr40(&self) -> bool {
        *self == Tmr13trigsel::Tmr40
    }
    #[doc = "Trigger source is TIMER 4 Output 1"]
    #[inline(always)]
    pub fn is_tmr41(&self) -> bool {
        *self == Tmr13trigsel::Tmr41
    }
    #[doc = "Trigger source is TIMER 5 Output 0"]
    #[inline(always)]
    pub fn is_tmr50(&self) -> bool {
        *self == Tmr13trigsel::Tmr50
    }
    #[doc = "Trigger source is TIMER 5 Output 1"]
    #[inline(always)]
    pub fn is_tmr51(&self) -> bool {
        *self == Tmr13trigsel::Tmr51
    }
    #[doc = "Trigger source is TIMER 6 Output 0"]
    #[inline(always)]
    pub fn is_tmr60(&self) -> bool {
        *self == Tmr13trigsel::Tmr60
    }
    #[doc = "Trigger source is TIMER 6 Output 1"]
    #[inline(always)]
    pub fn is_tmr61(&self) -> bool {
        *self == Tmr13trigsel::Tmr61
    }
    #[doc = "Trigger source is TIMER 7 Output 0"]
    #[inline(always)]
    pub fn is_tmr70(&self) -> bool {
        *self == Tmr13trigsel::Tmr70
    }
    #[doc = "Trigger source is TIMER 7 Output 1"]
    #[inline(always)]
    pub fn is_tmr71(&self) -> bool {
        *self == Tmr13trigsel::Tmr71
    }
    #[doc = "Trigger source is TIMER 8 Output 0"]
    #[inline(always)]
    pub fn is_tmr80(&self) -> bool {
        *self == Tmr13trigsel::Tmr80
    }
    #[doc = "Trigger source is TIMER 8 Output 1"]
    #[inline(always)]
    pub fn is_tmr81(&self) -> bool {
        *self == Tmr13trigsel::Tmr81
    }
    #[doc = "Trigger source is TIMER 9 Output 0"]
    #[inline(always)]
    pub fn is_tmr90(&self) -> bool {
        *self == Tmr13trigsel::Tmr90
    }
    #[doc = "Trigger source is TIMER 9 Output 1"]
    #[inline(always)]
    pub fn is_tmr91(&self) -> bool {
        *self == Tmr13trigsel::Tmr91
    }
    #[doc = "Trigger source is TIMER 10 Output 0"]
    #[inline(always)]
    pub fn is_tmr100(&self) -> bool {
        *self == Tmr13trigsel::Tmr100
    }
    #[doc = "Trigger source is TIMER 10 Output 1"]
    #[inline(always)]
    pub fn is_tmr101(&self) -> bool {
        *self == Tmr13trigsel::Tmr101
    }
    #[doc = "Trigger source is TIMER 11 Output 0"]
    #[inline(always)]
    pub fn is_tmr110(&self) -> bool {
        *self == Tmr13trigsel::Tmr110
    }
    #[doc = "Trigger source is TIMER 11 Output 1"]
    #[inline(always)]
    pub fn is_tmr111(&self) -> bool {
        *self == Tmr13trigsel::Tmr111
    }
    #[doc = "Trigger source is TIMER 12 Output 0"]
    #[inline(always)]
    pub fn is_tmr120(&self) -> bool {
        *self == Tmr13trigsel::Tmr120
    }
    #[doc = "Trigger source is TIMER 12 Output 1"]
    #[inline(always)]
    pub fn is_tmr121(&self) -> bool {
        *self == Tmr13trigsel::Tmr121
    }
    #[doc = "Trigger source is TIMER 13 Output 0"]
    #[inline(always)]
    pub fn is_tmr130(&self) -> bool {
        *self == Tmr13trigsel::Tmr130
    }
    #[doc = "Trigger source is TIMER 13 Output 1"]
    #[inline(always)]
    pub fn is_tmr131(&self) -> bool {
        *self == Tmr13trigsel::Tmr131
    }
    #[doc = "Trigger source is TIMER 14 Output 0"]
    #[inline(always)]
    pub fn is_tmr140(&self) -> bool {
        *self == Tmr13trigsel::Tmr140
    }
    #[doc = "Trigger source is TIMER 14 Output 1"]
    #[inline(always)]
    pub fn is_tmr141(&self) -> bool {
        *self == Tmr13trigsel::Tmr141
    }
    #[doc = "Trigger source is TIMER 15 Output 0"]
    #[inline(always)]
    pub fn is_tmr150(&self) -> bool {
        *self == Tmr13trigsel::Tmr150
    }
    #[doc = "Trigger source is TIMER 15 Output 1"]
    #[inline(always)]
    pub fn is_tmr151(&self) -> bool {
        *self == Tmr13trigsel::Tmr151
    }
    #[doc = "Trigger source is GPIO #0"]
    #[inline(always)]
    pub fn is_gpio0(&self) -> bool {
        *self == Tmr13trigsel::Gpio0
    }
    #[doc = "Trigger source is GPIO #127"]
    #[inline(always)]
    pub fn is_gpio127(&self) -> bool {
        *self == Tmr13trigsel::Gpio127
    }
}
#[doc = "Field `TMR13TRIGSEL` writer - Counter/Timer 13 Trigger Source Selection"]
pub type Tmr13trigselW<'a, REG> = crate::FieldWriter<'a, REG, 8, Tmr13trigsel>;
impl<'a, REG> Tmr13trigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger source is TIMER 0 Output 0"]
    #[inline(always)]
    pub fn tmr00(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr00)
    }
    #[doc = "Trigger source is TIMER 0 Output 1"]
    #[inline(always)]
    pub fn tmr01(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr01)
    }
    #[doc = "Trigger source is TIMER 1 Output 0"]
    #[inline(always)]
    pub fn tmr10(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr10)
    }
    #[doc = "Trigger source is TIMER 1 Output 1"]
    #[inline(always)]
    pub fn tmr11(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr11)
    }
    #[doc = "Trigger source is TIMER 2 Output 0"]
    #[inline(always)]
    pub fn tmr20(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr20)
    }
    #[doc = "Trigger source is TIMER 2 Output 1"]
    #[inline(always)]
    pub fn tmr21(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr21)
    }
    #[doc = "Trigger source is TIMER 3 Output 0"]
    #[inline(always)]
    pub fn tmr30(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr30)
    }
    #[doc = "Trigger source is TIMER 3 Output 1"]
    #[inline(always)]
    pub fn tmr31(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr31)
    }
    #[doc = "Trigger source is TIMER 4 Output 0"]
    #[inline(always)]
    pub fn tmr40(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr40)
    }
    #[doc = "Trigger source is TIMER 4 Output 1"]
    #[inline(always)]
    pub fn tmr41(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr41)
    }
    #[doc = "Trigger source is TIMER 5 Output 0"]
    #[inline(always)]
    pub fn tmr50(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr50)
    }
    #[doc = "Trigger source is TIMER 5 Output 1"]
    #[inline(always)]
    pub fn tmr51(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr51)
    }
    #[doc = "Trigger source is TIMER 6 Output 0"]
    #[inline(always)]
    pub fn tmr60(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr60)
    }
    #[doc = "Trigger source is TIMER 6 Output 1"]
    #[inline(always)]
    pub fn tmr61(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr61)
    }
    #[doc = "Trigger source is TIMER 7 Output 0"]
    #[inline(always)]
    pub fn tmr70(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr70)
    }
    #[doc = "Trigger source is TIMER 7 Output 1"]
    #[inline(always)]
    pub fn tmr71(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr71)
    }
    #[doc = "Trigger source is TIMER 8 Output 0"]
    #[inline(always)]
    pub fn tmr80(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr80)
    }
    #[doc = "Trigger source is TIMER 8 Output 1"]
    #[inline(always)]
    pub fn tmr81(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr81)
    }
    #[doc = "Trigger source is TIMER 9 Output 0"]
    #[inline(always)]
    pub fn tmr90(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr90)
    }
    #[doc = "Trigger source is TIMER 9 Output 1"]
    #[inline(always)]
    pub fn tmr91(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr91)
    }
    #[doc = "Trigger source is TIMER 10 Output 0"]
    #[inline(always)]
    pub fn tmr100(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr100)
    }
    #[doc = "Trigger source is TIMER 10 Output 1"]
    #[inline(always)]
    pub fn tmr101(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr101)
    }
    #[doc = "Trigger source is TIMER 11 Output 0"]
    #[inline(always)]
    pub fn tmr110(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr110)
    }
    #[doc = "Trigger source is TIMER 11 Output 1"]
    #[inline(always)]
    pub fn tmr111(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr111)
    }
    #[doc = "Trigger source is TIMER 12 Output 0"]
    #[inline(always)]
    pub fn tmr120(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr120)
    }
    #[doc = "Trigger source is TIMER 12 Output 1"]
    #[inline(always)]
    pub fn tmr121(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr121)
    }
    #[doc = "Trigger source is TIMER 13 Output 0"]
    #[inline(always)]
    pub fn tmr130(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr130)
    }
    #[doc = "Trigger source is TIMER 13 Output 1"]
    #[inline(always)]
    pub fn tmr131(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr131)
    }
    #[doc = "Trigger source is TIMER 14 Output 0"]
    #[inline(always)]
    pub fn tmr140(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr140)
    }
    #[doc = "Trigger source is TIMER 14 Output 1"]
    #[inline(always)]
    pub fn tmr141(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr141)
    }
    #[doc = "Trigger source is TIMER 15 Output 0"]
    #[inline(always)]
    pub fn tmr150(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr150)
    }
    #[doc = "Trigger source is TIMER 15 Output 1"]
    #[inline(always)]
    pub fn tmr151(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Tmr151)
    }
    #[doc = "Trigger source is GPIO #0"]
    #[inline(always)]
    pub fn gpio0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Gpio0)
    }
    #[doc = "Trigger source is GPIO #127"]
    #[inline(always)]
    pub fn gpio127(self) -> &'a mut crate::W<REG> {
        self.variant(Tmr13trigsel::Gpio127)
    }
}
impl R {
    #[doc = "Bits 8:15 - Counter/Timer 13 Trigger Source Selection"]
    #[inline(always)]
    pub fn tmr13trigsel(&self) -> Tmr13trigselR {
        Tmr13trigselR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Counter/Timer 13 Trigger Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13trigsel(&mut self) -> Tmr13trigselW<Mode13Spec> {
        Tmr13trigselW::new(self, 8)
    }
}
#[doc = "The mode register contains optional mode controls for the timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mode13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mode13Spec;
impl crate::RegisterSpec for Mode13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode13::R`](R) reader structure"]
impl crate::Readable for Mode13Spec {}
#[doc = "`write(|w| ..)` method takes [`mode13::W`](W) writer structure"]
impl crate::Writable for Mode13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE13 to value 0"]
impl crate::Resettable for Mode13Spec {
    const RESET_VALUE: u32 = 0;
}
