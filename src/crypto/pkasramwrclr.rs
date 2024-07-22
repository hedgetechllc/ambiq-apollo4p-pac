#[doc = "Register `PKASRAMWRCLR` reader"]
pub type R = crate::R<PkasramwrclrSpec>;
#[doc = "Register `PKASRAMWRCLR` writer"]
pub type W = crate::W<PkasramwrclrSpec>;
#[doc = "Field `PKASRAMWRCLR` reader - Clear the write buffer."]
pub type PkasramwrclrR = crate::FieldReader<u32>;
#[doc = "Field `PKASRAMWRCLR` writer - Clear the write buffer."]
pub type PkasramwrclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Clear the write buffer."]
    #[inline(always)]
    pub fn pkasramwrclr(&self) -> PkasramwrclrR {
        PkasramwrclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the write buffer."]
    #[inline(always)]
    #[must_use]
    pub fn pkasramwrclr(&mut self) -> PkasramwrclrW<PkasramwrclrSpec> {
        PkasramwrclrW::new(self, 0)
    }
}
#[doc = "Write buffer clean.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramwrclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramwrclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkasramwrclrSpec;
impl crate::RegisterSpec for PkasramwrclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkasramwrclr::R`](R) reader structure"]
impl crate::Readable for PkasramwrclrSpec {}
#[doc = "`write(|w| ..)` method takes [`pkasramwrclr::W`](W) writer structure"]
impl crate::Writable for PkasramwrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKASRAMWRCLR to value 0"]
impl crate::Resettable for PkasramwrclrSpec {
    const RESET_VALUE: u32 = 0;
}
