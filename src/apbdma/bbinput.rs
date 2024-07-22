#[doc = "Register `BBINPUT` reader"]
pub type R = crate::R<BbinputSpec>;
#[doc = "Register `BBINPUT` writer"]
pub type W = crate::W<BbinputSpec>;
#[doc = "Field `DATAIN` reader - PIO values"]
pub type DatainR = crate::FieldReader;
#[doc = "Field `DATAIN` writer - PIO values"]
pub type DatainW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PIO values"]
    #[inline(always)]
    pub fn datain(&self) -> DatainR {
        DatainR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PIO values"]
    #[inline(always)]
    #[must_use]
    pub fn datain(&mut self) -> DatainW<BbinputSpec> {
        DatainW::new(self, 0)
    }
}
#[doc = "PIO Input Values\n\nYou can [`read`](crate::Reg::read) this register and get [`bbinput::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbinput::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BbinputSpec;
impl crate::RegisterSpec for BbinputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bbinput::R`](R) reader structure"]
impl crate::Readable for BbinputSpec {}
#[doc = "`write(|w| ..)` method takes [`bbinput::W`](W) writer structure"]
impl crate::Writable for BbinputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BBINPUT to value 0"]
impl crate::Resettable for BbinputSpec {
    const RESET_VALUE: u32 = 0;
}
