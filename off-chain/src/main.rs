use instarand_benchmarking::{
    impls::{BlsVrfHashless, GoldbergVrf},
    types::vrf::Vrf,
};

fn main() {
    generate_test_keys();
}

fn generate_test_keys() {
    let mut keys_bls = Vec::new();
    let mut keys_ddh = Vec::new();
    let mut pretty_out_bls = String::new();
    let mut pretty_out_ddh = String::new();
    for _ in 0..10 {
        let (sk_bls, pk_bls) = BlsVrfHashless::keygen();
        let (sk_ddh, pk_ddh) = GoldbergVrf::keygen();
        pretty_out_bls.push_str(&format!("sk = {:?}, vk = {:?}\n", sk_bls, pk_bls));
        pretty_out_ddh.push_str(&format!("sk = {:?}, vk = {:?}\n", sk_ddh, pk_ddh));
        keys_bls.push((sk_bls, pk_bls));
        keys_ddh.push((sk_ddh, pk_ddh));
    }
    println!("DDH Keys Pretty Out\n");
    println!("{}", pretty_out_ddh);
    println!("DDH Keys Raw Out\n");
    println!("{:?}", keys_ddh);

    println!("BLS Keys Pretty Out\n");
    println!("{}", pretty_out_bls);
    println!("BLS Keys Raw Out\n");
    println!("{:?}", keys_bls);
}
