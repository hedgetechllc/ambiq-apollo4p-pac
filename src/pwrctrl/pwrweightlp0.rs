#[doc = "Register `PWRWEIGHTLP0` reader"]
pub type R = crate::R<Pwrweightlp0Spec>;
#[doc = "Register `PWRWEIGHTLP0` writer"]
pub type W = crate::W<Pwrweightlp0Spec>;
#[doc = "Field `WTLPMCU` reader - Weight used for LP mode MCU"]
pub type WtlpmcuR = crate::FieldReader;
#[doc = "Field `WTLPMCU` writer - Weight used for LP mode MCU"]
pub type WtlpmcuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPDSP0` reader - Weight used for LP mode DSP0"]
pub type Wtlpdsp0R = crate::FieldReader;
#[doc = "Field `WTLPDSP0` writer - Weight used for LP mode DSP0"]
pub type Wtlpdsp0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPDSP1` reader - Weight used for LP mode DSP1"]
pub type Wtlpdsp1R = crate::FieldReader;
#[doc = "Field `WTLPDSP1` writer - Weight used for LP mode DSP1"]
pub type Wtlpdsp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPIOS` reader - Weight used for LP mode IOS"]
pub type WtlpiosR = crate::FieldReader;
#[doc = "Field `WTLPIOS` writer - Weight used for LP mode IOS"]
pub type WtlpiosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPUART0` reader - Weight used for LP mode UART0"]
pub type Wtlpuart0R = crate::FieldReader;
#[doc = "Field `WTLPUART0` writer - Weight used for LP mode UART0"]
pub type Wtlpuart0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPUART1` reader - Weight used for LP mode UART1"]
pub type Wtlpuart1R = crate::FieldReader;
#[doc = "Field `WTLPUART1` writer - Weight used for LP mode UART1"]
pub type Wtlpuart1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPUART2` reader - Weight used for LP mode UART2"]
pub type Wtlpuart2R = crate::FieldReader;
#[doc = "Field `WTLPUART2` writer - Weight used for LP mode UART2"]
pub type Wtlpuart2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTLPUART3` reader - Weight used for LP mode UART3"]
pub type Wtlpuart3R = crate::FieldReader;
#[doc = "Field `WTLPUART3` writer - Weight used for LP mode UART3"]
pub type Wtlpuart3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for LP mode MCU"]
    #[inline(always)]
    pub fn wtlpmcu(&self) -> WtlpmcuR {
        WtlpmcuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode DSP0"]
    #[inline(always)]
    pub fn wtlpdsp0(&self) -> Wtlpdsp0R {
        Wtlpdsp0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for LP mode DSP1"]
    #[inline(always)]
    pub fn wtlpdsp1(&self) -> Wtlpdsp1R {
        Wtlpdsp1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for LP mode IOS"]
    #[inline(always)]
    pub fn wtlpios(&self) -> WtlpiosR {
        WtlpiosR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode UART0"]
    #[inline(always)]
    pub fn wtlpuart0(&self) -> Wtlpuart0R {
        Wtlpuart0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for LP mode UART1"]
    #[inline(always)]
    pub fn wtlpuart1(&self) -> Wtlpuart1R {
        Wtlpuart1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for LP mode UART2"]
    #[inline(always)]
    pub fn wtlpuart2(&self) -> Wtlpuart2R {
        Wtlpuart2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode UART3"]
    #[inline(always)]
    pub fn wtlpuart3(&self) -> Wtlpuart3R {
        Wtlpuart3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for LP mode MCU"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpmcu(&mut self) -> WtlpmcuW<Pwrweightlp0Spec> {
        WtlpmcuW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for LP mode DSP0"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpdsp0(&mut self) -> Wtlpdsp0W<Pwrweightlp0Spec> {
        Wtlpdsp0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for LP mode DSP1"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpdsp1(&mut self) -> Wtlpdsp1W<Pwrweightlp0Spec> {
        Wtlpdsp1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for LP mode IOS"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpios(&mut self) -> WtlpiosW<Pwrweightlp0Spec> {
        WtlpiosW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for LP mode UART0"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpuart0(&mut self) -> Wtlpuart0W<Pwrweightlp0Spec> {
        Wtlpuart0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for LP mode UART1"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpuart1(&mut self) -> Wtlpuart1W<Pwrweightlp0Spec> {
        Wtlpuart1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for LP mode UART2"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpuart2(&mut self) -> Wtlpuart2W<Pwrweightlp0Spec> {
        Wtlpuart2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for LP mode UART3"]
    #[inline(always)]
    #[must_use]
    pub fn wtlpuart3(&mut self) -> Wtlpuart3W<Pwrweightlp0Spec> {
        Wtlpuart3W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweightlp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweightlp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweightlp0Spec;
impl crate::RegisterSpec for Pwrweightlp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweightlp0::R`](R) reader structure"]
impl crate::Readable for Pwrweightlp0Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweightlp0::W`](W) writer structure"]
impl crate::Writable for Pwrweightlp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTLP0 to value 0"]
impl crate::Resettable for Pwrweightlp0Spec {
    const RESET_VALUE: u32 = 0;
}
