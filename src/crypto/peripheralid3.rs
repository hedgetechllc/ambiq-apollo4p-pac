#[doc = "Register `PERIPHERALID3` reader"]
pub type R = crate::R<Peripheralid3Spec>;
#[doc = "Register `PERIPHERALID3` writer"]
pub type W = crate::W<Peripheralid3Spec>;
#[doc = "Field `CMOD` reader - Customer Modified, normally zero, but if a partner applies any changes themselves, they must change this value."]
pub type CmodR = crate::FieldReader;
#[doc = "Field `CMOD` writer - Customer Modified, normally zero, but if a partner applies any changes themselves, they must change this value."]
pub type CmodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REVAND` reader - starts at zero for every Revision, and increments if metal fixes are applied between 2 IP releases."]
pub type RevandR = crate::FieldReader;
#[doc = "Field `REVAND` writer - starts at zero for every Revision, and increments if metal fixes are applied between 2 IP releases."]
pub type RevandW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Customer Modified, normally zero, but if a partner applies any changes themselves, they must change this value."]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - starts at zero for every Revision, and increments if metal fixes are applied between 2 IP releases."]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Customer Modified, normally zero, but if a partner applies any changes themselves, they must change this value."]
    #[inline(always)]
    #[must_use]
    pub fn cmod(&mut self) -> CmodW<Peripheralid3Spec> {
        CmodW::new(self, 0)
    }
    #[doc = "Bits 4:7 - starts at zero for every Revision, and increments if metal fixes are applied between 2 IP releases."]
    #[inline(always)]
    #[must_use]
    pub fn revand(&mut self) -> RevandW<Peripheralid3Spec> {
        RevandW::new(self, 4)
    }
}
#[doc = "Peripheral ID 3 (PID3).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Peripheralid3Spec;
impl crate::RegisterSpec for Peripheralid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peripheralid3::R`](R) reader structure"]
impl crate::Readable for Peripheralid3Spec {}
#[doc = "`write(|w| ..)` method takes [`peripheralid3::W`](W) writer structure"]
impl crate::Writable for Peripheralid3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIPHERALID3 to value 0"]
impl crate::Resettable for Peripheralid3Spec {
    const RESET_VALUE: u32 = 0;
}
