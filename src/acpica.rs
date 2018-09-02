use multiboot;
use core::fmt;
use x86_64::instructions::port::Port;

#[allow(non_camel_case_types)]
type ACPI_SIZE = u64;
#[allow(non_camel_case_types)]
type ACPI_PHYSICAL_ADDRESS = u64;
#[allow(non_camel_case_types)]
type ACPI_STATUS = u32;
#[allow(non_camel_case_types)]
type VOID_PTR = usize;
#[allow(non_camel_case_types)]
type ACPI_THREAD_ID = u64;
#[allow(non_camel_case_types)]
type ACPI_SEMAPHORE = usize;
#[allow(non_camel_case_types)]
type ACPI_SPINLOCK = usize;



#[derive(Copy, Clone)]
struct CStringPrinter(usize);
impl fmt::Debug for CStringPrinter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let arr = &*(self.0 as *const [u8; 200]);
            for item in arr.iter() {
                if *item == 0 {
                    break;
                }
                write!(f, "{}", *item as char);
            }
        }
        Ok(())
    }
}

static mut LAST_PTR: usize = 0x10_00000;

pub fn malloc(size: u64) -> usize {
    unsafe {
        let result = LAST_PTR;
        LAST_PTR += size as usize;
        result
    }
}

#[link(name = "acpica")]
extern "C" {
    fn AcpiInitializeSubsystem() -> ACPI_STATUS;
    fn AcpiInitializeTables(InitialTableArray: VOID_PTR, InitialTableCount: u32, AllowResize: bool) -> ACPI_STATUS;
    fn AcpiFindRootPointer(TableAddress: &mut ACPI_SIZE) -> ACPI_STATUS;
    fn AcpiLoadTables() -> ACPI_STATUS;
    fn AcpiEnableSubsystem(Flags: u32) -> ACPI_STATUS;
    fn AcpiInitializeObjects(Flags: u32) -> ACPI_STATUS;

    fn AcpiUpdateAllGpes() -> ACPI_STATUS;
    fn AcpiEnableAllRuntimeGpes() -> ACPI_STATUS;

    fn AcpiEnableEvent(Event: u32, Flags: u32) -> ACPI_STATUS;
    fn AcpiInstallFixedEventHandler(Event: u32, Handler: VOID_PTR, Context: VOID_PTR) -> ACPI_STATUS;
}


pub fn init() {
    unsafe {
        // let mut asdf: u64 = 0;
        // println!("AcpiFindRootPointer: {}", multiboot::get().rsdp_v1_tag().unwrap().signature().unwrap());
        // let result = &(multiboot::get().rsdp_v1_tag().unwrap() as *const _ as usize);
        // let find = AcpiFindRootPointer(&mut asdf);
        // println!("AcpiFindRootPointer: {:x} {:x} {:x}", find, asdf, result);
        let init = AcpiInitializeSubsystem();
        println!("AcpiInitializeSubsystem: {:x}", init);
        let tables = AcpiInitializeTables(0, 0, true);
        println!("AcpiInitializeTables: {:x}", tables);
        let load = AcpiLoadTables();
        println!("AcpiLoadTables: {:x}", load);
        let enable = AcpiEnableSubsystem(0);
        println!("AcpiEnableSubsystem: {:x}", enable);
        let objects = AcpiInitializeObjects(0);
        println!("AcpiInitializeObjects: {:x}", objects);

        let gpeup = AcpiUpdateAllGpes();
        println!("AcpiUpdateAllGpes: {:x}", gpeup);
        let gpeenable = AcpiEnableAllRuntimeGpes();
        println!("AcpiEnableAllRuntimeGpes: {:x}", gpeenable);

        let mut res;
        res = AcpiEnableEvent(2, 0);
        println!("AcpiEnableEvent: 2 - {:x}", res);
        res = AcpiEnableEvent(0, 0);
        println!("AcpiEnableEvent: 0 - {:x}", res);
        res = AcpiEnableEvent(1, 0);
        println!("AcpiEnableEvent: 1 - {:x}", res);
        res = AcpiEnableEvent(4, 0);
        println!("AcpiEnableEvent: 4 - {:x}", res);

        res = AcpiInstallFixedEventHandler(2, handler as VOID_PTR, 0);
        println!("AcpiInstallFixedEventHandler: 2 - {:x}", res);
        res = AcpiEnableEvent(2, 0);
        println!("AcpiEnableEvent: 2 - {:x}", res);
    }
}

fn handler() -> u32 {
    println!("Event!");
    0
}

#[no_mangle]
pub extern "C" fn AcpiOsPrintf() {
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsAllocate(Size: ACPI_SIZE) -> VOID_PTR {
    let result = malloc(Size);
    // println!("AcpiOsAllocate({}) -> {:x}", Size, result);
    result
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsFree(Memory: VOID_PTR) {
    // println!("AcpiOsFree({:x})", Memory);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsMapMemory(Where: VOID_PTR, Length: ACPI_SIZE) -> VOID_PTR {
    println!("AcpiOsMapMemory({:x}, {})", Where, Length);
    return Where;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsUnmapMemory(LogicalAddress: VOID_PTR, Size: ACPI_SIZE) {
    println!("AcpiOsUnmapMemory({:x}, {})", LogicalAddress, Size);
    ()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsGetTimer() {
    println!("AcpiOsGetTimer");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsReadPciConfiguration() {
    println!("AcpiOsReadPciConfiguration");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsWritePciConfiguration() {
    println!("AcpiOsWritePciConfiguration");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsSignalSemaphore(Handle: ACPI_SEMAPHORE, Units: u32) -> ACPI_STATUS {
    // println!("AcpiOsSignalSemaphore({}, {})", Handle, Units);
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsDeleteSemaphore() {
    println!("AcpiOsDeleteSemaphore");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsAcquireLock(Handle: ACPI_SPINLOCK) -> u64 {
    // println!("AcpiOsAcquireLock({})", Handle);
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsReleaseLock(Handle: ACPI_SPINLOCK, Flags: u64) {
    // println!("AcpiOsReleaseLock({}, {})", Handle, Flags);
}

static mut SEMAPHORE_ID: usize = 1;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsCreateSemaphore(MaxUnits: u32, InitialUnits: u32, OutHandle: VOID_PTR) -> ACPI_STATUS {
    let result;
    unsafe {
        let ptr = &mut*(OutHandle as *mut usize);
        result = SEMAPHORE_ID;
        *ptr = result;
        SEMAPHORE_ID += 1;
    }
    println!("AcpiOsCreateSemaphore({}, {}, {:x}) -> {}", MaxUnits, InitialUnits, OutHandle, result);
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsSignal() {
    println!("AcpiOsSignal");
    // TODO
}

static mut LOCK_ID: usize = 1;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsCreateLock(OutHandle: VOID_PTR) -> ACPI_STATUS {
    let result;
    unsafe {
        let ptr = &mut*(OutHandle as *mut usize);
        result = LOCK_ID;
        *ptr = result;
        LOCK_ID += 1;
    }
    println!("AcpiOsCreateLock() -> {}", result);
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsGetThreadId() -> ACPI_THREAD_ID {
    // println!("AcpiOsGetThreadId() -> 1");
    1
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsDeleteLock() {
    println!("AcpiOsDeleteLock");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsReadPort(Address: u64, Value: &mut u32, Width: u32) -> ACPI_STATUS {
    *Value = match Width {
        8 => unsafe { Port::<u8>::new(Address as u16).read() as u32 }
        16 => unsafe { Port::<u16>::new(Address as u16).read() as u32 }
        32 => unsafe { Port::<u32>::new(Address as u16).read() }
        _ => panic!()
    };

    println!("AcpiOsReadPort(0x{:x}, ..., {}) -> 0x{:x}", Address as u16, Width, *Value);
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsWritePort(Address: u64, Value: u32, Width: u32) -> ACPI_STATUS {
    match Width {
        8 => unsafe { Port::new(Address as u16).write(Value as u8); }
        16 => unsafe { Port::new(Address as u16).write(Value as u16); }
        32 => unsafe { Port::new(Address as u16).write(Value); }
        _ => panic!()
    };

    println!("AcpiOsWritePort(0x{:x}, 0x{:x}, {})", Address as u16, Value, Width);
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsTableOverride(_ExistingTable: VOID_PTR, NewTable: &mut VOID_PTR) -> ACPI_STATUS {
    println!("AcpiOsTableOverride()");
    *NewTable = 0;
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsPhysicalTableOverride(_ExistingTable: VOID_PTR, NewTable: &mut VOID_PTR, NewTableLength: &mut u32) -> ACPI_STATUS {
    println!("AcpiOsPhysicalTableOverride()");
    *NewTable = 0;
    *NewTableLength = 0;
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsStall() {
    println!("AcpiOsStall");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsExecute() {
    println!("AcpiOsExecute");
    panic!();
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsGetRootPointer() -> ACPI_PHYSICAL_ADDRESS {
    // TODO: figure out why multiboot is not right
    // let result = multiboot::get().rsdp_v1_tag().unwrap() as *const multiboot2::RsdpV1Tag as u64;
    let mut result = 0;
    unsafe {
        AcpiFindRootPointer(&mut result);
    }
    println!("AcpiOsGetRootPointer() -> {:x}", result);
    result
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsVprintf(Format: VOID_PTR, _Args: VOID_PTR) {
    println!("AcpiOsVprintf({:?})", CStringPrinter(Format));
    // panic!();
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsWaitSemaphore(Handle: ACPI_SEMAPHORE, Units: u32, Timeout: u16) -> ACPI_STATUS {
    // println!("AcpiOsWaitSemaphore({}, {}, {})", Handle, Units, Timeout);
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsWaitEventsComplete() {
    println!("AcpiOsWaitEventsComplete");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsInstallInterruptHandler(InterruptLevel: u32, Handler: usize, Context: VOID_PTR) -> ACPI_STATUS {
    println!("AcpiOsInstallInterruptHandler(0x{:x})", InterruptLevel);
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsRemoveInterruptHandler() {
    println!("AcpiOsRemoveInterruptHandler");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsInitialize() -> ACPI_STATUS {
    println!("AcpiOsInitialize");
    0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsSleep() {
    println!("AcpiOsSleep");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsReadMemory() {
    println!("AcpiOsReadMemory");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsWriteMemory() {
    println!("AcpiOsWriteMemory");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsTerminate() {
    println!("AcpiOsTerminate");
    // TODO
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn AcpiOsPredefinedOverride(_PredefinedObject: VOID_PTR, NewValue: VOID_PTR) -> ACPI_STATUS {
    println!("AcpiOsPredefinedOverride()");
    unsafe {
        let ptr = &mut*(NewValue as *mut usize);
        *ptr = 0;
    }
    0
}
