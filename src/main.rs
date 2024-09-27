/* On désactive la librairie standard pour pouvoir éxécuter le code
sur du bare_metal sans la librairie standard */
#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga_buffer;

/* Obligé de définir un panic handler (Une fonction appelée quand une erreur critique survient)
car il est normalement défini dans le standard */
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

/* By using the #[no_mangle] attribute, we disable name mangling to ensure that the Rust compiler really outputs a function with the name _start. Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE */
#[no_mangle]
/* _start est le point d'entrée par défaut de la plupart des systèmes*/
pub extern "C" fn _start() -> ! {
    /*
    let vga_buffer = 0xb8000 as *mut u8; //Cast d'une valeur en un pointeur

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; //L'octet représentant le caractère
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; //L'octet représentant la couleur
        }
    }
    */
    println!("Hello World {}", "!");

    loop {}
}
