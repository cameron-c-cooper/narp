use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

use narp::{
    utils::{TOP_TCP, TOP_UDP},
    Target,
};

fn test_main() -> std::io::Result<()> {
    todo!();
    Ok(())
}

#[test]
fn test_target_self() {
    let tpts_arc = Arc::new(TOP_TCP);
    let upts_arc = Arc::new(TOP_UDP);

    let t1 = Target {
        addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        addr_type: (true, false),
        os: None,
        /* must be in method call syntax to be Arc<[u16]> and not Arc<[u16; 1000]> */
        tpts: tpts_arc.clone(),
        upts: Some(upts_arc.clone()),
        tpo: None,
        upo: None,
    };

    let t2 = Target::new_target_self(tpts_arc.clone(), Some(upts_arc.clone()));

    assert_eq!(t1, t2);
}
