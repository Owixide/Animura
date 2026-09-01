struct VS_Output {
    float4 position: SV_Position;
    float2 uv : TEXCOORD;
};

VS_Output vertex_main(uint vertex_id : SV_VertexID) {
    VS_Output output_struct;

    if (vertex_id == 0) {
        output_struct.position = float4(-1.0, -1.0, 0.0, 1.0);
        output_struct.uv = float2(0.0, 1.0);
    }
    else if (vertex_id == 1) {
        output_struct.position = float4(-1.0, 3.0, 0.0, 1.0);
        output_struct.uv = float2(0.0, -1.0);
    }
    else {
        output_struct.position = float4(3.0, -1.0, 0.0, 1.0);
        output_struct.uv = float2(2.0, 1.0);
    }
    
    return output_struct;
}