#[doc = "Register `ALFINIT` reader"]
pub type R = crate::R<AlfinitSpec>;
#[doc = "Register `ALFINIT` writer"]
pub type W = crate::W<AlfinitSpec>;
#[doc = "Field `ALFXY` reader - Alfa value of STARTXY pixel"]
pub type AlfxyR = crate::FieldReader<u32>;
#[doc = "Field `ALFXY` writer - Alfa value of STARTXY pixel"]
pub type AlfxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alfa value of STARTXY pixel"]
    #[inline(always)]
    pub fn alfxy(&self) -> AlfxyR {
        AlfxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alfa value of STARTXY pixel"]
    #[inline(always)]
    #[must_use]
    pub fn alfxy(&mut self) -> AlfxyW<AlfinitSpec> {
        AlfxyW::new(self, 0)
    }
}
#[doc = "Alfa value of STARTXY pixel, (16, 16 fixed point) Shader Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`alfinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alfinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlfinitSpec;
impl crate::RegisterSpec for AlfinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alfinit::R`](R) reader structure"]
impl crate::Readable for AlfinitSpec {}
#[doc = "`write(|w| ..)` method takes [`alfinit::W`](W) writer structure"]
impl crate::Writable for AlfinitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALFINIT to value 0"]
impl crate::Resettable for AlfinitSpec {
    const RESET_VALUE: u32 = 0;
}
