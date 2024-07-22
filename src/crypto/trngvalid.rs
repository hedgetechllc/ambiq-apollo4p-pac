#[doc = "Register `TRNGVALID` reader"]
pub type R = crate::R<TrngvalidSpec>;
#[doc = "Register `TRNGVALID` writer"]
pub type W = crate::W<TrngvalidSpec>;
#[doc = "Field `EHRVALID` reader - 0x1 indicates that collection of bits in the TRNG is completed, and data can be read from the EHR_DATA register."]
pub type EhrvalidR = crate::BitReader;
#[doc = "Field `EHRVALID` writer - 0x1 indicates that collection of bits in the TRNG is completed, and data can be read from the EHR_DATA register."]
pub type EhrvalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0x1 indicates that collection of bits in the TRNG is completed, and data can be read from the EHR_DATA register."]
    #[inline(always)]
    pub fn ehrvalid(&self) -> EhrvalidR {
        EhrvalidR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0x1 indicates that collection of bits in the TRNG is completed, and data can be read from the EHR_DATA register."]
    #[inline(always)]
    #[must_use]
    pub fn ehrvalid(&mut self) -> EhrvalidW<TrngvalidSpec> {
        EhrvalidW::new(self, 0)
    }
}
#[doc = "This register indicates that the TRNG data is valid.\n\nYou can [`read`](crate::Reg::read) this register and get [`trngvalid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngvalid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrngvalidSpec;
impl crate::RegisterSpec for TrngvalidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trngvalid::R`](R) reader structure"]
impl crate::Readable for TrngvalidSpec {}
#[doc = "`write(|w| ..)` method takes [`trngvalid::W`](W) writer structure"]
impl crate::Writable for TrngvalidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRNGVALID to value 0"]
impl crate::Resettable for TrngvalidSpec {
    const RESET_VALUE: u32 = 0;
}
