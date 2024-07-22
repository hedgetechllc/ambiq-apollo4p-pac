#[doc = "Register `PERIPHERALID0` reader"]
pub type R = crate::R<Peripheralid0Spec>;
#[doc = "Register `PERIPHERALID0` writer"]
pub type W = crate::W<Peripheralid0Spec>;
#[doc = "Field `PART0` reader - Identification register part number, bits\\[7:0\\]"]
pub type Part0R = crate::FieldReader;
#[doc = "Field `PART0` writer - Identification register part number, bits\\[7:0\\]"]
pub type Part0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Identification register part number, bits\\[7:0\\]"]
    #[inline(always)]
    pub fn part0(&self) -> Part0R {
        Part0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Identification register part number, bits\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn part0(&mut self) -> Part0W<Peripheralid0Spec> {
        Part0W::new(self, 0)
    }
}
#[doc = "Peripheral ID 0 (PID0).\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheralid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheralid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Peripheralid0Spec;
impl crate::RegisterSpec for Peripheralid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peripheralid0::R`](R) reader structure"]
impl crate::Readable for Peripheralid0Spec {}
#[doc = "`write(|w| ..)` method takes [`peripheralid0::W`](W) writer structure"]
impl crate::Writable for Peripheralid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIPHERALID0 to value 0xc0"]
impl crate::Resettable for Peripheralid0Spec {
    const RESET_VALUE: u32 = 0xc0;
}
