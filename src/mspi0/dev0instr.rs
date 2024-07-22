#[doc = "Register `DEV0INSTR` reader"]
pub type R = crate::R<Dev0instrSpec>;
#[doc = "Register `DEV0INSTR` writer"]
pub type W = crate::W<Dev0instrSpec>;
#[doc = "Field `WRITEINSTR0` reader - Write command sent for DMA operations"]
pub type Writeinstr0R = crate::FieldReader<u16>;
#[doc = "Field `WRITEINSTR0` writer - Write command sent for DMA operations"]
pub type Writeinstr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `READINSTR0` reader - Read command sent to flash for DMA/XIP operations"]
pub type Readinstr0R = crate::FieldReader<u16>;
#[doc = "Field `READINSTR0` writer - Read command sent to flash for DMA/XIP operations"]
pub type Readinstr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write command sent for DMA operations"]
    #[inline(always)]
    pub fn writeinstr0(&self) -> Writeinstr0R {
        Writeinstr0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read command sent to flash for DMA/XIP operations"]
    #[inline(always)]
    pub fn readinstr0(&self) -> Readinstr0R {
        Readinstr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write command sent for DMA operations"]
    #[inline(always)]
    #[must_use]
    pub fn writeinstr0(&mut self) -> Writeinstr0W<Dev0instrSpec> {
        Writeinstr0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read command sent to flash for DMA/XIP operations"]
    #[inline(always)]
    #[must_use]
    pub fn readinstr0(&mut self) -> Readinstr0W<Dev0instrSpec> {
        Readinstr0W::new(self, 16)
    }
}
#[doc = "When any SPI flash is configured, this register must be properly programmed before XIP or AUTO DMA operations commence.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0instr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0instr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0instrSpec;
impl crate::RegisterSpec for Dev0instrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0instr::R`](R) reader structure"]
impl crate::Readable for Dev0instrSpec {}
#[doc = "`write(|w| ..)` method takes [`dev0instr::W`](W) writer structure"]
impl crate::Writable for Dev0instrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0INSTR to value 0x000b_0006"]
impl crate::Resettable for Dev0instrSpec {
    const RESET_VALUE: u32 = 0x000b_0006;
}
