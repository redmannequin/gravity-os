use crate::mailbox::{
    tag::RequestTag, TAG_ALLOCATE_FB, TAG_GET_PITCH, TAG_SET_DEPTH, TAG_SET_PHY_WH,
    TAG_SET_PXL_ORDER, TAG_SET_VIRT_OFF, TAG_SET_VIRT_WH,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AllocateBufferRequest {
    pub alignment: u32,
}

impl AllocateBufferRequest {
    pub fn new(alignment: u32) -> Self {
        Self { alignment }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AllocateBufferResponse {
    pub buffer_address: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GetPitchRequest;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GetPitchResponse {
    pub pitch: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ColorDepth {
    Rgba8 = 32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetColorDepthRequest {
    pub depth: ColorDepth,
}

impl SetColorDepthRequest {
    pub fn new(depth: ColorDepth) -> Self {
        Self { depth }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetColorDepthResponse {
    pub depth: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetPhysicalSizeRequest {
    pub width: u32,
    pub height: u32,
}

impl SetPhysicalSizeRequest {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetPhysicalSizeResponse {
    pub width: u32,
    pub height: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum PixelOrder {
    Rgb = 0x1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetPixelOrderRequest {
    pub order: PixelOrder,
}

impl SetPixelOrderRequest {
    pub fn new(order: PixelOrder) -> Self {
        Self { order }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetPixelOrderResponse {
    pub order: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetVirtualSizeRequest {
    pub width: u32,
    pub height: u32,
}

impl SetVirtualSizeRequest {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetVirtualSizeResponse {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetVirtualOffsetRequest {
    pub x_offset: u32,
    pub y_offset: u32,
}

impl SetVirtualOffsetRequest {
    pub fn new(x_offset: u32, y_offset: u32) -> Self {
        Self { x_offset, y_offset }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SetVirtualOffsetResponse {
    pub x_offset: u32,
    pub y_offset: u32,
}

impl RequestTag for AllocateBufferRequest {
    const TAG_ID: u32 = TAG_ALLOCATE_FB;
    type ReponseType = AllocateBufferResponse;
}

impl RequestTag for GetPitchRequest {
    const TAG_ID: u32 = TAG_GET_PITCH;
    type ReponseType = GetPitchResponse;
}

impl RequestTag for SetColorDepthRequest {
    const TAG_ID: u32 = TAG_SET_DEPTH;
    type ReponseType = SetColorDepthResponse;
}

impl RequestTag for SetPhysicalSizeRequest {
    const TAG_ID: u32 = TAG_SET_PHY_WH;
    type ReponseType = SetPhysicalSizeResponse;
}

impl RequestTag for SetPixelOrderRequest {
    const TAG_ID: u32 = TAG_SET_PXL_ORDER;
    type ReponseType = SetPixelOrderResponse;
}

impl RequestTag for SetVirtualSizeRequest {
    const TAG_ID: u32 = TAG_SET_VIRT_WH;
    type ReponseType = SetVirtualSizeResponse;
}

impl RequestTag for SetVirtualOffsetRequest {
    const TAG_ID: u32 = TAG_SET_VIRT_OFF;
    type ReponseType = SetVirtualOffsetResponse;
}
