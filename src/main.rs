use raw_cpuid::{CpuId, SgxSectionInfo};

fn main() {
    println!("SGX CPU information");
    let cpuid = CpuId::new();

    println!("\n# CPU feature information");
    let feature_info = cpuid
        .get_feature_info()
        .expect("Cannot get feature information");
    println!("{:#02x?}", feature_info);
    println!("stepping {}", feature_info.stepping_id());
    println!("model {}", feature_info.model_id());
    println!("family {}", feature_info.family_id());
    // println!("processor type {}", feature_info.processor_id());
    println!("extended model {}", feature_info.extended_model_id());
    println!("extended family {}", feature_info.extended_family_id());
    println!("smx support {}", feature_info.has_smx());

    println!("\n# Extended feature bits");
    let extended_features = cpuid
        .get_extended_feature_info()
        .expect("Cannot get extended features information");
    println!("{:#02x?}", extended_features);
    println!("sgx available: {}", extended_features.has_sgx());
    println!("sgx launch control: {}", extended_features.has_sgx_lc());

    println!("\n# Intel SGX capabilities");
    println!("\n## Sub-leaf 0 (ECX=0)");
    let sgx_info = cpuid.get_sgx_info().expect("Cannot get SGX information");
    println!("{:#02x?}", sgx_info);
    println!("sgx 1 supported: {}", sgx_info.has_sgx1());
    println!("sgx 2 supported: {}", sgx_info.has_sgx2());
    println!(
        "MaxEnclaveSize_Not64: {:#02x}",
        sgx_info.max_enclave_size_non_64bit()
    );
    println!(
        "MaxEnclaveSize_64: {:#02x}",
        sgx_info.max_enclave_size_64bit()
    );

    println!("\n## Sub-leaf 1 (ECX=1)");
    let (eax, ecx) = sgx_info.secs_attributes();
    println!("eax: {:#02x?}, ebx: 0, ecx: {:#02x?}, edx: 0", eax, ecx);

    for (idx, SgxSectionInfo::Epc(epc_section)) in sgx_info.iter().enumerate() {
        println!("Sub-leaf {} (ECX={})", idx + 2, idx + 2);
        println!("EPC (Enclave Page Cache) section: {:#02x?}", epc_section);
        println!(
            "physical base address: {:#02x?}",
            epc_section.physical_base()
        );
        println!(
            "size of EPC section in Processor Reserved Memory: {} MB",
            epc_section.size() / 1_048_576
        );
    }
}
