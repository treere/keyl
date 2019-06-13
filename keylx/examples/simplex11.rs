use keylx::hardware::*;

fn main() {
    let disp = XDisplay::new();

    let mut keyboard = XKeyboard::new(disp);


    loop {
        let k = keyboard.next_keycode();
        let s = keyboard.keycode2keysym(k).unwrap();
        let name = keyboard.keysym2str(s);
        println!("{} {} {}", k, s, name);
    }
}
