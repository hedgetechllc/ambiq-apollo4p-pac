#[doc = "Register `LEN` reader"]
pub type R = crate::R<LenSpec>;
#[doc = "Register `LEN` writer"]
pub type W = crate::W<LenSpec>;
#[doc = "Field `LEN` reader - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
pub type LenR = crate::FieldReader<u32>;
#[doc = "Field `LEN` writer - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 2:23 - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits >> 2) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 2:23 - Buffer size (bottom two bits assumed to be zero to ensure a multiple of 4 bytes)"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<LenSpec> {
        LenW::new(self, 2)
    }
}
#[doc = "Length\n\nYou can [`read`](crate::Reg::read) this register and get [`len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LenSpec;
impl crate::RegisterSpec for LenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`len::R`](R) reader structure"]
impl crate::Readable for LenSpec {}
#[doc = "`write(|w| ..)` method takes [`len::W`](W) writer structure"]
impl crate::Writable for LenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LEN to value 0"]
impl crate::Resettable for LenSpec {
    const RESET_VALUE: u32 = 0;
}
