#[doc = "Register `BLOCK` reader"]
pub type R = crate::R<BlockSpec>;
#[doc = "Register `BLOCK` writer"]
pub type W = crate::W<BlockSpec>;
#[doc = "This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Transferblocksize {
    #[doc = "0: No Data Transfer"]
    Nodataxfer = 0,
    #[doc = "1: 1 Byte"]
    _1byte = 1,
    #[doc = "2: 2 Bytes"]
    _2bytes = 2,
    #[doc = "3: 3 Bytes"]
    _3bytes = 3,
    #[doc = "4: 4 Bytes (and so on from 1-2048)"]
    _4bytes = 4,
    #[doc = "511: 511 Bytes"]
    _511bytes = 511,
    #[doc = "512: 512 Bytes"]
    _512bytes = 512,
    #[doc = "2048: 2048 Bytes"]
    _2048bytes = 2048,
}
impl From<Transferblocksize> for u16 {
    #[inline(always)]
    fn from(variant: Transferblocksize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Transferblocksize {
    type Ux = u16;
}
impl crate::IsEnum for Transferblocksize {}
#[doc = "Field `TRANSFERBLOCKSIZE` reader - This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored."]
pub type TransferblocksizeR = crate::FieldReader<Transferblocksize>;
impl TransferblocksizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Transferblocksize> {
        match self.bits {
            0 => Some(Transferblocksize::Nodataxfer),
            1 => Some(Transferblocksize::_1byte),
            2 => Some(Transferblocksize::_2bytes),
            3 => Some(Transferblocksize::_3bytes),
            4 => Some(Transferblocksize::_4bytes),
            511 => Some(Transferblocksize::_511bytes),
            512 => Some(Transferblocksize::_512bytes),
            2048 => Some(Transferblocksize::_2048bytes),
            _ => None,
        }
    }
    #[doc = "No Data Transfer"]
    #[inline(always)]
    pub fn is_nodataxfer(&self) -> bool {
        *self == Transferblocksize::Nodataxfer
    }
    #[doc = "1 Byte"]
    #[inline(always)]
    pub fn is_1byte(&self) -> bool {
        *self == Transferblocksize::_1byte
    }
    #[doc = "2 Bytes"]
    #[inline(always)]
    pub fn is_2bytes(&self) -> bool {
        *self == Transferblocksize::_2bytes
    }
    #[doc = "3 Bytes"]
    #[inline(always)]
    pub fn is_3bytes(&self) -> bool {
        *self == Transferblocksize::_3bytes
    }
    #[doc = "4 Bytes (and so on from 1-2048)"]
    #[inline(always)]
    pub fn is_4bytes(&self) -> bool {
        *self == Transferblocksize::_4bytes
    }
    #[doc = "511 Bytes"]
    #[inline(always)]
    pub fn is_511bytes(&self) -> bool {
        *self == Transferblocksize::_511bytes
    }
    #[doc = "512 Bytes"]
    #[inline(always)]
    pub fn is_512bytes(&self) -> bool {
        *self == Transferblocksize::_512bytes
    }
    #[doc = "2048 Bytes"]
    #[inline(always)]
    pub fn is_2048bytes(&self) -> bool {
        *self == Transferblocksize::_2048bytes
    }
}
#[doc = "Field `TRANSFERBLOCKSIZE` writer - This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored."]
pub type TransferblocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, Transferblocksize>;
impl<'a, REG> TransferblocksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No Data Transfer"]
    #[inline(always)]
    pub fn nodataxfer(self) -> &'a mut crate::W<REG> {
        self.variant(Transferblocksize::Nodataxfer)
    }
    #[doc = "1 Byte"]
    #[inline(always)]
    pub fn _1byte(self) -> &'a mut crate::W<REG> {
        self.variant(Transferblocksize::_1byte)
    }
    #[doc = "2 Bytes"]
    #[inline(always)]
    pub fn _2bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Transferblocksize::_2bytes)
    }
    #[doc = "3 Bytes"]
    #[inline(always)]
    pub fn _3bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Transferblocksize::_3bytes)
    }
    #[doc = "4 Bytes (and so on from 1-2048)"]
    #[inline(always)]
    pub fn _4bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Transferblocksize::_4bytes)
    }
    #[doc = "511 Bytes"]
    #[inline(always)]
    pub fn _511bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Transferblocksize::_511bytes)
    }
    #[doc = "512 Bytes"]
    #[inline(always)]
    pub fn _512bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Transferblocksize::_512bytes)
    }
    #[doc = "2048 Bytes"]
    #[inline(always)]
    pub fn _2048bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Transferblocksize::_2048bytes)
    }
}
#[doc = "To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hostsdmabufsz {
    #[doc = "0: 4KB(Detects A11 Carry out)"]
    _4kb = 0,
    #[doc = "1: 8KB(Detects A12 Carry out)"]
    _8kb = 1,
    #[doc = "2: 16KB(Detects A13 Carry out)"]
    _16kb = 2,
    #[doc = "3: 32KB(Detects A14 Carry out)"]
    _32kb = 3,
    #[doc = "4: 64KB(Detects A15 Carry out)"]
    _64kb = 4,
    #[doc = "5: 128KB(Detects A16 Carry out)"]
    _128kb = 5,
    #[doc = "6: 256KB(Detects A17 Carry out)"]
    _256kb = 6,
    #[doc = "7: 512KB(Detects A18 Carry out)"]
    _512kb = 7,
}
impl From<Hostsdmabufsz> for u8 {
    #[inline(always)]
    fn from(variant: Hostsdmabufsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hostsdmabufsz {
    type Ux = u8;
}
impl crate::IsEnum for Hostsdmabufsz {}
#[doc = "Field `HOSTSDMABUFSZ` reader - To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
pub type HostsdmabufszR = crate::FieldReader<Hostsdmabufsz>;
impl HostsdmabufszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hostsdmabufsz {
        match self.bits {
            0 => Hostsdmabufsz::_4kb,
            1 => Hostsdmabufsz::_8kb,
            2 => Hostsdmabufsz::_16kb,
            3 => Hostsdmabufsz::_32kb,
            4 => Hostsdmabufsz::_64kb,
            5 => Hostsdmabufsz::_128kb,
            6 => Hostsdmabufsz::_256kb,
            7 => Hostsdmabufsz::_512kb,
            _ => unreachable!(),
        }
    }
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Hostsdmabufsz::_4kb
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == Hostsdmabufsz::_8kb
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Hostsdmabufsz::_16kb
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn is_32kb(&self) -> bool {
        *self == Hostsdmabufsz::_32kb
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Hostsdmabufsz::_64kb
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn is_128kb(&self) -> bool {
        *self == Hostsdmabufsz::_128kb
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline(always)]
    pub fn is_256kb(&self) -> bool {
        *self == Hostsdmabufsz::_256kb
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn is_512kb(&self) -> bool {
        *self == Hostsdmabufsz::_512kb
    }
}
#[doc = "Field `HOSTSDMABUFSZ` writer - To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
pub type HostsdmabufszW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hostsdmabufsz, crate::Safe>;
impl<'a, REG> HostsdmabufszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabufsz::_4kb)
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline(always)]
    pub fn _8kb(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabufsz::_8kb)
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabufsz::_16kb)
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline(always)]
    pub fn _32kb(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabufsz::_32kb)
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabufsz::_64kb)
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline(always)]
    pub fn _128kb(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabufsz::_128kb)
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline(always)]
    pub fn _256kb(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabufsz::_256kb)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline(always)]
    pub fn _512kb(self) -> &'a mut crate::W<REG> {
        self.variant(Hostsdmabufsz::_512kb)
    }
}
#[doc = "This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Blkcnt {
    #[doc = "0: Stop Count"]
    Stopcnt = 0,
    #[doc = "1: 1 block"]
    _1block = 1,
    #[doc = "2: 2 blocks (and so on from 1-65535)"]
    _2blocks = 2,
    #[doc = "65535: 65535 blocks"]
    _65535blocks = 65535,
}
impl From<Blkcnt> for u16 {
    #[inline(always)]
    fn from(variant: Blkcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blkcnt {
    type Ux = u16;
}
impl crate::IsEnum for Blkcnt {}
#[doc = "Field `BLKCNT` reader - This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count."]
pub type BlkcntR = crate::FieldReader<Blkcnt>;
impl BlkcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blkcnt> {
        match self.bits {
            0 => Some(Blkcnt::Stopcnt),
            1 => Some(Blkcnt::_1block),
            2 => Some(Blkcnt::_2blocks),
            65535 => Some(Blkcnt::_65535blocks),
            _ => None,
        }
    }
    #[doc = "Stop Count"]
    #[inline(always)]
    pub fn is_stopcnt(&self) -> bool {
        *self == Blkcnt::Stopcnt
    }
    #[doc = "1 block"]
    #[inline(always)]
    pub fn is_1block(&self) -> bool {
        *self == Blkcnt::_1block
    }
    #[doc = "2 blocks (and so on from 1-65535)"]
    #[inline(always)]
    pub fn is_2blocks(&self) -> bool {
        *self == Blkcnt::_2blocks
    }
    #[doc = "65535 blocks"]
    #[inline(always)]
    pub fn is_65535blocks(&self) -> bool {
        *self == Blkcnt::_65535blocks
    }
}
#[doc = "Field `BLKCNT` writer - This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count."]
pub type BlkcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, Blkcnt>;
impl<'a, REG> BlkcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Stop Count"]
    #[inline(always)]
    pub fn stopcnt(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnt::Stopcnt)
    }
    #[doc = "1 block"]
    #[inline(always)]
    pub fn _1block(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnt::_1block)
    }
    #[doc = "2 blocks (and so on from 1-65535)"]
    #[inline(always)]
    pub fn _2blocks(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnt::_2blocks)
    }
    #[doc = "65535 blocks"]
    #[inline(always)]
    pub fn _65535blocks(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnt::_65535blocks)
    }
}
impl R {
    #[doc = "Bits 0:11 - This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored."]
    #[inline(always)]
    pub fn transferblocksize(&self) -> TransferblocksizeR {
        TransferblocksizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
    #[inline(always)]
    pub fn hostsdmabufsz(&self) -> HostsdmabufszR {
        HostsdmabufszR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count."]
    #[inline(always)]
    pub fn blkcnt(&self) -> BlkcntR {
        BlkcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This register specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53. It can be accessed only if no transaction is executing (i.e after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn transferblocksize(&mut self) -> TransferblocksizeW<BlockSpec> {
        TransferblocksizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn hostsdmabufsz(&mut self) -> HostsdmabufszW<BlockSpec> {
        HostsdmabufszW::new(self, 12)
    }
    #[doc = "Bits 16:31 - This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count."]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BlkcntW<BlockSpec> {
        BlkcntW::new(self, 16)
    }
}
#[doc = "Block size\n\nYou can [`read`](crate::Reg::read) this register and get [`block::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockSpec;
impl crate::RegisterSpec for BlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`block::R`](R) reader structure"]
impl crate::Readable for BlockSpec {}
#[doc = "`write(|w| ..)` method takes [`block::W`](W) writer structure"]
impl crate::Writable for BlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLOCK to value 0"]
impl crate::Resettable for BlockSpec {
    const RESET_VALUE: u32 = 0;
}
