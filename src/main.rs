use sgx_cpu::SgxCpuInfo;

fn main() {
    println!("SGX CPU information\n");

    let sgx_cpu_info = SgxCpuInfo::new();
    println!("{}", sgx_cpu_info);
}
