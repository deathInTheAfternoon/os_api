use std::ffi::CString;
use nix::errno::Errno;
use nix::unistd;
use nix::sched;
use lib_container_runtime;

fn main() {
    println!("Starting the incredible app, today.");
    println!("HERE IT IS {}", lib_container_runtime::add_one(23));

    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed to get hostname");
    let hostname = hostname_cstr.to_str().expect("Invalid hostname (not UTF-8)");
    println!("Hostname {}", hostname);

    let dir = unistd::getcwd().unwrap();
    println!("Current working dir {:?}", dir);

    let egid = unistd::getegid();
    println!("Effective group id of this process: {}", egid);

    let euid = unistd::geteuid();
    println!("Effective user id of this process: {}", euid);

    let user = CString::new("nthakur").unwrap();
    let gid = unistd::Gid::from_raw(123);
    let glist = unistd::getgrouplist(&user, gid);
    println!("Group list for user {:?}", glist);

    println!("Supplementary group IDs of calling process: {:?}", unistd::getgroups());

    let pid = Some(unistd::Pid::this());
    let pgid = unistd::getpgid(pid).expect("Failed to get pgid");
    println!("Group id of calling proc with id {}: {}", pid.unwrap(), pgid);

    println!("Group id of calling proc: {}", unistd::getpgrp());

    println!("Parent pid: {}", unistd::getppid());

    println!("Real, effective, saved group ids of this proc: {:?}", unistd::getresgid());

    println!("Real, effective, saved user ids of this proc: {:?}", unistd::getresuid());

    println!("This thread id: {}", unistd::gettid());

    // Create UTS namespace
    let current_hostname = lib_container_runtime::get_host_name();
    println!("Host hostname {}", current_hostname);

    // How to respond to a specific error, in this case EPERM...
    sched::unshare(sched::CloneFlags::CLONE_NEWUTS).unwrap_or_else(|error| {
        if error == Errno::EPERM {
            panic!("This process doesn't have permissions to call 'unshare()'");
        } else {
            panic!("Error {:?} calling unshare().", error);
        }
    });
    println!("Successfully called unshare(CLONE_NEWUTS)");

    let newhostname = "blahblah";
    unistd::sethostname(newhostname).expect("sethostname() failed.");
    println!("Succesfully changed hostname to {}", newhostname);

    let result = lib_container_runtime::get_host_name();
    println!("Retreived hostname {}", result);
}
