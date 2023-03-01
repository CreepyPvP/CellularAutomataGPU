#version 440

layout(local_size_x = 1, local_size_y = 1) in;

layout(shared, binding = 0) readonly buffer InputData {
    float c_in[];
};

layout(shared, binding = 1) writeonly buffer OutputData {
    float c_out[];
};

void main() {
    c_out[0] = c_in[0] + 1.0f;
}
