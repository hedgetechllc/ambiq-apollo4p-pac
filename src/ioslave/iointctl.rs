#[doc = "Register `IOINTCTL` reader"]
pub type R = crate::R<IointctlSpec>;
#[doc = "Register `IOINTCTL` writer"]
pub type W = crate::W<IointctlSpec>;
#[doc = "Field `IOINTEN` reader - These read-only bits indicate whether the IOINT interrupts are enabled."]
pub type IointenR = crate::FieldReader;
#[doc = "Field `IOINTEN` writer - These read-only bits indicate whether the IOINT interrupts are enabled."]
pub type IointenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOINT` reader - These bits read the IOINT interrupts."]
pub type IointR = crate::FieldReader;
#[doc = "Field `IOINT` writer - These bits read the IOINT interrupts."]
pub type IointW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOINTCLR` reader - This bit clears all of the IOINT interrupts when written with a 1."]
pub type IointclrR = crate::BitReader;
#[doc = "Field `IOINTCLR` writer - This bit clears all of the IOINT interrupts when written with a 1."]
pub type IointclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOINTSET` reader - These bits set the IOINT interrupts when written with a 1."]
pub type IointsetR = crate::FieldReader;
#[doc = "Field `IOINTSET` writer - These bits set the IOINT interrupts when written with a 1."]
pub type IointsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These read-only bits indicate whether the IOINT interrupts are enabled."]
    #[inline(always)]
    pub fn iointen(&self) -> IointenR {
        IointenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These bits read the IOINT interrupts."]
    #[inline(always)]
    pub fn ioint(&self) -> IointR {
        IointR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit clears all of the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointclr(&self) -> IointclrR {
        IointclrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - These bits set the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub fn iointset(&self) -> IointsetR {
        IointsetR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These read-only bits indicate whether the IOINT interrupts are enabled."]
    #[inline(always)]
    #[must_use]
    pub fn iointen(&mut self) -> IointenW<IointctlSpec> {
        IointenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - These bits read the IOINT interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn ioint(&mut self) -> IointW<IointctlSpec> {
        IointW::new(self, 8)
    }
    #[doc = "Bit 16 - This bit clears all of the IOINT interrupts when written with a 1."]
    #[inline(always)]
    #[must_use]
    pub fn iointclr(&mut self) -> IointclrW<IointctlSpec> {
        IointclrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - These bits set the IOINT interrupts when written with a 1."]
    #[inline(always)]
    #[must_use]
    pub fn iointset(&mut self) -> IointsetW<IointctlSpec> {
        IointsetW::new(self, 24)
    }
}
#[doc = "I/O Interrupt Control\n\nYou can [`read`](crate::Reg::read) this register and get [`iointctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iointctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IointctlSpec;
impl crate::RegisterSpec for IointctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iointctl::R`](R) reader structure"]
impl crate::Readable for IointctlSpec {}
#[doc = "`write(|w| ..)` method takes [`iointctl::W`](W) writer structure"]
impl crate::Writable for IointctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOINTCTL to value 0"]
impl crate::Resettable for IointctlSpec {
    const RESET_VALUE: u32 = 0;
}
