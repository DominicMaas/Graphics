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

fn taylorInvSqrt4(r: vec4<f32>) -> vec4<f32> { 
    return 1.79284291400159 - 0.85373472095314 * r; 
}

fn fade2(t: vec2<f32>) -> vec2<f32> { 
    return t * t * t * (t * (t * 6. - 15.) + 10.); 
}

fn fade3(t: vec3<f32>) -> vec3<f32> { 
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

fn perlinNoise3(P: vec3<f32>) -> f32 {
    var Pi0 : vec3<f32> = floor(P); // Integer part for indexing
    var Pi1 : vec3<f32> = Pi0 + vec3<f32>(1.); // Integer part + 1
    Pi0 = Pi0 % vec3<f32>(289.);
    Pi1 = Pi1 % vec3<f32>(289.);
    let Pf0 = fract(P); // Fractional part for interpolation
    let Pf1 = Pf0 - vec3<f32>(1.); // Fractional part - 1.
    let ix = vec4<f32>(Pi0.x, Pi1.x, Pi0.x, Pi1.x);
    let iy = vec4<f32>(Pi0.yy, Pi1.yy);
    let iz0 = Pi0.zzzz;
    let iz1 = Pi1.zzzz;

    let ixy = permute4(permute4(ix) + iy);
    let ixy0 = permute4(ixy + iz0);
    let ixy1 = permute4(ixy + iz1);

    var gx0: vec4<f32> = ixy0 / 7.;
    var gy0: vec4<f32> = fract(floor(gx0) / 7.) - 0.5;
    gx0 = fract(gx0);
    var gz0: vec4<f32> = vec4<f32>(0.5) - abs(gx0) - abs(gy0);
    var sz0: vec4<f32> = step(gz0, vec4<f32>(0.));
    gx0 = gx0 + sz0 * (step(vec4<f32>(0.), gx0) - 0.5);
    gy0 = gy0 + sz0 * (step(vec4<f32>(0.), gy0) - 0.5);

    var gx1: vec4<f32> = ixy1 / 7.;
    var gy1: vec4<f32> = fract(floor(gx1) / 7.) - 0.5;
    gx1 = fract(gx1);
    var gz1: vec4<f32> = vec4<f32>(0.5) - abs(gx1) - abs(gy1);
    var sz1: vec4<f32> = step(gz1, vec4<f32>(0.));
    gx1 = gx1 - sz1 * (step(vec4<f32>(0.), gx1) - 0.5);
    gy1 = gy1 - sz1 * (step(vec4<f32>(0.), gy1) - 0.5);

    var g000: vec3<f32> = vec3<f32>(gx0.x, gy0.x, gz0.x);
    var g100: vec3<f32> = vec3<f32>(gx0.y, gy0.y, gz0.y);
    var g010: vec3<f32> = vec3<f32>(gx0.z, gy0.z, gz0.z);
    var g110: vec3<f32> = vec3<f32>(gx0.w, gy0.w, gz0.w);
    var g001: vec3<f32> = vec3<f32>(gx1.x, gy1.x, gz1.x);
    var g101: vec3<f32> = vec3<f32>(gx1.y, gy1.y, gz1.y);
    var g011: vec3<f32> = vec3<f32>(gx1.z, gy1.z, gz1.z);
    var g111: vec3<f32> = vec3<f32>(gx1.w, gy1.w, gz1.w);

    let norm0 = taylorInvSqrt4(
        vec4<f32>(dot(g000, g000), dot(g010, g010), dot(g100, g100), dot(g110, g110)));
    g000 = g000 * norm0.x;
    g010 = g010 * norm0.y;
    g100 = g100 * norm0.z;
    g110 = g110 * norm0.w;
    let norm1 = taylorInvSqrt4(
        vec4<f32>(dot(g001, g001), dot(g011, g011), dot(g101, g101), dot(g111, g111)));
    g001 = g001 * norm1.x;
    g011 = g011 * norm1.y;
    g101 = g101 * norm1.z;
    g111 = g111 * norm1.w;

    let n000 = dot(g000, Pf0);
    let n100 = dot(g100, vec3<f32>(Pf1.x, Pf0.yz));
    let n010 = dot(g010, vec3<f32>(Pf0.x, Pf1.y, Pf0.z));
    let n110 = dot(g110, vec3<f32>(Pf1.xy, Pf0.z));
    let n001 = dot(g001, vec3<f32>(Pf0.xy, Pf1.z));
    let n101 = dot(g101, vec3<f32>(Pf1.x, Pf0.y, Pf1.z));
    let n011 = dot(g011, vec3<f32>(Pf0.x, Pf1.yz));
    let n111 = dot(g111, Pf1);

    var fade_xyz: vec3<f32> = fade3(Pf0);
    let temp = vec4<f32>(f32(fade_xyz.z)); // simplify after chrome bug fix
    let n_z = mix(vec4<f32>(n000, n100, n010, n110), vec4<f32>(n001, n101, n011, n111), temp);
    let n_yz = mix(n_z.xy, n_z.zw, vec2<f32>(f32(fade_xyz.y))); // simplify after chrome bug fix
    let n_xyz = mix(n_yz.x, n_yz.y, fade_xyz.x);
    return 2.2 * n_xyz;
}

fn simplexNoise3(v: vec3<f32>) -> f32 {
    let C = vec2<f32>(1. / 6., 1. / 3.);
    let D = vec4<f32>(0., 0.5, 1., 2.);

    // First corner
    var i: vec3<f32>  = floor(v + dot(v, C.yyy));
    let x0 = v - i + dot(i, C.xxx);

    // Other corners
    let g = step(x0.yzx, x0.xyz);
    let l = 1.0 - g;
    let i1 = min(g.xyz, l.zxy);
    let i2 = max(g.xyz, l.zxy);

    // x0 = x0 - 0. + 0. * C
    let x1 = x0 - i1 + 1. * C.xxx;
    let x2 = x0 - i2 + 2. * C.xxx;
    let x3 = x0 - 1. + 3. * C.xxx;

    // Permutations
    i = i % vec3<f32>(289.);
    let p = permute4(permute4(permute4(
        i.z + vec4<f32>(0., i1.z, i2.z, 1. )) +
        i.y + vec4<f32>(0., i1.y, i2.y, 1. )) +
        i.x + vec4<f32>(0., i1.x, i2.x, 1. ));

    // Gradients (NxN points uniformly over a square, mapped onto an octahedron.)
    var n_: f32 = 1. / 7.; // N=7
    let ns = n_ * D.wyz - D.xzx;

    let j = p - 49. * floor(p * ns.z * ns.z); // mod(p, N*N)

    let x_ = floor(j * ns.z);
    let y_ = floor(j - 7.0 * x_); // mod(j, N)

    let x = x_ *ns.x + ns.yyyy;
    let y = y_ *ns.x + ns.yyyy;
    let h = 1.0 - abs(x) - abs(y);

    let b0 = vec4<f32>( x.xy, y.xy );
    let b1 = vec4<f32>( x.zw, y.zw );

    let s0 = floor(b0)*2.0 + 1.0;
    let s1 = floor(b1)*2.0 + 1.0;
    let sh = -step(h, vec4<f32>(0.));

    let a0 = b0.xzyw + s0.xzyw*sh.xxyy ;
    let a1 = b1.xzyw + s1.xzyw*sh.zzww ;

    var p0: vec3<f32> = vec3<f32>(a0.xy, h.x);
    var p1: vec3<f32> = vec3<f32>(a0.zw, h.y);
    var p2: vec3<f32> = vec3<f32>(a1.xy, h.z);
    var p3: vec3<f32> = vec3<f32>(a1.zw, h.w);

    // Normalise gradients
    let norm = taylorInvSqrt4(vec4<f32>(dot(p0,p0), dot(p1,p1), dot(p2,p2), dot(p3,p3)));
    p0 = p0 * norm.x;
    p1 = p1 * norm.y;
    p2 = p2 * norm.z;
    p3 = p3 * norm.w;

    // Mix final noise value
    var m: vec4<f32> = 0.6 - vec4<f32>(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3));
    m = max(m, vec4<f32>(0.));
    m = m * m;
    return 42. * dot(m * m, vec4<f32>(dot(p0,x0), dot(p1,x1), dot(p2,x2), dot(p3,x3)));
}

fn noise3_func(position: vec3<f32>, octaves: i32, frequency: f32, persistence: f32) -> f32 {
    var total = 0.0;
	var max_amplitude = 0.0;
    var amplitude = 1.0;
    var frequency = frequency;
    
    for (var i: i32 = 0; i < octaves; i = i + 1) {
		total = total + noise3(position * frequency) * amplitude;
		frequency = frequency * 2.0;
		max_amplitude = max_amplitude * amplitude;
		amplitude = amplitude * persistence;
	}
    
	return total / max_amplitude;
}
