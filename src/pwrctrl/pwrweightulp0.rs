#[doc = "Register `PWRWEIGHTULP0` reader"]
pub type R = crate::R<Pwrweightulp0Spec>;
#[doc = "Register `PWRWEIGHTULP0` writer"]
pub type W = crate::W<Pwrweightulp0Spec>;
#[doc = "Field `WTULPMCU` reader - Weight used for ULP mode MCU"]
pub type WtulpmcuR = crate::FieldReader;
#[doc = "Field `WTULPMCU` writer - Weight used for ULP mode MCU"]
pub type WtulpmcuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPDSP0` reader - Weight used for ULP mode DSP0"]
pub type Wtulpdsp0R = crate::FieldReader;
#[doc = "Field `WTULPDSP0` writer - Weight used for ULP mode DSP0"]
pub type Wtulpdsp0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPDSP1` reader - Weight used for ULP mode DSP1"]
pub type Wtulpdsp1R = crate::FieldReader;
#[doc = "Field `WTULPDSP1` writer - Weight used for ULP mode DSP1"]
pub type Wtulpdsp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPIOS` reader - Weight used for ULP mode IOS"]
pub type WtulpiosR = crate::FieldReader;
#[doc = "Field `WTULPIOS` writer - Weight used for ULP mode IOS"]
pub type WtulpiosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPUART0` reader - Weight used for ULP mode UART0"]
pub type Wtulpuart0R = crate::FieldReader;
#[doc = "Field `WTULPUART0` writer - Weight used for ULP mode UART0"]
pub type Wtulpuart0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPUART1` reader - Weight used for ULP mode UART1"]
pub type Wtulpuart1R = crate::FieldReader;
#[doc = "Field `WTULPUART1` writer - Weight used for ULP mode UART1"]
pub type Wtulpuart1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPUART2` reader - Weight used for ULP mode UART2"]
pub type Wtulpuart2R = crate::FieldReader;
#[doc = "Field `WTULPUART2` writer - Weight used for ULP mode UART2"]
pub type Wtulpuart2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTULPUART3` reader - Weight used for ULP mode UART3"]
pub type Wtulpuart3R = crate::FieldReader;
#[doc = "Field `WTULPUART3` writer - Weight used for ULP mode UART3"]
pub type Wtulpuart3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for ULP mode MCU"]
    #[inline(always)]
    pub fn wtulpmcu(&self) -> WtulpmcuR {
        WtulpmcuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode DSP0"]
    #[inline(always)]
    pub fn wtulpdsp0(&self) -> Wtulpdsp0R {
        Wtulpdsp0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for ULP mode DSP1"]
    #[inline(always)]
    pub fn wtulpdsp1(&self) -> Wtulpdsp1R {
        Wtulpdsp1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for ULP mode IOS"]
    #[inline(always)]
    pub fn wtulpios(&self) -> WtulpiosR {
        WtulpiosR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode UART0"]
    #[inline(always)]
    pub fn wtulpuart0(&self) -> Wtulpuart0R {
        Wtulpuart0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for ULP mode UART1"]
    #[inline(always)]
    pub fn wtulpuart1(&self) -> Wtulpuart1R {
        Wtulpuart1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for ULP mode UART2"]
    #[inline(always)]
    pub fn wtulpuart2(&self) -> Wtulpuart2R {
        Wtulpuart2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode UART3"]
    #[inline(always)]
    pub fn wtulpuart3(&self) -> Wtulpuart3R {
        Wtulpuart3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for ULP mode MCU"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpmcu(&mut self) -> WtulpmcuW<Pwrweightulp0Spec> {
        WtulpmcuW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for ULP mode DSP0"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpdsp0(&mut self) -> Wtulpdsp0W<Pwrweightulp0Spec> {
        Wtulpdsp0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for ULP mode DSP1"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpdsp1(&mut self) -> Wtulpdsp1W<Pwrweightulp0Spec> {
        Wtulpdsp1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for ULP mode IOS"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpios(&mut self) -> WtulpiosW<Pwrweightulp0Spec> {
        WtulpiosW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for ULP mode UART0"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpuart0(&mut self) -> Wtulpuart0W<Pwrweightulp0Spec> {
        Wtulpuart0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for ULP mode UART1"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpuart1(&mut self) -> Wtulpuart1W<Pwrweightulp0Spec> {
        Wtulpuart1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for ULP mode UART2"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpuart2(&mut self) -> Wtulpuart2W<Pwrweightulp0Spec> {
        Wtulpuart2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for ULP mode UART3"]
    #[inline(always)]
    #[must_use]
    pub fn wtulpuart3(&mut self) -> Wtulpuart3W<Pwrweightulp0Spec> {
        Wtulpuart3W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightulp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightulp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightulp0Spec;
impl crate::RegisterSpec for Pwrweightulp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightulp0::R`](R) reader structure"]
impl crate::Readable for Pwrweightulp0Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightulp0::W`](W) writer structure"]
impl crate::Writable for Pwrweightulp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTULP0 to value 0"]
impl crate::Resettable for Pwrweightulp0Spec {
    const RESET_VALUE: u32 = 0;
}
