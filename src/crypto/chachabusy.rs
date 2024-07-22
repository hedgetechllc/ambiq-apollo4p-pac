#[doc = "Register `CHACHABUSY` reader"]
pub type R = crate::R<ChachabusySpec>;
#[doc = "Register `CHACHABUSY` writer"]
pub type W = crate::W<ChachabusySpec>;
#[doc = "Field `CHACHABUSY` reader - CHACHA_BUSY Register. This register is set when the CHACHA_SALSA core is active."]
pub type ChachabusyR = crate::BitReader;
#[doc = "Field `CHACHABUSY` writer - CHACHA_BUSY Register. This register is set when the CHACHA_SALSA core is active."]
pub type ChachabusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CHACHA_BUSY Register. This register is set when the CHACHA_SALSA core is active."]
    #[inline(always)]
    pub fn chachabusy(&self) -> ChachabusyR {
        ChachabusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHACHA_BUSY Register. This register is set when the CHACHA_SALSA core is active."]
    #[inline(always)]
    #[must_use]
    pub fn chachabusy(&mut self) -> ChachabusyW<ChachabusySpec> {
        ChachabusyW::new(self, 0)
    }
}
#[doc = "This register is set when the CHACHA_SALSA core is active\n\nYou can [`read`](crate::Reg::read) this register and get [`chachabusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachabusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachabusySpec;
impl crate::RegisterSpec for ChachabusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachabusy::R`](R) reader structure"]
impl crate::Readable for ChachabusySpec {}
#[doc = "`write(|w| ..)` method takes [`chachabusy::W`](W) writer structure"]
impl crate::Writable for ChachabusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHABUSY to value 0"]
impl crate::Resettable for ChachabusySpec {
    const RESET_VALUE: u32 = 0;
}
