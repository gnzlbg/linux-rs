//! Raw FFI bindings to the Linux kernel APIs
#![no_std]
#![cfg(target_os = "linux")]

extern crate libc;

pub mod dccp;
pub mod falloc;
pub mod futex;

/*
pub mod genetlink;
pub mod if_;
pub mod if_addr;
pub mod if_alg;;
pub mod if_ether;
pub mod if_tun;
pub mod input;
pub mod magic;
pub mod memfd;
pub mod pub module;
pub mod net_tstamp;
pub mod net_filter_nf_tables;
pub mod netfilter_ipv4;
pub mod netfilter_ipv6;
pub mod netlink;
pub mod quota;
pub mod random;
pub mod reboot;
pub mod rtnetlink;
pub mod seccomp;
pub mod sockios;
*/
