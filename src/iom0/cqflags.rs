#[doc = "Register `CQFLAGS` reader"]
pub type R = crate::R<CqflagsSpec>;
#[doc = "Register `CQFLAGS` writer"]
pub type W = crate::W<CqflagsSpec>;
#[doc = "Field `CQFLAGS` reader - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
pub type CqflagsR = crate::FieldReader<u16>;
#[doc = "Field `CQFLAGS` writer - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
pub type CqflagsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CQIRQMASK` reader - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
pub type CqirqmaskR = crate::FieldReader<u16>;
#[doc = "Field `CQIRQMASK` writer - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
pub type CqirqmaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    pub fn cqflags(&self) -> CqflagsR {
        CqflagsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
    #[inline(always)]
    pub fn cqirqmask(&self) -> CqirqmaskR {
        CqirqmaskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current flag status (read-only). Bits \\[7:0\\]
are software controllable and bits \\[15:8\\]
are hardware status."]
    #[inline(always)]
    #[must_use]
    pub fn cqflags(&mut self) -> CqflagsW<CqflagsSpec> {
        CqflagsW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn cqirqmask(&mut self) -> CqirqmaskW<CqflagsSpec> {
        CqirqmaskW::new(self, 16)
    }
}
#[doc = "Command Queue Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`cqflags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqflags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqflagsSpec;
impl crate::RegisterSpec for CqflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqflags::R`](R) reader structure"]
impl crate::Readable for CqflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`cqflags::W`](W) writer structure"]
impl crate::Writable for CqflagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQFLAGS to value 0"]
impl crate::Resettable for CqflagsSpec {
    const RESET_VALUE: u32 = 0;
}
