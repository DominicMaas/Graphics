// https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39

fn rand(n: f32) -> f32 { 
    return fract(sin(n) * 43758.5453123); 
}

fn rand2(n: vec2<f32>) -> f32 {
    return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453);
}

fn mod289(x: vec4<f32>) -> vec4<f32> { 
    return x - floor(x * (1. / 289.)) * 289.; 
}

fn perm4(x: vec4<f32>) -> vec4<f32> { 
    return mod289(((x * 34.) + 1.) * x); 
}

fn permute4(x: vec4<f32>) -> vec4<f32> { 
    return ((x * 34. + 1.) * x) % vec4<f32>(289.); 
}

fn fade2(t: vec2<f32>) -> vec2<f32> { 
    return t * t * t * (t * (t * 6. - 15.) + 10.); 
}

fn noise(p: f32) -> f32 {
    let fl = floor(p);
    let fc = fract(p);
    return mix(rand(fl), rand(fl + 1.), fc);
}

fn noise2(n: vec2<f32>) -> f32 {
    let d = vec2<f32>(0., 1.);
    let b = floor(n);
    let f = smoothStep(vec2<f32>(0.), vec2<f32>(1.), fract(n));
    return mix(mix(rand2(b), rand2(b + d.yx), f.x), mix(rand2(b + d.xy), rand2(b + d.yy), f.x), f.y);
}

fn noise3(p: vec3<f32>) -> f32 {
    let a = floor(p);
    var d: vec3<f32> = p - a;
    d = d * d * (3. - 2. * d);

    let b = a.xxyy + vec4<f32>(0., 1., 0., 1.);
    let k1 = perm4(b.xyxy);
    let k2 = perm4(k1.xyxy + b.zzww);

    let c = k2 + a.zzzz;
    let k3 = perm4(c);
    let k4 = perm4(c + 1.);

    let o1 = fract(k3 * (1. / 41.));
    let o2 = fract(k4 * (1. / 41.));

    let o3 = o2 * d.z + o1 * (1. - d.z);
    let o4 = o3.yw * d.x + o3.xz * (1. - d.x);

    return o4.y * d.y + o4.x * (1. - d.y);
}

fn perlinNoise2(P: vec2<f32>) -> f32 {
    var Pi: vec4<f32> = floor(P.xyxy) + vec4<f32>(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4<f32>(0., 0., 1., 1.);
    Pi = Pi % vec4<f32>(289.); // To avoid truncation effects in permutation
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4<f32> = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2<f32> = vec2<f32>(gx.x, gy.x);
    var g10: vec2<f32> = vec2<f32>(gx.y, gy.y);
    var g01: vec2<f32> = vec2<f32>(gx.z, gy.z);
    var g11: vec2<f32> = vec2<f32>(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 *
        vec4<f32>(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;

    let n00 = dot(g00, vec2<f32>(fx.x, fy.x));
    let n10 = dot(g10, vec2<f32>(fx.y, fy.y));
    let n01 = dot(g01, vec2<f32>(fx.z, fy.z));
    let n11 = dot(g11, vec2<f32>(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2<f32>(n00, n01), vec2<f32>(n10, n11), vec2<f32>(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}
