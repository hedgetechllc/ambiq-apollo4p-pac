#[doc = "Register `LDOREG2` reader"]
pub type R = crate::R<Ldoreg2Spec>;
#[doc = "Register `LDOREG2` writer"]
pub type W = crate::W<Ldoreg2Spec>;
#[doc = "Field `MEMLDOACTIVETRIM` reader - MEM LDO active trim"]
pub type MemldoactivetrimR = crate::FieldReader;
#[doc = "Field `MEMLDOACTIVETRIM` writer - MEM LDO active trim"]
pub type MemldoactivetrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MEMLDOLPTRIM` reader - MEM LDO LP trim"]
pub type MemldolptrimR = crate::FieldReader;
#[doc = "Field `MEMLDOLPTRIM` writer - MEM LDO LP trim"]
pub type MemldolptrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MEMLDOLPALTTRIM` reader - MEM LDO TRIM LP ALT SET"]
pub type MemldolpalttrimR = crate::FieldReader;
#[doc = "Field `MEMLDOLPALTTRIM` writer - MEM LDO TRIM LP ALT SET"]
pub type MemldolpalttrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MEMLPLDOTRIM` reader - MEM LPLDO TRIM"]
pub type MemlpldotrimR = crate::FieldReader;
#[doc = "Field `MEMLPLDOTRIM` writer - MEM LPLDO TRIM"]
pub type MemlpldotrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MEMLPLDOIBIASTRIM` reader - Mem LPLDO IBIAS trim"]
pub type MemlpldoibiastrimR = crate::BitReader;
#[doc = "Field `MEMLPLDOIBIASTRIM` writer - Mem LPLDO IBIAS trim"]
pub type MemlpldoibiastrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMLDOIBIASSEL` reader - Mem LDO IBIAS sel. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub type MemldoibiasselR = crate::BitReader;
#[doc = "Field `MEMLDOIBIASSEL` writer - Mem LDO IBIAS sel. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
pub type MemldoibiasselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIMANALDO` reader - Analog LDO Trim."]
pub type TrimanaldoR = crate::FieldReader;
#[doc = "Field `TRIMANALDO` writer - Analog LDO Trim."]
pub type TrimanaldoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - MEM LDO active trim"]
    #[inline(always)]
    pub fn memldoactivetrim(&self) -> MemldoactivetrimR {
        MemldoactivetrimR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - MEM LDO LP trim"]
    #[inline(always)]
    pub fn memldolptrim(&self) -> MemldolptrimR {
        MemldolptrimR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - MEM LDO TRIM LP ALT SET"]
    #[inline(always)]
    pub fn memldolpalttrim(&self) -> MemldolpalttrimR {
        MemldolpalttrimR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - MEM LPLDO TRIM"]
    #[inline(always)]
    pub fn memlpldotrim(&self) -> MemlpldotrimR {
        MemlpldotrimR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Mem LPLDO IBIAS trim"]
    #[inline(always)]
    pub fn memlpldoibiastrim(&self) -> MemlpldoibiastrimR {
        MemlpldoibiastrimR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mem LDO IBIAS sel. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    pub fn memldoibiassel(&self) -> MemldoibiasselR {
        MemldoibiasselR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - Analog LDO Trim."]
    #[inline(always)]
    pub fn trimanaldo(&self) -> TrimanaldoR {
        TrimanaldoR::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - MEM LDO active trim"]
    #[inline(always)]
    #[must_use]
    pub fn memldoactivetrim(&mut self) -> MemldoactivetrimW<Ldoreg2Spec> {
        MemldoactivetrimW::new(self, 0)
    }
    #[doc = "Bits 6:11 - MEM LDO LP trim"]
    #[inline(always)]
    #[must_use]
    pub fn memldolptrim(&mut self) -> MemldolptrimW<Ldoreg2Spec> {
        MemldolptrimW::new(self, 6)
    }
    #[doc = "Bits 12:17 - MEM LDO TRIM LP ALT SET"]
    #[inline(always)]
    #[must_use]
    pub fn memldolpalttrim(&mut self) -> MemldolpalttrimW<Ldoreg2Spec> {
        MemldolpalttrimW::new(self, 12)
    }
    #[doc = "Bits 18:23 - MEM LPLDO TRIM"]
    #[inline(always)]
    #[must_use]
    pub fn memlpldotrim(&mut self) -> MemlpldotrimW<Ldoreg2Spec> {
        MemlpldotrimW::new(self, 18)
    }
    #[doc = "Bit 24 - Mem LPLDO IBIAS trim"]
    #[inline(always)]
    #[must_use]
    pub fn memlpldoibiastrim(&mut self) -> MemlpldoibiastrimW<Ldoreg2Spec> {
        MemlpldoibiastrimW::new(self, 24)
    }
    #[doc = "Bit 25 - Mem LDO IBIAS sel. Note: the SWE mux select in PWRSEQ2SWE must be set for this to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn memldoibiassel(&mut self) -> MemldoibiasselW<Ldoreg2Spec> {
        MemldoibiasselW::new(self, 25)
    }
    #[doc = "Bits 26:29 - Analog LDO Trim."]
    #[inline(always)]
    #[must_use]
    pub fn trimanaldo(&mut self) -> TrimanaldoW<Ldoreg2Spec> {
        TrimanaldoW::new(self, 26)
    }
}
#[doc = "MEMLDO and MEMLPLDO Trims\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoreg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoreg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldoreg2Spec;
impl crate::RegisterSpec for Ldoreg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldoreg2::R`](R) reader structure"]
impl crate::Readable for Ldoreg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ldoreg2::W`](W) writer structure"]
impl crate::Writable for Ldoreg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDOREG2 to value 0x207f_cf3a"]
impl crate::Resettable for Ldoreg2Spec {
    const RESET_VALUE: u32 = 0x207f_cf3a;
}
