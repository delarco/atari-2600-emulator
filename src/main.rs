mod cpu;
use cpu::CPU;

fn main() {
    env_logger::init();

    let cpu = CPU::new();
}
