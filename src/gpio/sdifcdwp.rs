#[doc = "Register `SDIFCDWP` reader"]
pub type R = crate::R<SdifcdwpSpec>;
#[doc = "Register `SDIFCDWP` writer"]
pub type W = crate::W<SdifcdwpSpec>;
#[doc = "Field `SDIFCD` reader - SDIF CD pad select."]
pub type SdifcdR = crate::FieldReader;
#[doc = "Field `SDIFCD` writer - SDIF CD pad select."]
pub type SdifcdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SDIFWP` reader - SDIF WP pad select."]
pub type SdifwpR = crate::FieldReader;
#[doc = "Field `SDIFWP` writer - SDIF WP pad select."]
pub type SdifwpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - SDIF CD pad select."]
    #[inline(always)]
    pub fn sdifcd(&self) -> SdifcdR {
        SdifcdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - SDIF WP pad select."]
    #[inline(always)]
    pub fn sdifwp(&self) -> SdifwpR {
        SdifwpR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SDIF CD pad select."]
    #[inline(always)]
    #[must_use]
    pub fn sdifcd(&mut self) -> SdifcdW<SdifcdwpSpec> {
        SdifcdW::new(self, 0)
    }
    #[doc = "Bits 8:14 - SDIF WP pad select."]
    #[inline(always)]
    #[must_use]
    pub fn sdifwp(&mut self) -> SdifwpW<SdifcdwpSpec> {
        SdifwpW::new(self, 8)
    }
}
#[doc = "SDIF CD and WP Select.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdifcdwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdifcdwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdifcdwpSpec;
impl crate::RegisterSpec for SdifcdwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdifcdwp::R`](R) reader structure"]
impl crate::Readable for SdifcdwpSpec {}
#[doc = "`write(|w| ..)` method takes [`sdifcdwp::W`](W) writer structure"]
impl crate::Writable for SdifcdwpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIFCDWP to value 0x7f7f"]
impl crate::Resettable for SdifcdwpSpec {
    const RESET_VALUE: u32 = 0x7f7f;
}
