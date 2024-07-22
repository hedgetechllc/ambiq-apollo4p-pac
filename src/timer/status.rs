#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `ACTIVE` reader - Indicates which timers are currnetly active (enabled)"]
pub type ActiveR = crate::FieldReader<u16>;
#[doc = "Field `ACTIVE` writer - Indicates which timers are currnetly active (enabled)"]
pub type ActiveW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NTIMERS` reader - Indicates the number of timer blocks present in the design"]
pub type NtimersR = crate::FieldReader;
#[doc = "Field `NTIMERS` writer - Indicates the number of timer blocks present in the design"]
pub type NtimersW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:15 - Indicates which timers are currnetly active (enabled)"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Indicates the number of timer blocks present in the design"]
    #[inline(always)]
    pub fn ntimers(&self) -> NtimersR {
        NtimersR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates which timers are currnetly active (enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<StatusSpec> {
        ActiveW::new(self, 0)
    }
    #[doc = "Bits 16:20 - Indicates the number of timer blocks present in the design"]
    #[inline(always)]
    #[must_use]
    pub fn ntimers(&mut self) -> NtimersW<StatusSpec> {
        NtimersW::new(self, 16)
    }
}
#[doc = "General Timer status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x0010_0000"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
