#[doc = "Register `SRCADDR` reader"]
pub type R = crate::R<SrcaddrSpec>;
#[doc = "Register `SRCADDR` writer"]
pub type W = crate::W<SrcaddrSpec>;
#[doc = "Field `ADDR` reader - Source Buffer Address. Address may be byte aligned, but the length must be a multiple of 4 bits."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Source Buffer Address. Address may be byte aligned, but the length must be a multiple of 4 bits."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Buffer Address. Address may be byte aligned, but the length must be a multiple of 4 bits."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Buffer Address. Address may be byte aligned, but the length must be a multiple of 4 bits."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<SrcaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Source Addresss\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcaddrSpec;
impl crate::RegisterSpec for SrcaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcaddr::R`](R) reader structure"]
impl crate::Readable for SrcaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`srcaddr::W`](W) writer structure"]
impl crate::Writable for SrcaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCADDR to value 0"]
impl crate::Resettable for SrcaddrSpec {
    const RESET_VALUE: u32 = 0;
}
