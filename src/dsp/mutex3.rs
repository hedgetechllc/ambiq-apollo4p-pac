#[doc = "Register `MUTEX3` reader"]
pub type R = crate::R<Mutex3Spec>;
#[doc = "Register `MUTEX3` writer"]
pub type W = crate::W<Mutex3Spec>;
#[doc = "Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mutex3 {
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
impl From<Mutex3> for u8 {
    #[inline(always)]
    fn from(variant: Mutex3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mutex3 {
    type Ux = u8;
}
impl crate::IsEnum for Mutex3 {}
#[doc = "Field `MUTEX3` reader - Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)"]
pub type Mutex3R = crate::FieldReader<Mutex3>;
impl Mutex3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mutex3> {
        match self.bits {
            0 => Some(Mutex3::None),
            1 => Some(Mutex3::Cpu),
            2 => Some(Mutex3::Dsp0),
            4 => Some(Mutex3::Dsp1),
            6 => Some(Mutex3::Clear),
            7 => Some(Mutex3::Set),
            _ => None,
        }
    }
    #[doc = "Mutex is free"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Mutex3::None
    }
    #[doc = "CPU Owns Mutex"]
    #[inline(always)]
    pub fn is_cpu(&self) -> bool {
        *self == Mutex3::Cpu
    }
    #[doc = "DSP0 Owns Mutex"]
    #[inline(always)]
    pub fn is_dsp0(&self) -> bool {
        *self == Mutex3::Dsp0
    }
    #[doc = "DSP1 Owns Mutex"]
    #[inline(always)]
    pub fn is_dsp1(&self) -> bool {
        *self == Mutex3::Dsp1
    }
    #[doc = "Clear Mutex (conditional)"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Mutex3::Clear
    }
    #[doc = "Set Mutex (conditional)"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Mutex3::Set
    }
}
#[doc = "Field `MUTEX3` writer - Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)"]
pub type Mutex3W<'a, REG> = crate::FieldWriter<'a, REG, 3, Mutex3>;
impl<'a, REG> Mutex3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mutex is free"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex3::None)
    }
    #[doc = "CPU Owns Mutex"]
    #[inline(always)]
    pub fn cpu(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex3::Cpu)
    }
    #[doc = "DSP0 Owns Mutex"]
    #[inline(always)]
    pub fn dsp0(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex3::Dsp0)
    }
    #[doc = "DSP1 Owns Mutex"]
    #[inline(always)]
    pub fn dsp1(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex3::Dsp1)
    }
    #[doc = "Clear Mutex (conditional)"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex3::Clear)
    }
    #[doc = "Set Mutex (conditional)"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Mutex3::Set)
    }
}
impl R {
    #[doc = "Bits 0:2 - Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)"]
    #[inline(always)]
    pub fn mutex3(&self) -> Mutex3R {
        Mutex3R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mutex Value (000=resource free, 001=CPU owns mutex, 010=DSP0 owns mutex, 100=DSP1 owns mutex)"]
    #[inline(always)]
    #[must_use]
    pub fn mutex3(&mut self) -> Mutex3W<Mutex3Spec> {
        Mutex3W::new(self, 0)
    }
}
#[doc = "MUTEX 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mutex3Spec;
impl crate::RegisterSpec for Mutex3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mutex3::R`](R) reader structure"]
impl crate::Readable for Mutex3Spec {}
#[doc = "`write(|w| ..)` method takes [`mutex3::W`](W) writer structure"]
impl crate::Writable for Mutex3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUTEX3 to value 0"]
impl crate::Resettable for Mutex3Spec {
    const RESET_VALUE: u32 = 0;
}
