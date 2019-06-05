#![deny(warnings)]

extern crate ctest;

use std::env;

fn main() {
    match &env::var("TARGET").unwrap() {
        // t if t.contains("android") => return test_android(t),
        // t if t.contains("emscripten") => return test_emscripten(t),
        t if t.contains("linux") => return test_linux(t),
        t => panic!("unknown target {}", t),
    }
}

macro_rules! headers {
    ($cfg:ident: [$m:expr]: $header:literal) => {
        if $m {
            $cfg.header($header);
        }
    };
    ($cfg:ident: $header:literal) => {
        $cfg.header($header);
    };
    ($($cfg:ident: $([$c:expr]:)* $header:literal,)*) => {
        $(headers!($cfg: $([$c]:)* $header);)*
    };
    ($cfg:ident: $( $([$c:expr]:)* $header:literal,)*) => {
        headers!($($cfg: $([$c]:)* $header,)*);
    };
    ($cfg:ident: $( $([$c:expr]:)* $header:literal),*) => {
        headers!($($cfg: $([$c]:)* $header,)*);
    };
}

fn test_linux(target: &str) {
    assert!(target.contains("linux"));

    let arm = target.contains("arm");
    let x86_64 = target.contains("x86_64");
    let x86_32 = target.contains("i686");
    let x32 = target.contains("x32");
    let mips = target.contains("mips");
    let sparc64 = target.contains("sparc64");

    let mut cfg = ctest::TestGenerator::new();

    headers! {
        cfg:
        // "asm/mman.h",
        // "asm/termbits.h",
        "linux/dccp.h",
        "linux/falloc.h",
        "linux/fcntl.h"
        "linux/fs.h",
        "linux/futex.h",
        "linux/genetlink.h",
        "linux/if.h",
        "linux/if_addr.h",
        "linux/if_alg.h",
        "linux/if_ether.h",
        "linux/if_tun.h",
        "linux/in6.h",
        "linux/input.h",
        "linux/magic.h",
        "linux/memfd.h",
        "linux/module.h",
        "linux/net_tstamp.h",
        "linux/netfilter/nf_tables.h",
        "linux/netfilter_ipv4.h",
        "linux/netfilter_ipv6.h",
        "linux/netlink.h",
        "linux/quota.h",
        "linux/random.h",
        "linux/reboot.h",
        "linux/rtnetlink.h",
        "linux/seccomp.h",
        "linux/sockios.h",
    }

    cfg.skip_const(move |name| {
        match name {
            // Require Linux kernel 5.1:
            "F_SEAL_FUTURE_WRITE" => true,

            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");

    test_linux_like_apis(target);
}
