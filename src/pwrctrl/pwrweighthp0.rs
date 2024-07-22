#[doc = "Register `PWRWEIGHTHP0` reader"]
pub type R = crate::R<Pwrweighthp0Spec>;
#[doc = "Register `PWRWEIGHTHP0` writer"]
pub type W = crate::W<Pwrweighthp0Spec>;
#[doc = "Field `WTHPMCU` reader - Weight used for HP mode MCU"]
pub type WthpmcuR = crate::FieldReader;
#[doc = "Field `WTHPMCU` writer - Weight used for HP mode MCU"]
pub type WthpmcuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPDSP0` reader - Weight used for HP mode DSP0"]
pub type Wthpdsp0R = crate::FieldReader;
#[doc = "Field `WTHPDSP0` writer - Weight used for HP mode DSP0"]
pub type Wthpdsp0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPDSP1` reader - Weight used for HP mode DSP1"]
pub type Wthpdsp1R = crate::FieldReader;
#[doc = "Field `WTHPDSP1` writer - Weight used for HP mode DSP1"]
pub type Wthpdsp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPIOS` reader - Weight used for HP mode IOS"]
pub type WthpiosR = crate::FieldReader;
#[doc = "Field `WTHPIOS` writer - Weight used for HP mode IOS"]
pub type WthpiosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPUART0` reader - Weight used for HP mode UART0"]
pub type Wthpuart0R = crate::FieldReader;
#[doc = "Field `WTHPUART0` writer - Weight used for HP mode UART0"]
pub type Wthpuart0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPUART1` reader - Weight used for HP mode UART1"]
pub type Wthpuart1R = crate::FieldReader;
#[doc = "Field `WTHPUART1` writer - Weight used for HP mode UART1"]
pub type Wthpuart1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPUART2` reader - Weight used for HP mode UART2"]
pub type Wthpuart2R = crate::FieldReader;
#[doc = "Field `WTHPUART2` writer - Weight used for HP mode UART2"]
pub type Wthpuart2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WTHPUART3` reader - Weight used for HP mode UART3"]
pub type Wthpuart3R = crate::FieldReader;
#[doc = "Field `WTHPUART3` writer - Weight used for HP mode UART3"]
pub type Wthpuart3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Weight used for HP mode MCU"]
    #[inline(always)]
    pub fn wthpmcu(&self) -> WthpmcuR {
        WthpmcuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode DSP0"]
    #[inline(always)]
    pub fn wthpdsp0(&self) -> Wthpdsp0R {
        Wthpdsp0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Weight used for HP mode DSP1"]
    #[inline(always)]
    pub fn wthpdsp1(&self) -> Wthpdsp1R {
        Wthpdsp1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Weight used for HP mode IOS"]
    #[inline(always)]
    pub fn wthpios(&self) -> WthpiosR {
        WthpiosR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode UART0"]
    #[inline(always)]
    pub fn wthpuart0(&self) -> Wthpuart0R {
        Wthpuart0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Weight used for HP mode UART1"]
    #[inline(always)]
    pub fn wthpuart1(&self) -> Wthpuart1R {
        Wthpuart1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Weight used for HP mode UART2"]
    #[inline(always)]
    pub fn wthpuart2(&self) -> Wthpuart2R {
        Wthpuart2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode UART3"]
    #[inline(always)]
    pub fn wthpuart3(&self) -> Wthpuart3R {
        Wthpuart3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight used for HP mode MCU"]
    #[inline(always)]
    #[must_use]
    pub fn wthpmcu(&mut self) -> WthpmcuW<Pwrweighthp0Spec> {
        WthpmcuW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Weight used for HP mode DSP0"]
    #[inline(always)]
    #[must_use]
    pub fn wthpdsp0(&mut self) -> Wthpdsp0W<Pwrweighthp0Spec> {
        Wthpdsp0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Weight used for HP mode DSP1"]
    #[inline(always)]
    #[must_use]
    pub fn wthpdsp1(&mut self) -> Wthpdsp1W<Pwrweighthp0Spec> {
        Wthpdsp1W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Weight used for HP mode IOS"]
    #[inline(always)]
    #[must_use]
    pub fn wthpios(&mut self) -> WthpiosW<Pwrweighthp0Spec> {
        WthpiosW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Weight used for HP mode UART0"]
    #[inline(always)]
    #[must_use]
    pub fn wthpuart0(&mut self) -> Wthpuart0W<Pwrweighthp0Spec> {
        Wthpuart0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Weight used for HP mode UART1"]
    #[inline(always)]
    #[must_use]
    pub fn wthpuart1(&mut self) -> Wthpuart1W<Pwrweighthp0Spec> {
        Wthpuart1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Weight used for HP mode UART2"]
    #[inline(always)]
    #[must_use]
    pub fn wthpuart2(&mut self) -> Wthpuart2W<Pwrweighthp0Spec> {
        Wthpuart2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Weight used for HP mode UART3"]
    #[inline(always)]
    #[must_use]
    pub fn wthpuart3(&mut self) -> Wthpuart3W<Pwrweighthp0Spec> {
        Wthpuart3W::new(self, 28)
    }
}
#[doc = "Weights specified in this register are applied to each of the masters active requests. The aggregate of all the masters is compared against the allowed value to change the buck from active to inactive mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrweighthp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrweighthp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwrweighthp0Spec;
impl crate::RegisterSpec for Pwrweighthp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrweighthp0::R`](R) reader structure"]
impl crate::Readable for Pwrweighthp0Spec {}
#[doc = "`write(|w| ..)` method takes [`pwrweighthp0::W`](W) writer structure"]
impl crate::Writable for Pwrweighthp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRWEIGHTHP0 to value 0"]
impl crate::Resettable for Pwrweighthp0Spec {
    const RESET_VALUE: u32 = 0;
}
