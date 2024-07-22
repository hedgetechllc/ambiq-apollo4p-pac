#[doc = "Register `IBRD` reader"]
pub type R = crate::R<IbrdSpec>;
#[doc = "Register `IBRD` writer"]
pub type W = crate::W<IbrdSpec>;
#[doc = "Field `DIVINT` reader - This field holds the integer baud rate divisor. It is set in conjunction with the DIVFRAC field in the FBRD register and the clock rate as set in the CLKSEL field of the CR register. The field should be set to the integer part of the ratio: Clock Rate / (16 x intended baud rate)."]
pub type DivintR = crate::FieldReader<u16>;
#[doc = "Field `DIVINT` writer - This field holds the integer baud rate divisor. It is set in conjunction with the DIVFRAC field in the FBRD register and the clock rate as set in the CLKSEL field of the CR register. The field should be set to the integer part of the ratio: Clock Rate / (16 x intended baud rate)."]
pub type DivintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field holds the integer baud rate divisor. It is set in conjunction with the DIVFRAC field in the FBRD register and the clock rate as set in the CLKSEL field of the CR register. The field should be set to the integer part of the ratio: Clock Rate / (16 x intended baud rate)."]
    #[inline(always)]
    pub fn divint(&self) -> DivintR {
        DivintR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field holds the integer baud rate divisor. It is set in conjunction with the DIVFRAC field in the FBRD register and the clock rate as set in the CLKSEL field of the CR register. The field should be set to the integer part of the ratio: Clock Rate / (16 x intended baud rate)."]
    #[inline(always)]
    #[must_use]
    pub fn divint(&mut self) -> DivintW<IbrdSpec> {
        DivintW::new(self, 0)
    }
}
#[doc = "Integer Baud Rate Divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`ibrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IbrdSpec;
impl crate::RegisterSpec for IbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibrd::R`](R) reader structure"]
impl crate::Readable for IbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`ibrd::W`](W) writer structure"]
impl crate::Writable for IbrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IbrdSpec {
    const RESET_VALUE: u32 = 0;
}