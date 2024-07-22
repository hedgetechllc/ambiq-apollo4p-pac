#[doc = "Register `DSP0CONTROL` reader"]
pub type R = crate::R<Dsp0controlSpec>;
#[doc = "Register `DSP0CONTROL` writer"]
pub type W = crate::W<Dsp0controlSpec>;
#[doc = "Field `DSP0STATVECSEL` reader - DSP 0 StatVectorSel"]
pub type Dsp0statvecselR = crate::BitReader;
#[doc = "Field `DSP0STATVECSEL` writer - DSP 0 StatVectorSel"]
pub type Dsp0statvecselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP0BRESET` reader - DSP0 BReset. This is the reset used for Xtensa core. S/w must clear this reset to use Dsp."]
pub type Dsp0bresetR = crate::BitReader;
#[doc = "Field `DSP0BRESET` writer - DSP0 BReset. This is the reset used for Xtensa core. S/w must clear this reset to use Dsp."]
pub type Dsp0bresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP0DRESET` reader - DSP0 DReset. This is the reset used for debug functionality like OCD/TRAX etc."]
pub type Dsp0dresetR = crate::BitReader;
#[doc = "Field `DSP0DRESET` writer - DSP0 DReset. This is the reset used for debug functionality like OCD/TRAX etc."]
pub type Dsp0dresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSP0RUNSTALL` reader - DSP 0 RunStall. When asserted, DSP 0 will stall until bit is cleared."]
pub type Dsp0runstallR = crate::BitReader;
#[doc = "Field `DSP0RUNSTALL` writer - DSP 0 RunStall. When asserted, DSP 0 will stall until bit is cleared."]
pub type Dsp0runstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DSP 0 IDMA Trigger Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsp0idmatrig {
    #[doc = "3: Trigger is disabled until a cross trigger pulse is asserted. This will allow another source (determined by the XTRIGSRC register) to allow the DMA descriptor chain to proceed."]
    Xtrig = 3,
    #[doc = "2: Trigger is disabled until a trigger pulse (PULSE in IDMATRIG register) is asserted. This will allow a single step in the DMA descriptor chain to be enabled until next completion."]
    Sstep = 2,
    #[doc = "1: Trigger is always enabled. With this set, any trigger out will immediately generate a trigger in"]
    Aon = 1,
    #[doc = "0: Trigger is disabled. This will pause the iDMA indefinitely until enabled."]
    Disable = 0,
}
impl From<Dsp0idmatrig> for u8 {
    #[inline(always)]
    fn from(variant: Dsp0idmatrig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsp0idmatrig {
    type Ux = u8;
}
impl crate::IsEnum for Dsp0idmatrig {}
#[doc = "Field `DSP0IDMATRIG` reader - DSP 0 IDMA Trigger Control"]
pub type Dsp0idmatrigR = crate::FieldReader<Dsp0idmatrig>;
impl Dsp0idmatrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsp0idmatrig {
        match self.bits {
            3 => Dsp0idmatrig::Xtrig,
            2 => Dsp0idmatrig::Sstep,
            1 => Dsp0idmatrig::Aon,
            0 => Dsp0idmatrig::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger is disabled until a cross trigger pulse is asserted. This will allow another source (determined by the XTRIGSRC register) to allow the DMA descriptor chain to proceed."]
    #[inline(always)]
    pub fn is_xtrig(&self) -> bool {
        *self == Dsp0idmatrig::Xtrig
    }
    #[doc = "Trigger is disabled until a trigger pulse (PULSE in IDMATRIG register) is asserted. This will allow a single step in the DMA descriptor chain to be enabled until next completion."]
    #[inline(always)]
    pub fn is_sstep(&self) -> bool {
        *self == Dsp0idmatrig::Sstep
    }
    #[doc = "Trigger is always enabled. With this set, any trigger out will immediately generate a trigger in"]
    #[inline(always)]
    pub fn is_aon(&self) -> bool {
        *self == Dsp0idmatrig::Aon
    }
    #[doc = "Trigger is disabled. This will pause the iDMA indefinitely until enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dsp0idmatrig::Disable
    }
}
#[doc = "Field `DSP0IDMATRIG` writer - DSP 0 IDMA Trigger Control"]
pub type Dsp0idmatrigW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsp0idmatrig, crate::Safe>;
impl<'a, REG> Dsp0idmatrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger is disabled until a cross trigger pulse is asserted. This will allow another source (determined by the XTRIGSRC register) to allow the DMA descriptor chain to proceed."]
    #[inline(always)]
    pub fn xtrig(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0idmatrig::Xtrig)
    }
    #[doc = "Trigger is disabled until a trigger pulse (PULSE in IDMATRIG register) is asserted. This will allow a single step in the DMA descriptor chain to be enabled until next completion."]
    #[inline(always)]
    pub fn sstep(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0idmatrig::Sstep)
    }
    #[doc = "Trigger is always enabled. With this set, any trigger out will immediately generate a trigger in"]
    #[inline(always)]
    pub fn aon(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0idmatrig::Aon)
    }
    #[doc = "Trigger is disabled. This will pause the iDMA indefinitely until enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dsp0idmatrig::Disable)
    }
}
#[doc = "Field `DSP0IDMAXTRIGSRC` reader - DSP 0 IDMA Cross Trigger Source. All enabled sources are ANDed to generate a trigger enable. \n Bit30-12:IRQ18-0 Bit11: IDMATRIGPULSE, Bit10: DSP Timer1, Bit9: DSP Timer0, Bit8: alternate DSP iDMA trigger out"]
pub type Dsp0idmaxtrigsrcR = crate::FieldReader<u32>;
#[doc = "Field `DSP0IDMAXTRIGSRC` writer - DSP 0 IDMA Cross Trigger Source. All enabled sources are ANDed to generate a trigger enable. \n Bit30-12:IRQ18-0 Bit11: IDMATRIGPULSE, Bit10: DSP Timer1, Bit9: DSP Timer0, Bit8: alternate DSP iDMA trigger out"]
pub type Dsp0idmaxtrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - DSP 0 StatVectorSel"]
    #[inline(always)]
    pub fn dsp0statvecsel(&self) -> Dsp0statvecselR {
        Dsp0statvecselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSP0 BReset. This is the reset used for Xtensa core. S/w must clear this reset to use Dsp."]
    #[inline(always)]
    pub fn dsp0breset(&self) -> Dsp0bresetR {
        Dsp0bresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DSP0 DReset. This is the reset used for debug functionality like OCD/TRAX etc."]
    #[inline(always)]
    pub fn dsp0dreset(&self) -> Dsp0dresetR {
        Dsp0dresetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSP 0 RunStall. When asserted, DSP 0 will stall until bit is cleared."]
    #[inline(always)]
    pub fn dsp0runstall(&self) -> Dsp0runstallR {
        Dsp0runstallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DSP 0 IDMA Trigger Control"]
    #[inline(always)]
    pub fn dsp0idmatrig(&self) -> Dsp0idmatrigR {
        Dsp0idmatrigR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:30 - DSP 0 IDMA Cross Trigger Source. All enabled sources are ANDed to generate a trigger enable. \n Bit30-12:IRQ18-0 Bit11: IDMATRIGPULSE, Bit10: DSP Timer1, Bit9: DSP Timer0, Bit8: alternate DSP iDMA trigger out"]
    #[inline(always)]
    pub fn dsp0idmaxtrigsrc(&self) -> Dsp0idmaxtrigsrcR {
        Dsp0idmaxtrigsrcR::new((self.bits >> 8) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - DSP 0 StatVectorSel"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0statvecsel(&mut self) -> Dsp0statvecselW<Dsp0controlSpec> {
        Dsp0statvecselW::new(self, 0)
    }
    #[doc = "Bit 1 - DSP0 BReset. This is the reset used for Xtensa core. S/w must clear this reset to use Dsp."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0breset(&mut self) -> Dsp0bresetW<Dsp0controlSpec> {
        Dsp0bresetW::new(self, 1)
    }
    #[doc = "Bit 2 - DSP0 DReset. This is the reset used for debug functionality like OCD/TRAX etc."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0dreset(&mut self) -> Dsp0dresetW<Dsp0controlSpec> {
        Dsp0dresetW::new(self, 2)
    }
    #[doc = "Bit 3 - DSP 0 RunStall. When asserted, DSP 0 will stall until bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn dsp0runstall(&mut self) -> Dsp0runstallW<Dsp0controlSpec> {
        Dsp0runstallW::new(self, 3)
    }
    #[doc = "Bits 4:5 - DSP 0 IDMA Trigger Control"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0idmatrig(&mut self) -> Dsp0idmatrigW<Dsp0controlSpec> {
        Dsp0idmatrigW::new(self, 4)
    }
    #[doc = "Bits 8:30 - DSP 0 IDMA Cross Trigger Source. All enabled sources are ANDed to generate a trigger enable. \n Bit30-12:IRQ18-0 Bit11: IDMATRIGPULSE, Bit10: DSP Timer1, Bit9: DSP Timer0, Bit8: alternate DSP iDMA trigger out"]
    #[inline(always)]
    #[must_use]
    pub fn dsp0idmaxtrigsrc(&mut self) -> Dsp0idmaxtrigsrcW<Dsp0controlSpec> {
        Dsp0idmaxtrigsrcW::new(self, 8)
    }
}
#[doc = "DSP 0 control settings\n\nYou can [`read`](crate::Reg::read) this register and get [`dsp0control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsp0control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsp0controlSpec;
impl crate::RegisterSpec for Dsp0controlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp0control::R`](R) reader structure"]
impl crate::Readable for Dsp0controlSpec {}
#[doc = "`write(|w| ..)` method takes [`dsp0control::W`](W) writer structure"]
impl crate::Writable for Dsp0controlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSP0CONTROL to value 0x06"]
impl crate::Resettable for Dsp0controlSpec {
    const RESET_VALUE: u32 = 0x06;
}
