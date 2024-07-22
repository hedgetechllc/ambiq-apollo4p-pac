#[doc = "Register `ADDPTR` reader"]
pub type R = crate::R<AddptrSpec>;
#[doc = "Register `ADDPTR` writer"]
pub type W = crate::W<AddptrSpec>;
#[doc = "Field `ADDPTR` reader - The current value in the Address pointer."]
pub type AddptrR = crate::FieldReader;
#[doc = "Field `ADDPTR` writer - The current value in the Address pointer."]
pub type AddptrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The current value in the Address pointer."]
    #[inline(always)]
    pub fn addptr(&self) -> AddptrR {
        AddptrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The current value in the Address pointer."]
    #[inline(always)]
    #[must_use]
    pub fn addptr(&mut self) -> AddptrW<AddptrSpec> {
        AddptrW::new(self, 0)
    }
}
#[doc = "Address pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`addptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddptrSpec;
impl crate::RegisterSpec for AddptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addptr::R`](R) reader structure"]
impl crate::Readable for AddptrSpec {}
#[doc = "`write(|w| ..)` method takes [`addptr::W`](W) writer structure"]
impl crate::Writable for AddptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDPTR to value 0"]
impl crate::Resettable for AddptrSpec {
    const RESET_VALUE: u32 = 0;
}
