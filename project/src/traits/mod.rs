pub trait Processor {
    fn get_carry_flag(self) -> bool;
    fn get_parity_flag(self) -> bool;
    fn get_auxiliary_carry_flag(self) -> bool;
    fn get_zero_flag(self) -> bool;
    fn get_sign_flag(self) -> bool;
    fn get_trap_flag(self) -> bool;
    fn get_interrupt_flag(self) -> bool;
    fn get_directional_flag(self) -> bool;
    fn get_overflow_flag(self) -> bool;
    fn get_supervisor_flag(&self) -> bool;
}