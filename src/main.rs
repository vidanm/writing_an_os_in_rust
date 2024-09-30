/* On désactive la librairie standard pour pouvoir éxécuter le code
sur du bare_metal sans la librairie standard */
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)] // Nécessaire car le module test est défini dans le standard
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"] //Le test runner va chercher à s'éxecuter dans un main
                                             //qu'il va générer hors on à spécifié ne pas vouloir de                                              //main, il faut donc lui dire que le
                                             //nouveau nom pour la fonction qui va s'occuper des
                                             //tests est "test_main". On l'appelle ensuite dans
                                             //_start (notre entrypoint)
use core::panic::PanicInfo;
use rust_os::println;

/* Obligé de définir un panic handler (Une fonction appelée quand une erreur critique survient)
car il est normalement défini dans le standard */

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    /* PanicInfo contient le fichier et la ligne qui à fait survenir l'erreur critique
    La fonction return ! qui signfie en fait que c'est une diverging function (qui ne doit jamais
    rien retourner)
    Par exemple le mot clé "continue" ne retourne rien ce qui permets son utilisation dans des
    contextes de pattern matching sans faire panic le programme
    https://doc.rust-lang.org/rust-by-example/fn/diverging.html */
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

/* By using the #[no_mangle] attribute, we disable name mangling to ensure that the Rust compiler really outputs a function with the name _start. Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE */
#[no_mangle]
/* _start est le point d'entrée par défaut de la plupart des systèmes*/
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
