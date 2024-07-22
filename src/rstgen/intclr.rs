#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
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
    pub fn bodh(&mut self) -> BodhW<IntclrSpec> {
        BodhW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC"]
    #[inline(always)]
    #[must_use]
    pub fn boddigc(&mut self) -> BoddigcW<IntclrSpec> {
        BoddigcW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDF"]
    #[inline(always)]
    #[must_use]
    pub fn boddigf(&mut self) -> BoddigfW<IntclrSpec> {
        BoddigfW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDS"]
    #[inline(always)]
    #[must_use]
    pub fn boddigs(&mut self) -> BoddigsW<IntclrSpec> {
        BoddigsW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables an interrupt that triggers when simobuck digital detects inactivity on VDDC_LV"]
    #[inline(always)]
    #[must_use]
    pub fn boddigclv(&mut self) -> BoddigclvW<IntclrSpec> {
        BoddigclvW::new(self, 4)
    }
}
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
