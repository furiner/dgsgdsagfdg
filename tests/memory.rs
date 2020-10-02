use akusesu::memory;
use akusesu::memory::structures::Pointer;

#[test]
fn get_process() {
    let pid = memory::pid("notepad.exe");

    assert_ne!(0, pid);
}

#[test]
fn pointer_structure() {
    let mut ptr = Pointer {
        address: 0x0,
        offsets: vec![0],
        .. Pointer::default()
    };

    assert_eq!(0x0, ptr.get_address()); // address assertion
    assert_eq!(0, ptr.get_offset(0)); // offset assertion

    // add offset
    ptr <<= 1;

    assert_eq!(1, ptr.get_offset(1)); // assert new offset exists and is 1

    // add another offset, and add to the addend
    ptr += 1;
    ptr <<= 1;

    assert_eq!(0x3, ptr.get());

    // delete 1
    ptr -= 1;
    assert_eq!(0x2, ptr.get());
}