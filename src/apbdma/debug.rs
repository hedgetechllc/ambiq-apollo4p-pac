#[doc = "Register `DEBUG` reader"]
pub type R = crate::R<DebugSpec>;
#[doc = "Register `DEBUG` writer"]
pub type W = crate::W<DebugSpec>;
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Debugen {
    #[doc = "0: Debug Disabled"]
    Off = 0,
    #[doc = "1: Debug Arb values"]
    Arb = 1,
}
impl From<Debugen> for u8 {
    #[inline(always)]
    fn from(variant: Debugen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Debugen {
    type Ux = u8;
}
impl crate::IsEnum for Debugen {}
#[doc = "Field `DEBUGEN` reader - Debug Enable"]
pub type DebugenR = crate::FieldReader<Debugen>;
impl DebugenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Debugen> {
        match self.bits {
            0 => Some(Debugen::Off),
            1 => Some(Debugen::Arb),
            _ => None,
        }
    }
    #[doc = "Debug Disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Debugen::Off
    }
    #[doc = "Debug Arb values"]
    #[inline(always)]
    pub fn is_arb(&self) -> bool {
        *self == Debugen::Arb
    }
}
#[doc = "Field `DEBUGEN` writer - Debug Enable"]
pub type DebugenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Debugen>;
impl<'a, REG> DebugenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Debug Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Debugen::Off)
    }
    #[doc = "Debug Arb values"]
    #[inline(always)]
    pub fn arb(self) -> &'a mut crate::W<REG> {
        self.variant(Debugen::Arb)
    }
}
impl R {
    #[doc = "Bits 0:3 - Debug Enable"]
    #[inline(always)]
    pub fn debugen(&self) -> DebugenR {
        DebugenR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugen(&mut self) -> DebugenW<DebugSpec> {
        DebugenW::new(self, 0)
    }
}
#[doc = "PIO Input Values\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSpec;
impl crate::RegisterSpec for DebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DebugSpec {}
#[doc = "`write(|w| ..)` method takes [`debug::W`](W) writer structure"]
impl crate::Writable for DebugSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DebugSpec {
    const RESET_VALUE: u32 = 0;
}
