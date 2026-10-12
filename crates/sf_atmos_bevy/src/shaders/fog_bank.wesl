// The ground-fog bank mask (weather.md "Fog", F2), shared by the cloud
// trace's far-fog march and the froxels (F3), each with its own bindings of
// the weather map: its G channel (smooth FBM) at two scales, domain-warped by
// a slower third sample, then thresholded (gaps between banks, not
// thinning). The CPU twin is `sf_atmos::FogBanks::mask`. Bank function after
// the owner's Megalophobia `ground_fog.wgsl` (MIT OR Apache-2.0,
// ARCHITECTURE §10).

import super::fog::Fog;
import super::cloud_common::ramp;

// `p` is an origin-relative world position; `weather` the cloud weather map
// (repeating `samp`).
fn fog_bank_mask(f: Fog, weather: texture_2d<f32>, samp: sampler, p: vec3<f32>) -> f32 {
    let wuv = p.xz / f.warp_tile + f.warp_off;
    let warp = vec2(
        textureSampleLevel(weather, samp, wuv, 0.0).g,
        textureSampleLevel(weather, samp, wuv.yx + 0.37, 0.0).g,
    ) * 2.0 - 1.0;
    let s = p.xz + warp * f.warp_amount;
    let coarse = textureSampleLevel(weather, samp, s / f.bank_tile + f.bank_off, 0.0).g;
    let fine = textureSampleLevel(weather, samp, (s / f.detail_tile + f.detail_off).yx, 0.0).g;
    // Stretched around the channel's median (`sf_atmos::fog::NOISE_MID`, `NOISE_GAIN`).
    let n = (coarse * 0.65 + fine * 0.35 - 0.357) * 6.0 + 0.5;
    return ramp(f.bank_coverage, f.bank_coverage + f.bank_sharpness, n);
}
