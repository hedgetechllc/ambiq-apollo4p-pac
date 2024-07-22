#[doc = "Register `DPDMPULLDOWN` reader"]
pub type R = crate::R<DpdmpulldownSpec>;
#[doc = "Register `DPDMPULLDOWN` writer"]
pub type W = crate::W<DpdmpulldownSpec>;
#[doc = "Field `DMPULLDOWN` reader - Enables a pulldown resistor(15K) on D-"]
pub type DmpulldownR = crate::BitReader;
#[doc = "Field `DMPULLDOWN` writer - Enables a pulldown resistor(15K) on D-"]
pub type DmpulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPPULLDOWN` reader - Enables a pulldown resistor(15K) on D+"]
pub type DppulldownR = crate::BitReader;
#[doc = "Field `DPPULLDOWN` writer - Enables a pulldown resistor(15K) on D+"]
pub type DppulldownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables a pulldown resistor(15K) on D-"]
    #[inline(always)]
    pub fn dmpulldown(&self) -> DmpulldownR {
        DmpulldownR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables a pulldown resistor(15K) on D+"]
    #[inline(always)]
    pub fn dppulldown(&self) -> DppulldownR {
        DppulldownR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables a pulldown resistor(15K) on D-"]
    #[inline(always)]
    #[must_use]
    pub fn dmpulldown(&mut self) -> DmpulldownW<DpdmpulldownSpec> {
        DmpulldownW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables a pulldown resistor(15K) on D+"]
    #[inline(always)]
    #[must_use]
    pub fn dppulldown(&mut self) -> DppulldownW<DpdmpulldownSpec> {
        DppulldownW::new(self, 1)
    }
}
#[doc = "Enables a pulldown resistor(15K) on D+ or D-\n\nYou can [`read`](crate::Reg::read) this register and get [`dpdmpulldown::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpdmpulldown::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpdmpulldownSpec;
impl crate::RegisterSpec for DpdmpulldownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpdmpulldown::R`](R) reader structure"]
impl crate::Readable for DpdmpulldownSpec {}
#[doc = "`write(|w| ..)` method takes [`dpdmpulldown::W`](W) writer structure"]
impl crate::Writable for DpdmpulldownSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPDMPULLDOWN to value 0"]
impl crate::Resettable for DpdmpulldownSpec {
    const RESET_VALUE: u32 = 0;
}
