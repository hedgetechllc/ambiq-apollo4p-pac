#[doc = "Register `IOM3IRQ` reader"]
pub type R = crate::R<Iom3irqSpec>;
#[doc = "Register `IOM3IRQ` writer"]
pub type W = crate::W<Iom3irqSpec>;
#[doc = "Field `IOM3IRQ` reader - IOM3 IRQ pad select."]
pub type Iom3irqR = crate::FieldReader;
#[doc = "Field `IOM3IRQ` writer - IOM3 IRQ pad select."]
pub type Iom3irqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - IOM3 IRQ pad select."]
    #[inline(always)]
    pub fn iom3irq(&self) -> Iom3irqR {
        Iom3irqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - IOM3 IRQ pad select."]
    #[inline(always)]
    #[must_use]
    pub fn iom3irq(&mut self) -> Iom3irqW<Iom3irqSpec> {
        Iom3irqW::new(self, 0)
    }
}
#[doc = "IOM3 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom3irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom3irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iom3irqSpec;
impl crate::RegisterSpec for Iom3irqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iom3irq::R`](R) reader structure"]
impl crate::Readable for Iom3irqSpec {}
#[doc = "`write(|w| ..)` method takes [`iom3irq::W`](W) writer structure"]
impl crate::Writable for Iom3irqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOM3IRQ to value 0x3f"]
impl crate::Resettable for Iom3irqSpec {
    const RESET_VALUE: u32 = 0x3f;
}
