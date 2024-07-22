#[doc = "Register `BUSCTRL` reader"]
pub type R = crate::R<BusctrlSpec>;
#[doc = "Register `BUSCTRL` writer"]
pub type W = crate::W<BusctrlSpec>;
#[doc = "Field `BUSCTRL` reader - Bus Control"]
pub type BusctrlR = crate::FieldReader<u32>;
#[doc = "Field `BUSCTRL` writer - Bus Control"]
pub type BusctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Control"]
    #[inline(always)]
    pub fn busctrl(&self) -> BusctrlR {
        BusctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bus Control"]
    #[inline(always)]
    #[must_use]
    pub fn busctrl(&mut self) -> BusctrlW<BusctrlSpec> {
        BusctrlW::new(self, 0)
    }
}
#[doc = "Bus Control\n\nYou can [`read`](crate::Reg::read) this register and get [`busctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusctrlSpec;
impl crate::RegisterSpec for BusctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busctrl::R`](R) reader structure"]
impl crate::Readable for BusctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`busctrl::W`](W) writer structure"]
impl crate::Writable for BusctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSCTRL to value 0"]
impl crate::Resettable for BusctrlSpec {
    const RESET_VALUE: u32 = 0;
}
