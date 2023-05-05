// Todo: implement access_flags
// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.10.2.2

use rjvm::{jvm::Jvm, prelude::*, Log, LogDone, LogInit};

fn main() {
    // let class = JavaClass::new("java_std/classes/module-info.class");
    LogInit!();
    let mut jvm = Jvm::new();
    jvm.load_base(".");
    println!("Java std Loaded");
    jvm.add_class("Main.class".try_into().unwrap(), "Main");
    jvm.add_class("Main$Statics.class".try_into().unwrap(), "Main$Statics");
    println!("Main loaded!");
    jvm.init("Main").unwrap();
    // println!("{:#?}", jvm.classes.get("Main").unwrap());
    //println!("Logger: {:#?}", rjvm::logger::get_logger());
    jvm.execute("Main", "main", Vec::new()).unwrap();
}
