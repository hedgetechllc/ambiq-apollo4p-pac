#[doc = "Register `MUTEX5` reader"]
pub type R = crate::R<Mutex5Spec>;
#[doc = "Register `MUTEX5` writer"]
pub type W = crate::W<Mutex5Spec>;
#[doc = "Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mutex5 {
    #[doc = "0: Mutex is free"]
    None = 0,
    #[doc = "1: CPU Owns Mutex"]
    Cpu = 1,
    #[doc = "2: DSP0 Owns Mutex"]
    Dsp0 = 2,
    #[doc = "4: DSP1 Owns Mutex"]
    Dsp1 = 4,
    #[doc = "6: Clear Mutex (conditional)"]
    Clear = 6,
    #[doc = "7: Set Mutex (conditional)"]
    Set = 7,
}
impl From<Mutex5> for u8 {
    #[inline(always)]
    fn from(variant: Mutex5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mutex5 {
    type Ux = u8;
}
impl crate::IsEnum for Mutex5 {}
#[doc = "Field `MUTEX5` reader - Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)"]
pub type Mutex5R = crate::FieldReader<Mutex5>;
impl Mutex5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mutex5> {
        match self.bits {
            0 => Some(Mutex5::None),
            1 => Some(Mutex5::Cpu),
            2 => Some(Mutex5::Dsp0),
            4 => Some(Mutex5::Dsp1),
            6 => Some(Mutex5::Clear),
            7 => Some(Mutex5::Set),
            _ => None,
        }
    }
    #[doc = "Mutex is free"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Mutex5::None
    }
    #[doc = "CPU Owns Mutex"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == Mutex5::Cpu
    }
    #[doc = "DSP0 Owns Mutex"]
    #[inline(always)]
    pub fn is_dsp0(&self) -> bool {
        *self == Mutex5::Dsp0
    }
    #[doc = "DSP1 Owns Mutex"]
    #[inline(always)]
    pub fn is_dsp1(&self) -> bool {
        *self == Mutex5::Dsp1
    }
    #[doc = "Clear Mutex (conditional)"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Mutex5::Clear
    }
    #[doc = "Set Mutex (conditional)"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mutex5::Set
    }
}
#[doc = "Field `MUTEX5` writer - Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)"]
pub type Mutex5W<'a, REG> = crate::FieldWriter<'a, REG, 3, Mutex5>;
impl<'a, REG> Mutex5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mutex is free"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex5::None)
    }
    #[doc = "CPU Owns Mutex"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex5::Cpu)
    }
    #[doc = "DSP0 Owns Mutex"]
    #[inline(always)]
    pub fn dsp0(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex5::Dsp0)
    }
    #[doc = "DSP1 Owns Mutex"]
    #[inline(always)]
    pub fn dsp1(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex5::Dsp1)
    }
    #[doc = "Clear Mutex (conditional)"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex5::Clear)
    }
    #[doc = "Set Mutex (conditional)"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex5::Set)
    }
}
impl R {
    #[doc = "Bits 0:2 - Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)"]
    #[inline(always)]
    pub fn mutex5(&self) -> Mutex5R {
        Mutex5R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)"]
    #[inline(always)]
    #[must_use]
    pub fn mutex5(&mut self) -> Mutex5W<Mutex5Spec> {
        Mutex5W::new(self, 0)
    }
}
#[doc = "MUTEX 5\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mutex5Spec;
impl crate::RegisterSpec for Mutex5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mutex5::R`](R) reader structure"]
impl crate::Readable for Mutex5Spec {}
#[doc = "`write(|w| ..)` method takes [`mutex5::W`](W) writer structure"]
impl crate::Writable for Mutex5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUTEX5 to value 0"]
impl crate::Resettable for Mutex5Spec {
    const RESET_VALUE: u32 = 0;
}
