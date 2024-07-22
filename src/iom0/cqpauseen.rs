#[doc = "Register `CQPAUSEEN` reader"]
pub type R = crate::R<CqpauseenSpec>;
#[doc = "Register `CQPAUSEEN` writer"]
pub type W = crate::W<CqpauseenSpec>;
#[doc = "Enables the specified event to pause command processing when active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cqpen {
    #[doc = "32768: Pauses the command queue when the current index matches the last index"]
    Idxeq = 32768,
    #[doc = "16384: Pause command queue when input BLE bit XORed with SWFLAG4 is '1'"]
    Blexoren = 16384,
    #[doc = "8192: Pause command queue when input IOM bit XORed with SWFLAG3 is '1'"]
    Iomxoren = 8192,
    #[doc = "4096: Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1'"]
    Gpioxoren = 4096,
    #[doc = "2048: Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1'"]
    Mspi1xnoren = 2048,
    #[doc = "1024: Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1'"]
    Mspi0xnoren = 1024,
    #[doc = "512: Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1'"]
    Mspi1xoren = 512,
    #[doc = "256: Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1'"]
    Mspi0xoren = 256,
    #[doc = "128: Pause the command queue when software flag bit 7 is '1'."]
    Swflagen7 = 128,
    #[doc = "64: Pause the command queue when software flag bit 6 is '1'"]
    Swflagen6 = 64,
    #[doc = "32: Pause the command queue when software flag bit 5 is '1'"]
    Swflagen5 = 32,
    #[doc = "16: Pause the command queue when software flag bit 4 is '1'"]
    Swflagen4 = 16,
    #[doc = "8: Pause the command queue when software flag bit 3 is '1'"]
    Swflagen3 = 8,
    #[doc = "4: Pause the command queue when software flag bit 2 is '1'"]
    Swflagen2 = 4,
    #[doc = "2: Pause the command queue when software flag bit 1 is '1'"]
    Swflagen1 = 2,
    #[doc = "1: Pause the command queue when software flag bit 0 is '1'"]
    Swflagen0 = 1,
}
impl From<Cqpen> for u16 {
    #[inline(always)]
    fn from(variant: Cqpen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cqpen {
    type Ux = u16;
}
impl crate::IsEnum for Cqpen {}
#[doc = "Field `CQPEN` reader - Enables the specified event to pause command processing when active"]
pub type CqpenR = crate::FieldReader<Cqpen>;
impl CqpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cqpen> {
        match self.bits {
            32768 => Some(Cqpen::Idxeq),
            16384 => Some(Cqpen::Blexoren),
            8192 => Some(Cqpen::Iomxoren),
            4096 => Some(Cqpen::Gpioxoren),
            2048 => Some(Cqpen::Mspi1xnoren),
            1024 => Some(Cqpen::Mspi0xnoren),
            512 => Some(Cqpen::Mspi1xoren),
            256 => Some(Cqpen::Mspi0xoren),
            128 => Some(Cqpen::Swflagen7),
            64 => Some(Cqpen::Swflagen6),
            32 => Some(Cqpen::Swflagen5),
            16 => Some(Cqpen::Swflagen4),
            8 => Some(Cqpen::Swflagen3),
            4 => Some(Cqpen::Swflagen2),
            2 => Some(Cqpen::Swflagen1),
            1 => Some(Cqpen::Swflagen0),
            _ => None,
        }
    }
    #[doc = "Pauses the command queue when the current index matches the last index"]
    #[inline(always)]
    pub fn is_idxeq(&self) -> bool {
        *self == Cqpen::Idxeq
    }
    #[doc = "Pause command queue when input BLE bit XORed with SWFLAG4 is '1'"]
    #[inline(always)]
    pub fn is_blexoren(&self) -> bool {
        *self == Cqpen::Blexoren
    }
    #[doc = "Pause command queue when input IOM bit XORed with SWFLAG3 is '1'"]
    #[inline(always)]
    pub fn is_iomxoren(&self) -> bool {
        *self == Cqpen::Iomxoren
    }
    #[doc = "Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1'"]
    #[inline(always)]
    pub fn is_gpioxoren(&self) -> bool {
        *self == Cqpen::Gpioxoren
    }
    #[doc = "Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1'"]
    #[inline(always)]
    pub fn is_mspi1xnoren(&self) -> bool {
        *self == Cqpen::Mspi1xnoren
    }
    #[doc = "Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1'"]
    #[inline(always)]
    pub fn is_mspi0xnoren(&self) -> bool {
        *self == Cqpen::Mspi0xnoren
    }
    #[doc = "Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1'"]
    #[inline(always)]
    pub fn is_mspi1xoren(&self) -> bool {
        *self == Cqpen::Mspi1xoren
    }
    #[doc = "Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1'"]
    #[inline(always)]
    pub fn is_mspi0xoren(&self) -> bool {
        *self == Cqpen::Mspi0xoren
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1'."]
    #[inline(always)]
    pub fn is_swflagen7(&self) -> bool {
        *self == Cqpen::Swflagen7
    }
    #[doc = "Pause the command queue when software flag bit 6 is '1'"]
    #[inline(always)]
    pub fn is_swflagen6(&self) -> bool {
        *self == Cqpen::Swflagen6
    }
    #[doc = "Pause the command queue when software flag bit 5 is '1'"]
    #[inline(always)]
    pub fn is_swflagen5(&self) -> bool {
        *self == Cqpen::Swflagen5
    }
    #[doc = "Pause the command queue when software flag bit 4 is '1'"]
    #[inline(always)]
    pub fn is_swflagen4(&self) -> bool {
        *self == Cqpen::Swflagen4
    }
    #[doc = "Pause the command queue when software flag bit 3 is '1'"]
    #[inline(always)]
    pub fn is_swflagen3(&self) -> bool {
        *self == Cqpen::Swflagen3
    }
    #[doc = "Pause the command queue when software flag bit 2 is '1'"]
    #[inline(always)]
    pub fn is_swflagen2(&self) -> bool {
        *self == Cqpen::Swflagen2
    }
    #[doc = "Pause the command queue when software flag bit 1 is '1'"]
    #[inline(always)]
    pub fn is_swflagen1(&self) -> bool {
        *self == Cqpen::Swflagen1
    }
    #[doc = "Pause the command queue when software flag bit 0 is '1'"]
    #[inline(always)]
    pub fn is_swflagen0(&self) -> bool {
        *self == Cqpen::Swflagen0
    }
}
#[doc = "Field `CQPEN` writer - Enables the specified event to pause command processing when active"]
pub type CqpenW<'a, REG> = crate::FieldWriter<'a, REG, 16, Cqpen>;
impl<'a, REG> CqpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Pauses the command queue when the current index matches the last index"]
    #[inline(always)]
    pub fn idxeq(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Idxeq)
    }
    #[doc = "Pause command queue when input BLE bit XORed with SWFLAG4 is '1'"]
    #[inline(always)]
    pub fn blexoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Blexoren)
    }
    #[doc = "Pause command queue when input IOM bit XORed with SWFLAG3 is '1'"]
    #[inline(always)]
    pub fn iomxoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Iomxoren)
    }
    #[doc = "Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1'"]
    #[inline(always)]
    pub fn gpioxoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Gpioxoren)
    }
    #[doc = "Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1'"]
    #[inline(always)]
    pub fn mspi1xnoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Mspi1xnoren)
    }
    #[doc = "Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1'"]
    #[inline(always)]
    pub fn mspi0xnoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Mspi0xnoren)
    }
    #[doc = "Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1'"]
    #[inline(always)]
    pub fn mspi1xoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Mspi1xoren)
    }
    #[doc = "Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1'"]
    #[inline(always)]
    pub fn mspi0xoren(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Mspi0xoren)
    }
    #[doc = "Pause the command queue when software flag bit 7 is '1'."]
    #[inline(always)]
    pub fn swflagen7(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Swflagen7)
    }
    #[doc = "Pause the command queue when software flag bit 6 is '1'"]
    #[inline(always)]
    pub fn swflagen6(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Swflagen6)
    }
    #[doc = "Pause the command queue when software flag bit 5 is '1'"]
    #[inline(always)]
    pub fn swflagen5(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Swflagen5)
    }
    #[doc = "Pause the command queue when software flag bit 4 is '1'"]
    #[inline(always)]
    pub fn swflagen4(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Swflagen4)
    }
    #[doc = "Pause the command queue when software flag bit 3 is '1'"]
    #[inline(always)]
    pub fn swflagen3(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Swflagen3)
    }
    #[doc = "Pause the command queue when software flag bit 2 is '1'"]
    #[inline(always)]
    pub fn swflagen2(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Swflagen2)
    }
    #[doc = "Pause the command queue when software flag bit 1 is '1'"]
    #[inline(always)]
    pub fn swflagen1(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Swflagen1)
    }
    #[doc = "Pause the command queue when software flag bit 0 is '1'"]
    #[inline(always)]
    pub fn swflagen0(self) -> &'a mut crate::W<REG> {
        self.variant(Cqpen::Swflagen0)
    }
}
impl R {
    #[doc = "Bits 0:15 - Enables the specified event to pause command processing when active"]
    #[inline(always)]
    pub fn cqpen(&self) -> CqpenR {
        CqpenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Enables the specified event to pause command processing when active"]
    #[inline(always)]
    #[must_use]
    pub fn cqpen(&mut self) -> CqpenW<CqpauseenSpec> {
        CqpenW::new(self, 0)
    }
}
#[doc = "Enables a flag to pause an active command queue operation. If a bit is '1' and the corresponding bit in the CQFLAG register is '1', CQ processing will halt until either value is changed to '0'.\n\nYou can [`read`](crate::Reg::read) this register and get [`cqpauseen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqpauseen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqpauseenSpec;
impl crate::RegisterSpec for CqpauseenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqpauseen::R`](R) reader structure"]
impl crate::Readable for CqpauseenSpec {}
#[doc = "`write(|w| ..)` method takes [`cqpauseen::W`](W) writer structure"]
impl crate::Writable for CqpauseenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQPAUSEEN to value 0"]
impl crate::Resettable for CqpauseenSpec {
    const RESET_VALUE: u32 = 0;
}
