use rand::seq::IndexedRandom;
use serde::Serialize;

static WEBGL_RENDERERS: &[&str] = &[
    "ANGLE (Intel, Intel(R) UHD Graphics (0x00008A56) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 2060 SUPER Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon (TM) Graphics (0x000015E7) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon RX 580 2048SP Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon RX 6600 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon(TM) Graphics (0x000015D8) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon(TM) Graphics (0x00001636) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon(TM) Graphics (0x00001638) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon(TM) Graphics (0x0000164C) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon(TM) Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, AMD Radeon(TM) Vega 8 Graphics (0x000015D8) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, Radeon RX 570 Series Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (AMD, Radeon RX 580 Series Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Apple, ANGLE Metal Renderer: Apple M1 Pro, Unspecified Version)",
    "ANGLE (Apple, ANGLE Metal Renderer: Apple M1, Unspecified Version)",
    "ANGLE (Apple, ANGLE Metal Renderer: Apple M2, Unspecified Version)",
    "ANGLE (Apple, ANGLE Metal Renderer: Apple M3, Unspecified Version)",
    "ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)",
    "ANGLE (Intel, Intel(R) HD Graphics (0x00000152) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) HD Graphics 400 Direct3D11 vs_5_0 ps_5_0), or similar",
    "ANGLE (Intel, Intel(R) HD Graphics 4000 (0x00000166) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) HD Graphics 4600 (0x00000412) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) HD Graphics 520 (0x00001916) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) HD Graphics 530 (0x00001912) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) HD Graphics 5500 (0x00001616) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) HD Graphics 620 (0x00005916) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) HD Graphics 630 (0x00005912) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) HD Graphics Direct3D11 vs_5_0 ps_5_0), or similar",
    "ANGLE (Intel, Intel(R) HD Graphics Direct3D9Ex vs_3_0 ps_3_0, igdumd64.dll)",
    "ANGLE (Intel, Intel(R) HD Graphics Family (0x00000A16) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) Iris(R) Xe Graphics (0x000046A6) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) Iris(R) Xe Graphics (0x000046A8) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) Iris(R) Xe Graphics (0x00009A49) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) Iris(R) Xe Graphics (0x0000A7A0) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) Iris(R) Xe Graphics (0x0000A7A1) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) Iris(R) Xe Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics (0x00004628) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics (0x000046A3) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics (0x000046B3) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics (0x00008A56) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics (0x00009A78) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics (0x00009B41) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics (0x00009BC4) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics (0x0000A721) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 600 (0x00003185) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 620 (0x00003EA0) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 620 (0x00005917) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 630 (0x00003E92) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 630 (0x00003E9B) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 630 (0x00009BC8) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 730 (0x00004692) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 770 (0x00004680) Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (Intel, Mesa Intel(R) UHD Graphics 600 (GLK 2), OpenGL ES 3.2)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1050 Ti Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1060 3GB Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1060 6GB Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1070 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1650 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 SUPER Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 Ti Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 980 Direct3D11 vs_5_0 ps_5_0), or similar",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 2060 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 2060 SUPER Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 2070 SUPER Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 3050 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 3050 Laptop GPU Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 Laptop GPU Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 Ti Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 3070 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 3070 Ti Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 3080 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 4050 Laptop GPU Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 Laptop GPU Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 Ti Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 4070 Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 4070 SUPER Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce RTX 4070 Ti Direct3D11 vs_5_0 ps_5_0, D3D11)",
    "Intel(R) HD Graphics",
];

#[derive(Serialize, Debug)]
pub struct WebglRenderers(&'static str);

impl WebglRenderers {
    pub fn random() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = WEBGL_RENDERERS.choose(&mut rng).unwrap();
        Ok(WebglRenderers(res))
    }
}
