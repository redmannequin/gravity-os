use crate::{mmio_read, mmio_write, wait_cycles, MMIO_BASE};

pub const MBOX_BASE: u32 = MMIO_BASE + 0x0000_B880;
pub const MBOX_READ: u32 = MBOX_BASE;
pub const MBOX_WRITE: u32 = MBOX_BASE + 0x0000_0020;
pub const MBOX_STATUS: u32 = MBOX_BASE + 0x0000_0018;

pub const MBOX_FULL: u32 = 0x8000_0000;
pub const MBOX_EMPTY: u32 = 0x4000_0000;

pub const ARM2VC: u8 = 0x8;

pub const MSG_STATE_REQ: u32 = 0x0000_0000;
pub const MSG_STATE_OK: u32 = 0x8000_0000;
pub const MSG_STATE_ERR: u32 = 0x8000_0001;

// Video Core
pub const TAG_GET_FRIMWARE_REV: u32 = 0x0000_0001;

// Frame Buffer
pub const TAG_SET_PHY_WH: u32 = 0x0004_8003;
pub const TAG_SET_VIRT_WH: u32 = 0x004_8004;
pub const TAG_SET_VIRT_OFF: u32 = 0x0004_8009;
pub const TAG_SET_DEPTH: u32 = 0x0004_8005;
pub const TAG_SET_PXL_ORDER: u32 = 0x0004_8006;

pub const TAG_ALLOCATE_FB: u32 = 0x0004_0001;
pub const TAG_GET_PITCH: u32 = 0x0004_0008;

pub const TAG_LAST: u32 = 0x0;

#[repr(C, align(16))]
pub struct A16<const N: usize> {
    inner: [u32; N],
}

pub struct Mailbox;

impl Mailbox {
    pub fn init_frame_buffer() {
        let frame_buffer_msg: A16<35> = A16 {
            inner: [
                35 * 4,
                MSG_STATE_REQ,
                TAG_SET_PHY_WH,
                8,
                0,
                1920,
                1080,
                TAG_SET_VIRT_WH,
                8,
                0,
                1920,
                1080,
                TAG_SET_VIRT_OFF,
                8,
                0,
                0,
                0,
                TAG_SET_DEPTH,
                4,
                0,
                16,
                TAG_SET_PXL_ORDER,
                4,
                0,
                1,
                TAG_ALLOCATE_FB,
                8,
                0,
                4,
                0,
                TAG_GET_PITCH,
                4,
                0,
                0,
                TAG_LAST,
            ],
        };
        let fb = Mailbox::send_msg(ARM2VC, frame_buffer_msg);

        let fb_ptr = fb.inner[28];
        let fb_pitch = fb.inner[33];

        for x in 100..200 {
            for y in 100..200 {
                let ptr = fb_ptr as *mut u16;
                unsafe { core::ptr::write_volatile(ptr.offset((x + (y * fb_pitch)) as _), 0xff) }
            }
        }
    }

    pub fn get_firmware_rev() {
        let request = A16 {
            inner: [6 * 4, MSG_STATE_REQ, TAG_GET_FRIMWARE_REV, 4, 0, TAG_LAST],
        };
        Self::send_msg(ARM2VC, request);
    }

    fn send_msg<const N: usize>(ch: u8, msg: A16<N>) -> A16<N> {
        let msg_ptr = {
            let mbox_address: *const A16<N> = &msg;
            let mbox_address_int = mbox_address as usize;
            ((mbox_address_int & !0xF) | (ch as usize)) as u32
        };

        Mailbox::write(ch, msg_ptr);
        let _n = Mailbox::read(ch);

        let mut msg = A16 { inner: [0; N] };
        unsafe {
            core::ptr::copy((msg_ptr & !0xF) as *const A16<N>, &mut msg as *mut _, 1);
        };

        wait_cycles(10);

        msg
    }

    fn read(channel: u8) -> u32 {
        loop {
            while (mmio_read(MBOX_STATUS) & MBOX_EMPTY) != 0x0 {}
            let data = mmio_read(MBOX_READ);
            if (data & 0xF) == channel as u32 {
                return data;
            }
        }
    }

    fn write(channel: u8, data: u32) {
        while (mmio_read(MBOX_STATUS) & MBOX_FULL) != 0x0 {}
        let value = (data & 0xFFFF_FFF0) | ((channel & 0xF) as u32);
        mmio_write(MBOX_WRITE, value);
    }
}
