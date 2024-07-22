#[doc = "Register `MAXIMUM1` reader"]
pub type R = crate::R<Maximum1Spec>;
#[doc = "Register `MAXIMUM1` writer"]
pub type W = crate::W<Maximum1Spec>;
#[doc = "Maximum Current for 3.3V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxcurr33v {
    #[doc = "255: 1020mA = 255 * 4mA"]
    _1020mA = 255,
    #[doc = "1: 1020mA, 255 * 4mA"]
    _4mA = 1,
}
impl From<Maxcurr33v> for u8 {
    #[inline(always)]
    fn from(variant: Maxcurr33v) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxcurr33v {
    type Ux = u8;
}
impl crate::IsEnum for Maxcurr33v {}
#[doc = "Field `MAXCURR33V` reader - Maximum Current for 3.3V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
pub type Maxcurr33vR = crate::FieldReader<Maxcurr33v>;
impl Maxcurr33vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxcurr33v> {
        match self.bits {
            255 => Some(Maxcurr33v::_1020mA),
            1 => Some(Maxcurr33v::_4mA),
            _ => None,
        }
    }
    #[doc = "1020mA = 255 * 4mA"]
    #[inline(always)]
    pub fn is_1020m_a(&self) -> bool {
        *self == Maxcurr33v::_1020mA
    }
    #[doc = "1020mA, 255 * 4mA"]
    #[inline(always)]
    pub fn is_4m_a(&self) -> bool {
        *self == Maxcurr33v::_4mA
    }
}
#[doc = "Field `MAXCURR33V` writer - Maximum Current for 3.3V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
pub type Maxcurr33vW<'a, REG> = crate::FieldWriter<'a, REG, 8, Maxcurr33v>;
impl<'a, REG> Maxcurr33vW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1020mA = 255 * 4mA"]
    #[inline(always)]
    pub fn _1020m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Maxcurr33v::_1020mA)
    }
    #[doc = "1020mA, 255 * 4mA"]
    #[inline(always)]
    pub fn _4m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Maxcurr33v::_4mA)
    }
}
#[doc = "Maximum Current for 3.0V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxcurr30v {
    #[doc = "255: 1020mA = 255 * 4mA"]
    _1020mA = 255,
    #[doc = "1: 1020mA, 255 * 4mA"]
    _4mA = 1,
}
impl From<Maxcurr30v> for u8 {
    #[inline(always)]
    fn from(variant: Maxcurr30v) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxcurr30v {
    type Ux = u8;
}
impl crate::IsEnum for Maxcurr30v {}
#[doc = "Field `MAXCURR30V` reader - Maximum Current for 3.0V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
pub type Maxcurr30vR = crate::FieldReader<Maxcurr30v>;
impl Maxcurr30vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxcurr30v> {
        match self.bits {
            255 => Some(Maxcurr30v::_1020mA),
            1 => Some(Maxcurr30v::_4mA),
            _ => None,
        }
    }
    #[doc = "1020mA = 255 * 4mA"]
    #[inline(always)]
    pub fn is_1020m_a(&self) -> bool {
        *self == Maxcurr30v::_1020mA
    }
    #[doc = "1020mA, 255 * 4mA"]
    #[inline(always)]
    pub fn is_4m_a(&self) -> bool {
        *self == Maxcurr30v::_4mA
    }
}
#[doc = "Field `MAXCURR30V` writer - Maximum Current for 3.0V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
pub type Maxcurr30vW<'a, REG> = crate::FieldWriter<'a, REG, 8, Maxcurr30v>;
impl<'a, REG> Maxcurr30vW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1020mA = 255 * 4mA"]
    #[inline(always)]
    pub fn _1020m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Maxcurr30v::_1020mA)
    }
    #[doc = "1020mA, 255 * 4mA"]
    #[inline(always)]
    pub fn _4m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Maxcurr30v::_4mA)
    }
}
#[doc = "Maximum Current for 1.8V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxcurr18v {
    #[doc = "255: 1020mA = 255 * 4mA"]
    _1020mA = 255,
    #[doc = "1: 1020mA, 255 * 4mA"]
    _4mA = 1,
}
impl From<Maxcurr18v> for u8 {
    #[inline(always)]
    fn from(variant: Maxcurr18v) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxcurr18v {
    type Ux = u8;
}
impl crate::IsEnum for Maxcurr18v {}
#[doc = "Field `MAXCURR18V` reader - Maximum Current for 1.8V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
pub type Maxcurr18vR = crate::FieldReader<Maxcurr18v>;
impl Maxcurr18vR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maxcurr18v> {
        match self.bits {
            255 => Some(Maxcurr18v::_1020mA),
            1 => Some(Maxcurr18v::_4mA),
            _ => None,
        }
    }
    #[doc = "1020mA = 255 * 4mA"]
    #[inline(always)]
    pub fn is_1020m_a(&self) -> bool {
        *self == Maxcurr18v::_1020mA
    }
    #[doc = "1020mA, 255 * 4mA"]
    #[inline(always)]
    pub fn is_4m_a(&self) -> bool {
        *self == Maxcurr18v::_4mA
    }
}
#[doc = "Field `MAXCURR18V` writer - Maximum Current for 1.8V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
pub type Maxcurr18vW<'a, REG> = crate::FieldWriter<'a, REG, 8, Maxcurr18v>;
impl<'a, REG> Maxcurr18vW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1020mA = 255 * 4mA"]
    #[inline(always)]
    pub fn _1020m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Maxcurr18v::_1020mA)
    }
    #[doc = "1020mA, 255 * 4mA"]
    #[inline(always)]
    pub fn _4m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Maxcurr18v::_4mA)
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
    #[inline(always)]
    pub fn maxcurr33v(&self) -> Maxcurr33vR {
        Maxcurr33vR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
    #[inline(always)]
    pub fn maxcurr30v(&self) -> Maxcurr30vR {
        Maxcurr30vR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
    #[inline(always)]
    pub fn maxcurr18v(&self) -> Maxcurr18vR {
        Maxcurr18vR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
    #[inline(always)]
    #[must_use]
    pub fn maxcurr33v(&mut self) -> Maxcurr33vW<Maximum1Spec> {
        Maxcurr33vW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
    #[inline(always)]
    #[must_use]
    pub fn maxcurr30v(&mut self) -> Maxcurr30vW<Maximum1Spec> {
        Maxcurr30vW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V. The current value is specified as MAXCURR18V * 4mA. Some example enums follow:"]
    #[inline(always)]
    #[must_use]
    pub fn maxcurr18v(&mut self) -> Maxcurr18vW<Maximum1Spec> {
        Maxcurr18vW::new(self, 16)
    }
}
#[doc = "Maximum current capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`maximum1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maximum1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maximum1Spec;
impl crate::RegisterSpec for Maximum1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maximum1::R`](R) reader structure"]
impl crate::Readable for Maximum1Spec {}
#[doc = "`write(|w| ..)` method takes [`maximum1::W`](W) writer structure"]
impl crate::Writable for Maximum1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXIMUM1 to value 0"]
impl crate::Resettable for Maximum1Spec {
    const RESET_VALUE: u32 = 0;
}
