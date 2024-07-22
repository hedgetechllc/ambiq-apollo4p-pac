#[doc = "Register `ILPR` reader"]
pub type R = crate::R<IlprSpec>;
#[doc = "Register `ILPR` writer"]
pub type W = crate::W<IlprSpec>;
#[doc = "Field `ILPDVSR` reader - 8-bit low-power divisor value. These bits are cleared to 0 at reset. Programming a zero value results in no IrLPBaud16 pulses being generated."]
pub type IlpdvsrR = crate::FieldReader;
#[doc = "Field `ILPDVSR` writer - 8-bit low-power divisor value. These bits are cleared to 0 at reset. Programming a zero value results in no IrLPBaud16 pulses being generated."]
pub type IlpdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value. These bits are cleared to 0 at reset. Programming a zero value results in no IrLPBaud16 pulses being generated."]
    #[inline(always)]
    pub fn ilpdvsr(&self) -> IlpdvsrR {
        IlpdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value. These bits are cleared to 0 at reset. Programming a zero value results in no IrLPBaud16 pulses being generated."]
    #[inline(always)]
    #[must_use]
    pub fn ilpdvsr(&mut self) -> IlpdvsrW<IlprSpec> {
        IlpdvsrW::new(self, 0)
    }
}
#[doc = "IrDA Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ilpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IlprSpec;
impl crate::RegisterSpec for IlprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ilpr::R`](R) reader structure"]
impl crate::Readable for IlprSpec {}
#[doc = "`write(|w| ..)` method takes [`ilpr::W`](W) writer structure"]
impl crate::Writable for IlprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ILPR to value 0"]
impl crate::Resettable for IlprSpec {
    const RESET_VALUE: u32 = 0;
}
