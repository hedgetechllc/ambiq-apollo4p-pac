#[doc = "Register `PKAWORDACCESS` reader"]
pub type R = crate::R<PkawordaccessSpec>;
#[doc = "Register `PKAWORDACCESS` writer"]
pub type W = crate::W<PkawordaccessSpec>;
#[doc = "Field `PKAWORDACCESS` reader - 32 bit read_write data."]
pub type PkawordaccessR = crate::FieldReader<u32>;
#[doc = "Field `PKAWORDACCESS` writer - 32 bit read_write data."]
pub type PkawordaccessW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit read_write data."]
    #[inline(always)]
    pub fn pkawordaccess(&self) -> PkawordaccessR {
        PkawordaccessR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bit read_write data."]
    #[inline(always)]
    #[must_use]
    pub fn pkawordaccess(&mut self) -> PkawordaccessW<PkawordaccessSpec> {
        PkawordaccessW::new(self, 0)
    }
}
#[doc = "This register holds the data written to PKA memory using the wop opcode.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkawordaccess::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkawordaccess::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkawordaccessSpec;
impl crate::RegisterSpec for PkawordaccessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkawordaccess::R`](R) reader structure"]
impl crate::Readable for PkawordaccessSpec {}
#[doc = "`write(|w| ..)` method takes [`pkawordaccess::W`](W) writer structure"]
impl crate::Writable for PkawordaccessSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKAWORDACCESS to value 0"]
impl crate::Resettable for PkawordaccessSpec {
    const RESET_VALUE: u32 = 0;
}
