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

    println!("msg status: {:#010x}", fb.inner[1]);
    println!("fb status: {:#010x}", fb.inner[27]);
    println!("fb size: {}", fb.inner[29]);

    let fb_ptr = (fb.inner[28] | 0x40000000) & 0x3FFFFFFF;
    let fb_pitch = fb.inner[33];

    println!("fb ptr: {:#010x}", fb_ptr);
    println!("fb pitch: {}", fb_pitch);

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

pub fn render_gradient(fb: &mut FrameBuffer, x_color_offset: u32, y_color_offset: u32) {
    for y in 0..fb.height {
        let y_offset = y * fb.width;
        for x in 0..fb.width {
            let ptr = fb.ptr as *mut u32;
            let offset = (x + y_offset) as _;
            let pixel = pixel(0, (y + y_color_offset) as _, (x + x_color_offset) as _);
            unsafe { core::ptr::write_volatile(ptr.offset(offset), pixel) }
        }
    }
}

pub fn pixel(r: u8, g: u8, b: u8) -> u32 {
    b as u32 | ((g as u32) << 8) | ((r as u32) << 16)
}
