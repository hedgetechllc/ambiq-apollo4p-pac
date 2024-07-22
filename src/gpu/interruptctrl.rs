#[doc = "Register `INTERRUPTCTRL` reader"]
pub type R = crate::R<InterruptctrlSpec>;
#[doc = "Register `INTERRUPTCTRL` writer"]
pub type W = crate::W<InterruptctrlSpec>;
#[doc = "Field `IRQACTIVE` reader - if set to zero IRQ is active high, if set to one IRQ is active low"]
pub type IrqactiveR = crate::BitReader;
#[doc = "Field `IRQACTIVE` writer - if set to zero IRQ is active high, if set to one IRQ is active low"]
pub type IrqactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTCMDEND` reader - if set, signals interrupt at the end of command list"]
pub type IntcmdendR = crate::BitReader;
#[doc = "Field `INTCMDEND` writer - if set, signals interrupt at the end of command list"]
pub type IntcmdendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTDRAWEND` reader - if set, signals interrupt at the end of drawing command"]
pub type IntdrawendR = crate::BitReader;
#[doc = "Field `INTDRAWEND` writer - if set, signals interrupt at the end of drawing command"]
pub type IntdrawendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCLR` reader - if set, auto clears interrupt"]
pub type AutoclrR = crate::BitReader;
#[doc = "Field `AUTOCLR` writer - if set, auto clears interrupt"]
pub type AutoclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - This bitfield is reserved."]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - This bitfield is reserved."]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `CHANGEFREQ` reader - change frequency of asynchronous clock"]
pub type ChangefreqR = crate::FieldReader;
#[doc = "Field `CHANGEFREQ` writer - change frequency of asynchronous clock"]
pub type ChangefreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - if set to zero IRQ is active high, if set to one IRQ is active low"]
    #[inline(always)]
    pub fn irqactive(&self) -> IrqactiveR {
        IrqactiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - if set, signals interrupt at the end of command list"]
    #[inline(always)]
    pub fn intcmdend(&self) -> IntcmdendR {
        IntcmdendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - if set, signals interrupt at the end of drawing command"]
    #[inline(always)]
    pub fn intdrawend(&self) -> IntdrawendR {
        IntdrawendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - if set, auto clears interrupt"]
    #[inline(always)]
    pub fn autoclr(&self) -> AutoclrR {
        AutoclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:29 - This bitfield is reserved."]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x03ff_ffff)
    }
    #[doc = "Bits 30:31 - change frequency of asynchronous clock"]
    #[inline(always)]
    pub fn changefreq(&self) -> ChangefreqR {
        ChangefreqR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - if set to zero IRQ is active high, if set to one IRQ is active low"]
    #[inline(always)]
    #[must_use]
    pub fn irqactive(&mut self) -> IrqactiveW<InterruptctrlSpec> {
        IrqactiveW::new(self, 0)
    }
    #[doc = "Bit 1 - if set, signals interrupt at the end of command list"]
    #[inline(always)]
    #[must_use]
    pub fn intcmdend(&mut self) -> IntcmdendW<InterruptctrlSpec> {
        IntcmdendW::new(self, 1)
    }
    #[doc = "Bit 2 - if set, signals interrupt at the end of drawing command"]
    #[inline(always)]
    #[must_use]
    pub fn intdrawend(&mut self) -> IntdrawendW<InterruptctrlSpec> {
        IntdrawendW::new(self, 2)
    }
    #[doc = "Bit 3 - if set, auto clears interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn autoclr(&mut self) -> AutoclrW<InterruptctrlSpec> {
        AutoclrW::new(self, 3)
    }
    #[doc = "Bits 4:29 - This bitfield is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<InterruptctrlSpec> {
        RsvdW::new(self, 4)
    }
    #[doc = "Bits 30:31 - change frequency of asynchronous clock"]
    #[inline(always)]
    #[must_use]
    pub fn changefreq(&mut self) -> ChangefreqW<InterruptctrlSpec> {
        ChangefreqW::new(self, 30)
    }
}
#[doc = "On write, clears the IRQ (CHECK address!).\n\nYou can [`read`](crate::Reg::read) this register and get [`interruptctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interruptctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptctrlSpec;
impl crate::RegisterSpec for InterruptctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interruptctrl::R`](R) reader structure"]
impl crate::Readable for InterruptctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`interruptctrl::W`](W) writer structure"]
impl crate::Writable for InterruptctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPTCTRL to value 0"]
impl crate::Resettable for InterruptctrlSpec {
    const RESET_VALUE: u32 = 0;
}
