#[doc = "Register `IRQID` reader"]
pub type R = crate::R<IrqidSpec>;
#[doc = "Register `IRQID` writer"]
pub type W = crate::W<IrqidSpec>;
#[doc = "Field `IRQID` reader - Signals interrupt when set (CHECK address!"]
pub type IrqidR = crate::FieldReader<u32>;
#[doc = "Field `IRQID` writer - Signals interrupt when set (CHECK address!"]
pub type IrqidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Signals interrupt when set (CHECK address!"]
    #[inline(always)]
    pub fn irqid(&self) -> IrqidR {
        IrqidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Signals interrupt when set (CHECK address!"]
    #[inline(always)]
    #[must_use]
    pub fn irqid(&mut self) -> IrqidW<IrqidSpec> {
        IrqidW::new(self, 0)
    }
}
#[doc = "Signals interrupt when set (CHECK address!).\n\nYou can [`read`](crate::Reg::read) this register and get [`irqid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqidSpec;
impl crate::RegisterSpec for IrqidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqid::R`](R) reader structure"]
impl crate::Readable for IrqidSpec {}
#[doc = "`write(|w| ..)` method takes [`irqid::W`](W) writer structure"]
impl crate::Writable for IrqidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQID to value 0"]
impl crate::Resettable for IrqidSpec {
    const RESET_VALUE: u32 = 0;
}
