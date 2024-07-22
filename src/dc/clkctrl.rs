#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Value of first clock divider\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dividevalue {
    #[doc = "0: Divided by 0"]
    Fdiv0 = 0,
    #[doc = "2: Divided by 2"]
    Fdiv2 = 2,
    #[doc = "3: Divided by 3"]
    Fdiv3 = 3,
    #[doc = "4: Divided by 4"]
    Fdiv4 = 4,
    #[doc = "5: Divided by 5"]
    Fdiv5 = 5,
    #[doc = "6: Divided by 6"]
    Fdiv6 = 6,
    #[doc = "7: Divided by 7"]
    Fdiv7 = 7,
    #[doc = "8: Divided by 8"]
    Fdiv8 = 8,
    #[doc = "9: Divided by 9"]
    Fdiv9 = 9,
    #[doc = "10: Divided by 10"]
    Fdiv10 = 10,
    #[doc = "11: Divided by 11"]
    Fdiv11 = 11,
    #[doc = "12: Divided by 12"]
    Fdiv12 = 12,
    #[doc = "13: Divided by 13"]
    Fdiv13 = 13,
    #[doc = "14: Divided by 14"]
    Fdiv14 = 14,
    #[doc = "15: Divided by 15"]
    Fdiv15 = 15,
    #[doc = "16: Divided by 16"]
    Fdiv16 = 16,
    #[doc = "17: Divided by 17"]
    Fdiv17 = 17,
    #[doc = "18: Divided by 18"]
    Fdiv18 = 18,
    #[doc = "19: Divided by 19"]
    Fdiv19 = 19,
    #[doc = "20: Divided by 20"]
    Fdiv20 = 20,
    #[doc = "21: Divided by 21"]
    Fdiv21 = 21,
    #[doc = "22: Divided by 22"]
    Fdiv22 = 22,
    #[doc = "23: Divided by 23"]
    Fdiv23 = 23,
    #[doc = "24: Divided by 24"]
    Fdiv24 = 24,
    #[doc = "25: Divided by 25"]
    Fdiv25 = 25,
    #[doc = "26: Divided by 26"]
    Fdiv26 = 26,
    #[doc = "27: Divided by 27"]
    Fdiv27 = 27,
    #[doc = "28: Divided by 28"]
    Fdiv28 = 28,
    #[doc = "29: Divided by 29"]
    Fdiv29 = 29,
    #[doc = "30: Divided by 30"]
    Fdiv30 = 30,
    #[doc = "31: Divided by 31"]
    Fdiv31 = 31,
}
impl From<Dividevalue> for u8 {
    #[inline(always)]
    fn from(variant: Dividevalue) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dividevalue {
    type Ux = u8;
}
impl crate::IsEnum for Dividevalue {}
#[doc = "Field `DIVIDEVALUE` reader - Value of first clock divider"]
pub type DividevalueR = crate::FieldReader<Dividevalue>;
impl DividevalueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dividevalue> {
        match self.bits {
            0 => Some(Dividevalue::Fdiv0),
            2 => Some(Dividevalue::Fdiv2),
            3 => Some(Dividevalue::Fdiv3),
            4 => Some(Dividevalue::Fdiv4),
            5 => Some(Dividevalue::Fdiv5),
            6 => Some(Dividevalue::Fdiv6),
            7 => Some(Dividevalue::Fdiv7),
            8 => Some(Dividevalue::Fdiv8),
            9 => Some(Dividevalue::Fdiv9),
            10 => Some(Dividevalue::Fdiv10),
            11 => Some(Dividevalue::Fdiv11),
            12 => Some(Dividevalue::Fdiv12),
            13 => Some(Dividevalue::Fdiv13),
            14 => Some(Dividevalue::Fdiv14),
            15 => Some(Dividevalue::Fdiv15),
            16 => Some(Dividevalue::Fdiv16),
            17 => Some(Dividevalue::Fdiv17),
            18 => Some(Dividevalue::Fdiv18),
            19 => Some(Dividevalue::Fdiv19),
            20 => Some(Dividevalue::Fdiv20),
            21 => Some(Dividevalue::Fdiv21),
            22 => Some(Dividevalue::Fdiv22),
            23 => Some(Dividevalue::Fdiv23),
            24 => Some(Dividevalue::Fdiv24),
            25 => Some(Dividevalue::Fdiv25),
            26 => Some(Dividevalue::Fdiv26),
            27 => Some(Dividevalue::Fdiv27),
            28 => Some(Dividevalue::Fdiv28),
            29 => Some(Dividevalue::Fdiv29),
            30 => Some(Dividevalue::Fdiv30),
            31 => Some(Dividevalue::Fdiv31),
            _ => None,
        }
    }
    #[doc = "Divided by 0"]
    #[inline(always)]
    pub fn is_fdiv_0(&self) -> bool {
        *self == Dividevalue::Fdiv0
    }
    #[doc = "Divided by 2"]
    #[inline(always)]
    pub fn is_fdiv_2(&self) -> bool {
        *self == Dividevalue::Fdiv2
    }
    #[doc = "Divided by 3"]
    #[inline(always)]
    pub fn is_fdiv_3(&self) -> bool {
        *self == Dividevalue::Fdiv3
    }
    #[doc = "Divided by 4"]
    #[inline(always)]
    pub fn is_fdiv_4(&self) -> bool {
        *self == Dividevalue::Fdiv4
    }
    #[doc = "Divided by 5"]
    #[inline(always)]
    pub fn is_fdiv_5(&self) -> bool {
        *self == Dividevalue::Fdiv5
    }
    #[doc = "Divided by 6"]
    #[inline(always)]
    pub fn is_fdiv_6(&self) -> bool {
        *self == Dividevalue::Fdiv6
    }
    #[doc = "Divided by 7"]
    #[inline(always)]
    pub fn is_fdiv_7(&self) -> bool {
        *self == Dividevalue::Fdiv7
    }
    #[doc = "Divided by 8"]
    #[inline(always)]
    pub fn is_fdiv_8(&self) -> bool {
        *self == Dividevalue::Fdiv8
    }
    #[doc = "Divided by 9"]
    #[inline(always)]
    pub fn is_fdiv_9(&self) -> bool {
        *self == Dividevalue::Fdiv9
    }
    #[doc = "Divided by 10"]
    #[inline(always)]
    pub fn is_fdiv_10(&self) -> bool {
        *self == Dividevalue::Fdiv10
    }
    #[doc = "Divided by 11"]
    #[inline(always)]
    pub fn is_fdiv_11(&self) -> bool {
        *self == Dividevalue::Fdiv11
    }
    #[doc = "Divided by 12"]
    #[inline(always)]
    pub fn is_fdiv_12(&self) -> bool {
        *self == Dividevalue::Fdiv12
    }
    #[doc = "Divided by 13"]
    #[inline(always)]
    pub fn is_fdiv_13(&self) -> bool {
        *self == Dividevalue::Fdiv13
    }
    #[doc = "Divided by 14"]
    #[inline(always)]
    pub fn is_fdiv_14(&self) -> bool {
        *self == Dividevalue::Fdiv14
    }
    #[doc = "Divided by 15"]
    #[inline(always)]
    pub fn is_fdiv_15(&self) -> bool {
        *self == Dividevalue::Fdiv15
    }
    #[doc = "Divided by 16"]
    #[inline(always)]
    pub fn is_fdiv_16(&self) -> bool {
        *self == Dividevalue::Fdiv16
    }
    #[doc = "Divided by 17"]
    #[inline(always)]
    pub fn is_fdiv_17(&self) -> bool {
        *self == Dividevalue::Fdiv17
    }
    #[doc = "Divided by 18"]
    #[inline(always)]
    pub fn is_fdiv_18(&self) -> bool {
        *self == Dividevalue::Fdiv18
    }
    #[doc = "Divided by 19"]
    #[inline(always)]
    pub fn is_fdiv_19(&self) -> bool {
        *self == Dividevalue::Fdiv19
    }
    #[doc = "Divided by 20"]
    #[inline(always)]
    pub fn is_fdiv_20(&self) -> bool {
        *self == Dividevalue::Fdiv20
    }
    #[doc = "Divided by 21"]
    #[inline(always)]
    pub fn is_fdiv_21(&self) -> bool {
        *self == Dividevalue::Fdiv21
    }
    #[doc = "Divided by 22"]
    #[inline(always)]
    pub fn is_fdiv_22(&self) -> bool {
        *self == Dividevalue::Fdiv22
    }
    #[doc = "Divided by 23"]
    #[inline(always)]
    pub fn is_fdiv_23(&self) -> bool {
        *self == Dividevalue::Fdiv23
    }
    #[doc = "Divided by 24"]
    #[inline(always)]
    pub fn is_fdiv_24(&self) -> bool {
        *self == Dividevalue::Fdiv24
    }
    #[doc = "Divided by 25"]
    #[inline(always)]
    pub fn is_fdiv_25(&self) -> bool {
        *self == Dividevalue::Fdiv25
    }
    #[doc = "Divided by 26"]
    #[inline(always)]
    pub fn is_fdiv_26(&self) -> bool {
        *self == Dividevalue::Fdiv26
    }
    #[doc = "Divided by 27"]
    #[inline(always)]
    pub fn is_fdiv_27(&self) -> bool {
        *self == Dividevalue::Fdiv27
    }
    #[doc = "Divided by 28"]
    #[inline(always)]
    pub fn is_fdiv_28(&self) -> bool {
        *self == Dividevalue::Fdiv28
    }
    #[doc = "Divided by 29"]
    #[inline(always)]
    pub fn is_fdiv_29(&self) -> bool {
        *self == Dividevalue::Fdiv29
    }
    #[doc = "Divided by 30"]
    #[inline(always)]
    pub fn is_fdiv_30(&self) -> bool {
        *self == Dividevalue::Fdiv30
    }
    #[doc = "Divided by 31"]
    #[inline(always)]
    pub fn is_fdiv_31(&self) -> bool {
        *self == Dividevalue::Fdiv31
    }
}
#[doc = "Field `DIVIDEVALUE` writer - Value of first clock divider"]
pub type DividevalueW<'a, REG> = crate::FieldWriter<'a, REG, 6, Dividevalue>;
impl<'a, REG> DividevalueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divided by 0"]
    #[inline(always)]
    pub fn fdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv0)
    }
    #[doc = "Divided by 2"]
    #[inline(always)]
    pub fn fdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv2)
    }
    #[doc = "Divided by 3"]
    #[inline(always)]
    pub fn fdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv3)
    }
    #[doc = "Divided by 4"]
    #[inline(always)]
    pub fn fdiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv4)
    }
    #[doc = "Divided by 5"]
    #[inline(always)]
    pub fn fdiv_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv5)
    }
    #[doc = "Divided by 6"]
    #[inline(always)]
    pub fn fdiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv6)
    }
    #[doc = "Divided by 7"]
    #[inline(always)]
    pub fn fdiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv7)
    }
    #[doc = "Divided by 8"]
    #[inline(always)]
    pub fn fdiv_8(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv8)
    }
    #[doc = "Divided by 9"]
    #[inline(always)]
    pub fn fdiv_9(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv9)
    }
    #[doc = "Divided by 10"]
    #[inline(always)]
    pub fn fdiv_10(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv10)
    }
    #[doc = "Divided by 11"]
    #[inline(always)]
    pub fn fdiv_11(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv11)
    }
    #[doc = "Divided by 12"]
    #[inline(always)]
    pub fn fdiv_12(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv12)
    }
    #[doc = "Divided by 13"]
    #[inline(always)]
    pub fn fdiv_13(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv13)
    }
    #[doc = "Divided by 14"]
    #[inline(always)]
    pub fn fdiv_14(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv14)
    }
    #[doc = "Divided by 15"]
    #[inline(always)]
    pub fn fdiv_15(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv15)
    }
    #[doc = "Divided by 16"]
    #[inline(always)]
    pub fn fdiv_16(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv16)
    }
    #[doc = "Divided by 17"]
    #[inline(always)]
    pub fn fdiv_17(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv17)
    }
    #[doc = "Divided by 18"]
    #[inline(always)]
    pub fn fdiv_18(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv18)
    }
    #[doc = "Divided by 19"]
    #[inline(always)]
    pub fn fdiv_19(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv19)
    }
    #[doc = "Divided by 20"]
    #[inline(always)]
    pub fn fdiv_20(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv20)
    }
    #[doc = "Divided by 21"]
    #[inline(always)]
    pub fn fdiv_21(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv21)
    }
    #[doc = "Divided by 22"]
    #[inline(always)]
    pub fn fdiv_22(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv22)
    }
    #[doc = "Divided by 23"]
    #[inline(always)]
    pub fn fdiv_23(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv23)
    }
    #[doc = "Divided by 24"]
    #[inline(always)]
    pub fn fdiv_24(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv24)
    }
    #[doc = "Divided by 25"]
    #[inline(always)]
    pub fn fdiv_25(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv25)
    }
    #[doc = "Divided by 26"]
    #[inline(always)]
    pub fn fdiv_26(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv26)
    }
    #[doc = "Divided by 27"]
    #[inline(always)]
    pub fn fdiv_27(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv27)
    }
    #[doc = "Divided by 28"]
    #[inline(always)]
    pub fn fdiv_28(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv28)
    }
    #[doc = "Divided by 29"]
    #[inline(always)]
    pub fn fdiv_29(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv29)
    }
    #[doc = "Divided by 30"]
    #[inline(always)]
    pub fn fdiv_30(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv30)
    }
    #[doc = "Divided by 31"]
    #[inline(always)]
    pub fn fdiv_31(self) -> &'a mut crate::W<REG> {
        self.variant(Dividevalue::Fdiv31)
    }
}
#[doc = "Field `RSVD0` reader - This field is reserved."]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `RSVD0` writer - This field is reserved."]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINENUM` reader - Number of lines to be prefetched before starting the frame through DMA. Maximum value is 32"]
pub type LinenumR = crate::FieldReader;
#[doc = "Field `LINENUM` writer - Number of lines to be prefetched before starting the frame through DMA. Maximum value is 32"]
pub type LinenumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RSVD1` reader - This field is reserved."]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `RSVD1` writer - This field is reserved."]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL` reader - Select PLL Clock"]
pub type PllR = crate::FieldReader;
#[doc = "Field `PLL` writer - Select PLL Clock"]
pub type PllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LVDS` reader - Clock phase shift value for LVDS operation"]
pub type LvdsR = crate::FieldReader;
#[doc = "Field `LVDS` writer - Clock phase shift value for LVDS operation"]
pub type LvdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Value of secondary clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Secclkdiv {
    #[doc = "0: No division"]
    Sdiv0 = 0,
    #[doc = "1: No division"]
    Sdiv1 = 1,
    #[doc = "2: Divided by 2"]
    Sdiv2 = 2,
    #[doc = "3: Divided by 3"]
    Sdiv3 = 3,
    #[doc = "4: Divided by 4"]
    Sdiv4 = 4,
    #[doc = "5: Divided by 5"]
    Sdiv5 = 5,
    #[doc = "6: Divided by 6"]
    Sdiv6 = 6,
    #[doc = "7: Divided by 7"]
    Sdiv7 = 7,
    #[doc = "8: Divided by 8"]
    Sdiv8 = 8,
    #[doc = "9: Divided by 9"]
    Sdiv9 = 9,
    #[doc = "10: Divided by 10"]
    Sdiv10 = 10,
    #[doc = "11: Divided by 11"]
    Sdiv11 = 11,
    #[doc = "12: Divided by 12"]
    Sdiv12 = 12,
    #[doc = "13: Divided by 13"]
    Sdiv13 = 13,
    #[doc = "14: Divided by 14"]
    Sdiv14 = 14,
    #[doc = "15: Divided by 15"]
    Sdiv15 = 15,
}
impl From<Secclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Secclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Secclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Secclkdiv {}
#[doc = "Field `SECCLKDIV` reader - Value of secondary clock divider"]
pub type SecclkdivR = crate::FieldReader<Secclkdiv>;
impl SecclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Secclkdiv> {
        match self.bits {
            0 => Some(Secclkdiv::Sdiv0),
            1 => Some(Secclkdiv::Sdiv1),
            2 => Some(Secclkdiv::Sdiv2),
            3 => Some(Secclkdiv::Sdiv3),
            4 => Some(Secclkdiv::Sdiv4),
            5 => Some(Secclkdiv::Sdiv5),
            6 => Some(Secclkdiv::Sdiv6),
            7 => Some(Secclkdiv::Sdiv7),
            8 => Some(Secclkdiv::Sdiv8),
            9 => Some(Secclkdiv::Sdiv9),
            10 => Some(Secclkdiv::Sdiv10),
            11 => Some(Secclkdiv::Sdiv11),
            12 => Some(Secclkdiv::Sdiv12),
            13 => Some(Secclkdiv::Sdiv13),
            14 => Some(Secclkdiv::Sdiv14),
            15 => Some(Secclkdiv::Sdiv15),
            _ => None,
        }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_sdiv_0(&self) -> bool {
        *self == Secclkdiv::Sdiv0
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_sdiv_1(&self) -> bool {
        *self == Secclkdiv::Sdiv1
    }
    #[doc = "Divided by 2"]
    #[inline(always)]
    pub fn is_sdiv_2(&self) -> bool {
        *self == Secclkdiv::Sdiv2
    }
    #[doc = "Divided by 3"]
    #[inline(always)]
    pub fn is_sdiv_3(&self) -> bool {
        *self == Secclkdiv::Sdiv3
    }
    #[doc = "Divided by 4"]
    #[inline(always)]
    pub fn is_sdiv_4(&self) -> bool {
        *self == Secclkdiv::Sdiv4
    }
    #[doc = "Divided by 5"]
    #[inline(always)]
    pub fn is_sdiv_5(&self) -> bool {
        *self == Secclkdiv::Sdiv5
    }
    #[doc = "Divided by 6"]
    #[inline(always)]
    pub fn is_sdiv_6(&self) -> bool {
        *self == Secclkdiv::Sdiv6
    }
    #[doc = "Divided by 7"]
    #[inline(always)]
    pub fn is_sdiv_7(&self) -> bool {
        *self == Secclkdiv::Sdiv7
    }
    #[doc = "Divided by 8"]
    #[inline(always)]
    pub fn is_sdiv_8(&self) -> bool {
        *self == Secclkdiv::Sdiv8
    }
    #[doc = "Divided by 9"]
    #[inline(always)]
    pub fn is_sdiv_9(&self) -> bool {
        *self == Secclkdiv::Sdiv9
    }
    #[doc = "Divided by 10"]
    #[inline(always)]
    pub fn is_sdiv_10(&self) -> bool {
        *self == Secclkdiv::Sdiv10
    }
    #[doc = "Divided by 11"]
    #[inline(always)]
    pub fn is_sdiv_11(&self) -> bool {
        *self == Secclkdiv::Sdiv11
    }
    #[doc = "Divided by 12"]
    #[inline(always)]
    pub fn is_sdiv_12(&self) -> bool {
        *self == Secclkdiv::Sdiv12
    }
    #[doc = "Divided by 13"]
    #[inline(always)]
    pub fn is_sdiv_13(&self) -> bool {
        *self == Secclkdiv::Sdiv13
    }
    #[doc = "Divided by 14"]
    #[inline(always)]
    pub fn is_sdiv_14(&self) -> bool {
        *self == Secclkdiv::Sdiv14
    }
    #[doc = "Divided by 15"]
    #[inline(always)]
    pub fn is_sdiv_15(&self) -> bool {
        *self == Secclkdiv::Sdiv15
    }
}
#[doc = "Field `SECCLKDIV` writer - Value of secondary clock divider"]
pub type SecclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 5, Secclkdiv>;
impl<'a, REG> SecclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn sdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv0)
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn sdiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv1)
    }
    #[doc = "Divided by 2"]
    #[inline(always)]
    pub fn sdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv2)
    }
    #[doc = "Divided by 3"]
    #[inline(always)]
    pub fn sdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv3)
    }
    #[doc = "Divided by 4"]
    #[inline(always)]
    pub fn sdiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv4)
    }
    #[doc = "Divided by 5"]
    #[inline(always)]
    pub fn sdiv_5(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv5)
    }
    #[doc = "Divided by 6"]
    #[inline(always)]
    pub fn sdiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv6)
    }
    #[doc = "Divided by 7"]
    #[inline(always)]
    pub fn sdiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv7)
    }
    #[doc = "Divided by 8"]
    #[inline(always)]
    pub fn sdiv_8(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv8)
    }
    #[doc = "Divided by 9"]
    #[inline(always)]
    pub fn sdiv_9(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv9)
    }
    #[doc = "Divided by 10"]
    #[inline(always)]
    pub fn sdiv_10(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv10)
    }
    #[doc = "Divided by 11"]
    #[inline(always)]
    pub fn sdiv_11(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv11)
    }
    #[doc = "Divided by 12"]
    #[inline(always)]
    pub fn sdiv_12(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv12)
    }
    #[doc = "Divided by 13"]
    #[inline(always)]
    pub fn sdiv_13(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv13)
    }
    #[doc = "Divided by 14"]
    #[inline(always)]
    pub fn sdiv_14(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv14)
    }
    #[doc = "Divided by 15"]
    #[inline(always)]
    pub fn sdiv_15(self) -> &'a mut crate::W<REG> {
        self.variant(Secclkdiv::Sdiv15)
    }
}
impl R {
    #[doc = "Bits 0:5 - Value of first clock divider"]
    #[inline(always)]
    pub fn dividevalue(&self) -> DividevalueR {
        DividevalueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Number of lines to be prefetched before starting the frame through DMA. Maximum value is 32"]
    #[inline(always)]
    pub fn linenum(&self) -> LinenumR {
        LinenumR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - This field is reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Select PLL Clock"]
    #[inline(always)]
    pub fn pll(&self) -> PllR {
        PllR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Clock phase shift value for LVDS operation"]
    #[inline(always)]
    pub fn lvds(&self) -> LvdsR {
        LvdsR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - Value of secondary clock divider"]
    #[inline(always)]
    pub fn secclkdiv(&self) -> SecclkdivR {
        SecclkdivR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Value of first clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn dividevalue(&mut self) -> DividevalueW<ClkctrlSpec> {
        DividevalueW::new(self, 0)
    }
    #[doc = "Bits 6:7 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> Rsvd0W<ClkctrlSpec> {
        Rsvd0W::new(self, 6)
    }
    #[doc = "Bits 8:13 - Number of lines to be prefetched before starting the frame through DMA. Maximum value is 32"]
    #[inline(always)]
    #[must_use]
    pub fn linenum(&mut self) -> LinenumW<ClkctrlSpec> {
        LinenumW::new(self, 8)
    }
    #[doc = "Bits 14:15 - This field is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> Rsvd1W<ClkctrlSpec> {
        Rsvd1W::new(self, 14)
    }
    #[doc = "Bits 16:23 - Select PLL Clock"]
    #[inline(always)]
    #[must_use]
    pub fn pll(&mut self) -> PllW<ClkctrlSpec> {
        PllW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Clock phase shift value for LVDS operation"]
    #[inline(always)]
    #[must_use]
    pub fn lvds(&mut self) -> LvdsW<ClkctrlSpec> {
        LvdsW::new(self, 24)
    }
    #[doc = "Bits 27:31 - Value of secondary clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn secclkdiv(&mut self) -> SecclkdivW<ClkctrlSpec> {
        SecclkdivW::new(self, 27)
    }
}
#[doc = "Setup proper timing with divisor control bits and specify the number of lines to be prefetched before the start of frame.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0x0401"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u32 = 0x0401;
}
