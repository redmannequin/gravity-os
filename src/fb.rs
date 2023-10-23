use crate::{
    mailbox::{
        message::Message,
        tag::Tag,
        tags::framebuffer::{
            AllocateBufferRequest, AllocateBufferResponse, ColorDepth, GetPitchRequest,
            GetPitchResponse, PixelOrder, SetColorDepthRequest, SetPhysicalSizeRequest,
            SetPixelOrderRequest, SetVirtualOffsetRequest, SetVirtualSizeRequest,
        },
        Mailbox, ARM2VC,
    },
    println,
};

pub fn init_fb(width: u32, height: u32) -> FrameBuffer {
    let msg_req = Message::new()
        .with_tag(SetPhysicalSizeRequest::new(width, height))
        .with_tag(SetVirtualSizeRequest::new(width, height))
        .with_tag(SetVirtualOffsetRequest::new(0, 0))
        .with_tag(SetColorDepthRequest::new(ColorDepth::Rgba8))
        .with_tag(SetPixelOrderRequest::new(PixelOrder::Rgb))
        .with_tag(AllocateBufferRequest::new(4092))
        .with_tag(GetPitchRequest);

    let msg_res = Mailbox::send_msg(ARM2VC, msg_req);

    let fb: &Tag<AllocateBufferResponse> = msg_res.get_tag();
    let pitch: &Tag<GetPitchResponse> = msg_res.get_tag();

    println!("msg status: {:#010x}", msg_res.state);
    println!("fb status: {:#010x}", { fb.state });
    println!("fb size: {}", { fb.data.size });

    let fb_ptr = (fb.data.buffer_address | 0x40000000) & 0x3FFFFFFF;
    let fb_pitch = pitch.data.pitch;

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
