Texture2D mtexture : register(t0);
SamplerState sampler_s : register(s0);

float4 pixel_main(float4 coord_pos : SV_Position, float2 uv : TEXCOORD): SV_Target {
    float4 color = mtexture.Sample(sampler_s, uv);

    return color;
}

