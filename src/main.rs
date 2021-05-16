use sgx_cpu::SgxCpuInfo;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(short, long)]
    verbose: bool,
}

fn main() {
    println!("SGX CPU information\n");
    let opts = Opts::from_args();

    let sgx_cpu_info = SgxCpuInfo::new();
    if opts.verbose {
        println!("{:?}", sgx_cpu_info);
    } else {
        println!("{}", sgx_cpu_info);
    }
}
