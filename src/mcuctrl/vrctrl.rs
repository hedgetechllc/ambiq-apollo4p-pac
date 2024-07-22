#[doc = "Register `VRCTRL` reader"]
pub type R = crate::R<VrctrlSpec>;
#[doc = "Register `VRCTRL` writer"]
pub type W = crate::W<VrctrlSpec>;
#[doc = "Field `CORELDOOVER` reader - Override control for CORE LDO signals"]
pub type CoreldooverR = crate::BitReader;
#[doc = "Field `CORELDOOVER` writer - Override control for CORE LDO signals"]
pub type CoreldooverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORELDOPDNB` reader - CORE LDO PDNB control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
pub type CoreldopdnbR = crate::BitReader;
#[doc = "Field `CORELDOPDNB` writer - CORE LDO PDNB control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
pub type CoreldopdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORELDOACTIVEEARLY` reader - CORE LDO EARLY ACTIVE control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
pub type CoreldoactiveearlyR = crate::BitReader;
#[doc = "Field `CORELDOACTIVEEARLY` writer - CORE LDO EARLY ACTIVE control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
pub type CoreldoactiveearlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORELDOACTIVE` reader - CORE LDO ACTIVE control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
pub type CoreldoactiveR = crate::BitReader;
#[doc = "Field `CORELDOACTIVE` writer - CORE LDO ACTIVE control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
pub type CoreldoactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORELDOCOLDSTARTEN` reader - CORE LDO COLDSTART EN control. This is a shadow backed register and no need to set CORELDOOVER."]
pub type CoreldocoldstartenR = crate::BitReader;
#[doc = "Field `CORELDOCOLDSTARTEN` writer - CORE LDO COLDSTART EN control. This is a shadow backed register and no need to set CORELDOOVER."]
pub type CoreldocoldstartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLDOOVER` reader - Override control for MEM LDO signals"]
pub type MemldooverR = crate::BitReader;
#[doc = "Field `MEMLDOOVER` writer - Override control for MEM LDO signals"]
pub type MemldooverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLDOPDNB` reader - MEM LDO PDNB control. Override signal for PWRCTRL going to analog when MEMLDOOVER = 1"]
pub type MemldopdnbR = crate::BitReader;
#[doc = "Field `MEMLDOPDNB` writer - MEM LDO PDNB control. Override signal for PWRCTRL going to analog when MEMLDOOVER = 1"]
pub type MemldopdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLDOACTIVEEARLY` reader - MEM LDO EARLY ACTIVE control. Override for PWRCTRL going to analog when MEMLDOOVER = 1"]
pub type MemldoactiveearlyR = crate::BitReader;
#[doc = "Field `MEMLDOACTIVEEARLY` writer - MEM LDO EARLY ACTIVE control. Override for PWRCTRL going to analog when MEMLDOOVER = 1"]
pub type MemldoactiveearlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLDOACTIVE` reader - MEM LDO ACTIVE control. Override for PWRCTRL going to analog when MEMLDOOVER = 1"]
pub type MemldoactiveR = crate::BitReader;
#[doc = "Field `MEMLDOACTIVE` writer - MEM LDO ACTIVE control. Override for PWRCTRL going to analog when MEMLDOOVER = 1"]
pub type MemldoactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLDOCOLDSTARTEN` reader - MEM LDO COLDSTART EN control. This is a shadow backed register and no need to set MEMLDOOVER."]
pub type MemldocoldstartenR = crate::BitReader;
#[doc = "Field `MEMLDOCOLDSTARTEN` writer - MEM LDO COLDSTART EN control. This is a shadow backed register and no need to set MEMLDOOVER."]
pub type MemldocoldstartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLPLDOOVER` reader - Override control for MEM LP LDO signals"]
pub type MemlpldooverR = crate::BitReader;
#[doc = "Field `MEMLPLDOOVER` writer - Override control for MEM LP LDO signals"]
pub type MemlpldooverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLPLDOPDNB` reader - MEM LP LDO PDNB control. Override for PWRCTRL going to analog when MEMLPLDOOVER = 1"]
pub type MemlpldopdnbR = crate::BitReader;
#[doc = "Field `MEMLPLDOPDNB` writer - MEM LP LDO PDNB control. Override for PWRCTRL going to analog when MEMLPLDOOVER = 1"]
pub type MemlpldopdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLPLDOACTIVE` reader - MEM LP LDO ACTVIVE control. Override for PWRCTRL going to analog when MEMLPLDOOVER = 1"]
pub type MemlpldoactiveR = crate::BitReader;
#[doc = "Field `MEMLPLDOACTIVE` writer - MEM LP LDO ACTVIVE control. Override for PWRCTRL going to analog when MEMLPLDOOVER = 1"]
pub type MemlpldoactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANALDOOVER` reader - Override control for ANALDO signals"]
pub type AnaldooverR = crate::BitReader;
#[doc = "Field `ANALDOOVER` writer - Override control for ANALDO signals"]
pub type AnaldooverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANALDOPDNB` reader - ANALDO PDNB control. Override for PWRCTRL going to analog when ANALDOOVER = 1"]
pub type AnaldopdnbR = crate::BitReader;
#[doc = "Field `ANALDOPDNB` writer - ANALDO PDNB control. Override for PWRCTRL going to analog when ANALDOOVER = 1"]
pub type AnaldopdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANALDOACTIVE` reader - ANALDO LDO ACTIVE control. Override for PWRCTRL going to analog when ANALDOOVER = 1"]
pub type AnaldoactiveR = crate::BitReader;
#[doc = "Field `ANALDOACTIVE` writer - ANALDO LDO ACTIVE control. Override for PWRCTRL going to analog when ANALDOOVER = 1"]
pub type AnaldoactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOBUCKOVER` reader - Override control for SIMO BUCK signals"]
pub type SimobuckoverR = crate::BitReader;
#[doc = "Field `SIMOBUCKOVER` writer - Override control for SIMO BUCK signals"]
pub type SimobuckoverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOBUCKPDNB` reader - SIMO BUCK PDNB control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
pub type SimobuckpdnbR = crate::BitReader;
#[doc = "Field `SIMOBUCKPDNB` writer - SIMO BUCK PDNB control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
pub type SimobuckpdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOBUCKRSTB` reader - SIMO BUCK RSTB control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
pub type SimobuckrstbR = crate::BitReader;
#[doc = "Field `SIMOBUCKRSTB` writer - SIMO BUCK RSTB control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
pub type SimobuckrstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOBUCKACTIVE` reader - SIMO BUCK ACTIVE control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
pub type SimobuckactiveR = crate::BitReader;
#[doc = "Field `SIMOBUCKACTIVE` writer - SIMO BUCK ACTIVE control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
pub type SimobuckactiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Override control for CORE LDO signals"]
    #[inline(always)]
    pub fn coreldoover(&self) -> CoreldooverR {
        CoreldooverR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CORE LDO PDNB control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
    #[inline(always)]
    pub fn coreldopdnb(&self) -> CoreldopdnbR {
        CoreldopdnbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CORE LDO EARLY ACTIVE control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
    #[inline(always)]
    pub fn coreldoactiveearly(&self) -> CoreldoactiveearlyR {
        CoreldoactiveearlyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CORE LDO ACTIVE control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
    #[inline(always)]
    pub fn coreldoactive(&self) -> CoreldoactiveR {
        CoreldoactiveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CORE LDO COLDSTART EN control. This is a shadow backed register and no need to set CORELDOOVER."]
    #[inline(always)]
    pub fn coreldocoldstarten(&self) -> CoreldocoldstartenR {
        CoreldocoldstartenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override control for MEM LDO signals"]
    #[inline(always)]
    pub fn memldoover(&self) -> MemldooverR {
        MemldooverR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MEM LDO PDNB control. Override signal for PWRCTRL going to analog when MEMLDOOVER = 1"]
    #[inline(always)]
    pub fn memldopdnb(&self) -> MemldopdnbR {
        MemldopdnbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MEM LDO EARLY ACTIVE control. Override for PWRCTRL going to analog when MEMLDOOVER = 1"]
    #[inline(always)]
    pub fn memldoactiveearly(&self) -> MemldoactiveearlyR {
        MemldoactiveearlyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MEM LDO ACTIVE control. Override for PWRCTRL going to analog when MEMLDOOVER = 1"]
    #[inline(always)]
    pub fn memldoactive(&self) -> MemldoactiveR {
        MemldoactiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MEM LDO COLDSTART EN control. This is a shadow backed register and no need to set MEMLDOOVER."]
    #[inline(always)]
    pub fn memldocoldstarten(&self) -> MemldocoldstartenR {
        MemldocoldstartenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Override control for MEM LP LDO signals"]
    #[inline(always)]
    pub fn memlpldoover(&self) -> MemlpldooverR {
        MemlpldooverR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MEM LP LDO PDNB control. Override for PWRCTRL going to analog when MEMLPLDOOVER = 1"]
    #[inline(always)]
    pub fn memlpldopdnb(&self) -> MemlpldopdnbR {
        MemlpldopdnbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MEM LP LDO ACTVIVE control. Override for PWRCTRL going to analog when MEMLPLDOOVER = 1"]
    #[inline(always)]
    pub fn memlpldoactive(&self) -> MemlpldoactiveR {
        MemlpldoactiveR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Override control for ANALDO signals"]
    #[inline(always)]
    pub fn analdoover(&self) -> AnaldooverR {
        AnaldooverR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ANALDO PDNB control. Override for PWRCTRL going to analog when ANALDOOVER = 1"]
    #[inline(always)]
    pub fn analdopdnb(&self) -> AnaldopdnbR {
        AnaldopdnbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ANALDO LDO ACTIVE control. Override for PWRCTRL going to analog when ANALDOOVER = 1"]
    #[inline(always)]
    pub fn analdoactive(&self) -> AnaldoactiveR {
        AnaldoactiveR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Override control for SIMO BUCK signals"]
    #[inline(always)]
    pub fn simobuckover(&self) -> SimobuckoverR {
        SimobuckoverR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SIMO BUCK PDNB control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
    #[inline(always)]
    pub fn simobuckpdnb(&self) -> SimobuckpdnbR {
        SimobuckpdnbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SIMO BUCK RSTB control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
    #[inline(always)]
    pub fn simobuckrstb(&self) -> SimobuckrstbR {
        SimobuckrstbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SIMO BUCK ACTIVE control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
    #[inline(always)]
    pub fn simobuckactive(&self) -> SimobuckactiveR {
        SimobuckactiveR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override control for CORE LDO signals"]
    #[inline(always)]
    #[must_use]
    pub fn coreldoover(&mut self) -> CoreldooverW<VrctrlSpec> {
        CoreldooverW::new(self, 0)
    }
    #[doc = "Bit 1 - CORE LDO PDNB control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn coreldopdnb(&mut self) -> CoreldopdnbW<VrctrlSpec> {
        CoreldopdnbW::new(self, 1)
    }
    #[doc = "Bit 2 - CORE LDO EARLY ACTIVE control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn coreldoactiveearly(&mut self) -> CoreldoactiveearlyW<VrctrlSpec> {
        CoreldoactiveearlyW::new(self, 2)
    }
    #[doc = "Bit 3 - CORE LDO ACTIVE control. Override for PWRCTRL going to analog when CORELDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn coreldoactive(&mut self) -> CoreldoactiveW<VrctrlSpec> {
        CoreldoactiveW::new(self, 3)
    }
    #[doc = "Bit 4 - CORE LDO COLDSTART EN control. This is a shadow backed register and no need to set CORELDOOVER."]
    #[inline(always)]
    #[must_use]
    pub fn coreldocoldstarten(&mut self) -> CoreldocoldstartenW<VrctrlSpec> {
        CoreldocoldstartenW::new(self, 4)
    }
    #[doc = "Bit 5 - Override control for MEM LDO signals"]
    #[inline(always)]
    #[must_use]
    pub fn memldoover(&mut self) -> MemldooverW<VrctrlSpec> {
        MemldooverW::new(self, 5)
    }
    #[doc = "Bit 6 - MEM LDO PDNB control. Override signal for PWRCTRL going to analog when MEMLDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn memldopdnb(&mut self) -> MemldopdnbW<VrctrlSpec> {
        MemldopdnbW::new(self, 6)
    }
    #[doc = "Bit 7 - MEM LDO EARLY ACTIVE control. Override for PWRCTRL going to analog when MEMLDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn memldoactiveearly(&mut self) -> MemldoactiveearlyW<VrctrlSpec> {
        MemldoactiveearlyW::new(self, 7)
    }
    #[doc = "Bit 8 - MEM LDO ACTIVE control. Override for PWRCTRL going to analog when MEMLDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn memldoactive(&mut self) -> MemldoactiveW<VrctrlSpec> {
        MemldoactiveW::new(self, 8)
    }
    #[doc = "Bit 9 - MEM LDO COLDSTART EN control. This is a shadow backed register and no need to set MEMLDOOVER."]
    #[inline(always)]
    #[must_use]
    pub fn memldocoldstarten(&mut self) -> MemldocoldstartenW<VrctrlSpec> {
        MemldocoldstartenW::new(self, 9)
    }
    #[doc = "Bit 10 - Override control for MEM LP LDO signals"]
    #[inline(always)]
    #[must_use]
    pub fn memlpldoover(&mut self) -> MemlpldooverW<VrctrlSpec> {
        MemlpldooverW::new(self, 10)
    }
    #[doc = "Bit 11 - MEM LP LDO PDNB control. Override for PWRCTRL going to analog when MEMLPLDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn memlpldopdnb(&mut self) -> MemlpldopdnbW<VrctrlSpec> {
        MemlpldopdnbW::new(self, 11)
    }
    #[doc = "Bit 12 - MEM LP LDO ACTVIVE control. Override for PWRCTRL going to analog when MEMLPLDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn memlpldoactive(&mut self) -> MemlpldoactiveW<VrctrlSpec> {
        MemlpldoactiveW::new(self, 12)
    }
    #[doc = "Bit 13 - Override control for ANALDO signals"]
    #[inline(always)]
    #[must_use]
    pub fn analdoover(&mut self) -> AnaldooverW<VrctrlSpec> {
        AnaldooverW::new(self, 13)
    }
    #[doc = "Bit 14 - ANALDO PDNB control. Override for PWRCTRL going to analog when ANALDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn analdopdnb(&mut self) -> AnaldopdnbW<VrctrlSpec> {
        AnaldopdnbW::new(self, 14)
    }
    #[doc = "Bit 15 - ANALDO LDO ACTIVE control. Override for PWRCTRL going to analog when ANALDOOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn analdoactive(&mut self) -> AnaldoactiveW<VrctrlSpec> {
        AnaldoactiveW::new(self, 15)
    }
    #[doc = "Bit 16 - Override control for SIMO BUCK signals"]
    #[inline(always)]
    #[must_use]
    pub fn simobuckover(&mut self) -> SimobuckoverW<VrctrlSpec> {
        SimobuckoverW::new(self, 16)
    }
    #[doc = "Bit 17 - SIMO BUCK PDNB control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn simobuckpdnb(&mut self) -> SimobuckpdnbW<VrctrlSpec> {
        SimobuckpdnbW::new(self, 17)
    }
    #[doc = "Bit 18 - SIMO BUCK RSTB control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn simobuckrstb(&mut self) -> SimobuckrstbW<VrctrlSpec> {
        SimobuckrstbW::new(self, 18)
    }
    #[doc = "Bit 19 - SIMO BUCK ACTIVE control. Override for PWRCTRL going to analog when SIMOBUCKOVER = 1"]
    #[inline(always)]
    #[must_use]
    pub fn simobuckactive(&mut self) -> SimobuckactiveW<VrctrlSpec> {
        SimobuckactiveW::new(self, 19)
    }
}
#[doc = "Overrides for Voltage Regulators Controls\n\nYou can [`read`](crate::Reg::read) this register and get [`vrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrctrlSpec;
impl crate::RegisterSpec for VrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrctrl::R`](R) reader structure"]
impl crate::Readable for VrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vrctrl::W`](W) writer structure"]
impl crate::Writable for VrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VRCTRL to value 0"]
impl crate::Resettable for VrctrlSpec {
    const RESET_VALUE: u32 = 0;
}
