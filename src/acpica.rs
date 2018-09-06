use multiboot;
use core::fmt;
use core::str;
use x86_64::instructions::port::Port;
use ::mem::malloc;

#[allow(non_camel_case_types)]
type ACPI_SIZE = usize;
#[allow(non_camel_case_types)]
type ACPI_PHYSICAL_ADDRESS = usize;
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
#[allow(non_camel_case_types)]
type ACPI_OBJECT_TYPE = u32;
#[allow(non_camel_case_types)]
type ACPI_HANDLE = VOID_PTR;
#[allow(non_camel_case_types)]
type ACPI_BUFFER = AcpiBuffer;

const ACPI_TYPE_ANY: ACPI_OBJECT_TYPE = 0x00;
const ACPI_TYPE_INTEGER: ACPI_OBJECT_TYPE = 0x01;
const ACPI_TYPE_STRING: ACPI_OBJECT_TYPE = 0x02;
const ACPI_TYPE_BUFFER: ACPI_OBJECT_TYPE = 0x03;
const ACPI_TYPE_PACKAGE: ACPI_OBJECT_TYPE = 0x04;
const ACPI_TYPE_FIELD_UNIT: ACPI_OBJECT_TYPE = 0x05;
const ACPI_TYPE_DEVICE: ACPI_OBJECT_TYPE = 0x06;
const ACPI_TYPE_EVENT: ACPI_OBJECT_TYPE = 0x07;
const ACPI_TYPE_METHOD: ACPI_OBJECT_TYPE = 0x08;
const ACPI_TYPE_MUTEX: ACPI_OBJECT_TYPE = 0x09;
const ACPI_TYPE_REGION: ACPI_OBJECT_TYPE = 0x0A;
const ACPI_TYPE_POWER: ACPI_OBJECT_TYPE = 0x0B;
const ACPI_TYPE_PROCESSOR: ACPI_OBJECT_TYPE = 0x0C;
const ACPI_TYPE_THERMAL: ACPI_OBJECT_TYPE = 0x0D;
const ACPI_TYPE_BUFFER_FIELD: ACPI_OBJECT_TYPE = 0x0E;
const ACPI_TYPE_DDB_HANDLE: ACPI_OBJECT_TYPE = 0x0F;
const ACPI_TYPE_DEBUG_OBJECT: ACPI_OBJECT_TYPE = 0x10;

const ACPI_ROOT_OBJECT: ACPI_HANDLE = 0xFFFFFFFF_FFFFFFFFusize;

const AE_OK: ACPI_STATUS = 0;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
#[allow(non_camel_case_types)]
struct AcpiBuffer {
    Length: ACPI_SIZE,
    Pointer: VOID_PTR
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
#[allow(non_camel_case_types)]
struct ACPI_DEVICE_INFO {
    InfoSize: u32,
    Name: ::acpi::Signature4,
    Type: ACPI_OBJECT_TYPE,
    ParamCount: u8,
    Valid: u8,
    Flags: u8,
    HighestDstates: [u8; 4],
    LowestDstates: [u8; 5],
    CurrentStatus: u32,
    Address: u64,
    HardwareId: ACPI_PNP_DEVICE_ID,
    UniqueId: ACPI_PNP_DEVICE_ID,
    SubsystemId: ACPI_PNP_DEVICE_ID,
    CompatibleIdList: ACPI_PNP_DEVICE_ID_LIST
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
#[allow(non_camel_case_types)]
struct ACPI_PNP_DEVICE_ID {
    Length: u32,
    Str: CString
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
#[allow(non_camel_case_types)]
struct ACPI_PNP_DEVICE_ID_LIST {
    Count: u32,
    ListSize: u32,
    Ids: [ACPI_PNP_DEVICE_ID; 1]
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct CString(*const u8);
impl CString {
    pub unsafe fn to_str(&self) -> &str {
        let len = strlen(self.0);
        let sl = core::slice::from_raw_parts(self.0, len);
        str::from_utf8_unchecked(sl)
    }
}

pub struct CStringBuffer(CString, [u8; 256]);
impl CStringBuffer {
    pub fn new() -> CStringBuffer {
        let mut buf = [0u8; 256];
        let cs = CString(&mut buf[0]);
        CStringBuffer(cs, buf)
    }
}

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

    fn AcpiWalkNamespace(Type: ACPI_OBJECT_TYPE, StartObject: ACPI_HANDLE, MaxDepth: u32, DescendingCallback: usize, AscendingCallback: usize, UserContext: VOID_PTR, ReturnValue: &mut VOID_PTR) -> ACPI_STATUS;
    fn AcpiGetName(Object: ACPI_HANDLE, NameType: u32, OutName: &mut ACPI_BUFFER) -> ACPI_STATUS;
    fn AcpiGetHandle(Parent: ACPI_HANDLE, Pathname: *const u8, OutHandle: &mut ACPI_HANDLE) -> ACPI_STATUS;
    fn AcpiGetObjectInfo(Object: ACPI_HANDLE, OutBuffer: &mut&ACPI_DEVICE_INFO) -> ACPI_STATUS;
    fn AcpiEvaluateObject(Object: ACPI_HANDLE, Pathname: *const u8, MethodParams: *const u8, ReturnBuffer: &ACPI_BUFFER) -> ACPI_STATUS;

    fn strlen(data: *const u8) -> usize;
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

        // ACPI_TYPE_DEVICE
        res = AcpiWalkNamespace(ACPI_TYPE_ANY, ACPI_ROOT_OBJECT, 0xFFFFFFFF, walk_callback as usize, walk_callback2 as usize, 0, &mut*(0 as *mut VOID_PTR));
        println!("AcpiWalkNamespace: {:x}", res);

        // \_SB_.PCI0._HID
        let handle = get_handle(None, "\\_SB_.PCI0").expect("this should be good");
        let res = evaluate_object(Some(handle), "_UID").expect("hid");
        println!("res: {:?}", res);
        // let res = evaluate_object(None, "\\_SB_.PCI0").expect("hid");
        // println!("res: {:?}", res);
        // let handle = get_handle(None, "\\_SB_.PCI0._HID").expect("this should be good");
        // let res = get_object_info(handle).expect("all good");
        // println!("res: {:?}", res);
    }
}

#[repr(packed)]
struct CStr([u8; 256]);
impl CStr {
    fn new(s: &str) -> Self {
        let mut c_str = [0u8; 256];
        let len = s.len();
        c_str[..len].clone_from_slice(s.as_bytes());
        c_str[len] = 0;
        CStr(c_str)
    }

    fn as_ptr(&self) -> *const u8 {
        &self.0[0] as *const u8
    }
}

fn get_handle(parent: Option<ACPI_HANDLE>, path: &str) -> Result<ACPI_HANDLE, ACPI_STATUS> {
    let parent_acpi = match parent {
        Some(x) => x,
        None => 0
    };
    let mut out: ACPI_HANDLE = 0;
    let res = unsafe { AcpiGetHandle(parent_acpi, CStr::new(path).as_ptr(), &mut out) };

    match res {
        0 => Ok(out),
        x => Err(res)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct AcpiStrObject {
    ttype: u32,
    value: u64
    // length: u32,
    // ptr: *const u8
}

// https://github.com/spotify/linux/blob/master/drivers/acpi/acpica/exutils.c

fn evaluate_object(parent: Option<ACPI_HANDLE>, path: &str) -> Result<ACPI_BUFFER, ACPI_STATUS> {
    let parent_acpi = match parent {
        Some(x) => x,
        None => 0
    };

    let mut out = ACPI_BUFFER {
        Length: 0xFFFFFFFF_FFFFFFFFusize,
        Pointer: 0
    };
    let res = unsafe { AcpiEvaluateObject(parent_acpi, CStr::new(path).as_ptr(), 0 as *const u8, &mut out) };

    unsafe {
        let asdf = &*(out.Pointer as *const AcpiStrObject);
        // let sss = CString(asdf.ptr);
        // println!("!!!!! {:?} {:x}", out, out.Pointer);
        // println!("!!!!! {:?}", asdf);
        // println!("!!!!! {}", sss.to_str());
        println!("!!!!! {:?}", asdf);
    }

    match res {
        0 => Ok(out),
        x => Err(res)
    }
}

#[derive(Debug)]
struct OwnedDeviceInfo(&'static ACPI_DEVICE_INFO);
impl Drop for OwnedDeviceInfo {
    fn drop(&mut self) {
        unsafe {
            AcpiOsFree(self.0 as *const _ as usize);
        }
    }
}

fn get_object_info(handle: ACPI_HANDLE) -> Result<OwnedDeviceInfo, ACPI_STATUS> {
    let mut info: &ACPI_DEVICE_INFO = unsafe { &*(0 as *mut _) };
    let res = unsafe { AcpiGetObjectInfo(handle, &mut info) };
    match res {
        0 => Ok(OwnedDeviceInfo(info)),
        x => Err(res)
    }
}

extern "C" fn walk_callback(ObjHandle: ACPI_HANDLE, Level: u32, Context: VOID_PTR) -> VOID_PTR {
    if Level != 1 && false {
        return 0
    }

    unsafe {
        let mut buf = [0u8; 256];
        let mut buffer = AcpiBuffer {
            Length: 256,
            Pointer: &mut buf as *mut _ as usize
        };

        AcpiGetName(ObjHandle, 0, &mut buffer);
        let name = str::from_utf8_unchecked(&buf[..strlen(&buf[0])]);

        let info = get_object_info(ObjHandle).expect("cannot get object info");
        println!("Walk: {:x} {} {} {}", ObjHandle, Level, name, info.0.Type);
        0
    }
}

extern "C" fn walk_callback2(ObjHandle: ACPI_HANDLE, Level: u32, Context: VOID_PTR) -> VOID_PTR {
    0
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
    let result = malloc(Size as u64);
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
