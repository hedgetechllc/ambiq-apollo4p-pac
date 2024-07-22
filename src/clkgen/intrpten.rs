#[doc = "Register `INTRPTEN` reader"]
pub type R = crate::R<IntrptenSpec>;
#[doc = "Register `INTRPTEN` writer"]
pub type W = crate::W<IntrptenSpec>;
#[doc = "Field `OF` reader - XT Oscillator Fail interrupt"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - XT Oscillator Fail interrupt"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XT Oscillator Fail interrupt"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XT Oscillator Fail interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OfW<IntrptenSpec> {
        OfW::new(self, 0)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrpten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrpten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrptenSpec;
impl crate::RegisterSpec for IntrptenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intrpten::R`](R) reader structure"]
impl crate::Readable for IntrptenSpec {}
#[doc = "`write(|w| ..)` method takes [`intrpten::W`](W) writer structure"]
impl crate::Writable for IntrptenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTRPTEN to value 0"]
impl crate::Resettable for IntrptenSpec {
    const RESET_VALUE: u32 = 0;
}
