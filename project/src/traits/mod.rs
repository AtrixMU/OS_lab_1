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
    fn get_supervisor_flag(self) -> bool;
    fn set_carry_flag(&mut self, value: bool);
    fn set_parity_flag(&mut self, value: bool);
    fn set_auxiliary_carry_flag(&mut self, value: bool);
    fn set_zero_flag(&mut self, value: bool);
    fn set_sign_flag(&mut self, value: bool);
    fn set_trap_flag(&mut self, value: bool);
    fn set_interrupt_flag(&mut self, value: bool);
    fn set_directional_flag(&mut self, value: bool);
    fn set_overflow_flag(&mut self, value: bool);
    fn set_supervisor_flag(&mut self, value: bool);
}