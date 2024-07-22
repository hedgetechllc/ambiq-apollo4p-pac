#[doc = "Register `IOM2IRQ` reader"]
pub type R = crate::R<Iom2irqSpec>;
#[doc = "Register `IOM2IRQ` writer"]
pub type W = crate::W<Iom2irqSpec>;
#[doc = "Field `IOM2IRQ` reader - IOM2 IRQ pad select."]
pub type Iom2irqR = crate::FieldReader;
#[doc = "Field `IOM2IRQ` writer - IOM2 IRQ pad select."]
pub type Iom2irqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - IOM2 IRQ pad select."]
    #[inline(always)]
    pub fn iom2irq(&self) -> Iom2irqR {
        Iom2irqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - IOM2 IRQ pad select."]
    #[inline(always)]
    #[must_use]
    pub fn iom2irq(&mut self) -> Iom2irqW<Iom2irqSpec> {
        Iom2irqW::new(self, 0)
    }
}
#[doc = "IOM2 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom2irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom2irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iom2irqSpec;
impl crate::RegisterSpec for Iom2irqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iom2irq::R`](R) reader structure"]
impl crate::Readable for Iom2irqSpec {}
#[doc = "`write(|w| ..)` method takes [`iom2irq::W`](W) writer structure"]
impl crate::Writable for Iom2irqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOM2IRQ to value 0x3f"]
impl crate::Resettable for Iom2irqSpec {
    const RESET_VALUE: u32 = 0x3f;
}
