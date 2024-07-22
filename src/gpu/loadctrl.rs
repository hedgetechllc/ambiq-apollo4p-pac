#[doc = "Register `LOADCTRL` reader"]
pub type R = crate::R<LoadctrlSpec>;
#[doc = "Register `LOADCTRL` writer"]
pub type W = crate::W<LoadctrlSpec>;
#[doc = "Field `LOADCTRL` reader - Load Control"]
pub type LoadctrlR = crate::FieldReader<u32>;
#[doc = "Field `LOADCTRL` writer - Load Control"]
pub type LoadctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Load Control"]
    #[inline(always)]
    pub fn loadctrl(&self) -> LoadctrlR {
        LoadctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Load Control"]
    #[inline(always)]
    #[must_use]
    pub fn loadctrl(&mut self) -> LoadctrlW<LoadctrlSpec> {
        LoadctrlW::new(self, 0)
    }
}
#[doc = "Load Control\n\nYou can [`read`](crate::Reg::read) this register and get [`loadctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loadctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadctrlSpec;
impl crate::RegisterSpec for LoadctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loadctrl::R`](R) reader structure"]
impl crate::Readable for LoadctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`loadctrl::W`](W) writer structure"]
impl crate::Writable for LoadctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOADCTRL to value 0xf403_0105"]
impl crate::Resettable for LoadctrlSpec {
    const RESET_VALUE: u32 = 0xf403_0105;
}
