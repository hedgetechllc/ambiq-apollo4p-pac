#[doc = "Register `LDOREG1` reader"]
pub type R = crate::R<Ldoreg1Spec>;
#[doc = "Register `LDOREG1` writer"]
pub type W = crate::W<Ldoreg1Spec>;
#[doc = "Field `CORELDOACTIVETRIM` reader - CORE LDO active trim"]
pub type CoreldoactivetrimR = crate::FieldReader<u16>;
#[doc = "Field `CORELDOACTIVETRIM` writer - CORE LDO active trim"]
pub type CoreldoactivetrimW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CORELDOTEMPCOTRIM` reader - CORE LDO TEMPCO trim"]
pub type CoreldotempcotrimR = crate::FieldReader;
#[doc = "Field `CORELDOTEMPCOTRIM` writer - CORE LDO TEMPCO trim"]
pub type CoreldotempcotrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CORELDOLPTRIM` reader - CORE LDO Low Power Trim"]
pub type CoreldolptrimR = crate::FieldReader;
#[doc = "Field `CORELDOLPTRIM` writer - CORE LDO Low Power Trim"]
pub type CoreldolptrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CORELDOIBIASTRIM` reader - CORE LDO IBIAS Trim"]
pub type CoreldoibiastrimR = crate::BitReader;
#[doc = "Field `CORELDOIBIASTRIM` writer - CORE LDO IBIAS Trim"]
pub type CoreldoibiastrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORELDOIBIASSEL` reader - Core LDO IBIAS sel. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub type CoreldoibiasselR = crate::BitReader;
#[doc = "Field `CORELDOIBIASSEL` writer - Core LDO IBIAS sel. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub type CoreldoibiasselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - CORE LDO active trim"]
    #[inline(always)]
    pub fn coreldoactivetrim(&self) -> CoreldoactivetrimR {
        CoreldoactivetrimR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:13 - CORE LDO TEMPCO trim"]
    #[inline(always)]
    pub fn coreldotempcotrim(&self) -> CoreldotempcotrimR {
        CoreldotempcotrimR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:19 - CORE LDO Low Power Trim"]
    #[inline(always)]
    pub fn coreldolptrim(&self) -> CoreldolptrimR {
        CoreldolptrimR::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - CORE LDO IBIAS Trim"]
    #[inline(always)]
    pub fn coreldoibiastrim(&self) -> CoreldoibiastrimR {
        CoreldoibiastrimR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Core LDO IBIAS sel. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn coreldoibiassel(&self) -> CoreldoibiasselR {
        CoreldoibiasselR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - CORE LDO active trim"]
    #[inline(always)]
    #[must_use]
    pub fn coreldoactivetrim(&mut self) -> CoreldoactivetrimW<Ldoreg1Spec> {
        CoreldoactivetrimW::new(self, 0)
    }
    #[doc = "Bits 10:13 - CORE LDO TEMPCO trim"]
    #[inline(always)]
    #[must_use]
    pub fn coreldotempcotrim(&mut self) -> CoreldotempcotrimW<Ldoreg1Spec> {
        CoreldotempcotrimW::new(self, 10)
    }
    #[doc = "Bits 14:19 - CORE LDO Low Power Trim"]
    #[inline(always)]
    #[must_use]
    pub fn coreldolptrim(&mut self) -> CoreldolptrimW<Ldoreg1Spec> {
        CoreldolptrimW::new(self, 14)
    }
    #[doc = "Bit 20 - CORE LDO IBIAS Trim"]
    #[inline(always)]
    #[must_use]
    pub fn coreldoibiastrim(&mut self) -> CoreldoibiastrimW<Ldoreg1Spec> {
        CoreldoibiastrimW::new(self, 20)
    }
    #[doc = "Bit 21 - Core LDO IBIAS sel. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn coreldoibiassel(&mut self) -> CoreldoibiasselW<Ldoreg1Spec> {
        CoreldoibiasselW::new(self, 21)
    }
}
#[doc = "CORELDO trims Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoreg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoreg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldoreg1Spec;
impl crate::RegisterSpec for Ldoreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldoreg1::R`](R) reader structure"]
impl crate::Readable for Ldoreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ldoreg1::W`](W) writer structure"]
impl crate::Writable for Ldoreg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDOREG1 to value 0x0006_d3a3"]
impl crate::Resettable for Ldoreg1Spec {
    const RESET_VALUE: u32 = 0x0006_d3a3;
}
