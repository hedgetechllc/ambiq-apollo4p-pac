#[doc = "Register `EMONCTRL` reader"]
pub type R = crate::R<EmonctrlSpec>;
#[doc = "Register `EMONCTRL` writer"]
pub type W = crate::W<EmonctrlSpec>;
#[doc = "Field `FREEZE` reader - Freeze the counter. Each bit corresponds to a counter. 0: Let the counter run. 1: Stop the counter."]
pub type FreezeR = crate::FieldReader;
#[doc = "Field `FREEZE` writer - Freeze the counter. Each bit corresponds to a counter. 0: Let the counter run. 1: Stop the counter."]
pub type FreezeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLEAR` reader - Clear the counter. Each bit corresponds to a counter. 0: Let the counter run run on its input clk. 1: Clear the counter"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear the counter. Each bit corresponds to a counter. 0: Let the counter run run on its input clk. 1: Clear the counter"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Freeze the counter. Each bit corresponds to a counter. 0: Let the counter run. 1: Stop the counter."]
    #[inline(always)]
    pub fn freeze(&self) -> FreezeR {
        FreezeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clear the counter. Each bit corresponds to a counter. 0: Let the counter run run on its input clk. 1: Clear the counter"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Freeze the counter. Each bit corresponds to a counter. 0: Let the counter run. 1: Stop the counter."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FreezeW<EmonctrlSpec> {
        FreezeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clear the counter. Each bit corresponds to a counter. 0: Let the counter run run on its input clk. 1: Clear the counter"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<EmonctrlSpec> {
        ClearW::new(self, 8)
    }
}
#[doc = "Controls each of the energy monitor conuters\n\nYou can [`read`](crate::Reg::read) this register and get [`emonctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emonctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmonctrlSpec;
impl crate::RegisterSpec for EmonctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emonctrl::R`](R) reader structure"]
impl crate::Readable for EmonctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`emonctrl::W`](W) writer structure"]
impl crate::Writable for EmonctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCTRL to value 0xff"]
impl crate::Resettable for EmonctrlSpec {
    const RESET_VALUE: u32 = 0xff;
}
