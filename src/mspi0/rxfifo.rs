#[doc = "Register `RXFIFO` reader"]
pub type R = crate::R<RxfifoSpec>;
#[doc = "Register `RXFIFO` writer"]
pub type W = crate::W<RxfifoSpec>;
#[doc = "Field `RXFIFO` reader - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
pub type RxfifoR = crate::FieldReader<u32>;
#[doc = "Field `RXFIFO` writer - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
pub type RxfifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
    #[inline(always)]
    pub fn rxfifo(&self) -> RxfifoR {
        RxfifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive data. Data is aligned to the LSB (padded zeros on upper bits) unless BIGENDIAN is set."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo(&mut self) -> RxfifoW<RxfifoSpec> {
        RxfifoW::new(self, 0)
    }
}
#[doc = "RX Data FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifoSpec;
impl crate::RegisterSpec for RxfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifo::R`](R) reader structure"]
impl crate::Readable for RxfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfifo::W`](W) writer structure"]
impl crate::Writable for RxfifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFIFO to value 0"]
impl crate::Resettable for RxfifoSpec {
    const RESET_VALUE: u32 = 0;
}
