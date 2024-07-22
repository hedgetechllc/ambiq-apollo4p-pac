#[doc = "Register `AHBMSINGLES` reader"]
pub type R = crate::R<AhbmsinglesSpec>;
#[doc = "Register `AHBMSINGLES` writer"]
pub type W = crate::W<AhbmsinglesSpec>;
#[doc = "Field `AHBSINGLES` reader - Force ahb singles"]
pub type AhbsinglesR = crate::BitReader;
#[doc = "Field `AHBSINGLES` writer - Force ahb singles"]
pub type AhbsinglesW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force ahb singles"]
    #[inline(always)]
    pub fn ahbsingles(&self) -> AhbsinglesR {
        AhbsinglesR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force ahb singles"]
    #[inline(always)]
    #[must_use]
    pub fn ahbsingles(&mut self) -> AhbsinglesW<AhbmsinglesSpec> {
        AhbsinglesW::new(self, 0)
    }
}
#[doc = "This register forces the ahb transactions to be always singles.\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmsingles::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmsingles::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmsinglesSpec;
impl crate::RegisterSpec for AhbmsinglesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmsingles::R`](R) reader structure"]
impl crate::Readable for AhbmsinglesSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmsingles::W`](W) writer structure"]
impl crate::Writable for AhbmsinglesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBMSINGLES to value 0"]
impl crate::Resettable for AhbmsinglesSpec {
    const RESET_VALUE: u32 = 0;
}
