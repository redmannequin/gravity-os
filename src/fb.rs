use crate::{
    mailbox::{
        Mailbox, A16, ARM2VC, MSG_STATE_REQ, TAG_ALLOCATE_FB, TAG_GET_PITCH, TAG_LAST,
        TAG_SET_DEPTH, TAG_SET_PHY_WH, TAG_SET_PXL_ORDER, TAG_SET_VIRT_OFF, TAG_SET_VIRT_WH,
    },
    println,
};

pub fn init_fb(width: u32, height: u32) -> FrameBuffer {
    let frame_buffer_msg: A16<35> = A16::new([
        35 * 4,
        MSG_STATE_REQ,
        TAG_SET_PHY_WH,
        8,
        0,
        width,
        height,
        TAG_SET_VIRT_WH,
        8,
        0,
        width,
        height,
        TAG_SET_VIRT_OFF,
        8,
        0,
        0,
        0,
        TAG_SET_DEPTH,
        4,
        0,
        32,
        TAG_SET_PXL_ORDER,
        4,
        0,
        1,
        TAG_ALLOCATE_FB,
        8,
        0,
        4096,
        0,
        TAG_GET_PITCH,
        4,
        0,
        0,
        TAG_LAST,
    ]);
    let fb = Mailbox::send_msg(ARM2VC, frame_buffer_msg);

    let fb_ptr = fb.inner[28];
    let fb_pitch = fb.inner[33];

    println!("pitch: {}", fb_pitch);

    FrameBuffer {
        ptr: fb_ptr,
        width: fb_pitch / 4,
        height,
    }
}

pub struct FrameBuffer {
    pub ptr: u32,
    pub width: u32,
    pub height: u32,
}

pub fn run(fb: &mut FrameBuffer) {
    for x in 1..(fb.width - 1) {
        for y in 1..(fb.height - 1) {
            let ptr = fb.ptr as *mut u32;
            let offset = (x + (y * fb.width)) as _;
            unsafe { core::ptr::write_volatile(ptr.offset(offset), pixel(x as _, y as _, 0)) }
        }
    }
}

pub fn pixel(r: u8, g: u8, b: u8) -> u32 {
    r as u32 | ((g as u32) << 8) | ((b as u32) << 16)
}