#[doc = "Register `NNPT0T1ADDR` reader"]
pub type R = crate::R<Nnpt0t1addrSpec>;
#[doc = "Register `NNPT0T1ADDR` writer"]
pub type W = crate::W<Nnpt0t1addrSpec>;
#[doc = "Field `NVIRTUALADDR` reader - Virtual address of register N."]
pub type NvirtualaddrR = crate::FieldReader;
#[doc = "Field `NVIRTUALADDR` writer - Virtual address of register N."]
pub type NvirtualaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NPVIRTUALADDR` reader - Virtual address of register NP."]
pub type NpvirtualaddrR = crate::FieldReader;
#[doc = "Field `NPVIRTUALADDR` writer - Virtual address of register NP."]
pub type NpvirtualaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T0VIRTUALADDR` reader - Virtual address of temporary register number 0"]
pub type T0virtualaddrR = crate::FieldReader;
#[doc = "Field `T0VIRTUALADDR` writer - Virtual address of temporary register number 0"]
pub type T0virtualaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T1VIRTUALADDR` reader - Virtual address of temporary register number 1"]
pub type T1virtualaddrR = crate::FieldReader;
#[doc = "Field `T1VIRTUALADDR` writer - Virtual address of temporary register number 1"]
pub type T1virtualaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Virtual address of register N."]
    #[inline(always)]
    pub fn nvirtualaddr(&self) -> NvirtualaddrR {
        NvirtualaddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Virtual address of register NP."]
    #[inline(always)]
    pub fn npvirtualaddr(&self) -> NpvirtualaddrR {
        NpvirtualaddrR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Virtual address of temporary register number 0"]
    #[inline(always)]
    pub fn t0virtualaddr(&self) -> T0virtualaddrR {
        T0virtualaddrR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Virtual address of temporary register number 1"]
    #[inline(always)]
    pub fn t1virtualaddr(&self) -> T1virtualaddrR {
        T1virtualaddrR::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Virtual address of register N."]
    #[inline(always)]
    #[must_use]
    pub fn nvirtualaddr(&mut self) -> NvirtualaddrW<Nnpt0t1addrSpec> {
        NvirtualaddrW::new(self, 0)
    }
    #[doc = "Bits 5:9 - Virtual address of register NP."]
    #[inline(always)]
    #[must_use]
    pub fn npvirtualaddr(&mut self) -> NpvirtualaddrW<Nnpt0t1addrSpec> {
        NpvirtualaddrW::new(self, 5)
    }
    #[doc = "Bits 10:14 - Virtual address of temporary register number 0"]
    #[inline(always)]
    #[must_use]
    pub fn t0virtualaddr(&mut self) -> T0virtualaddrW<Nnpt0t1addrSpec> {
        T0virtualaddrW::new(self, 10)
    }
    #[doc = "Bits 15:19 - Virtual address of temporary register number 1"]
    #[inline(always)]
    #[must_use]
    pub fn t1virtualaddr(&mut self) -> T1virtualaddrW<Nnpt0t1addrSpec> {
        T1virtualaddrW::new(self, 15)
    }
}
#[doc = "This register maps N_NP_T0_T1 to a virtual address.\n\nYou can [`read`](crate::Reg::read) this register and get [`nnpt0t1addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nnpt0t1addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nnpt0t1addrSpec;
impl crate::RegisterSpec for Nnpt0t1addrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nnpt0t1addr::R`](R) reader structure"]
impl crate::Readable for Nnpt0t1addrSpec {}
#[doc = "`write(|w| ..)` method takes [`nnpt0t1addr::W`](W) writer structure"]
impl crate::Writable for Nnpt0t1addrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NNPT0T1ADDR to value 0x000f_f820"]
impl crate::Resettable for Nnpt0t1addrSpec {
    const RESET_VALUE: u32 = 0x000f_f820;
}
