use windows::{
    Win32::{
        Foundation::{HMODULE, HWND},
        Graphics::{
            Direct3D::{
                D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
                D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
            },
            Direct3D11::{
                D3D11_BIND_SHADER_RESOURCE, D3D11_COMPARISON_NEVER, D3D11_CPU_ACCESS_WRITE,
                D3D11_FILTER_MIN_MAG_MIP_LINEAR, D3D11_FLOAT32_MAX, D3D11_MAP_WRITE_DISCARD,
                D3D11_MAPPED_SUBRESOURCE, D3D11_RESOURCE_MISC_FLAG, D3D11_SAMPLER_DESC,
                D3D11_SDK_VERSION, D3D11_TEXTURE_ADDRESS_CLAMP, D3D11_TEXTURE2D_DESC,
                D3D11_USAGE_DYNAMIC, D3D11_VIEWPORT, D3D11CreateDeviceAndSwapChain, ID3D11Device,
                ID3D11DeviceContext, ID3D11PixelShader, ID3D11RenderTargetView, ID3D11SamplerState,
                ID3D11ShaderResourceView, ID3D11Texture2D, ID3D11VertexShader,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_MODE_SCALING_UNSPECIFIED,
                    DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_RATIONAL, DXGI_SAMPLE_DESC,
                },
                DXGI_PRESENT, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT_FLIP_DISCARD,
                DXGI_USAGE_RENDER_TARGET_OUTPUT, IDXGISwapChain,
            },
        },
    },
    core::Result,
};

pub struct Renderer {
    render_target_view: ID3D11RenderTargetView,
    swap_chain: IDXGISwapChain,
    context: ID3D11DeviceContext,
    // device: ID3D11Device,
    width: u32,
    height: u32,
    vertex: ID3D11VertexShader,
    pixel: ID3D11PixelShader,
    sampler: ID3D11SamplerState,
    texture_srv: ID3D11ShaderResourceView,
    texture: ID3D11Texture2D,
}

impl Renderer {
    pub fn create_renderer(
        hwnd: HWND,
        width: u32,
        height: u32,
        width_video: u32,
        heigth_video: u32,
    ) -> Result<Self> {
        unsafe {
            let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
                BufferDesc: DXGI_MODE_DESC {
                    Width: width,
                    Height: height,
                    RefreshRate: DXGI_RATIONAL {
                        Numerator: 0,
                        Denominator: 0,
                    },
                    Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
                },
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                BufferCount: 2,
                OutputWindow: hwnd,
                Windowed: true.into(),
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                Flags: 0,
            };

            let future_levels = [D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_11_0];

            let mut swap_chain: Option<IDXGISwapChain> = None;
            let mut device: Option<ID3D11Device> = None;
            let mut feature_level = D3D_FEATURE_LEVEL_11_1;
            let mut context: Option<ID3D11DeviceContext> = None;

            D3D11CreateDeviceAndSwapChain(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HMODULE::default(),
                Default::default(),
                Some(&future_levels),
                D3D11_SDK_VERSION,
                Some(&swap_chain_desc),
                Some(&mut swap_chain),
                Some(&mut device),
                Some(&mut feature_level),
                Some(&mut context),
            )?;

            let device = device.unwrap();
            let context = context.unwrap();
            let swap_chain = swap_chain.unwrap();
            let back_buffer: ID3D11Texture2D = swap_chain.GetBuffer(0)?;
            let mut render_target_view: Option<ID3D11RenderTargetView> = None;

            device.CreateRenderTargetView(&back_buffer, None, Some(&mut render_target_view))?;

            let render_target_view = render_target_view.unwrap();

            const VERTEX_BYTECODE: &[u8; 776] = include_bytes!(r"../shaders/assets/vs.cso");
            const PIXEL_BYTECODE: &[u8; 660] = include_bytes!(r"../shaders/assets/ps.cso");

            let mut vertex: Option<ID3D11VertexShader> = None;
            let mut pixel: Option<ID3D11PixelShader> = None;

            device.CreateVertexShader(VERTEX_BYTECODE, None, Some(&mut vertex))?;
            device.CreatePixelShader(PIXEL_BYTECODE, None, Some(&mut pixel))?;

            let vertex = vertex.unwrap();
            let pixel = pixel.unwrap();

            let texture_d = D3D11_TEXTURE2D_DESC {
                Width: width_video,
                Height: heigth_video,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_DYNAMIC,
                BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
                CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                MiscFlags: D3D11_RESOURCE_MISC_FLAG(0).0 as u32,
            };

            let mut texture: Option<ID3D11Texture2D> = None;

            device.CreateTexture2D(&texture_d, None, Some(&mut texture))?;

            let texture = texture.unwrap();
            let mut texture_srv: Option<ID3D11ShaderResourceView> = None;

            device.CreateShaderResourceView(&texture, None, Some(&mut texture_srv))?;

            let texture_srv = texture_srv.unwrap();

            let sampler_desc = D3D11_SAMPLER_DESC {
                Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
                AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
                AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
                MipLODBias: 0.0,
                MaxAnisotropy: 1,
                ComparisonFunc: D3D11_COMPARISON_NEVER,
                BorderColor: [0.0, 0.0, 0.0, 0.0],
                MinLOD: 0.0,
                MaxLOD: D3D11_FLOAT32_MAX,
            };

            let mut sampler: Option<ID3D11SamplerState> = None;

            device.CreateSamplerState(&sampler_desc, Some(&mut sampler))?;

            let sampler = sampler.unwrap();

            return Ok(Renderer {
                render_target_view,
                swap_chain,
                context,
                // device,
                width,
                height,
                vertex,
                pixel,
                sampler,
                texture_srv,
                texture,
            });
        }
    }

    pub fn render(&self, color: [f32; 4], vsync: u32) {
        unsafe {
            self.context
                .OMSetRenderTargets(Some(&[Some(self.render_target_view.clone())]), None);

            let viewport: D3D11_VIEWPORT = D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: self.width as f32,
                Height: self.height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };

            self.context.RSSetViewports(Some(&[viewport]));

            self.context
                .ClearRenderTargetView(&self.render_target_view, &color);

            self.context.VSSetShader(&self.vertex, None);

            self.context.PSSetShader(&self.pixel, None);

            self.context
                .PSSetShaderResources(0, Some(&[Some(self.texture_srv.clone())]));

            self.context
                .PSSetSamplers(0, Some(&[Some(self.sampler.clone())]));

            self.context
                .IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

            self.context.Draw(3, 0);

            let _ = self.swap_chain.Present(vsync, DXGI_PRESENT(0));
        }
    }

    pub fn change_texture(&self, data: &[u8], width: u32, height: u32) -> Result<()> {
        unsafe {
            let mut mapped_img = D3D11_MAPPED_SUBRESOURCE::default();

            self.context.Map(
                &self.texture,
                0,
                D3D11_MAP_WRITE_DISCARD,
                0,
                Some(&mut mapped_img),
            )?;

            let src = data.as_ptr();
            let dst = mapped_img.pData as *mut u8;
            let bytes_in_one_row = width * 4;
            let pitch = mapped_img.RowPitch as usize;

            for num_str in 0..height {
                let src_row = src.add((num_str * bytes_in_one_row) as usize);
                let dst_row = dst.add((num_str as usize * pitch) as usize);

                std::ptr::copy_nonoverlapping(src_row, dst_row, bytes_in_one_row as usize);
            }

            self.context.Unmap(&self.texture, 0);

            Ok(())
        }
    }
}
