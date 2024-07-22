#[doc = "Register `DBIRDAT` reader"]
pub type R = crate::R<DbirdatSpec>;
#[doc = "Register `DBIRDAT` writer"]
pub type W = crate::W<DbirdatSpec>;
#[doc = "Field `READTYPEB` reader - Read data from DBI Type-B interface"]
pub type ReadtypebR = crate::FieldReader<u32>;
#[doc = "Field `READTYPEB` writer - Read data from DBI Type-B interface"]
pub type ReadtypebW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read data from DBI Type-B interface"]
    #[inline(always)]
    pub fn readtypeb(&self) -> ReadtypebR {
        ReadtypebR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read data from DBI Type-B interface"]
    #[inline(always)]
    #[must_use]
    pub fn readtypeb(&mut self) -> ReadtypebW<DbirdatSpec> {
        ReadtypebW::new(self, 0)
    }
}
#[doc = "Data read by DBI Type-B interface are stored in the DBI_RDAT register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbirdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbirdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbirdatSpec;
impl crate::RegisterSpec for DbirdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbirdat::R`](R) reader structure"]
impl crate::Readable for DbirdatSpec {}
#[doc = "`write(|w| ..)` method takes [`dbirdat::W`](W) writer structure"]
impl crate::Writable for DbirdatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBIRDAT to value 0"]
impl crate::Resettable for DbirdatSpec {
    const RESET_VALUE: u32 = 0;
}
