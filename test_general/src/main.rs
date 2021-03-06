mod casting;
mod channels;
mod enums;
mod greet;
mod largest;
mod next_id_1;
mod next_id_2;
mod option;
mod reverse;
mod vectors;

fn main() {
    greet::run();
    next_id_1::run();
    next_id_2::run();
    reverse::run();
    enums::run();
    casting::run();
    option::run();
    vectors::run();
    largest::run();

    println!("channels\n");
    channels::run();
}
