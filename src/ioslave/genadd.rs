#[doc = "Register `GENADD` reader"]
pub type R = crate::R<GenaddSpec>;
#[doc = "Register `GENADD` writer"]
pub type W = crate::W<GenaddSpec>;
#[doc = "Field `GADATA` reader - The data supplied on the last General Address reference."]
pub type GadataR = crate::FieldReader;
#[doc = "Field `GADATA` writer - The data supplied on the last General Address reference."]
pub type GadataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The data supplied on the last General Address reference."]
    #[inline(always)]
    pub fn gadata(&self) -> GadataR {
        GadataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The data supplied on the last General Address reference."]
    #[inline(always)]
    #[must_use]
    pub fn gadata(&mut self) -> GadataW<GenaddSpec> {
        GadataW::new(self, 0)
    }
}
#[doc = "General Address Data\n\nYou can [`read`](crate::Reg::read) this register and get [`genadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenaddSpec;
impl crate::RegisterSpec for GenaddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`genadd::R`](R) reader structure"]
impl crate::Readable for GenaddSpec {}
#[doc = "`write(|w| ..)` method takes [`genadd::W`](W) writer structure"]
impl crate::Writable for GenaddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GENADD to value 0"]
impl crate::Resettable for GenaddSpec {
    const RESET_VALUE: u32 = 0;
}
