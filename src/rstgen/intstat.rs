#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `BODH` reader - Enables an interrupt that triggers when VCC is below BODH level."]
pub type BodhR = crate::BitReader;
#[doc = "Field `BODH` writer - Enables an interrupt that triggers when VCC is below BODH level."]
pub type BodhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODDIGC` reader - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC"]
pub type BoddigcR = crate::BitReader;
#[doc = "Field `BODDIGC` writer - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC"]
pub type BoddigcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODDIGF` reader - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDF"]
pub type BoddigfR = crate::BitReader;
#[doc = "Field `BODDIGF` writer - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDF"]
pub type BoddigfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODDIGS` reader - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDS"]
pub type BoddigsR = crate::BitReader;
#[doc = "Field `BODDIGS` writer - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDS"]
pub type BoddigsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODDIGCLV` reader - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC_LV"]
pub type BoddigclvR = crate::BitReader;
#[doc = "Field `BODDIGCLV` writer - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC_LV"]
pub type BoddigclvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub fn bodh(&self) -> BodhR {
        BodhR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC"]
    #[inline(always)]
    pub fn boddigc(&self) -> BoddigcR {
        BoddigcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDF"]
    #[inline(always)]
    pub fn boddigf(&self) -> BoddigfR {
        BoddigfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDS"]
    #[inline(always)]
    pub fn boddigs(&self) -> BoddigsR {
        BoddigsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC_LV"]
    #[inline(always)]
    pub fn boddigclv(&self) -> BoddigclvR {
        BoddigclvR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    #[must_use]
    pub fn bodh(&mut self) -> BodhW<IntstatSpec> {
        BodhW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC"]
    #[inline(always)]
    #[must_use]
    pub fn boddigc(&mut self) -> BoddigcW<IntstatSpec> {
        BoddigcW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDF"]
    #[inline(always)]
    #[must_use]
    pub fn boddigf(&mut self) -> BoddigfW<IntstatSpec> {
        BoddigfW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDS"]
    #[inline(always)]
    #[must_use]
    pub fn boddigs(&mut self) -> BoddigsW<IntstatSpec> {
        BoddigsW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC_LV"]
    #[inline(always)]
    #[must_use]
    pub fn boddigclv(&mut self) -> BoddigclvW<IntstatSpec> {
        BoddigclvW::new(self, 4)
    }
}
#[doc = "Read bits from this register to discover the cause of a recent interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
