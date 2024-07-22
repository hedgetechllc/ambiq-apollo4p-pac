#[doc = "Register `DEVCFG` reader"]
pub type R = crate::R<DevcfgSpec>;
#[doc = "Register `DEVCFG` writer"]
pub type W = crate::W<DevcfgSpec>;
#[doc = "Field `DEVADDR` reader - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
pub type DevaddrR = crate::FieldReader<u16>;
#[doc = "Field `DEVADDR` writer - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DevaddrW<DevcfgSpec> {
        DevaddrW::new(self, 0)
    }
}
#[doc = "Contains the I2C device address.\n\nYou can [`read`](crate::Reg::read) this register and get [`devcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevcfgSpec;
impl crate::RegisterSpec for DevcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devcfg::R`](R) reader structure"]
impl crate::Readable for DevcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`devcfg::W`](W) writer structure"]
impl crate::Writable for DevcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVCFG to value 0"]
impl crate::Resettable for DevcfgSpec {
    const RESET_VALUE: u32 = 0;
}
