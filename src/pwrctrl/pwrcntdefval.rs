#[doc = "Register `PWRCNTDEFVAL` reader"]
pub type R = crate::R<PwrcntdefvalSpec>;
#[doc = "Register `PWRCNTDEFVAL` writer"]
pub type W = crate::W<PwrcntdefvalSpec>;
#[doc = "Field `PWRDEFVALDEVSTMC` reader - Default count max value for dev ST MC"]
pub type PwrdefvaldevstmcR = crate::FieldReader;
#[doc = "Field `PWRDEFVALDEVSTMC` writer - Default count max value for dev ST MC"]
pub type PwrdefvaldevstmcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PWRACKWAITDELSIMOSTMC` reader - Default counter max for buck ST MC"]
pub type PwrackwaitdelsimostmcR = crate::FieldReader<u16>;
#[doc = "Field `PWRACKWAITDELSIMOSTMC` writer - Default counter max for buck ST MC"]
pub type PwrackwaitdelsimostmcW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - Default count max value for dev ST MC"]
    #[inline(always)]
    pub fn pwrdefvaldevstmc(&self) -> PwrdefvaldevstmcR {
        PwrdefvaldevstmcR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:15 - Default counter max for buck ST MC"]
    #[inline(always)]
    pub fn pwrackwaitdelsimostmc(&self) -> PwrackwaitdelsimostmcR {
        PwrackwaitdelsimostmcR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Default count max value for dev ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdefvaldevstmc(&mut self) -> PwrdefvaldevstmcW<PwrcntdefvalSpec> {
        PwrdefvaldevstmcW::new(self, 0)
    }
    #[doc = "Bits 6:15 - Default counter max for buck ST MC"]
    #[inline(always)]
    #[must_use]
    pub fn pwrackwaitdelsimostmc(&mut self) -> PwrackwaitdelsimostmcW<PwrcntdefvalSpec> {
        PwrackwaitdelsimostmcW::new(self, 6)
    }
}
#[doc = "This register contains programmble dealy values for various state michines. Fields contain dev st machine default value, SIMOBUCK state machine wait delay counter etc.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcntdefval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcntdefval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcntdefvalSpec;
impl crate::RegisterSpec for PwrcntdefvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcntdefval::R`](R) reader structure"]
impl crate::Readable for PwrcntdefvalSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcntdefval::W`](W) writer structure"]
impl crate::Writable for PwrcntdefvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCNTDEFVAL to value 0x0208"]
impl crate::Resettable for PwrcntdefvalSpec {
    const RESET_VALUE: u32 = 0x0208;
}
