struct Numbers
{
    data: [[stride(4)]] array<u32>;
};


[[group(0), binding(0)]]
var<storage, read_write> numbers: Numbers;

[[stage(compute), workgroup_size(1)]]
fn main()
{
    numbers.data[0] = numbers.data[0] + u32(1);
    numbers.data[1] = numbers.data[1] + u32(1);
    numbers.data[2] = numbers.data[2] + u32(1);
}
