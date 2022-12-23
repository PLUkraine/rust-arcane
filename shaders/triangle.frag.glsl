#version 330 core

in VS_OUT {
    vec4 color;
} in_fg;
out vec4 color;

void main(void)
{
    color = in_fg.color;
}
