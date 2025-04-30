#[derive(Debug)]
enum VersionIp {
    V4,
    V6,
}

fn ip_version(ip_version: &VersionIp) -> String{

   let your_ip =  match ip_version {
        VersionIp::V4 => 1,
        VersionIp::V6 => 2,
    };

     if your_ip == 1 {
       "Your loopback is 127.0.0.1".to_string()
     }else {
        "Your loopback is :1".to_string()
     }
}
     

fn main() {
   println!("{:#?}",ip_version(&VersionIp::V6));
}
